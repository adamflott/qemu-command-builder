use bon::Builder;

use crate::to_command::ToCommand;

/// Set parameter arg for item id of type group
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder)]
pub struct Set {
    group: String,
    value: String,
}

impl ToCommand for Set {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec![];

        cmd.push("-set".to_string());

        let mut arg = vec![self.group.clone()];
        arg.push(format!("={}", self.value));

        cmd.append(&mut arg);

        cmd
    }
}
