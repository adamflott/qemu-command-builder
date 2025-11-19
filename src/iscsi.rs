use bon::Builder;
use proptest_derive::Arbitrary;
use std::str::FromStr;

use crate::to_command::ToCommand;

pub(crate) const ARG_ISCSI: &str = "-iscsi";

/// Configure iSCSI session parameters.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
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
    fn command(&self) -> String {
        ARG_ISCSI.to_string()
    }
    fn to_args(&self) -> Vec<String> {
        let mut args = vec![];

        if let Some(user) = &self.user {
            args.push(format!("user={}", user));
        }
        if let Some(password) = &self.password {
            args.push(format!("password={}", password));
        }
        if let Some(password_secret) = &self.password_secret {
            args.push(format!("password-secret={}", password_secret));
        }
        if let Some(header_digest) = &self.header_digest {
            args.push(format!("header-digest={}", header_digest));
        }
        if let Some(initiator_name) = &self.initiator_name {
            args.push(format!("initiator-name={}", initiator_name));
        }
        if let Some(id) = &self.id {
            args.push(format!("id={}", id));
        }
        if let Some(timeout) = self.timeout {
            args.push(format!("timeout={}", timeout));
        }

        args
    }
}

impl FromStr for Iscsi {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
