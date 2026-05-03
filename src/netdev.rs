use crate::chardev::CharDev;
use crate::common::OnOff;
use crate::parsers::DELIM_COMMA;
use crate::to_command::ToArg;
use crate::to_command::ToCommand;
use crate::{QIpv4Net, QIpv6Net};
use bon::Builder;
use proptest_derive::Arbitrary;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::path::PathBuf;
use std::str::FromStr;

pub(crate) const ARG_NETDEV: &str = "-netdev";

/// A QEMU `-netdev` backend.
///
/// The parser supports the same canonical comma-separated forms that the
/// formatter emits for the implemented backend variants. The current round-trip
/// coverage focuses on `tap`.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct SMB {
    dir: PathBuf,
    smbserver: Option<String>,
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Arbitrary)]
pub enum TcpUdp {
    #[default]
    Tcp,
    Udp,
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum ScriptOrNot {
    Script(PathBuf),
    None,
}

impl ToCommand for ScriptOrNot {
    fn to_args(&self) -> Vec<String> {
        match self {
            ScriptOrNot::Script(path) => {
                vec![path.display().to_string()]
            }
            ScriptOrNot::None => {
                vec!["no".to_string()]
            }
        }
    }
}

impl FromStr for ScriptOrNot {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "no" {
            Ok(Self::None)
        } else {
            Ok(Self::Script(PathBuf::from(s)))
        }
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct HostForward {
    protocol: Option<TcpUdp>,
    hostaddr: Option<String>,
    hostport: u16,
    guestaddr: Option<String>,
    guestport: u16,
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum GuestForwardTarget {
    Device(CharDev),
    Cmd((String, Vec<String>)),
}
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct GuestForward {
    server: String,
    port: u16,
    target: GuestForwardTarget,
}
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct User {
    id: String,
    ipv4: Option<OnOff>,
    net: Option<QIpv4Net>,
    host: Option<Ipv4Addr>,
    ipv6: Option<OnOff>,
    ipv6_net: Option<QIpv6Net>,
    ipv6_host: Option<Ipv6Addr>,
    restrict: Option<OnOff>,
    hostname: Option<String>,
    dhcpstart: Option<Ipv4Addr>,
    dns: Option<Ipv4Addr>,
    ipv6_dns: Option<Ipv6Addr>,
    dnssearch: Option<Vec<String>>,
    domainname: Option<String>,
    tftp: Option<PathBuf>,
    tftp_server_name: Option<String>,
    bootfile: Option<PathBuf>,
    smb: Option<SMB>,
    hostfwd: Option<Vec<HostForward>>,
    guestfwd: Option<Vec<GuestForward>>,
}

impl ToCommand for User {
    fn to_args(&self) -> Vec<String> {
        let mut args = vec!["user".to_string(), format!("id={}", self.id.to_string())];

        if let Some(ipv4) = &self.ipv4 {
            args.push(format!("ipv4={}", ipv4.to_arg()));
        }
        if let Some(net) = &self.net {
            args.push(format!("net={}", net.ip));
        }
        if let Some(host) = &self.host {
            args.push(format!("host={}", host));
        }
        if let Some(ipv6) = &self.ipv6 {
            args.push(format!("ipv6={}", ipv6.to_arg()));
        }
        if let Some(ipv6_net) = &self.ipv6_net {
            args.push(format!("ipv6-net={}", ipv6_net.ip));
        }
        if let Some(ipv6_host) = &self.ipv6_host {
            args.push(format!("ipv6-host={}", ipv6_host));
        }
        if let Some(restrict) = &self.restrict {
            args.push(format!("restrict={}", restrict.to_arg()));
        }
        if let Some(hostname) = &self.hostname {
            args.push(format!("hostname={}", hostname));
        }
        if let Some(dhcpstart) = &self.dhcpstart {
            args.push(format!("dhcpstart={}", dhcpstart));
        }
        if let Some(dns) = &self.dns {
            args.push(format!("dns={}", dns));
        }
        if let Some(ipv6_dns) = &self.ipv6_dns {
            args.push(format!("ipv6-dns={}", ipv6_dns));
        }
        if let Some(dnssearch) = &self.dnssearch {
            args.push(format!("dnssearch={}", dnssearch.join(",")));
        }
        if let Some(domainname) = &self.domainname {
            args.push(format!("domainname={}", domainname));
        }
        if let Some(tftp) = &self.tftp {
            args.push(format!("tftp={}", tftp.display()));
        }
        if let Some(tftp_server_name) = &self.tftp_server_name {
            args.push(format!("tftp-server-name={}", tftp_server_name));
        }
        if let Some(bootfile) = &self.bootfile {
            args.push(format!("bootfile={}", bootfile.display()));
        }
        if let Some(smb) = &self.smb {
            args.push(format!("smb={}", smb.dir.display()));
            if let Some(smbserver) = &smb.smbserver {
                args.push(format!("smbserver={}", smbserver));
            }
        }
        if let Some(hostfwds) = &self.hostfwd {
            for hostfwd in hostfwds {
                let mut subargs = vec![];
                if let Some(proto) = &hostfwd.protocol {
                    match proto {
                        TcpUdp::Tcp => {
                            subargs.push("tcp".to_string());
                        }
                        TcpUdp::Udp => {
                            subargs.push("udp".to_string());
                        }
                    }
                }
                if let Some(hostaddr) = &hostfwd.hostaddr {
                    subargs.push(hostaddr.to_string());
                }
                subargs.push(format!("{}-", hostfwd.hostport));
                if let Some(guestaddr) = &hostfwd.guestaddr {
                    subargs.push(guestaddr.to_string());
                }
                subargs.push(format!("{}", hostfwd.guestport));
                args.push(subargs.join(";"));
            }
        }
        if let Some(guestfwds) = &self.guestfwd {
            for guestfwd in guestfwds {
                let mut subargs = vec!["tcp".to_string()];

                subargs.push(guestfwd.server.to_string());
                subargs.push(format!("{}", guestfwd.port));

                match &guestfwd.target {
                    GuestForwardTarget::Device(dev) => {
                        subargs.push(format!("device={}", dev.to_command().join(" ")));
                    }
                    GuestForwardTarget::Cmd((cmd, args)) => {
                        subargs.push(format!("cmd:{} {}", cmd, args.join(" ")));
                    }
                }
                args.push(subargs.join(":"));
            }
        }
        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for User {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct Tap {
    id: String,
    fd: Option<String>,
    fds: Option<Vec<String>>,
    ifname: Option<String>,
    script: Option<ScriptOrNot>,
    downscript: Option<ScriptOrNot>,
    br: Option<String>,
    helper: Option<String>,
    sndbuf: Option<usize>,
    vnet_hdr: Option<OnOff>,
    vhost: Option<OnOff>,
    vhostfd: Option<String>,
    vhostforce: Option<OnOff>,
    queues: Option<usize>,
    poll_us: Option<usize>,
}

impl ToCommand for Tap {
    fn to_args(&self) -> Vec<String> {
        let mut args = vec!["tap".to_string(), format!("id={}", self.id.to_string())];

        if let Some(fd) = &self.fd {
            args.push(format!("fd={}", fd));
        }
        if let Some(fds) = &self.fds {
            args.push(format!("fds={}", fds.join(":")));
        }
        if let Some(ifname) = &self.ifname {
            args.push(format!("ifname={}", ifname));
        }
        if let Some(script) = &self.script {
            args.push(format!("script={}", script.to_command().join("")));
        }
        if let Some(downscript) = &self.downscript {
            args.push(format!("downscript={}", downscript.to_command().join("")));
        }
        if let Some(br) = &self.br {
            args.push(format!("br={}", br));
        }
        if let Some(helper) = &self.helper {
            args.push(format!("helper={}", helper));
        }
        if let Some(sndbuf) = self.sndbuf {
            args.push(format!("sndbuf={}", sndbuf));
        }
        if let Some(vnet_hdr) = &self.vnet_hdr {
            args.push(format!("vnet_hdr={}", vnet_hdr.to_arg()));
        }
        if let Some(vhost) = &self.vhost {
            args.push(format!("vhost={}", vhost.to_arg()));
        }
        if let Some(vhostfd) = &self.vhostfd {
            args.push(format!("vhostfd={}", vhostfd));
        }
        if let Some(vhostforce) = &self.vhostforce {
            args.push(format!("vhostforce={}", vhostforce.to_arg()));
        }
        if let Some(queues) = self.queues {
            args.push(format!("queues={}", queues));
        }
        if let Some(poll_us) = self.poll_us {
            args.push(format!("poll_us={}", poll_us));
        }

        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for Tap {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let backend = parts.next().ok_or_else(|| "empty netdev argument".to_string())?;
        if backend != "tap" {
            return Err(format!("expected tap backend, got {backend}"));
        }

        let mut id = None;
        let mut fd = None;
        let mut fds = None;
        let mut ifname = None;
        let mut script = None;
        let mut downscript = None;
        let mut br = None;
        let mut helper = None;
        let mut sndbuf = None;
        let mut vnet_hdr = None;
        let mut vhost = None;
        let mut vhostfd = None;
        let mut vhostforce = None;
        let mut queues = None;
        let mut poll_us = None;

        for part in parts {
            let (key, value) = part.split_once('=').ok_or_else(|| format!("invalid tap option: {part}"))?;
            match key {
                "id" => id = Some(value.to_string()),
                "fd" => fd = Some(value.to_string()),
                "fds" => fds = Some(value.split(':').map(|v| v.to_string()).collect()),
                "ifname" => ifname = Some(value.to_string()),
                "script" => script = Some(value.parse::<ScriptOrNot>()?),
                "downscript" => downscript = Some(value.parse::<ScriptOrNot>()?),
                "br" => br = Some(value.to_string()),
                "helper" => helper = Some(value.to_string()),
                "sndbuf" => sndbuf = Some(value.parse::<usize>().map_err(|e| e.to_string())?),
                "vnet_hdr" => vnet_hdr = Some(value.parse::<OnOff>().map_err(|_| format!("invalid vnet_hdr value: {value}"))?),
                "vhost" => vhost = Some(value.parse::<OnOff>().map_err(|_| format!("invalid vhost value: {value}"))?),
                "vhostfd" => vhostfd = Some(value.to_string()),
                "vhostforce" => vhostforce = Some(value.parse::<OnOff>().map_err(|_| format!("invalid vhostforce value: {value}"))?),
                "queues" => queues = Some(value.parse::<usize>().map_err(|e| e.to_string())?),
                "poll_us" => poll_us = Some(value.parse::<usize>().map_err(|e| e.to_string())?),
                other => return Err(format!("unsupported tap option: {other}")),
            }
        }

        Ok(Self {
            id: id.ok_or_else(|| "tap netdev requires id=".to_string())?,
            fd,
            fds,
            ifname,
            script,
            downscript,
            br,
            helper,
            sndbuf,
            vnet_hdr,
            vhost,
            vhostfd,
            vhostforce,
            queues,
            poll_us,
        })
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct Bridge {
    id: String,
    bridge: Option<String>,
    helper: Option<String>,
}

impl ToCommand for Bridge {
    fn to_args(&self) -> Vec<String> {
        let mut args = vec!["bridge".to_string(), format!("id={}", self.id)];

        if let Some(br) = &self.bridge {
            args.push(format!("bridge={}", br));
        }
        if let Some(helper) = &self.helper {
            args.push(format!("helper={}", helper));
        }
        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for Bridge {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct HostAndPort {
    host: String,
    port: u16,
}
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct HostAndMaybePort {
    host: String,
    port: Option<u16>,
}
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct SocketRegular {
    id: String,
    fd: Option<String>,
    listen: Option<HostAndMaybePort>,
    connection: Option<HostAndPort>,
}

impl ToCommand for SocketRegular {
    fn to_args(&self) -> Vec<String> {
        let mut args = vec!["socket".to_string(), format!("id={}", self.id)];

        if let Some(fd) = &self.fd {
            args.push(format!("fd={}", fd));
        }
        if let Some(listen) = &self.listen {
            if let Some(port) = &listen.port {
                args.push(format!("listen={}:{}", listen.host, port));
            } else {
                args.push(format!("listen={}", listen.host));
            }
            if let Some(connection) = &self.connection {
                args.push(format!("{}:{}", connection.host, connection.port));
            }
        }

        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for SocketRegular {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct SocketMulticast {
    id: String,
    fd: Option<String>,
    mcast: Option<HostAndPort>,
    localaddr: Option<String>,
}

impl ToCommand for SocketMulticast {
    fn to_args(&self) -> Vec<String> {
        let mut args = vec!["socket".to_string(), format!("id={}", self.id)];

        if let Some(fd) = &self.fd {
            args.push(format!("fd={}", fd));
        }
        if let Some(mcast) = &self.mcast {
            args.push(format!("mcast={}:{}", mcast.host, mcast.port));
        }
        if let Some(localaddr) = &self.localaddr {
            args.push(format!("localaddr={}", localaddr));
        }
        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for SocketMulticast {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct SocketUdpTunnel {
    id: String,
    fd: Option<String>,
    udp: Option<HostAndPort>,
    localaddr: Option<HostAndPort>,
}

impl ToCommand for SocketUdpTunnel {
    fn to_args(&self) -> Vec<String> {
        let mut args = vec!["socket".to_string(), format!("id={}", self.id)];

        if let Some(fd) = &self.fd {
            args.push(format!("fd={}", fd));
        }
        if let Some(udp) = &self.udp {
            args.push(format!("udp={}:{}", udp.host, udp.port));
        }
        if let Some(localaddr) = &self.localaddr {
            args.push(format!("localaddr={}:{}", localaddr.host, localaddr.port));
        }
        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for SocketUdpTunnel {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum Socket {
    SocketRegular(SocketRegular),
    Multicast(SocketMulticast),
    UDPTunnel(SocketUdpTunnel),
}

impl ToCommand for Socket {
    fn to_args(&self) -> Vec<String> {
        match self {
            Socket::SocketRegular(s) => s.to_args(),
            Socket::Multicast(s) => s.to_args(),
            Socket::UDPTunnel(s) => s.to_args(),
        }
    }
}

impl FromStr for Socket {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct StreamOverTcp {
    id: String,
    server: Option<OnOff>,
    addr_host: String,
    addr_port: u16,
    to: Option<u16>,
    numeric: Option<OnOff>,
    keep_alive: Option<OnOff>,
    mptcp: Option<OnOff>,
    addr_ipv4: Option<OnOff>,
    addr_ipv6: Option<OnOff>,
    reconnect_ms: Option<usize>,
}

impl ToCommand for StreamOverTcp {
    fn to_args(&self) -> Vec<String> {
        let mut args = vec!["stream".to_string(), format!("id={}", self.id)];

        if let Some(server) = &self.server {
            args.push(format!("server={}", server.to_arg()));
        }
        args.push("add.type=inet".to_string());
        args.push(format!("addr.host={}", self.addr_host));
        args.push(format!("addr.port={}", self.addr_port));
        if let Some(to) = &self.to {
            args.push(format!("to={}", to));
        }
        if let Some(numeric) = &self.numeric {
            args.push(format!("numeric={}", numeric.to_arg()));
        }
        if let Some(keep_alive) = &self.keep_alive {
            args.push(format!("keep-alive={}", keep_alive.to_arg()));
        }
        if let Some(mptcp) = &self.mptcp {
            args.push(format!("mptcp={}", mptcp.to_arg()));
        }
        if let Some(ipv4) = &self.addr_ipv4 {
            args.push(format!("addr.ipv4={}", ipv4.to_arg()));
        }
        if let Some(ipv6) = &self.addr_ipv6 {
            args.push(format!("addr.ipv6={}", ipv6.to_arg()));
        }
        if let Some(reconnect_ms) = self.reconnect_ms {
            args.push(format!("reconnect-ms={}", reconnect_ms));
        }
        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for StreamOverTcp {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct StreamOverUds {
    id: String,
    server: Option<OnOff>,
    addr_path: String,
    abstract_arg: Option<OnOff>,
    tight: Option<OnOff>,
    reconnect_ms: Option<usize>,
}

impl ToCommand for StreamOverUds {
    fn to_args(&self) -> Vec<String> {
        let mut args = vec!["stream".to_string(), format!("id={}", self.id)];

        if let Some(server) = &self.server {
            args.push(format!("server={}", server.to_arg()));
        }
        args.push("add.type=unix".to_string());
        args.push(format!("addr.path={}", self.addr_path));
        if let Some(abstract_arg) = &self.abstract_arg {
            args.push(format!("abstract={}", abstract_arg.to_arg()));
        }
        if let Some(tight) = &self.tight {
            args.push(format!("tight={}", tight.to_arg()));
        }
        if let Some(reconnect_ms) = self.reconnect_ms {
            args.push(format!("reconnect-ms={}", reconnect_ms));
        }
        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for StreamOverUds {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct StreamOverFd {
    id: String,
    server: Option<OnOff>,
    addr_str: String,
    reconnect_ms: Option<usize>,
}

impl ToCommand for StreamOverFd {
    fn to_args(&self) -> Vec<String> {
        let mut args = vec!["stream".to_string(), format!("id={}", self.id)];

        if let Some(server) = &self.server {
            args.push(format!("server={}", server.to_arg()));
        }
        args.push("add.type=fd".to_string());
        args.push(format!("addr.str={}", self.addr_str));
        if let Some(reconnect_ms) = self.reconnect_ms {
            args.push(format!("reconnect-ms={}", reconnect_ms));
        }
        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for StreamOverFd {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum Stream {
    StreamOverTcp(StreamOverTcp),
    StreamOverUds(StreamOverUds),
    StreamOverFd(StreamOverFd),
}

impl ToCommand for Stream {
    fn to_args(&self) -> Vec<String> {
        match self {
            Stream::StreamOverTcp(s) => s.to_args(),
            Stream::StreamOverUds(s) => s.to_args(),
            Stream::StreamOverFd(s) => s.to_args(),
        }
    }
}

impl FromStr for Stream {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct DgramMulticast {
    id: String,
    remote_host: String,
    remote_port: u16,
    local_host: Option<String>,
}

impl ToCommand for DgramMulticast {
    fn to_args(&self) -> Vec<String> {
        let mut args = vec![
            "dgram".to_string(),
            format!("id={}", self.id.to_string()),
            "remote.type=inet".to_string(),
            format!("remote.host={}", self.remote_host),
            format!("remote.port={}", self.remote_port),
        ];

        if let Some(local_host) = &self.local_host {
            args.push("local.type=inet".to_string());
            args.push(format!("local.host={}", local_host));
        }
        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for DgramMulticast {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct DgramMulticastUdpFd {
    id: String,
    remote_host: String,
    remote_port: u16,
    local_str: Option<String>,
}

impl ToCommand for DgramMulticastUdpFd {
    fn to_args(&self) -> Vec<String> {
        let mut args = vec![
            "dgram".to_string(),
            format!("id={}", self.id.to_string()),
            "remote.type=inet".to_string(),
            format!("remote.host={}", self.remote_host),
            format!("remote.port={}", self.remote_port),
        ];

        if let Some(local_str) = &self.local_str {
            args.push("local.type=fd".to_string());
            args.push(format!("local.str={}", local_str));
        }
        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for DgramMulticastUdpFd {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct DgramSocket {
    id: String,
    local_host: String,
    local_port: usize,
    remote_host: Option<String>,
    remote_port: Option<u16>,
}

impl ToCommand for DgramSocket {
    fn to_args(&self) -> Vec<String> {
        let mut args = vec![
            "dgram".to_string(),
            format!("id={}", self.id.to_string()),
            "local.type=inet".to_string(),
            format!("local.host={}", self.local_host),
            format!("local.port={}", self.local_port),
        ];

        if let Some(remote_host) = &self.remote_host {
            args.push("remote.type=inet".to_string());
            args.push(format!("remote.host={}", remote_host));
        }
        if let Some(remote_port) = &self.remote_port {
            args.push(format!("remote.port={}", remote_port));
        }
        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for DgramSocket {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct DgramUds {
    id: String,
    local_path: PathBuf,
    remote_path: Option<PathBuf>,
}

impl ToCommand for DgramUds {
    fn to_args(&self) -> Vec<String> {
        let mut args = vec![
            "dgram".to_string(),
            format!("id={}", self.id.to_string()),
            "local.type=unix".to_string(),
            format!("local.path={}", self.local_path.display()),
        ];
        if let Some(remote) = &self.remote_path {
            args.push("remote.type=unix".to_string());
            args.push(format!("remote.path={}", remote.display()));
        }
        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for DgramUds {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct DgramFd {
    id: String,
    local_str: String,
}

impl ToCommand for DgramFd {
    fn to_args(&self) -> Vec<String> {
        vec![
            vec![
                "dgram".to_string(),
                format!("id={}", self.id.to_string()),
                "local.type=fd".to_string(),
                format!("local.str={}", self.local_str),
            ]
            .join(DELIM_COMMA),
        ]
    }
}

impl FromStr for DgramFd {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum Dgram {
    DgramMulticast(DgramMulticast),
    DgramMulticastUdpFd(DgramMulticastUdpFd),
    DgramSocket(DgramSocket),
    DgramUds(DgramUds),
    DgramFd(DgramFd),
}

impl ToCommand for Dgram {
    fn to_args(&self) -> Vec<String> {
        match self {
            Dgram::DgramMulticast(args) => args.to_args(),
            Dgram::DgramMulticastUdpFd(args) => args.to_args(),
            Dgram::DgramSocket(args) => args.to_args(),
            Dgram::DgramUds(args) => args.to_args(),
            Dgram::DgramFd(args) => args.to_args(),
        }
    }
}

impl FromStr for Dgram {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct Vde {
    id: String,
    sock: Option<PathBuf>,
    port: Option<u16>,
    group: Option<String>,
    mode: Option<String>,
}

impl ToCommand for Vde {
    fn to_args(&self) -> Vec<String> {
        let mut args = vec!["vde".to_string(), format!("id={}", self.id.to_string())];

        if let Some(sock) = &self.sock {
            args.push(format!("sock={}", sock.display()));
        }
        if let Some(port) = self.port {
            args.push(format!("port={}", port));
        }
        if let Some(group) = &self.group {
            args.push(format!("group={}", group));
        }
        if let Some(mode) = &self.mode {
            args.push(format!("mode={}", mode));
        }
        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for Vde {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct NetMap {
    id: String,
    ifname: String,
    devname: Option<String>,
}

impl ToCommand for NetMap {
    fn to_args(&self) -> Vec<String> {
        let mut args = vec!["netmap".to_string(), format!("id={}", self.id.to_string())];
        args.push(format!("ifname={}", self.ifname));
        if let Some(devname) = &self.devname {
            args.push(format!("devname={}", devname));
        }
        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for NetMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum NativeSkb {
    Native,
    Skb,
}

impl ToArg for NativeSkb {
    fn to_arg(&self) -> &str {
        match self {
            NativeSkb::Native => "native",
            NativeSkb::Skb => "skb",
        }
    }
}
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct AfXdp {
    id: String,
    ifname: String,
    mode: Option<NativeSkb>,
    force_copy: Option<OnOff>,
    queues: Option<usize>,
    start_queue: Option<usize>,
    inhibit: Option<OnOff>,
    sock_fds: Option<Vec<String>>,
}

impl ToCommand for AfXdp {
    fn to_args(&self) -> Vec<String> {
        let mut args = vec!["af-xdp".to_string(), format!("id={}", self.id.to_string()), format!("ifname={}", self.ifname)];

        if let Some(mode) = &self.mode {
            args.push(format!("mode={}", mode.to_arg()));
        }
        if let Some(force_copy) = &self.force_copy {
            args.push(format!("force-copy={}", force_copy.to_arg()));
        }
        if let Some(queues) = self.queues {
            args.push(format!("queues={}", queues));
        }
        if let Some(start_queue) = self.start_queue {
            args.push(format!("start-queue={}", start_queue));
        }
        if let Some(inhibit) = &self.inhibit {
            args.push(format!("inhibit={}", inhibit.to_arg()));
        }
        if let Some(sock_fds) = &self.sock_fds {
            args.push(format!("sock-fds={}", sock_fds.join(":")));
        }
        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for AfXdp {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct VhostUser {
    id: String,
    chardev: String,
    vhostforce: Option<OnOff>,
}

impl ToCommand for VhostUser {
    fn to_args(&self) -> Vec<String> {
        let mut args = vec!["vhost-user".to_string(), format!("id={}", self.id.to_string()), format!("chardev={}", self.chardev)];

        if let Some(vhostforce) = &self.vhostforce {
            args.push(format!("vhostforce={}", vhostforce.to_arg()));
        }
        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for VhostUser {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct VhostVdpa {
    id: String,
    vhostdev: Option<PathBuf>,
    vhostfd: Option<String>,
}

impl ToCommand for VhostVdpa {
    fn to_args(&self) -> Vec<String> {
        let mut args = vec!["vhost-vdpa".to_string(), format!("id={}", self.id.to_string())];
        if let Some(vhostdev) = &self.vhostdev {
            args.push(format!("vhostdev={}", vhostdev.to_str().unwrap()));
        }
        if let Some(vhostfd) = &self.vhostfd {
            args.push(format!("vhostfd={}", vhostfd));
        }
        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for VhostVdpa {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct VmnetHost {
    id: String,
    isolated: Option<OnOff>,
    net_uuid: Option<String>,
    start_address: Option<String>,
    end_address: Option<String>,
    subnet_mask: Option<String>,
}

impl ToCommand for VmnetHost {
    fn to_args(&self) -> Vec<String> {
        let mut args = vec!["vmnet-host".to_string(), format!("id={}", self.id.to_string())];

        if let Some(isolated) = &self.isolated {
            args.push(format!("isolated={}", isolated.to_arg()));
        }
        if let Some(net_uuid) = &self.net_uuid {
            args.push(format!("net_uuid={}", net_uuid));
        }
        if let Some(start_address) = &self.start_address {
            args.push(format!("start-address={}", start_address));
        }
        if let Some(end_address) = &self.end_address {
            args.push(format!("end-address={}", end_address));
        }
        if let Some(subnet_mask) = &self.subnet_mask {
            args.push(format!("subnet-mask={}", subnet_mask));
        }
        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for VmnetHost {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct VmnetShared {
    id: String,
    isolated: Option<OnOff>,
    nat66_prefix: Option<String>,
    start_address: Option<String>,
    end_address: Option<String>,
    subnet_mask: Option<String>,
}

impl ToCommand for VmnetShared {
    fn to_args(&self) -> Vec<String> {
        let mut args = vec!["vmnet-shared".to_string(), format!("id={}", self.id.to_string())];

        if let Some(isolated) = &self.isolated {
            args.push(format!("isolated={}", isolated.to_arg()));
        }
        if let Some(nat66_prefix) = &self.nat66_prefix {
            args.push(format!("nat66-prefix={}", nat66_prefix));
        }
        if let Some(start_address) = &self.start_address {
            args.push(format!("start-address={}", start_address));
        }
        if let Some(end_address) = &self.end_address {
            args.push(format!("end-address={}", end_address));
        }
        if let Some(subnet_mask) = &self.subnet_mask {
            args.push(format!("subnet-mask={}", subnet_mask));
        }
        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for VmnetShared {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct VmnetBridged {
    id: String,
    ifname: String,
    isolated: Option<OnOff>,
}

impl ToCommand for VmnetBridged {
    fn to_args(&self) -> Vec<String> {
        let mut args = vec!["vmnet-bridged".to_string(), format!("id={}", self.id.to_string()), format!("ifname={}", self.ifname)];

        if let Some(isolated) = &self.isolated {
            args.push(format!("isolated={}", isolated.to_arg()));
        }
        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for VmnetBridged {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct Hubport {
    id: String,
    hubid: usize,
    netdev: Option<String>,
}

impl ToCommand for Hubport {
    fn to_args(&self) -> Vec<String> {
        let mut args = vec!["hubport".to_string(), format!("id={}", self.id.to_string()), format!("hubid={}", self.hubid)];

        if let Some(netdev) = &self.netdev {
            args.push(format!("netdev={}", netdev));
        }
        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for Hubport {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum NetDev {
    User(User),
    // TODO L2tpv3,
    Tap(Tap),
    Bridge(Bridge),
    Socket(Socket),
    Stream(Stream),
    Dgram(Dgram),
    Vde(Vde),
    Netmap(NetMap),
    AfXdp(AfXdp),
    VhostUser(VhostUser),
    VhostVdpa(VhostVdpa),
    VmnetHost(VmnetHost),
    VmnetShared(VmnetShared),
    VmnetBridged(VmnetBridged),
    Hubport(Hubport),
}

impl ToCommand for NetDev {
    fn command(&self) -> String {
        ARG_NETDEV.to_string()
    }
    fn to_args(&self) -> Vec<String> {
        match self {
            NetDev::User(user) => user.to_args(),
            //NetDev::L2tpv3 => {}
            NetDev::Tap(tap) => tap.to_args(),
            NetDev::Bridge(bridge) => bridge.to_args(),
            NetDev::Socket(socket) => socket.to_args(),
            NetDev::Stream(stream) => stream.to_args(),
            NetDev::Dgram(dgram) => dgram.to_args(),
            NetDev::Vde(vde) => vde.to_args(),
            NetDev::Netmap(netmap) => netmap.to_args(),
            NetDev::AfXdp(af_xdp) => af_xdp.to_args(),
            NetDev::VhostUser(vhost_user) => vhost_user.to_args(),
            NetDev::VhostVdpa(vhost_vdpa) => vhost_vdpa.to_args(),
            NetDev::VmnetHost(vmnet_host) => vmnet_host.to_args(),
            NetDev::VmnetShared(vmnet_shared) => vmnet_shared.to_args(),
            NetDev::VmnetBridged(vmnet_bridged) => vmnet_bridged.to_args(),
            NetDev::Hubport(hubport) => hubport.to_args(),
        }
    }
}

impl FromStr for NetDev {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("tap,") || s == "tap" {
            return Ok(Self::Tap(s.parse::<Tap>()?));
        }

        Err(format!("unsupported netdev backend: {s}"))
    }
}
