use bon::Builder;
use std::collections::HashMap;

use crate::to_command::ToCommand;

#[derive(Default, Builder)]
pub struct AudioDev {
    driver: String,
    props: HashMap<String, String>,
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
