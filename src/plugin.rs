use crate::to_command::ToCommand;
use bon::Builder;
use proptest_derive::Arbitrary;
use std::path::PathBuf;
use std::str::FromStr;

pub(crate) const ARG_PLUGIN: &str = "-plugin";

/// A QEMU `-plugin [file=]file[,arg=value,...]` definition.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct Plugin {
    /// The shared library implementing the plugin.
    file: Option<PathBuf>,
    /// Plugin-specific key/value arguments.
    args: Option<Vec<(String, String)>>,
}

impl ToCommand for Plugin {
    fn command(&self) -> String {
        ARG_PLUGIN.to_string()
    }

    fn to_args(&self) -> Vec<String> {
        let mut args = vec![];

        if let Some(file) = &self.file {
            args.push(format!("file={}", file.display()));
        }
        if let Some(argss) = &self.args {
            for arg in argss {
                args.push(format!("{}={}", arg.0, arg.1));
            }
        }
        vec![args.join(",")]
    }
}

impl FromStr for Plugin {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let first = parts.next().ok_or_else(|| "empty -plugin argument".to_string())?;

        let file = if let Some(value) = first.strip_prefix("file=") {
            Some(PathBuf::from(value))
        } else if first.contains('=') {
            return Err(format!("unsupported first -plugin component: {first}"));
        } else {
            Some(PathBuf::from(first))
        };

        let mut args = Vec::new();
        for part in parts {
            let (key, value) = part.split_once('=').ok_or_else(|| format!("invalid -plugin option: {part}"))?;
            args.push((key.to_string(), value.to_string()));
        }

        Ok(Self { file, args: (!args.is_empty()).then_some(args) })
    }
}
