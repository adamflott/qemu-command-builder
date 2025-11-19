use std::collections::BTreeMap;
use std::str::FromStr;

use bon::Builder;
use proptest_derive::Arbitrary;

use crate::to_command::ToCommand;

pub(crate) const ARG_AUDIODEV: &str = "-audiodev";

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct AudioDev {
    driver: String,
    props: BTreeMap<String, String>,
}

impl ToCommand for AudioDev {
    fn command(&self) -> String {
        ARG_AUDIODEV.to_string()
    }
    fn to_args(&self) -> Vec<String> {
        let mut args = vec![self.driver.clone()];

        for (k, v) in &self.props {
            args.push(format!("{}={}", k, v));
        }
        args
    }
}

impl FromStr for AudioDev {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
