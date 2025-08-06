use bon::Builder;

use crate::to_command::ToCommand;

/// Set default value of driver's property prop to value
#[derive(Default, Builder)]
pub struct Global {
    driver: String,
    property: String,
    value: String,
}

impl ToCommand for Global {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec!["-global".to_string()];

        let mut args = vec![format!("driver={}", self.driver)];
        args.push(format!(",property={}", self.property));
        args.push(format!(",value={}", self.value));

        cmd.push(args.join(","));
        cmd
    }
}
