use bon::Builder;

use crate::to_command::ToCommand;

/// Configure iSCSI session parameters.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder)]
pub struct Iscsi {
    user: Option<String>,
    password: Option<String>,
    password_secret: Option<String>,
    header_digest: Option<String>,
    initiator_name: Option<String>,
    id: Option<String>,
    timeout: Option<usize>,
}

impl ToCommand for Iscsi {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec![];

        cmd.push("-iscsi".to_string());

        let mut arg = vec![];

        if let Some(user) = &self.user {
            arg.push(format!("user={}", user));
        }
        if let Some(password) = &self.password {
            arg.push(format!("password={}", password));
        }
        if let Some(password_secret) = &self.password_secret {
            arg.push(format!("password-secret={}", password_secret));
        }
        if let Some(header_digest) = &self.header_digest {
            arg.push(format!("header-digest={}", header_digest));
        }
        if let Some(initiator_name) = &self.initiator_name {
            arg.push(format!("initiator-name={}", initiator_name));
        }
        if let Some(id) = &self.id {
            arg.push(format!("id={}", id));
        }
        if let Some(timeout) = self.timeout {
            arg.push(format!("timeout={}", timeout));
        }

        cmd.push(arg.join(","));

        cmd
    }
}
