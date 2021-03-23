mod asynchronous;
mod synchronous;

use std::{
    cell::{Ref, RefCell},
    collections::HashSet,
    rc::Rc,
};

use crate::errors::L10nRegistrySetupError;
use crate::source::FileSource;

use crate::env::ErrorReporter;
use crate::fluent::FluentBundle;
use fluent_bundle::FluentResource;
use fluent_fallback::generator::BundleGenerator;
use unic_langid::LanguageIdentifier;

pub use asynchronous::GenerateBundles;
pub use synchronous::GenerateBundlesSync;

pub type FluentResourceSet = Vec<Rc<FluentResource>>;

#[derive(Default)]
struct Shared<P> {
    sources: RefCell<Vec<Vec<FileSource>>>,
    provider: P,
    adapt_bundle: Option<fn(&mut FluentBundle)>,
}

pub struct L10nRegistryLocked<'a> {
    lock: Ref<'a, Vec<Vec<FileSource>>>,
    adapt_bundle: Option<fn(&mut FluentBundle)>,
}

impl<'a> L10nRegistryLocked<'a> {
    pub fn iter(&self, metasource: usize) -> impl Iterator<Item = &FileSource> {
        self.lock[metasource].iter()
    }

    pub fn metasources_len(&self) -> usize {
        self.lock.len()
    }

    pub fn len(&self, metasource: usize) -> usize {
        self.lock[metasource].len()
    }

    pub fn is_empty(&self, metasource: usize) -> bool {
        self.len(metasource) == 0
    }

    pub fn source_idx(&self, metasource: usize, index: usize) -> &FileSource {
        let source_idx = self.len(metasource) - 1 - index;
        self.lock[metasource]
            .get(source_idx)
            .expect("Index out-of-range")
    }

    pub fn get_source(&self, metasource: usize, name: &str) -> Option<&FileSource> {
        self.lock[metasource]
            .iter()
            .find(|&source| source.name == name)
    }

    pub fn generate_sources_for_file<'l>(
        &'l self,
        metasource: usize,
        langid: &'l LanguageIdentifier,
        res_id: &'l str,
    ) -> impl Iterator<Item = &FileSource> {
        self.iter(metasource)
            .filter(move |source| source.has_file(langid, res_id) != Some(false))
    }
}

#[derive(Clone)]
pub struct L10nRegistry<P> {
    shared: Rc<Shared<P>>,
}

impl<P> L10nRegistry<P> {
    pub fn with_provider(provider: P) -> Self {
        Self {
            shared: Rc::new(Shared {
                sources: Default::default(),
                provider,
                adapt_bundle: None,
            }),
        }
    }

    pub fn set_adapt_bundle(
        &mut self,
        adapt_bundle: fn(&mut FluentBundle),
    ) -> Result<(), L10nRegistrySetupError> {
        let shared = Rc::get_mut(&mut self.shared).ok_or(L10nRegistrySetupError::RegistryLocked)?;
        shared.adapt_bundle = Some(adapt_bundle);
        Ok(())
    }

    pub fn lock(&self) -> L10nRegistryLocked<'_> {
        L10nRegistryLocked {
            lock: self.shared.sources.borrow(),
            adapt_bundle: self.shared.adapt_bundle,
        }
    }

    pub fn register_sources(
        &self,
        new_sources: Vec<FileSource>,
    ) -> Result<(), L10nRegistrySetupError> {
        let mut sources = self
            .shared
            .sources
            .try_borrow_mut()
            .map_err(|_| L10nRegistrySetupError::RegistryLocked)?;

        for new_source in new_sources {
            if let Some(metasource) = sources
                .iter_mut()
                .find(|source| source[0].metasource == new_source.metasource)
            {
                metasource.push(new_source);
            } else {
                sources.push(vec![new_source]);
            }
        }
        Ok(())
    }

    pub fn update_sources(
        &self,
        upd_sources: Vec<FileSource>,
    ) -> Result<(), L10nRegistrySetupError> {
        let mut sources = self
            .shared
            .sources
            .try_borrow_mut()
            .map_err(|_| L10nRegistrySetupError::RegistryLocked)?;

        for upd_source in upd_sources {
            if let Some(metasource) = sources
                .iter_mut()
                .find(|source| source[0].metasource == upd_source.metasource)
            {
                if let Some(idx) = metasource.iter().position(|source| *source == upd_source) {
                    *metasource.get_mut(idx).unwrap() = upd_source;
                } else {
                    return Err(L10nRegistrySetupError::MissingSource {
                        name: upd_source.name,
                    });
                }
            }
        }
        Ok(())
    }

    pub fn remove_sources<S>(&self, del_sources: Vec<S>) -> Result<(), L10nRegistrySetupError>
    where
        S: ToString,
    {
        let mut sources = self
            .shared
            .sources
            .try_borrow_mut()
            .map_err(|_| L10nRegistrySetupError::RegistryLocked)?;
        let del_sources: Vec<String> = del_sources.into_iter().map(|s| s.to_string()).collect();

        for metasource in sources.iter_mut() {
            metasource.retain(|source| !del_sources.contains(&source.name));
        }
        Ok(())
    }

    pub fn clear_sources(&self) -> Result<(), L10nRegistrySetupError> {
        let mut sources = self
            .shared
            .sources
            .try_borrow_mut()
            .map_err(|_| L10nRegistrySetupError::RegistryLocked)?;
        sources.clear();
        Ok(())
    }

    pub fn get_source_names(&self) -> Result<Vec<String>, L10nRegistrySetupError> {
        let sources = self
            .shared
            .sources
            .try_borrow_mut()
            .map_err(|_| L10nRegistrySetupError::RegistryLocked)?;
        Ok(sources.iter().flatten().map(|s| s.name.clone()).collect())
    }

    pub fn has_source(&self, name: &str) -> Result<bool, L10nRegistrySetupError> {
        let sources = self
            .shared
            .sources
            .try_borrow_mut()
            .map_err(|_| L10nRegistrySetupError::RegistryLocked)?;
        Ok(sources.iter().flatten().any(|source| source.name == name))
    }

    pub fn get_source(&self, name: &str) -> Result<Option<FileSource>, L10nRegistrySetupError> {
        let sources = self
            .shared
            .sources
            .try_borrow_mut()
            .map_err(|_| L10nRegistrySetupError::RegistryLocked)?;
        Ok(sources
            .iter()
            .flatten()
            .find(|source| source.name == name)
            .cloned())
    }
    pub fn get_available_locales(&self) -> Result<Vec<LanguageIdentifier>, L10nRegistrySetupError> {
        let sources = self
            .shared
            .sources
            .try_borrow_mut()
            .map_err(|_| L10nRegistrySetupError::RegistryLocked)?;
        let mut result = HashSet::new();
        for source in sources.iter().flatten() {
            for locale in source.locales() {
                result.insert(locale);
            }
        }
        Ok(result.into_iter().map(|l| l.to_owned()).collect())
    }
}

impl<P> BundleGenerator for L10nRegistry<P>
where
    P: ErrorReporter + Clone,
{
    type Resource = Rc<FluentResource>;
    type Iter = GenerateBundlesSync<P>;
    type Stream = GenerateBundles<P>;

    fn bundles_stream(
        &self,
        locales: std::vec::IntoIter<LanguageIdentifier>,
        resource_ids: Vec<String>,
    ) -> Self::Stream {
        self.generate_bundles(locales, resource_ids)
    }

    fn bundles_iter(
        &self,
        locales: std::vec::IntoIter<LanguageIdentifier>,
        resource_ids: Vec<String>,
    ) -> Self::Iter {
        self.generate_bundles_sync(locales, resource_ids)
    }
}
