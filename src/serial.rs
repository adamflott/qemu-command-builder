use bon::Builder;
use proptest_derive::Arbitrary;
use std::path::PathBuf;
use std::str::FromStr;

use crate::common::OnOff;
use crate::to_command::{ToArg, ToCommand};

pub(crate) const ARG_SERIAL: &str = "-serial";
pub(crate) const ARG_PARALLEL: &str = "-parallel";

/// A QEMU special character device target used by options such as
/// `-serial`, `-parallel`, `-monitor`, and `-qmp`.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct VC {
    is_pixel: bool,
    w: usize,
    h: usize,
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct Udp {
    remote_host: Option<String>,
    remote_port: u16,
    src_ip: Option<String>,
    src_port: Option<u16>,
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct Tcp {
    host: String,
    port: u16,
    server: Option<OnOff>,
    wait: Option<OnOff>,
    nodelay: Option<OnOff>,
    reconnect_ms: Option<usize>,
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct Telnet {
    host: String,
    port: u16,
    server: Option<OnOff>,
    wait: Option<OnOff>,
    nodelay: Option<OnOff>,
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct Websocket {
    host: String,
    port: u16,
    server: Option<OnOff>,
    wait: Option<OnOff>,
    nodelay: Option<OnOff>,
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct Unix {
    path: PathBuf,
    server: Option<OnOff>,
    wait: Option<OnOff>,
    reconnect_ms: Option<usize>,
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum SpecialDevice {
    VC(Option<VC>),
    Pty(Option<PathBuf>),
    None,
    Null,
    Chardev(String),
    Dev(String),
    Parport(usize),
    File(PathBuf),
    Stdio,
    Pipe(PathBuf),
    Com(usize),
    Udp(Udp),
    Tcp(Tcp),
    Telnet(Telnet),
    Websocket(Websocket),
    Unix(Unix),
    Mon(String),
    Braille,
    Msmouse,
}

impl ToCommand for SpecialDevice {
    fn to_args(&self) -> Vec<String> {
        let mut args = vec![];

        match self {
            SpecialDevice::VC(vc) => {
                if let Some(vc) = vc {
                    if vc.is_pixel {
                        args.push(format!("vc:{}x{}", vc.w, vc.h));
                    } else {
                        args.push(format!("vc:{}Cx{}C", vc.w, vc.h));
                    }
                } else {
                    args.push("vc".to_string());
                }
            }
            SpecialDevice::Pty(pty) => {
                if let Some(pty) = pty {
                    args.push(format!("pty:{}", pty.display()));
                } else {
                    args.push("pty".to_string());
                }
            }
            SpecialDevice::None => {
                args.push("none".to_string());
            }
            SpecialDevice::Null => {
                args.push("null".to_string());
            }
            SpecialDevice::Chardev(chardev) => {
                args.push(format!("chardev:{}", chardev));
            }
            SpecialDevice::Dev(dev) => {
                args.push(format!("/dev/{}", dev));
            }
            SpecialDevice::Parport(parport) => {
                args.push(format!("/dev/parport{}", parport));
            }
            SpecialDevice::File(file) => {
                args.push(format!("file:{}", file.display()));
            }
            SpecialDevice::Stdio => {
                args.push("stdio".to_string());
            }
            SpecialDevice::Pipe(pipe) => {
                args.push(format!("pipe:{}", pipe.display()));
            }
            SpecialDevice::Com(com) => {
                args.push(format!("COM{}", com));
            }
            SpecialDevice::Udp(udp) => {
                let mut udpargs = vec![];

                udpargs.push("udp".to_string());
                if let Some(remote_host) = &udp.remote_host {
                    udpargs.push(remote_host.to_string());
                }
                udpargs.push(format!("{}", udp.remote_port));
                if let Some(src_ip) = &udp.src_ip {
                    udpargs.push(format!("@{}", src_ip));
                }
                if let Some(src_port) = &udp.src_port {
                    udpargs.push(format!("{}", src_port));
                }
                args.push(udpargs.join(":"));
            }
            SpecialDevice::Tcp(tcp) => {
                let mut tcpargs = vec![];

                tcpargs.push(format!("tcp:{}:{}", tcp.host, tcp.port));
                if let Some(server) = &tcp.server {
                    tcpargs.push(format!("server={}", server.to_arg()));
                }
                if let Some(wait) = &tcp.wait {
                    tcpargs.push(format!("wait={}", wait.to_arg()));
                }
                if let Some(nodelay) = &tcp.nodelay {
                    tcpargs.push(format!("nodelay={}", nodelay.to_arg()));
                }
                if let Some(reconnect_ms) = &tcp.reconnect_ms {
                    tcpargs.push(format!("reconnect-ms={}", reconnect_ms));
                }
                args.push(tcpargs.join(","));
            }
            SpecialDevice::Telnet(telnet) => {
                let mut telnetargs = vec![];

                telnetargs.push(format!("telnet:{}:{}", telnet.host, telnet.port));
                if let Some(server) = &telnet.server {
                    telnetargs.push(format!("server={}", server.to_arg()));
                }
                if let Some(wait) = &telnet.wait {
                    telnetargs.push(format!("wait={}", wait.to_arg()));
                }
                if let Some(nodelay) = &telnet.nodelay {
                    telnetargs.push(format!("nodelay={}", nodelay.to_arg()));
                }
                args.push(telnetargs.join(","));
            }
            SpecialDevice::Websocket(ws) => {
                let mut wsargs = vec![];

                wsargs.push(format!("websocket:{}:{}", ws.host, ws.port));
                if let Some(server) = &ws.server {
                    wsargs.push(format!("server={}", server.to_arg()));
                }
                if let Some(wait) = &ws.wait {
                    wsargs.push(format!("wait={}", wait.to_arg()));
                }
                if let Some(nodelay) = &ws.nodelay {
                    wsargs.push(format!("nodelay={}", nodelay.to_arg()));
                }
                args.push(wsargs.join(","));
            }
            SpecialDevice::Unix(uds) => {
                let mut udsargs = vec![];

                udsargs.push(format!("unix:{}", uds.path.display()));
                if let Some(server) = &uds.server {
                    udsargs.push(format!("server={}", server.to_arg()));
                }
                if let Some(wait) = &uds.wait {
                    udsargs.push(format!("wait={}", wait.to_arg()));
                }
                if let Some(reconnect_ms) = &uds.reconnect_ms {
                    udsargs.push(format!("reconnect-ms={}", reconnect_ms));
                }
                args.push(udsargs.join(","));
            }
            SpecialDevice::Mon(mon) => {
                args.push(format!("mon:{}", mon));
            }
            SpecialDevice::Braille => {
                args.push("braille".to_string());
            }
            SpecialDevice::Msmouse => {
                args.push("msmouse".to_string());
            }
        }
        args
    }
}

impl FromStr for SpecialDevice {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "none" {
            return Ok(Self::None);
        }
        if s == "null" {
            return Ok(Self::Null);
        }
        if s == "stdio" {
            return Ok(Self::Stdio);
        }
        if s == "braille" {
            return Ok(Self::Braille);
        }
        if s == "msmouse" {
            return Ok(Self::Msmouse);
        }
        if let Some(chardev) = s.strip_prefix("chardev:") {
            return Ok(Self::Chardev(chardev.to_string()));
        }
        if let Some(mon) = s.strip_prefix("mon:") {
            return Ok(Self::Mon(mon.to_string()));
        }
        if let Some(path) = s.strip_prefix("file:") {
            return Ok(Self::File(PathBuf::from(path)));
        }
        if let Some(path) = s.strip_prefix("pipe:") {
            return Ok(Self::Pipe(PathBuf::from(path)));
        }
        if let Some(path) = s.strip_prefix("pty:") {
            return Ok(Self::Pty(Some(PathBuf::from(path))));
        }
        if s == "pty" {
            return Ok(Self::Pty(None));
        }
        if let Some(dev) = s.strip_prefix("/dev/parport") {
            let index = dev.parse::<usize>().map_err(|e| e.to_string())?;
            return Ok(Self::Parport(index));
        }
        if let Some(dev) = s.strip_prefix("/dev/") {
            return Ok(Self::Dev(dev.to_string()));
        }
        if let Some(port) = s.strip_prefix("COM") {
            return Ok(Self::Com(port.parse::<usize>().map_err(|e| e.to_string())?));
        }

        Err(format!("unsupported special device: {s}"))
    }
}
