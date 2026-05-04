use bon::Builder;
use proptest_derive::Arbitrary;
use std::str::FromStr;

use crate::common::OnOff;
use crate::parsers::DELIM_COMMA;
use crate::qao;
use crate::shell_string::{ShellString, ShellStringError};
use crate::to_command::ToCommand;

pub(crate) const ARG_NAME: &str = "-name";

const KEY_PROCESS: &str = "process=";
const KEY_DEBUG_THREADS: &str = "debug-threads=";

/// Sets the name of the guest. This name will be displayed in the SDL
/// window caption. The name will also be used for the VNC server. Also
/// optionally set the top visible process name in Linux. Naming of
/// individual threads can also be enabled on Linux to aid debugging.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct Name {
    /// The guest name shown by QEMU frontends.
    name: ShellString,
    /// Optional process name visible on the host.
    process: Option<ShellString>,
    /// Whether QEMU should name individual threads.
    debug_threads: Option<OnOff>,
}

impl ToCommand for Name {
    fn command(&self) -> String {
        ARG_NAME.to_string()
    }
    fn to_args(&self) -> Vec<String> {
        let mut args = vec![self.name.as_ref().to_string()];

        if let Some(process) = &self.process {
            args.push(format!("{}{}", KEY_PROCESS, process.as_ref()));
        }
        qao!(&self.debug_threads, args, KEY_DEBUG_THREADS);

        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for Name {
    type Err = ShellStringError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(DELIM_COMMA);
        let first = parts.next().ok_or_else(|| ShellStringError::new("empty -name argument"))?;

        let mut process = None;
        let mut debug_threads = None;

        for part in parts {
            let (key, value) = part.split_once('=').ok_or_else(|| ShellStringError::new(format!("invalid -name option: {part}")))?;
            match key {
                "process" => process = Some(ShellString::new(value)),
                "debug-threads" => debug_threads = Some(value.parse::<OnOff>().map_err(|_| ShellStringError::new(format!("invalid debug-threads value: {value}")))?),
                other => return Err(ShellStringError::new(format!("unsupported -name option: {other}"))),
            }
        }

        Ok(Name {
            name: ShellString::new(first),
            process,
            debug_threads,
        })
    }
}
