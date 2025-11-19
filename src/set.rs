use bon::Builder;
use proptest_derive::Arbitrary;
use std::str::FromStr;

use crate::to_command::ToCommand;

pub(crate) const ARG_SET: &str = "-set";

/// Set parameter arg for item id of type group
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct Set {
    group: String,
    value: String,
}

impl ToCommand for Set {
    fn command(&self) -> String {
        ARG_SET.to_string()
    }
    fn to_args(&self) -> Vec<String> {
        let mut args = vec![self.group.clone()];
        args.push(format!("={}", self.value));

        args
    }
}

impl FromStr for Set {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
