use std::path::PathBuf;

use bon::Builder;

use crate::common::OnOff;
use crate::to_command::{ToArg, ToCommand};

#[derive(Builder)]
pub struct VC {
    is_pixel: bool,
    w: usize,
    h: usize,
}

#[derive(Builder)]
pub struct Udp {
    remote_host: Option<String>,
    remote_port: u16,
    src_ip: Option<String>,
    src_port: Option<u16>,
}

#[derive(Builder)]
pub struct Tcp {
    host: String,
    port: u16,
    server: Option<OnOff>,
    wait: Option<OnOff>,
    nodelay: Option<OnOff>,
    reconnect_ms: Option<usize>,
}

#[derive(Builder)]
pub struct Telnet {
    host: String,
    port: u16,
    server: Option<OnOff>,
    wait: Option<OnOff>,
    nodelay: Option<OnOff>,
}

#[derive(Builder)]
pub struct Websocket {
    host: String,
    port: u16,
    server: Option<OnOff>,
    wait: Option<OnOff>,
    nodelay: Option<OnOff>,
}

#[derive(Builder)]
pub struct Unix {
    path: PathBuf,
    server: Option<OnOff>,
    wait: Option<OnOff>,
    reconnect_ms: Option<usize>,
}

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
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec![];

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
        cmd.push(args.join("").to_string());
        cmd
    }
}
