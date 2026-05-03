use annotate_snippets::renderer::DecorStyle;
use annotate_snippets::{AnnotationKind, Level, Renderer, Snippet};
use proptest_derive::Arbitrary;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::str::FromStr;
use winnow::error::{ContextError, ParseError};
use winnow::prelude::*;

fn shell_quote(s: &str) -> String {
    if !s.is_empty() && s.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '-' | '.' | '/' | ':' | ',' | '=' | '+')) {
        return s.to_string();
    }

    let escaped = s.replace('\'', r#"'\''"#);
    format!("'{}'", escaped)
}

fn parse_shell_text(s: &str) -> Result<String, String> {
    if s.contains(['\'', '"', '\\']) {
        let parsed = shellish_parse::parse(s, shellish_parse::ParseOptions::new()).map_err(|e| e.to_string())?;
        return Ok(parsed.join(" "));
    }

    Ok(s.to_string())
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub struct ShellString {
    #[proptest(regex = r#"[^,\n]{0,100}"#)]
    pub s: String,
}

impl ShellString {
    pub fn new(s: impl Into<String>) -> Self {
        Self { s: s.into() }
    }

    pub fn shell_quoted(&self) -> String {
        shell_quote(&self.s)
    }
}

impl Display for ShellString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.shell_quoted())
    }
}

impl FromStr for ShellString {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ShellString { s: parse_shell_text(s)? })
    }
}

impl From<ShellString> for String {
    fn from(val: ShellString) -> Self {
        val.s.to_string()
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
            .element(Snippet::source(&self.input).annotation(AnnotationKind::Primary.span(self.span.clone()).label("parse failed here")))];

        let renderer = Renderer::styled().decor_style(DecorStyle::Unicode);
        let rendered = renderer.render(report);
        rendered.fmt(f)
    }
}

impl std::error::Error for ShellStringError {}

pub(crate) fn shell_string_until_comma<'a>(input: &mut &'a str) -> ModalResult<&'a str> {
    shell_string_until(input, &[','])
}

fn shell_string_until<'a>(input: &mut &'a str, delimiters: &[char]) -> ModalResult<&'a str> {
    let mut single_quoted = false;
    let mut double_quoted = false;
    let mut escaped = false;

    for (idx, ch) in input.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }

        match ch {
            '\\' if !single_quoted => {
                escaped = true;
            }
            '\'' if !double_quoted => {
                single_quoted = !single_quoted;
            }
            '"' if !single_quoted => {
                double_quoted = !double_quoted;
            }
            _ if !single_quoted && !double_quoted && delimiters.contains(&ch) => {
                if idx == 0 {
                    return Err(winnow::error::ErrMode::Backtrack(ContextError::new()));
                }
                let (head, tail) = input.split_at(idx);
                *input = tail;
                return Ok(head);
            }
            _ => {}
        }
    }

    if escaped || single_quoted || double_quoted {
        return Err(winnow::error::ErrMode::Cut(ContextError::new()));
    }

    if input.is_empty() {
        return Err(winnow::error::ErrMode::Backtrack(ContextError::new()));
    }

    let head = *input;
    *input = "";
    Ok(head)
}
