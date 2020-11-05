// use l10nregistry::registry::L10nRegistry;
use l10nregistry::iter::*;

use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

static mut FILE_SYSTEM: Option<FileSystem> = None;

pub fn get_file_system() -> &'static FileSystem {
    unsafe { &mut FILE_SYSTEM }.get_or_insert_with(|| {
        FileSystem::new(vec![
            ("browser/branding/brand.ftl", "key = Brand"),
            ("browser/browser/branding/brandings.ftl", "key2 = Main"),
            ("toolkit/browser/branding/brandings.ftl", "key2 = Main"),
        ])
    })
}

fn main() {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::DEBUG)
        // builds the subscriber.
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let fs = get_file_system();

    let browser_fs = FileSource::new("browser", "browser/", fs);
    let toolkit_fs = FileSource::new("toolkit", "toolkit/", fs);

    let mut reg = L10nRegistry::default();
    reg.sources.push(browser_fs);
    reg.sources.push(toolkit_fs);

    let res_ids = vec![
        "branding/brand.ftl".to_string(),
        "browser/branding/brandings.ftl".to_string(),
    ];

    let mut perms = SourcePermutations::new(&reg, res_ids);
    perms.next();
    perms.next();
    perms.next();
}

// const res_ids: &[&str] = &[
//     "branding/brand.ftl",
//     "browser/branding/brandings.ftl",
//     "browser/branding/sync-brand.ftl",
//     "browser/preferences/preferences.ftl",
//     "browser/preferences/fonts.ftl",
//     "toolkit/featuregates/features.ftl",
//     "browser/preferences/addEngine.ftl",
//     "browser/preferences/blocklists.ftl",
//     "browser/preferences/clearSiteData.ftl",
//     "browser/preferences/colors.ftl",
//     "browser/preferences/connection.ftl",
//     "browser/preferences/languages.ftl",
//     "browser/preferences/permissions.ftl",
//     "browser/preferences/selectBookmark.ftl",
//     "browser/preferences/siteDataSettings.ftl",
//     "browser/aboutDialog.ftl",
//     "browser/sanitize.ftl",
//     "toolkit/updates/history.ftl",
//     "security/certificates/deviceManager.ftl",
//     "security/certificates/certManager.ftl",
// ];

// fn main() {
//     let subscriber = tracing_subscriber::FmtSubscriber::builder()
//         // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
//         // will be written to stdout.
//         .with_max_level(Level::TRACE)
//         // builds the subscriber.
//         .finish();

//     let locales = vec!["en-US".parse().unwrap()];
//     let mut reg = L10nRegistry::default();

//     reg.set_lang_ids(locales.clone());

//     let browser_fs = l10nregistry::tokio::file_source(
//         "browser".to_string(),
//         locales.clone(),
//         "/home/mozilla/projects/l10nregistry-rs/tests/resources/browser/{locale}".into(),
//     );
//     let toolkit_fs = l10nregistry::tokio::file_source(
//         "toolkit".to_string(),
//         locales.clone(),
//         "/home/mozilla/projects/l10nregistry-rs/tests/resources/toolkit/{locale}".into(),
//     );

//     reg.register_sources(vec![toolkit_fs, browser_fs]).unwrap();

//     let paths = res_ids.iter().map(|&r| r.into()).collect();
//     let mut i = reg.generate_bundles_for_lang_sync(locales[0].clone(), paths);

//     assert!(i.next().is_some());
// }
