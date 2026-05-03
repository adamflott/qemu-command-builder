use proptest_derive::Arbitrary;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::path::PathBuf;
use std::str::FromStr;
use winnow::prelude::*;
use winnow::token::take_while;

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
/// A shell-facing path or token used when rendering QEMU command arguments.
///
/// This type stores the raw path text. Shell escaping is applied later when a
/// full command line is flattened into a single string.
///
/// Use this type for path-like QEMU option values that should remain unquoted
/// in `argv` form but may need escaping when rendered as a single shell
/// command.
pub struct ShellPath {
    /// The underlying raw path or shell token.
    ///
    /// This value is stored exactly as provided.
    #[proptest(regex = r#"[^,\n:]{1,100}"#)]
    pub s: String,
}

impl ShellPath {
    /// Creates a new [`ShellPath`] from raw path text.
    pub fn new(s: impl Into<String>) -> Self {
        Self { s: s.into() }
    }
}

impl Display for ShellPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.s)
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

impl From<PathBuf> for ShellPath {
    fn from(value: PathBuf) -> Self {
        let s = value.into_os_string().into_string().expect("ShellPath only supports UTF-8 paths");
        Self { s }
    }
}

impl Deref for ShellPath {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl AsRef<str> for ShellPath {
    fn as_ref(&self) -> &str {
        &self.s
    }
}

pub(crate) fn shell_path_until_comma<'a>(input: &mut &'a str) -> ModalResult<&'a str> {
    take_while(1.., |c: char| c != ',').parse_next(input)
}

pub(crate) fn shell_path_until_colon<'a>(input: &mut &'a str) -> ModalResult<&'a str> {
    take_while(1.., |c: char| c != ':' && c != ',').parse_next(input)
}
