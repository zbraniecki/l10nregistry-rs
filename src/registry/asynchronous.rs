use std::{
    pin::Pin,
    task::{Context, Poll},
};

use super::{L10nRegistry, L10nRegistryLocked};
use crate::solver::{AsyncTester, ParallelProblemSolver};
use crate::{
    env::ErrorReporter,
    errors::L10nRegistryError,
    fluent::{FluentBundle, FluentError},
    source::{ResourceOption, ResourceStatus},
};

use fluent_fallback::generator::BundleStream;
use futures::{
    stream::{Collect, FuturesOrdered},
    Stream, StreamExt,
};
use std::future::Future;
use unic_langid::LanguageIdentifier;

impl<'a> L10nRegistryLocked<'a> {}

impl<P> L10nRegistry<P>
where
    P: Clone,
{
    pub fn generate_bundles_for_lang(
        &self,
        langid: LanguageIdentifier,
        resource_ids: Vec<String>,
    ) -> GenerateBundles<P> {
        let lang_ids = vec![langid];

        GenerateBundles::new(self.clone(), lang_ids.into_iter(), resource_ids)
    }

    pub fn generate_bundles(
        &self,
        locales: std::vec::IntoIter<LanguageIdentifier>,
        resource_ids: Vec<String>,
    ) -> GenerateBundles<P> {
        GenerateBundles::new(self.clone(), locales, resource_ids)
    }
}

enum State<P> {
    Empty,
    Locale(LanguageIdentifier),
    Solver {
        locale: LanguageIdentifier,
        solver: ParallelProblemSolver<GenerateBundles<P>>,
    },
}

impl<P> Default for State<P> {
    fn default() -> Self {
        Self::Empty
    }
}

impl<P> State<P> {
    fn get_locale(&self) -> &LanguageIdentifier {
        match self {
            Self::Locale(locale) => locale,
            Self::Solver { locale, .. } => locale,
            Self::Empty => unreachable!(),
        }
    }

    fn take_solver(&mut self) -> ParallelProblemSolver<GenerateBundles<P>> {
        replace_with::replace_with_or_default_and_return(self, |self_| match self_ {
            Self::Solver { locale, solver } => (solver, Self::Locale(locale)),
            _ => unreachable!(),
        })
    }

    fn put_back_solver(&mut self, solver: ParallelProblemSolver<GenerateBundles<P>>) {
        replace_with::replace_with_or_default(self, |self_| match self_ {
            Self::Locale(locale) => Self::Solver { locale, solver },
            _ => unreachable!(),
        })
    }
}

pub struct GenerateBundles<P> {
    reg: L10nRegistry<P>,
    locales: std::vec::IntoIter<LanguageIdentifier>,
    res_ids: Vec<String>,
    state: State<P>,
}

impl<P> GenerateBundles<P> {
    fn new(
        reg: L10nRegistry<P>,
        locales: std::vec::IntoIter<LanguageIdentifier>,
        res_ids: Vec<String>,
    ) -> Self {
        Self {
            reg,
            locales,
            res_ids,
            state: State::Empty,
        }
    }
}

pub type ResourceSetStream = Collect<FuturesOrdered<ResourceStatus>, Vec<ResourceOption>>;
pub struct TestResult(ResourceSetStream);
impl std::marker::Unpin for TestResult {}

impl Future for TestResult {
    type Output = Vec<bool>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let pinned = Pin::new(&mut self.0);
        pinned
            .poll(cx)
            .map(|set| set.iter().map(|c| c.is_some()).collect())
    }
}

impl<'l, P> AsyncTester for GenerateBundles<P> {
    type Result = TestResult;

    fn test_async(&self, query: Vec<(usize, usize)>) -> Self::Result {
        let locale = self.state.get_locale();
        let lock = self.reg.lock();

        let stream = query
            .iter()
            .map(|(res_idx, source_idx)| {
                let res = &self.res_ids[*res_idx];
                lock.source_idx(0 /*TODO*/, *source_idx)
                    .fetch_file(locale, res)
            })
            .collect::<FuturesOrdered<_>>();
        TestResult(stream.collect())
    }
}

#[async_trait::async_trait(?Send)]
impl<P> BundleStream for GenerateBundles<P> {
    async fn prefetch_async(&mut self) {
        todo!();
    }
}

impl<P> Stream for GenerateBundles<P>
where
    P: ErrorReporter,
{
    type Item = Result<FluentBundle, (FluentBundle, Vec<FluentError>)>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            if let State::Solver { .. } = self.state {
                let mut solver = self.state.take_solver();
                let pinned_solver = Pin::new(&mut solver);
                match pinned_solver.try_poll_next(cx, &self, false) {
                    std::task::Poll::Ready(order) => match order {
                        Ok(Some(order)) => {
                            let locale = self.state.get_locale();
                            let bundle = self.reg.lock().bundle_from_order(
                                0, /* TODO */
                                locale.clone(),
                                &order,
                                &self.res_ids,
                                &self.reg.shared.provider,
                            );
                            self.state.put_back_solver(solver);
                            if bundle.is_some() {
                                return bundle.into();
                            } else {
                                continue;
                            }
                        }
                        Ok(None) => {
                            self.state = State::Empty;
                            continue;
                        }
                        Err(idx) => {
                            self.reg.shared.provider.report_errors(vec![
                                L10nRegistryError::MissingResource {
                                    locale: self.state.get_locale().clone(),
                                    res_id: self.res_ids[idx].clone(),
                                },
                            ]);
                            self.state = State::Empty;
                            continue;
                        }
                    },
                    std::task::Poll::Pending => {
                        self.state.put_back_solver(solver);
                        return std::task::Poll::Pending;
                    }
                }
            } else if let Some(locale) = self.locales.next() {
                let solver = ParallelProblemSolver::new(
                    self.res_ids.len(),
                    self.reg.lock().len(0 /* TODO */),
                );
                self.state = State::Solver { locale, solver };
            } else {
                return None.into();
            }
        }
    }
}
