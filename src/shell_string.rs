use annotate_snippets::renderer::DecorStyle;
use annotate_snippets::{AnnotationKind, Level, Renderer, Snippet};
use proptest_derive::Arbitrary;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::str::FromStr;
use winnow::error::{ContextError, ParseError};

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub struct ShellString {
    #[proptest(regex = "[a-zA-Z0-9]{1,15}")]
    pub s: String,
}

impl ShellString {
    pub fn new(s: impl Into<String>) -> Self {
        Self { s: s.into() }
    }
}

impl Display for ShellString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.s.as_str().contains(" ") { write!(f, r#""{}""#, self.s) } else { write!(f, "{}", self.s) }
    }
}

impl FromStr for ShellString {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ShellString { s: s.to_string() })
    }
}

impl Into<String> for ShellString {
    fn into(self) -> String {
        self.s.to_string()
    }
}

impl TryFrom<String> for ShellString {
    type Error = String;

    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        Ok(ShellString { s: value })
    }
}

impl<'a> From<&'a str> for ShellString {
    fn from(s: &'a str) -> Self {
        ShellString { s: s.to_string() }
    }
}
impl Deref for ShellString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl AsRef<str> for ShellString {
    fn as_ref(&self) -> &str {
        &self.s
    }
}

#[derive(Debug)]
pub struct ShellStringError {
    message: String,
    span: std::ops::Range<usize>,
    input: String,
}

impl ShellStringError {
    pub(crate) fn from_parse(error: ParseError<&str, ContextError>) -> Self {
        let message = error.inner().to_string();
        let input = (*error.input()).to_owned();
        let span = error.char_span();
        Self { message, span, input }
    }
}
impl std::fmt::Display for ShellStringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let report = &[Level::ERROR
            .primary_title(self.message.as_str())
            .element(Snippet::source(&self.input).annotation(AnnotationKind::Primary.span(self.span.clone()).label("TODO")))];

        let renderer = Renderer::styled().decor_style(DecorStyle::Unicode);
        let rendered = renderer.render(report);
        rendered.fmt(f)
    }
}

impl std::error::Error for ShellStringError {}
