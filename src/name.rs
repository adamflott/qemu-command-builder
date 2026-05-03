use bon::Builder;
use proptest_derive::Arbitrary;
use std::str::FromStr;

use crate::common::OnOff;
use crate::parsers::DELIM_COMMA;
use crate::shell_string::{ShellString, ShellStringError, shell_string_until_comma};
use crate::to_command::ToCommand;
use crate::{pco, qao};
use winnow::ascii::alphanumeric1;
use winnow::combinator::opt;
use winnow::token::literal;
use winnow::{ModalResult, Parser};

pub(crate) const ARG_NAME: &str = "-name";

const KEY_PROCESS: &str = "process=";
const KEY_DEBUG_THREADS: &str = "debug-threads=";

/// Sets the name of the guest. This name will be displayed in the SDL
/// window caption. The name will also be used for the VNC server. Also
/// optionally set the top visible process name in Linux. Naming of
/// individual threads can also be enabled on Linux to aid debugging.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct Name {
    name: ShellString,
    process: Option<ShellString>,
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
        name.parse(s).map_err(|e| ShellStringError::from_parse(e))
    }
}

pco!(process, shell_string_until_comma, ShellString, KEY_PROCESS);
pco!(debug_threads, alphanumeric1, OnOff, KEY_DEBUG_THREADS);

fn name(s: &mut &str) -> ModalResult<Name> {
    let name = shell_string_until_comma.parse_to::<ShellString>().parse_next(s)?;
    let process = opt(process).parse_next(s)?;
    let debug_threads = opt(debug_threads).parse_next(s)?;
    Ok(Name { name, process, debug_threads })
}
