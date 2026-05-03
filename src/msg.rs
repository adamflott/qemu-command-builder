use std::str::FromStr;

use crate::common::OnOff;
use crate::parsers::DELIM_COMMA;
use crate::shell_string::ShellStringError;
use crate::to_command::ToCommand;
use crate::qao;
use bon::Builder;
use proptest_derive::Arbitrary;

pub(crate) const ARG_MSG: &str = "-msg";

const KEY_TIMESTAMP: &str = "timestamp=";
const KEY_GUEST_NAME: &str = "guest-name=";

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct Msg {
    /// Prefix messages with a timestamp when enabled.
    timestamp: Option<OnOff>,
    /// Prefix messages with the guest name when enabled.
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

impl FromStr for Msg {
    type Err = ShellStringError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut timestamp = None;
        let mut guest_name = None;

        for part in s.split(DELIM_COMMA).filter(|part| !part.is_empty()) {
            match part {
                "timestamp" => timestamp = Some(OnOff::On),
                "guest-name" => guest_name = Some(OnOff::On),
                _ => {
                    let (key, value) = part.split_once('=').ok_or_else(|| ShellStringError::new(format!("invalid -msg option: {part}")))?;
                    match key {
                        "timestamp" => {
                            timestamp =
                                Some(value.parse::<OnOff>().map_err(|_| ShellStringError::new(format!("invalid timestamp value: {value}")))?)
                        }
                        "guest-name" => {
                            guest_name =
                                Some(value.parse::<OnOff>().map_err(|_| ShellStringError::new(format!("invalid guest-name value: {value}")))?)
                        }
                        other => return Err(ShellStringError::new(format!("unsupported -msg option: {other}"))),
                    }
                }
            }
        }

        Ok(Msg { timestamp, guest_name })
    }
}
