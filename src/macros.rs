/// Translate and content formatting.
///
/// ## Example
///
/// Assume that translate file are as follows:
///
/// ```json
/// {
///     "Hello, {}": "Hello, {}",
///     "birthday": {
///         "Hello, {}": "Happy birthday, {}"
///     }
/// }
/// ```
///
/// [`tr`] will work like:
///
/// ```ignore
/// let name = "ho-229";
/// assert_eq!("Hello, ho-229", r18::tr!("Hello, {}", name));
/// assert_eq!("Hello, ho-229", r18::tr!([""] "Hello, {}", name));
/// assert_eq!("Happy birthday, ho-229", r18::tr!([".birthday"] "Hello, {}", name));
/// ```
#[macro_export]
macro_rules! tr {
    ($content:expr) => {
        ::r18::translate("", $content)
    };
    ($content:expr, $($arg:expr),+) => {{
        use ::r18::{Format, SimpleCurlyFormat};
        SimpleCurlyFormat.format(::r18::translate("", $content), &[$($arg),+])
            .unwrap_or_default()
    }};
    ([$prefix:expr] $content:expr) => {
        ::r18::translate($prefix, $content)
    };
    ([$prefix:expr] $content:expr, $($arg:expr),+) => {{
        use ::r18::{Format, SimpleCurlyFormat};
        SimpleCurlyFormat.format(::r18::translate($prefix, $content), &[$($arg),+])
            .unwrap_or_default()
    }};
}

/// Sets the current locale.
///
/// If the input language tag is invalid or not translated,
/// the translation will be disabled.
///
/// ## Example
///
/// ```ignore
/// r18::set_locale!("zh-CN");   // assume the zh-CN has been translated
/// assert_eq!(Some("zh-CN"), r18::locale!());
/// r18::set_locale!("");
/// assert_eq!(None, r18::locale!());
/// ```
#[macro_export]
#[allow(clippy::crate_in_macro_def)]
macro_rules! set_locale {
    ($locale:expr) => {
        // this function is generated by r18::init
        crate::__r18_gen::set_locale($locale)
    };
}

/// Returns the current locale.
#[macro_export]
macro_rules! locale {
    () => {
        $crate::CURRENT_LOCALE
            .get_or_init(|| ::std::sync::Mutex::new(None))
            .lock()
            .unwrap()
            .as_ref()
            .map(|l| l.name)
    };
}

/// Automatically sets the current locale.
#[macro_export]
macro_rules! auto_detect {
    () => {
        ::r18::get_locale().map(|l| ::r18::set_locale!(l))
    };
}
