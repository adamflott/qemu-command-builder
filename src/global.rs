use std::str::FromStr;

use bon::Builder;
use proptest_derive::Arbitrary;

use crate::to_command::ToCommand;

pub(crate) const ARG_GLOBAL: &str = "-global";

/// Set default value of driver's property prop to value
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct Global {
    driver: String,
    property: String,
    value: String,
}

impl ToCommand for Global {
    fn command(&self) -> String {
        ARG_GLOBAL.to_string()
    }
    fn to_args(&self) -> Vec<String> {
        let mut args = vec![format!("driver={}", self.driver)];
        args.push(format!(",property={}", self.property));
        args.push(format!(",value={}", self.value));

        args
    }
}

impl FromStr for Global {
    type Err = ();

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
