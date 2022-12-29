//! # 🔞
//!
//! `r18` is a crate for internationalising Rust projects.
//!
//! ## Usage
//!
//! Add `r18` as your project dependency.
//!
//! ```toml
//! [dependencies]
//! r18 = "*"
//! ```
//! Create a `JSON` translation file with name `BCP 47`
//! language tag as naming format, like below:
//!
//! ```json
//! // PATH: ./tr/zh-CN.json
//! {
//!     "Hello, {}": "你好，{}"
//! }
//! ```
//!
//! Then add [`init`] to the global scope of your code with
//! the directory where translation files in (in following example is `./tr`).
//!
//! ```ignore
//! r18::init!("tr");
//! ```
//!
//! After initialising the `r18`, use [`auto_detect`] to detect locale and load
//! translation model automatically.  
//! If you want, you can use [`set_locale`] to set locale manually.  
//! After above process, use [`tr`] to get your text which has been translated.
//!
//! ```ignore
//! r18::init!("tr");
//!
//! fn main() {
//!     r18::auto_detect!();
//!
//!     let name = "ho-229";
//!     println!("{}", r18::tr!("Hello, {}", name));
//!
//!     // reset locale to disable translation
//!     r18::set_locale!("");
//!     assert_eq!("Hello, ho-229", r18::tr!("Hello, {}", name));
//! }
//! ```

use std::sync::Mutex;

#[doc(hidden)]
pub use dynfmt::{Format, SimpleCurlyFormat};
#[doc(hidden)]
pub use once_cell::sync::Lazy;
#[doc(hidden)]
pub use oxilangtag::{LanguageTag, LanguageTagParseError};
#[doc(hidden)]
pub use phf;
pub use r18_proc_macros::init;
#[doc(hidden)]
pub use sys_locale::get_locale;

mod_use::mod_use!(macros);

#[doc(hidden)]
pub struct Locale {
    pub name: &'static str,
    pub translate: phf::Map<&'static str, &'static str>,
}

#[doc(hidden)]
pub static CURRENT_LOCALE: Lazy<Mutex<Option<&'static Locale>>> = Lazy::new(|| Mutex::new(None));

/// Translate content with the locale setting and given prefix.
///
/// We recommend using [`tr!`] instead of [`translate`] for translate your
/// content.
pub fn translate(prefix: impl AsRef<str>, content: &str) -> &str {
    let locale = CURRENT_LOCALE.lock().unwrap();
    let locale = match *locale {
        Some(l) => l,
        None => return content,
    };

    match locale
        .translate
        .get(format!("{}.{}", prefix.as_ref(), content).as_str())
    {
        Some(tr) => tr,
        None => content,
    }
}
