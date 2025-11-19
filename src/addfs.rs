use std::str::FromStr;

use bon::Builder;
use proptest_derive::Arbitrary;
use winnow::ascii::dec_uint;
use winnow::combinator::opt;
use winnow::token::literal;
use winnow::{ModalResult, Parser};

use crate::parsers::{DELIM_COMMA, optional_quotes_parser};
use crate::qao;
use crate::shell_string::{ShellString, ShellStringError};
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

impl ToCommand for AddFd {
    fn command(&self) -> String {
        ARG_ADD_FD.to_string()
    }

    fn to_args(&self) -> Vec<String> {
        let mut args = vec![format!("{}{}", KEY_FD, self.fd)];
        args.push(format!("{}{}", KEY_SET, self.set));
        qao!(&self.opaque, args, KEY_OPAQUE);

        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for AddFd {
    type Err = ShellStringError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        add_fd.parse(s).map_err(|e| ShellStringError::from_parse(e))
    }
}

fn opaque(s: &mut &str) -> ModalResult<ShellString> {
    let _ = literal(DELIM_COMMA).parse_next(s)?;
    let _ = literal(KEY_OPAQUE).parse_next(s)?;
    let op = optional_quotes_parser.parse_next(s)?;
    Ok(op)
}

fn add_fd(s: &mut &str) -> ModalResult<AddFd> {
    let _ = literal(KEY_FD).parse_next(s)?;
    let fd = dec_uint.parse_next(s)?;
    let _ = literal(DELIM_COMMA).parse_next(s)?;
    let _ = literal(KEY_SET).parse_next(s)?;
    let set = dec_uint.parse_next(s)?;
    let opaque = opt(opaque).parse_next(s)?;
    Ok(AddFd { fd, set, opaque })
}
