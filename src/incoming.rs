use std::path::PathBuf;
use std::str::FromStr;

use bon::Builder;
use proptest_derive::Arbitrary;

use crate::common::OnOff;
use crate::to_command::{ToArg, ToCommand};
use crate::parsers::DELIM_COMMA;

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
                vec![args.join(DELIM_COMMA)]
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
                vec![args.join(DELIM_COMMA)]
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
                vec![args.join(DELIM_COMMA)]
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
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "defer" {
            return Ok(Self::Defer);
        }
        if let Some(rest) = s.strip_prefix("tcp:") {
            let mut parts = rest.split(DELIM_COMMA);
            let endpoint = parts.next().ok_or_else(|| "invalid incoming tcp endpoint".to_string())?;
            let (host, port) = parse_optional_host_port(endpoint)?;
            let mut to = None;
            let mut ipv4 = None;
            let mut ipv6 = None;
            for part in parts {
                let (key, value) = part.split_once('=').ok_or_else(|| format!("invalid incoming tcp option: {part}"))?;
                match key {
                    "to" => to = Some(value.parse::<u16>().map_err(|e| e.to_string())?),
                    "ipv4" => ipv4 = Some(value.parse::<OnOff>().map_err(|_| format!("invalid ipv4 value: {value}"))?),
                    "ipv6" => ipv6 = Some(value.parse::<OnOff>().map_err(|_| format!("invalid ipv6 value: {value}"))?),
                    other => return Err(format!("unsupported incoming tcp option: {other}")),
                }
            }
            return Ok(Self::Tcp(Tcp { host, port, to, ipv4, ipv6 }));
        }
        if let Some(rest) = s.strip_prefix("rdma:") {
            let mut parts = rest.split(DELIM_COMMA);
            let endpoint = parts.next().ok_or_else(|| "invalid incoming rdma endpoint".to_string())?;
            let (host, port) = endpoint.rsplit_once(':').ok_or_else(|| format!("invalid rdma endpoint: {endpoint}"))?;
            let mut ipv4 = None;
            let mut ipv6 = None;
            for part in parts {
                let (key, value) = part.split_once('=').ok_or_else(|| format!("invalid incoming rdma option: {part}"))?;
                match key {
                    "ipv4" => ipv4 = Some(value.parse::<OnOff>().map_err(|_| format!("invalid ipv4 value: {value}"))?),
                    "ipv6" => ipv6 = Some(value.parse::<OnOff>().map_err(|_| format!("invalid ipv6 value: {value}"))?),
                    other => return Err(format!("unsupported incoming rdma option: {other}")),
                }
            }
            return Ok(Self::Rdma(Rdma {
                host: host.to_string(),
                port: port.parse::<u16>().map_err(|e| e.to_string())?,
                ipv4,
                ipv6,
            }));
        }
        if let Some(path) = s.strip_prefix("unix:") {
            return Ok(Self::Unix(PathBuf::from(path)));
        }
        if let Some(fd) = s.strip_prefix("fd:") {
            return Ok(Self::Fd(fd.to_string()));
        }
        if let Some(rest) = s.strip_prefix("file:") {
            let mut parts = rest.split(DELIM_COMMA);
            let filename = PathBuf::from(parts.next().ok_or_else(|| "invalid incoming file endpoint".to_string())?);
            let mut offset = None;
            for part in parts {
                let (key, value) = part.split_once('=').ok_or_else(|| format!("invalid incoming file option: {part}"))?;
                match key {
                    "offset" => offset = Some(value.to_string()),
                    other => return Err(format!("unsupported incoming file option: {other}")),
                }
            }
            return Ok(Self::File(File { filename, offset }));
        }
        if let Some(exec) = s.strip_prefix("exec:") {
            return Ok(Self::Exec(exec.to_string()));
        }
        if s.starts_with('{') || s.contains("addr.") || s.contains(',') {
            return Ok(Self::Channel(s.to_string()));
        }
        Ok(Self::Channel(s.to_string()))
    }
}

fn parse_optional_host_port(value: &str) -> Result<(Option<String>, u16), String> {
    let (host, port) = value.rsplit_once(':').ok_or_else(|| format!("invalid host:port endpoint: {value}"))?;
    let host = if host.is_empty() { None } else { Some(host.to_string()) };
    Ok((host, port.parse::<u16>().map_err(|e| e.to_string())?))
}
