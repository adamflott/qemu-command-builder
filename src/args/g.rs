use crate::parsers::ARG_G;
use crate::shell_string::ShellStringError;
use crate::to_command::ToCommand;
use bon::Builder;
use proptest_derive::Arbitrary;
use std::fmt::Display;
use std::str::FromStr;

/// Initial graphical resolution and optional depth for QEMU `-g`.
///
/// QEMU accepts `-g WxH[xDEPTH]`, for example `800x600` or
/// `1024x768x24`.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct G {
    pub width: usize,
    pub height: usize,
    pub depth: Option<usize>,
}

impl Display for G {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}", self.width, self.height)?;
        if let Some(depth) = self.depth {
            write!(f, "x{}", depth)?;
        }
        Ok(())
    }
}

impl FromStr for G {
    type Err = ShellStringError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_g(s).map_err(ShellStringError::new)
    }
}

impl ToCommand for G {
    fn command(&self) -> String {
        ARG_G.to_string()
    }

    fn to_args(&self) -> Vec<String> {
        vec![self.to_string()]
    }
}

fn parse_g(s: &str) -> Result<G, String> {
    if s.is_empty() {
        return Err("empty -g option".to_string());
    }

    let mut parts = s.split('x');
    let width = parse_dimension(parts.next(), "width")?;
    let height = parse_dimension(parts.next(), "height")?;
    let depth = parts.next().map(|value| parse_dimension(Some(value), "depth")).transpose()?;

    if parts.next().is_some() {
        return Err(format!("invalid -g option: expected WxH[xDEPTH], got {s}"));
    }

    Ok(G { width, height, depth })
}

fn parse_dimension(value: Option<&str>, name: &str) -> Result<usize, String> {
    let value = value.ok_or_else(|| format!("missing -g {name}"))?;
    if value.is_empty() {
        return Err(format!("empty -g {name}"));
    }
    let parsed = value.parse::<usize>().map_err(|e| format!("invalid -g {name}: {e}"))?;
    if parsed == 0 {
        return Err(format!("-g {name} must be greater than zero"));
    }
    Ok(parsed)
}
