use proptest_derive::Arbitrary;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub struct ShellPath {
    #[proptest(regex = r"[a-zA-Z0-9]{1,100}")]
    pub s: String,
}

impl Display for ShellPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.s.contains(" ") { write!(f, "\"{}\"", self.s) } else { write!(f, "{}", self.s) }
    }
}

impl FromStr for ShellPath {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ShellPath { s: s.to_string() })
    }
}

impl<'a> From<&'a str> for ShellPath {
    fn from(s: &'a str) -> Self {
        ShellPath { s: s.to_string() }
    }
}
