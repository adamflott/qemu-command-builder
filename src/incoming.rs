use std::path::PathBuf;
use std::str::FromStr;

use bon::Builder;
use proptest_derive::Arbitrary;

use crate::common::OnOff;
use crate::to_command::{ToArg, ToCommand};

pub(crate) const ARG_INCOMING: &str = "-incoming";

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct Tcp {
    host: Option<String>,
    port: u16,
    to: Option<u16>,
    ipv4: Option<OnOff>,
    ipv6: Option<OnOff>,
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct Rdma {
    host: String,
    port: u16,
    ipv4: Option<OnOff>,
    ipv6: Option<OnOff>,
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct File {
    filename: PathBuf,
    offset: Option<String>,
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum Incoming {
    Tcp(Tcp),
    Rdma(Rdma),
    Unix(PathBuf),
    Fd(String),
    File(File),
    Exec(String),
    Channel(String),
    Defer,
}

impl ToCommand for Incoming {
    fn command(&self) -> String {
        ARG_INCOMING.to_string()
    }
    fn to_args(&self) -> Vec<String> {
        match self {
            Incoming::Tcp(tcp) => {
                let mut args = vec![];
                if let Some(host) = &tcp.host {
                    args.push(format!("tcp:{}:{}", host, tcp.port));
                } else {
                    args.push(format!("tcp::{}", tcp.port));
                }
                if let Some(to) = &tcp.to {
                    args.push(format!("to={}", to));
                }
                if let Some(ipv4) = &tcp.ipv4 {
                    args.push(format!("ipv4={}", ipv4.to_arg()));
                }
                if let Some(ipv6) = &tcp.ipv6 {
                    args.push(format!("ipv6={}", ipv6.to_arg()));
                }
                args
            }
            Incoming::Rdma(rdma) => {
                let mut args = vec![];
                args.push(format!("rdma:{}:{}", rdma.host, rdma.port));
                if let Some(ipv4) = &rdma.ipv4 {
                    args.push(format!("ipv4={}", ipv4.to_arg()));
                }
                if let Some(ipv6) = &rdma.ipv6 {
                    args.push(format!("ipv6={}", ipv6.to_arg()));
                }
                args
            }
            Incoming::Unix(unix) => {
                vec![format!("unix:{}", unix.display())]
            }
            Incoming::Fd(fd) => {
                vec![format!("fd:{}", fd)]
            }
            Incoming::File(file) => {
                let mut args = vec![format!("file:{}", file.filename.display())];
                if let Some(offset) = &file.offset {
                    args.push(format!("offset={}", offset));
                }
                args
            }
            Incoming::Exec(exec) => {
                vec![format!("exec:{}", exec)]
            }
            Incoming::Channel(chrono) => {
                vec![format!("channel:{}", chrono)]
            }
            Incoming::Defer => {
                vec!["defer".to_string()]
            }
        }
    }
}

impl FromStr for Incoming {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
