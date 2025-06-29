use i18n_embed::{
    fluent::{fluent_language_loader, FluentLanguageLoader},
    LanguageLoader,
    RustEmbedNotifyAssets,
    // DefaultLocalizer,
};
use once_cell::sync::Lazy;
use rust_embed::RustEmbed;
use unic_langid::LanguageIdentifier;

#[derive(RustEmbed)]
#[folder = "i18n/"]
pub struct LocalizationsEmbed;

pub static LOCALIZATIONS: Lazy<RustEmbedNotifyAssets<LocalizationsEmbed>> =
    Lazy::new(|| RustEmbedNotifyAssets::new(std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("i18n/")));

pub static LANGUAGE_LOADER: Lazy<FluentLanguageLoader> = Lazy::new(|| {
    let loader: FluentLanguageLoader = fluent_language_loader!();

    // Load the fallback langauge by default so that users of the
    // library don't need to if they don't care about localization.
    loader
        .load_fallback_language(&*LOCALIZATIONS)
        .expect("Error while loading fallback language");

    loader
});

#[macro_export]
macro_rules! fl {
    ($message_id:literal) => {{
        i18n_embed_fl::fl!($crate::LANGUAGE_LOADER, $message_id)
    }};

    ($message_id:literal, $($args:expr),*) => {{
        i18n_embed_fl::fl!($crate::LANGUAGE_LOADER, $message_id, $($args), *)
    }};
}

// Get the `Localizer` to be used for localizing this library.
// pub fn localizer() -> DefaultLocalizer<'static> {
//     DefaultLocalizer::new(&*LANGUAGE_LOADER, &*LOCALIZATIONS)
// }

pub fn change_language(lang_code: &str) -> Result<(), String> {
    let lang: LanguageIdentifier = lang_code.parse().map_err(|_| format!("Invalid language tag: {lang_code}"))?;

    LANGUAGE_LOADER
        .load_languages(&*LOCALIZATIONS, &[lang])
        .map_err(|e| format!("Failed to load language: {e:?}"))?;

    Ok(())
}
