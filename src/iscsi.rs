use bon::Builder;
use proptest_derive::Arbitrary;
use std::str::FromStr;

use crate::parsers::DELIM_COMMA;
use crate::to_command::ToCommand;

pub(crate) const ARG_ISCSI: &str = "-iscsi";

/// Configure iSCSI session parameters.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct Iscsi {
    /// CHAP username.
    user: Option<String>,
    /// CHAP password.
    password: Option<String>,
    /// Secret object id containing the password.
    password_secret: Option<String>,
    /// Header digest mode.
    header_digest: Option<String>,
    /// iSCSI initiator name.
    initiator_name: Option<String>,
    /// Optional initiator id.
    id: Option<String>,
    /// Timeout in seconds.
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

        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for Iscsi {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut user = None;
        let mut password = None;
        let mut password_secret = None;
        let mut header_digest = None;
        let mut initiator_name = None;
        let mut id = None;
        let mut timeout = None;

        for part in s.split(DELIM_COMMA).filter(|part| !part.is_empty()) {
            let (key, value) = part.split_once('=').ok_or_else(|| format!("invalid -iscsi option: {part}"))?;
            match key {
                "user" => user = Some(value.to_string()),
                "password" => password = Some(value.to_string()),
                "password-secret" => password_secret = Some(value.to_string()),
                "header-digest" => header_digest = Some(value.to_string()),
                "initiator-name" => initiator_name = Some(value.to_string()),
                "id" => id = Some(value.to_string()),
                "timeout" => timeout = Some(value.parse::<usize>().map_err(|e| e.to_string())?),
                other => return Err(format!("unsupported -iscsi option: {other}")),
            }
        }

        Ok(Self {
            user,
            password,
            password_secret,
            header_digest,
            initiator_name,
            id,
            timeout,
        })
    }
}
