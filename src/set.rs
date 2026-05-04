use crate::parsers::ARG_SET;
use crate::to_command::ToCommand;
use bon::Builder;
use proptest_derive::Arbitrary;
use std::str::FromStr;

/// Set parameter arg for item id of type group
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct Set {
    /// The `group.id.arg` selector.
    group: String,
    /// The value assigned to the selector.
    value: String,
}

impl ToCommand for Set {
    fn command(&self) -> String {
        ARG_SET.to_string()
    }
    fn to_args(&self) -> Vec<String> {
        vec![format!("{}={}", self.group, self.value)]
    }
}

impl FromStr for Set {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (group, value) = s.split_once('=').ok_or_else(|| format!("invalid -set argument: {s}"))?;
        Ok(Self {
            group: group.to_string(),
            value: value.to_string(),
        })
    }
}
