use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

use l10nregistry::registry::L10nRegistry;
use fluent_fallback::{L10nKey, Localization};

const RES_IDS: &[&str] = &[
    "branding/brand.ftl",
    "browser/branding/brandings.ftl",
    "browser/branding/sync-brand.ftl",
    "browser/preferences/preferences.ftl",
    "browser/preferences/fonts.ftl",
    "toolkit/featuregates/features.ftl",
    "browser/preferences/addEngine.ftl",
    "browser/preferences/blocklists.ftl",
    "browser/preferences/clearSiteData.ftl",
    "browser/preferences/colors.ftl",
    "browser/preferences/connection.ftl",
    "browser/preferences/languages.ftl",
    "browser/preferences/permissions.ftl",
    "browser/preferences/selectBookmark.ftl",
    "browser/preferences/siteDataSettings.ftl",
    "browser/aboutDialog.ftl",
    "browser/sanitize.ftl",
    "toolkit/updates/history.ftl",
    "security/certificates/deviceManager.ftl",
    "security/certificates/certManager.ftl",
];

static L10N_IDS: &[&str] = &[
    "do-not-track-description",
    "do-not-track-learn-more",
    "do-not-track-option-default-content-blocking-known",
    "do-not-track-option-always",
    "pref-page",
    "search-input-box",
    "policies-notice",
    "pane-general-title",
    "category-general",
    "pane-home-title",
    "category-home",
    "pane-search-title",
    "category-search",
    "pane-privacy-title",
    "category-privacy",
    "pane-sync-title2",
    "category-sync2",
    "help-button-label",
    "addons-button-label",
    "focus-search",
    "close-button",
    "feature-enable-requires-restart",
    "feature-disable-requires-restart",
    "should-restart-title",
    "should-restart-ok",
    "cancel-no-restart-button",
    "restart-later",
    "extension-controlled-homepage-override",
    "extension-controlled-new-tab-url",
    "extension-controlled-web-notifications",
    "extension-controlled-default-search",
    "extension-controlled-privacy-containers",
    "extension-controlled-websites-content-blocking-all-trackers",
    "extension-controlled-proxy-config",
    "extension-controlled-enable",
    "search-results-header",
    "search-results-empty-message",
    "search-results-help-link",
    "startup-header",
    "separate-profile-mode",
    "use-firefox-sync",
    "get-started-not-logged-in",
    "get-started-configured",
    "always-check-default",
    "is-default",
    "is-not-default",
    "set-as-my-default-browser",
    "startup-restore-previous-session",
    "startup-restore-warn-on-quit",
    "disable-extension",
    "tabs-group-header",
    "ctrl-tab-recently-used-order",
    "open-new-link-as-tabs",
    "warn-on-close-multiple-tabs",
    "warn-on-open-many-tabs",
    "switch-links-to-new-tabs",
    "show-tabs-in-taskbar",
    "browser-containers-enabled",
    "browser-containers-learn-more",
    "browser-containers-settings",
    "containers-disable-alert-title",
    "containers-disable-alert-desc",
    "containers-disable-alert-ok-button",
    "containers-disable-alert-cancel-button",
    "containers-remove-alert-title",
    "containers-remove-alert-msg",
    "containers-remove-ok-button",
    "containers-remove-cancel-button",
    "language-and-appearance-header",
    "fonts-and-colors-header",
    "default-font",
    "default-font-size",
    "advanced-fonts",
    "colors-settings",
    "language-header",
    "choose-language-description",
    "choose-button",
    "choose-browser-language-description",
    "manage-browser-languages-button",
    "confirm-browser-language-change-description",
    "confirm-browser-language-change-button",
    "translate-web-pages",
    "translate-attribution",
    "translate-exceptions",
    "check-user-spelling",
    "files-and-applications-title",
    "download-header",
    "download-save-to",
    "download-choose-folder",
    "download-always-ask-where",
    "applications-header",
    "applications-description",
    "applications-filter",
    "applications-type-column",
    "applications-action-column",
    "drm-content-header",
    "play-drm-content",
    "play-drm-content-learn-more",
    "update-application-title",
    "update-application-description",
    "update-application-version",
    "update-history",
    "update-application-allow-description",
    "update-application-auto",
    "update-application-check-choose",
    "update-application-manual",
    "update-application-warning-cross-user-setting",
    "update-application-use-service",
    "update-enable-search-update",
    "update-pref-write-failure-title",
    "update-pref-write-failure-message",
    "performance-title",
    "performance-use-recommended-settings-checkbox",
    "performance-use-recommended-settings-desc",
    "performance-settings-learn-more",
    "performance-allow-hw-accel",
    "performance-limit-content-process-option",
    "performance-limit-content-process-enabled-desc",
    "performance-limit-content-process-blocked-desc",
    "performance-default-content-process-count",
    "browsing-title",
    "browsing-use-autoscroll",
    "browsing-use-smooth-scrolling",
    "browsing-use-onscreen-keyboard",
    "browsing-use-cursor-navigation",
    "browsing-search-on-start-typing",
    "browsing-cfr-recommendations",
    "browsing-cfr-features",
    "browsing-cfr-recommendations-learn-more",
    "network-settings-title",
    "network-proxy-connection-description",
    "network-proxy-connection-learn-more",
    "network-proxy-connection-settings",
    "home-new-windows-tabs-header",
    "home-new-windows-tabs-description2",
    "home-homepage-mode-label",
    "home-newtabs-mode-label",
    "home-restore-defaults",
    "home-mode-choice-default",
    "home-mode-choice-custom",
    "home-mode-choice-blank",
    "home-homepage-custom-url",
    "use-current-pages",
    "choose-bookmark",
    "search-bar-header",
    "search-bar-hidden",
    "search-bar-shown",
    "search-engine-default-header",
    "search-engine-default-desc",
    "search-suggestions-option",
    "search-show-suggestions-url-bar-option",
    "search-show-suggestions-above-history-option",
    "search-suggestions-cant-show",
    "search-one-click-header",
    "search-one-click-desc",
    "search-choose-engine-column",
    "search-choose-keyword-column",
    "search-restore-default",
    "search-remove-engine",
    "search-find-more-link",
    "search-keyword-warning-title",
    "search-keyword-warning-engine",
    "search-keyword-warning-bookmark",
    "containers-back-link",
    "containers-header",
    "containers-add-button",
    "containers-preferences-button",
    "containers-remove-button",
    "sync-signedout-caption",
    "sync-signedout-description",
    "sync-signedout-account-title",
    "sync-signedout-account-create",
    "sync-signedout-account-signin",
    "sync-mobile-promo",
    "sync-profile-picture",
    "sync-disconnect",
    "sync-manage-account",
    "sync-signedin-unverified",
    "sync-signedin-login-failure",
    "sync-resend-verification",
    "sync-remove-account",
    "sync-sign-in",
    "sync-signedin-settings-header",
    "sync-signedin-settings-desc",
    "sync-engine-bookmarks",
    "sync-engine-history",
    "sync-engine-tabs",
    "sync-engine-logins",
    "sync-engine-addresses",
    "sync-engine-creditcards",
    "sync-engine-addons",
    "sync-engine-prefs",
    "sync-device-name-header",
    "sync-device-name-change",
    "sync-device-name-cancel",
    "sync-device-name-save",
    "sync-connect-another-device",
    "sync-manage-devices",
    "sync-fxa-begin-pairing",
    "sync-tos-link",
    "sync-fxa-privacy-notice",
    "privacy-header",
    "logins-header",
    "forms-ask-to-save-logins",
    "forms-exceptions",
    "forms-saved-logins",
    "forms-master-pw-use",
    "forms-master-pw-change",
    "history-header",
    "history-remember-label",
    "history-remember-option-all",
    "history-remember-option-never",
    "history-remember-option-custom",
    "history-remember-description",
    "history-dontremember-description",
    "history-private-browsing-permanent",
    "history-remember-browser-option",
    "history-remember-search-option",
    "history-clear-on-close-option",
    "history-clear-on-close-settings",
    "history-clear-button",
    "sitedata-header",
    "sitedata-total-size-calculating",
    "sitedata-total-size",
    "sitedata-learn-more",
    "sitedata-delete-on-close",
    "sitedata-delete-on-close-private-browsing",
    "sitedata-allow-cookies-option",
    "sitedata-disallow-cookies-option",
    "sitedata-block-desc",
    "sitedata-option-block-trackers",
    "sitedata-option-block-unvisited",
    "sitedata-option-block-all-third-party",
    "sitedata-option-block-all",
    "sitedata-clear",
    "sitedata-settings",
    "sitedata-cookies-permissions",
    "addressbar-header",
    "addressbar-suggest",
    "addressbar-locbar-history-option",
    "addressbar-locbar-bookmarks-option",
    "addressbar-locbar-openpage-option",
    "addressbar-suggestions-settings",
    "content-blocking-header",
    "content-blocking-description",
    "content-blocking-learn-more",
    "content-blocking-setting-standard",
    "content-blocking-setting-strict",
    "content-blocking-setting-custom",
    "content-blocking-standard-description",
    "content-blocking-standard-desc",
    "content-blocking-strict-desc",
    "content-blocking-custom-desc",
    "content-blocking-private-trackers",
    "content-blocking-third-party-cookies",
    "content-blocking-all-windows-trackers",
    "content-blocking-all-third-party-cookies",
    "content-blocking-warning-title",
    "content-blocking-warning-description",
    "content-blocking-learn-how",
    "content-blocking-trackers-label",
    "content-blocking-tracking-protection-option-all-windows",
    "content-blocking-option-private",
    "content-blocking-tracking-protection-change-block-list",
    "content-blocking-cookies-label",
    "content-blocking-cryptominers-label",
    "content-blocking-fingerprinters-label",
    "tracking-manage-exceptions",
    "permissions-header",
    "permissions-location",
    "permissions-location-settings",
    "permissions-camera",
    "permissions-camera-settings",
    "permissions-microphone",
    "permissions-microphone-settings",
    "permissions-notification",
    "permissions-notification-settings",
    "permissions-notification-link",
    "permissions-notification-pause",
    "permissions-block-autoplay-media2",
    "permissions-block-autoplay-media-exceptions",
    "permissions-block-popups",
    "permissions-block-popups-exceptions",
    "permissions-addon-install-warning",
    "permissions-addon-exceptions",
    "permissions-a11y-privacy-checkbox",
    "permissions-a11y-privacy-link",
    "collection-header",
    "collection-description",
    "collection-privacy-notice",
    "collection-health-report",
    "collection-health-report-link",
    "collection-studies",
    "collection-studies-link",
    "addon-recommendations",
    "addon-recommendations-link",
    "collection-health-report-disabled",
    "collection-backlogged-crash-reports",
    "collection-backlogged-crash-reports-link",
    "security-header",
    "security-browsing-protection",
    "security-enable-safe-browsing",
    "security-enable-safe-browsing-link",
    "security-block-downloads",
    "security-block-uncommon-software",
    "certs-header",
    "certs-personal-label",
    "certs-select-auto-option",
    "certs-select-ask-option",
    "certs-enable-ocsp",
    "certs-view",
    "certs-devices",
    "space-alert-learn-more-button",
    "space-alert-over-5gb-pref-button",
    "space-alert-over-5gb-message",
    "space-alert-under-5gb-ok-button",
    "space-alert-under-5gb-message",
    "desktop-folder-name",
    "downloads-folder-name",
    "choose-download-folder-title",
    "save-files-to-cloud-storage",
];

fn preferences_bench(c: &mut Criterion) {
    let locales = vec!["en-US".parse().unwrap()];
    let mut reg = L10nRegistry::default();

    reg.set_lang_ids(locales.clone());

    let browser_fs = l10nregistry::tokio::file_source(
        "browser".to_string(),
        locales.clone(),
        "./tests/resources/browser/{locale}".into(),
    );
    let toolkit_fs = l10nregistry::tokio::file_source(
        "toolkit".to_string(),
        locales.clone(),
        "./tests/resources/toolkit/{locale}".into(),
    );

    reg.register_sources(vec![toolkit_fs, browser_fs]).unwrap();

    let res_ids: Vec<String> = RES_IDS.iter().map(|s| s.to_string()).collect();
    c.bench_function("localization/format_value_sync", |b| {
        b.iter(|| {
            let loc = Localization::with_generator(res_ids.clone(), true, reg.clone());
            let mut errors = vec![];

            for id in L10N_IDS.iter() {
                loc.format_value_sync(id, None, &mut errors);
            }
        })
    });

    let keys: Vec<L10nKey> = L10N_IDS.iter().map(|&id| L10nKey { id: id.into(), args: None }).collect();
    c.bench_function("localization/format_messages_sync", |b| {
        b.iter(|| {
            let loc = Localization::with_generator(res_ids.clone(), true, reg.clone());
            let mut errors = vec![];

            loc.format_messages_sync(&keys, &mut errors);
        })
    });
}

criterion_group!(benches, preferences_bench);
criterion_main!(benches);