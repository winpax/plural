use std::fmt::Display;

use super::PluralForm;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[must_use]
/// Provides a wrapper that implements the [`Pluralize`] trait.
pub struct Plural<'w> {
    pub(crate) word: &'w str,
    pub(crate) form: PluralForm,
}

impl<'w> Plural<'w> {
    /// Construct a new [`Plural`] instance.
    pub const fn new(word: &'w str) -> Self {
        Self {
            word,
            form: PluralForm::const_default(),
        }
    }

    /// Construct a new [`Plural`] instance with a custom form.
    pub const fn new_with_form(word: &'w str, form: PluralForm) -> Self {
        Self { word, form }
    }

    /// Assign a plural form to the [`Plural`] instance.
    pub fn with_form(self, form: PluralForm) -> Self {
        Self {
            word: self.word,
            form,
        }
    }

    #[must_use]
    /// Get the word that is being pluralized.
    pub const fn word(&self) -> &str {
        self.word
    }

    #[must_use]
    /// Determine the plural form of the word.
    pub const fn form(&self) -> &PluralForm {
        &self.form
    }
}

/// Trait providing methods for the pluralization of words.
///
/// This does not handle the implementation itself.
pub trait Pluralize {
    /// Get the plural form of the word.
    fn plural(&self) -> String;
}

impl<'w> Pluralize for Plural<'w> {
    fn plural(&self) -> String {
        self.to_string()
    }
}

impl<'w> Display for Plural<'w> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.form {
            PluralForm::AppendS => {
                self.word.fmt(f)?;
                write!(f, "s")
            }
            PluralForm::Custom(custom_form) => custom_form.fmt(f),
        }
    }
}

impl Pluralize for &str {
    /// Pluralizes the given word.
    ///
    /// Note that this uses the default form (appending an `s`).
    ///
    /// # Example
    ///
    /// ```
    /// use plural::Pluralize;
    ///
    /// assert_eq!("dog".plural(), "dogs");
    /// ```
    fn plural(&self) -> String {
        Plural::new(self).plural()
    }
}
