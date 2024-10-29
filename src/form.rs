#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PluralForm {
    AppendS,
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
