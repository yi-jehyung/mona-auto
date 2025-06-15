mod core;
mod gui;
mod i18n;
pub use crate::i18n::LANGUAGE_LOADER;

use core::*;
use gui::run_gui;

type BoxError = Box<dyn std::error::Error + Send + Sync>;

fn main() -> Result<(), BoxError> {
    run_gui().unwrap();
    Ok(())
}
