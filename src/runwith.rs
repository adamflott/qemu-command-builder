use crate::common::OnOff;
use crate::to_command::{ToArg, ToCommand};
use bon::Builder;
use std::path::PathBuf;

pub enum UserOrIds {
    User(String),
    Id { uid: usize, gid: usize },
}
#[derive(Default, Builder)]
pub struct RunWith {
    async_teardown: Option<OnOff>,
    chroot: Option<PathBuf>,
    user: Option<UserOrIds>,
}

impl ToCommand for RunWith {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec![];

        cmd.push("-run-with".to_string());

        let mut args = vec![];

        if let Some(async_teardown) = &self.async_teardown {
            args.push(format!("async-teardown={}", async_teardown.to_arg()))
        }
        if let Some(chroot) = &self.chroot {
            args.push(format!("chroot={}", chroot.display()));
        }
        if let Some(user) = &self.user {
            match user {
                UserOrIds::User(id) => {
                    args.push(format!("user={}", id));
                }
                UserOrIds::Id { uid, gid } => {
                    args.push(format!("user={}:{}", uid, gid));
                }
            }
        }
        cmd.push(args.join(","));
        cmd
    }
}
