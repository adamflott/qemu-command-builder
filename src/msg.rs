use std::str::FromStr;

use crate::common::OnOff;
use crate::parsers::DELIM_COMMA;
use crate::shell_string::ShellStringError;
use crate::to_command::ToCommand;
use crate::{pco0, qao};
use bon::Builder;
use proptest_derive::Arbitrary;
use winnow::ascii::alphanumeric1;
use winnow::combinator::opt;
use winnow::prelude::*;
use winnow::token::literal;

pub(crate) const ARG_MSG: &str = "-msg";

const KEY_TIMESTAMP: &str = "timestamp=";
const KEY_GUEST_NAME: &str = "guest-name=";

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct Msg {
    timestamp: Option<OnOff>,
    guest_name: Option<OnOff>,
}

impl ToCommand for Msg {
    fn command(&self) -> String {
        ARG_MSG.to_string()
    }
    fn has_args(&self) -> bool {
        self.timestamp.is_some() || self.guest_name.is_some()
    }
    fn to_args(&self) -> Vec<String> {
        let mut args = vec![];
        qao!(&self.timestamp, args, KEY_TIMESTAMP);
        qao!(&self.guest_name, args, KEY_GUEST_NAME);
        vec![args.join(DELIM_COMMA)]
    }
}

pco0!(timestamp, alphanumeric1, OnOff, KEY_TIMESTAMP);
pco0!(guest_name, alphanumeric1, OnOff, KEY_GUEST_NAME);

impl FromStr for Msg {
    type Err = ShellStringError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        msg.parse(s).map_err(|e| ShellStringError::from_parse(e))
    }
}

fn msg(s: &mut &str) -> ModalResult<Msg> {
    let timestamp = opt(timestamp).parse_next(s)?;
    let guest_name = opt(guest_name).parse_next(s)?;
    Ok(Msg { timestamp, guest_name })
}
