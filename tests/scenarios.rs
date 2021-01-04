use fluent_bundle::FluentArgs;
use fluent_fallback::Localization;
use fluent_testing::get_scenarios;
use l10nregistry::testing::{RegistrySetup, TestFileFetcher};

#[test]
fn scenarios_test() {
    let fetcher = TestFileFetcher::new();

    let scenarios = get_scenarios();

    for scenario in scenarios {
        let setup: RegistrySetup = (&scenario).into();
        let mut reg = fetcher.get_registry(setup);

        reg.set_adapt_bundle(|bundle| {
            bundle.set_use_isolating(false);
            bundle
                .add_function("PLATFORM", |_positional, _named| "linux".into())
                .expect("Failed to add a function to the bundle.");
        });

        let loc = Localization::with_generator(scenario.res_ids.clone(), true, reg);
        let mut errors = vec![];

        for query in scenario.queries.iter() {
            let args = query.input.args.as_ref().map(|args| {
                let mut result = FluentArgs::new();
                for arg in args.as_slice() {
                    result.add(arg.id.clone(), arg.value.clone().into());
                }
                result
            });
            if let Some(output) = &query.output {
                if let Some(value) = &output.value {
                    let v = loc.format_value_sync(&query.input.id, args.as_ref(), &mut errors);
                    assert_eq!(v.unwrap(), value.as_str());
                }
            }
            assert_eq!(errors, vec![]);
        }
    }
}