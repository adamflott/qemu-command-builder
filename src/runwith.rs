use crate::common::OnOff;
use crate::parsers::{DELIM_COLON, DELIM_COMMA};
use crate::shell_path::{ShellPath, shell_path_until_comma};
use crate::shell_string::{ShellString, ShellStringError, shell_string_until_comma};
use crate::to_command::ToCommand;
use crate::{pco0, qao};
use bon::Builder;
use proptest_derive::Arbitrary;
use std::str::FromStr;
use winnow::Result;
use winnow::ascii::{alphanumeric1, dec_uint};
use winnow::combinator::{alt, opt};
use winnow::prelude::*;
use winnow::token::literal;

pub(crate) const ARG_RUN_WITH: &str = "-run-with";

const KEY_ASYNC_TEARDOWN: &str = "async-teardown=";
const KEY_CHROOT: &str = "chroot=";
const KEY_USER: &str = "user=";

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum UserOrIds {
    User(ShellString),
    Id { uid: usize, gid: usize },
}
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct RunWith {
    async_teardown: Option<OnOff>,
    chroot: Option<ShellPath>,
    user: Option<UserOrIds>,
}

impl ToCommand for RunWith {
    fn has_args(&self) -> bool {
        self.async_teardown.is_some() || self.chroot.is_some() || self.user.is_some()
    }
    fn command(&self) -> String {
        ARG_RUN_WITH.to_string()
    }
    fn to_args(&self) -> Vec<String> {
        let mut args = vec![];

        qao!(&self.async_teardown, args, KEY_ASYNC_TEARDOWN);
        if let Some(chroot) = &self.chroot {
            args.push(format!("{}{}", KEY_CHROOT, chroot.as_ref()));
        }

        if let Some(user) = &self.user {
            match user {
                UserOrIds::User(id) => {
                    args.push(format!("{}{}", KEY_USER, id));
                }
                UserOrIds::Id { uid, gid } => {
                    args.push(format!("{}{}:{}", KEY_USER, uid, gid));
                }
            }
        }
        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for RunWith {
    type Err = ShellStringError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        run_with.parse(s).map_err(|e| ShellStringError::from_parse(e))
    }
}

pco0!(async_teardown, alphanumeric1, OnOff, KEY_ASYNC_TEARDOWN);
pco0!(chroot, shell_path_until_comma, ShellPath, KEY_CHROOT);

fn user(s: &mut &str) -> ModalResult<UserOrIds> {
    let _ = opt(literal(DELIM_COMMA)).parse_next(s)?;
    let _ = opt(literal(KEY_USER)).parse_next(s)?;
    alt((user_id, user_name)).parse_next(s)
}

fn user_name(s: &mut &str) -> ModalResult<UserOrIds> {
    let name = shell_string_until_comma.parse_to::<ShellString>().parse_next(s)?;
    Ok(UserOrIds::User(name))
}

fn user_id(s: &mut &str) -> ModalResult<UserOrIds> {
    let uid = dec_uint.parse_next(s)?;
    let _ = literal(DELIM_COLON).parse_next(s)?;
    let gid = dec_uint.parse_next(s)?;
    Ok(UserOrIds::Id { uid, gid })
}

pub fn run_with(s: &mut &str) -> ModalResult<RunWith> {
    let async_teardown = opt(async_teardown).parse_next(s)?;
    let chroot = opt(chroot).parse_next(s)?;
    let user = opt(user).parse_next(s)?;
    Ok(RunWith { async_teardown, chroot, user })
}
