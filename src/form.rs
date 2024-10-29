#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[allow(clippy::module_name_repetitions)]
/// An enum determining the plural form of a word.
///
/// This currently supports two forms:
///
/// - `AppendS`: Appends an `s` to the end of the word.
/// - `Custom`: A custom form provided by the user.
///
/// This may change in future hence the `#[non_exhaustive]` attribute.
pub enum PluralForm {
    /// Append an 's' to the end of the word
    AppendS,
    /// A custom form provided by the user
    ///
    /// This will simply be passed to the formatter.
    Custom(String),
}

impl PluralForm {
    pub(crate) const fn const_default() -> Self {
        Self::AppendS
    }
}

impl Default for PluralForm {
    fn default() -> Self {
        Self::const_default()
    }
}
