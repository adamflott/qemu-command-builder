use crate::common::OnOff;
use crate::to_command::{ToArg, ToCommand};
use bon::Builder;
use std::path::PathBuf;

#[derive(Builder)]
pub struct Tcp {
    host: Option<String>,
    port: u16,
    to: Option<u16>,
    ipv4: Option<OnOff>,
    ipv6: Option<OnOff>,
}

#[derive(Builder)]
pub struct Rdma {
    host: String,
    port: u16,
    ipv4: Option<OnOff>,
    ipv6: Option<OnOff>,
}

#[derive(Builder)]
pub struct File {
    filename: PathBuf,
    offset: Option<String>,
}

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
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec![];

        cmd.push("-incoming".to_string());

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
                cmd.push(args.join(","));
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
                cmd.push(args.join(","));
            }
            Incoming::Unix(unix) => {
                cmd.push(format!("unix:{}", unix.display()));
            }
            Incoming::Fd(fd) => {
                cmd.push(format!("fd:{}", fd));
            }
            Incoming::File(file) => {
                let mut args = vec![format!("file:{}", file.filename.display())];
                if let Some(offset) = &file.offset {
                    args.push(format!("offset={}", offset));
                }
                cmd.push(args.join(","));
            }
            Incoming::Exec(exec) => {
                cmd.push(format!("exec:{}", exec));
            }
            Incoming::Channel(chrono) => {
                cmd.push(format!("channel:{}", chrono));
            }
            Incoming::Defer => {
                cmd.push("defer".to_string());
            }
        }

        cmd
    }
}
