use std::str::FromStr;

use bon::Builder;
use proptest_derive::Arbitrary;

use crate::parsers::DELIM_COMMA;
use crate::shell_string::ShellString;
use crate::to_command::ToCommand;

pub(crate) const ARG_ADD_FD: &str = "-add-fd";

const KEY_FD: &str = "fd=";
const KEY_SET: &str = "set=";
const KEY_OPAQUE: &str = "opaque=";

/// Add a file descriptor to a fd set.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct AddFd {
    /// This option defines the file descriptor of which a duplicate is
    /// added to fd set. The file descriptor cannot be stdin, stdout, or
    /// stderr.
    pub fd: usize,
    /// This option defines the ID of the fd set to add the file
    /// descriptor to.
    pub set: usize,
    /// This option defines a free-form string that can be used to
    /// describe fd.
    pub opaque: Option<ShellString>,
}

impl AddFd {
    /// Creates an `-add-fd` mapping from a host file descriptor to a QEMU fd set.
    pub fn new(fd: usize, set: usize) -> Self {
        Self {
            fd,
            set,
            opaque: None,
        }
    }
}

impl ToCommand for AddFd {
    fn command(&self) -> String {
        ARG_ADD_FD.to_string()
    }

    fn to_args(&self) -> Vec<String> {
        let mut args = vec![format!("{}{}", KEY_FD, self.fd)];
        args.push(format!("{}{}", KEY_SET, self.set));
        if let Some(opaque) = &self.opaque {
            args.push(format!("{}{}", KEY_OPAQUE, opaque.as_ref()));
        }

        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for AddFd {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut fd = None;
        let mut set = None;
        let mut opaque = None;

        for part in s.split(DELIM_COMMA) {
            let (key, value) = part.split_once('=').ok_or_else(|| format!("invalid add-fd option: {part}"))?;
            match key {
                "fd" => fd = Some(value.parse::<usize>().map_err(|e| e.to_string())?),
                "set" => set = Some(value.parse::<usize>().map_err(|e| e.to_string())?),
                "opaque" => opaque = Some(ShellString::from_str(value)?),
                other => return Err(format!("unsupported add-fd option: {other}")),
            }
        }

        Ok(AddFd {
            fd: fd.ok_or_else(|| "missing fd= for -add-fd".to_string())?,
            set: set.ok_or_else(|| "missing set= for -add-fd".to_string())?,
            opaque,
        })
    }
}
