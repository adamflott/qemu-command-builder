use bon::Builder;
use std::collections::{BTreeMap};

use crate::to_command::ToCommand;

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder)]
pub struct AudioDev {
    driver: String,
    props: BTreeMap<String, String>,
}

impl ToCommand for AudioDev {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec!["-audiodev".to_string()];

        let mut args = vec![self.driver.clone()];

        for (k, v) in &self.props {
            args.push(format!("{}={}", k, v));
        }
        cmd.push(args.join(","));
        cmd
    }
}
