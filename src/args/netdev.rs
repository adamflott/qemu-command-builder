use crate::args::chardev::CharDev;
use crate::common::OnOff;
use crate::parsers::ARG_NETDEV;
use crate::parsers::DELIM_COMMA;
use crate::to_command::ToArg;
use crate::to_command::ToCommand;
use crate::{QIpv4Net, QIpv6Net};
use bon::Builder;
use proptest_derive::Arbitrary;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::path::PathBuf;
use std::str::FromStr;

/// A QEMU `-netdev` backend.
///
/// The parser supports the same canonical comma-separated forms that the
/// formatter emits for the implemented backend variants in this crate,
/// including `tap`, `bridge`, `socket`, `vhost-user`, `vhost-vdpa`, and
/// `hubport`.
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
    Unix,
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
        if s == "no" { Ok(Self::None) } else { Ok(Self::Script(PathBuf::from(s))) }
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct HostForward {
    protocol: Option<TcpUdp>,
    hostaddr: Option<String>,
    hostport: Option<u16>,
    hostpath: Option<String>,
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
                let mut value = String::new();
                match &hostfwd.protocol {
                    Some(TcpUdp::Tcp) => value.push_str("tcp:"),
                    Some(TcpUdp::Udp) => value.push_str("udp:"),
                    Some(TcpUdp::Unix) => value.push_str("unix:"),
                    None => {}
                }

                if matches!(hostfwd.protocol, Some(TcpUdp::Unix)) {
                    if let Some(hostpath) = &hostfwd.hostpath {
                        value.push_str(hostpath);
                    }
                    value.push('-');
                    if let Some(guestaddr) = &hostfwd.guestaddr {
                        value.push_str(guestaddr);
                    }
                    value.push(':');
                    value.push_str(&hostfwd.guestport.to_string());
                } else {
                    if let Some(hostaddr) = &hostfwd.hostaddr {
                        value.push_str(hostaddr);
                    }
                    if hostfwd.hostaddr.is_some() || hostfwd.hostport.is_some() {
                        value.push(':');
                    }
                    if let Some(hostport) = hostfwd.hostport {
                        value.push_str(&hostport.to_string());
                    }
                    value.push('-');
                    if let Some(guestaddr) = &hostfwd.guestaddr {
                        value.push_str(guestaddr);
                    }
                    value.push(':');
                    value.push_str(&hostfwd.guestport.to_string());
                }

                args.push(format!("hostfwd={value}"));
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
                args.push(format!("guestfwd={}", subargs.join(":")));
            }
        }
        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for User {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let props = parse_netdev_props(s, "user")?;
        let id = required_prop(&props, "id")?.to_string();
        let mut dnssearch = vec![];
        let mut hostfwd = vec![];
        let mut guestfwd = vec![];

        for value in all_props(&props, "dnssearch") {
            dnssearch.push(value.to_string());
        }
        for value in all_props(&props, "hostfwd") {
            hostfwd.push(parse_hostfwd(value)?);
        }
        for value in all_props(&props, "guestfwd") {
            guestfwd.push(parse_guestfwd(value)?);
        }

        let smb = first_prop(&props, "smb").map(|dir| SMB {
            dir: PathBuf::from(dir),
            smbserver: first_prop(&props, "smbserver").map(ToString::to_string),
        });

        Ok(Self {
            id,
            ipv4: parse_optional_onoff(first_prop(&props, "ipv4"))?,
            net: parse_optional_qipv4net(first_prop(&props, "net"))?,
            host: parse_optional_ipv4(first_prop(&props, "host"))?,
            ipv6: parse_optional_onoff(first_prop(&props, "ipv6"))?,
            ipv6_net: parse_optional_qipv6net(first_prop(&props, "ipv6-net"))?,
            ipv6_host: parse_optional_ipv6(first_prop(&props, "ipv6-host"))?,
            restrict: parse_optional_onoff(first_prop(&props, "restrict"))?,
            hostname: first_prop(&props, "hostname").map(ToString::to_string),
            dhcpstart: parse_optional_ipv4(first_prop(&props, "dhcpstart"))?,
            dns: parse_optional_ipv4(first_prop(&props, "dns"))?,
            ipv6_dns: parse_optional_ipv6(first_prop(&props, "ipv6-dns"))?,
            dnssearch: (!dnssearch.is_empty()).then_some(dnssearch),
            domainname: first_prop(&props, "domainname").map(ToString::to_string),
            tftp: first_prop(&props, "tftp").map(PathBuf::from),
            tftp_server_name: first_prop(&props, "tftp-server-name").map(ToString::to_string),
            bootfile: first_prop(&props, "bootfile").map(PathBuf::from),
            smb,
            hostfwd: (!hostfwd.is_empty()).then_some(hostfwd),
            guestfwd: (!guestfwd.is_empty()).then_some(guestfwd),
        })
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
    sndbuf: Option<u64>,
    vnet_hdr: Option<OnOff>,
    vhost: Option<OnOff>,
    vhostfd: Option<String>,
    vhostforce: Option<OnOff>,
    queues: Option<u64>,
    poll_us: Option<u64>,
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
                "sndbuf" => sndbuf = Some(value.parse::<u64>().map_err(|e| e.to_string())?),
                "vnet_hdr" => vnet_hdr = Some(value.parse::<OnOff>().map_err(|_| format!("invalid vnet_hdr value: {value}"))?),
                "vhost" => vhost = Some(value.parse::<OnOff>().map_err(|_| format!("invalid vhost value: {value}"))?),
                "vhostfd" => vhostfd = Some(value.to_string()),
                "vhostforce" => vhostforce = Some(value.parse::<OnOff>().map_err(|_| format!("invalid vhostforce value: {value}"))?),
                "queues" => queues = Some(value.parse::<u64>().map_err(|e| e.to_string())?),
                "poll_us" => poll_us = Some(value.parse::<u64>().map_err(|e| e.to_string())?),
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

/// A `-netdev bridge,...` backend.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct Bridge {
    id: String,
    bridge: Option<String>,
    helper: Option<String>,
}

/// Linux Ethernet-over-L2TPv3 pseudowire backend.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct L2tpv3 {
    id: String,
    src: String,
    dst: String,
    srcport: Option<u16>,
    dstport: Option<u16>,
    rxsession: Option<u32>,
    txsession: u32,
    ipv6: Option<OnOff>,
    udp: Option<OnOff>,
    cookie64: Option<OnOff>,
    counter: Option<OnOff>,
    pincounter: Option<OnOff>,
    txcookie: Option<u64>,
    rxcookie: Option<u64>,
    offset: Option<u32>,
}

impl ToCommand for L2tpv3 {
    fn command(&self) -> String {
        ARG_NETDEV.to_string()
    }

    fn to_args(&self) -> Vec<String> {
        let mut parts = vec!["l2tpv3".to_string(), format!("id={}", self.id), format!("src={}", self.src), format!("dst={}", self.dst)];
        macro_rules! push {
            ($field:ident) => {
                if let Some(value) = &self.$field {
                    parts.push(format!("{}={}", stringify!($field), value));
                }
            };
        }
        push!(srcport);
        push!(dstport);
        push!(rxsession);
        parts.push(format!("txsession={}", self.txsession));
        for (key, value) in [
            ("ipv6", &self.ipv6),
            ("udp", &self.udp),
            ("cookie64", &self.cookie64),
            ("counter", &self.counter),
            ("pincounter", &self.pincounter),
        ] {
            if let Some(value) = value {
                parts.push(format!("{}={}", key, value.to_arg()));
            }
        }
        push!(txcookie);
        push!(rxcookie);
        push!(offset);
        vec![parts.join(DELIM_COMMA)]
    }
}

impl FromStr for L2tpv3 {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut id = None;
        let mut src = None;
        let mut dst = None;
        let mut srcport = None;
        let mut dstport = None;
        let mut rxsession = None;
        let mut txsession = None;
        let mut ipv6 = None;
        let mut udp = None;
        let mut cookie64 = None;
        let mut counter = None;
        let mut pincounter = None;
        let mut txcookie = None;
        let mut rxcookie = None;
        let mut offset = None;
        for part in value.strip_prefix("l2tpv3,").unwrap_or(value).split(DELIM_COMMA) {
            let (key, raw) = part.split_once('=').ok_or_else(|| format!("invalid l2tpv3 option: {part}"))?;
            macro_rules! number {
                ($slot:ident, $type:ty) => {
                    $slot = Some({
                        let number = match raw.strip_prefix("0x") {
                            Some(hex) => u64::from_str_radix(hex, 16).map_err(|e| e.to_string())?,
                            None => raw.parse::<u64>().map_err(|e| e.to_string())?,
                        };
                        <$type>::try_from(number).map_err(|_| format!("{key} value is out of range: {raw}"))?
                    })
                };
            }
            match key {
                "id" => id = Some(raw.to_string()),
                "src" => src = Some(raw.to_string()),
                "dst" => dst = Some(raw.to_string()),
                "srcport" => number!(srcport, u16),
                "dstport" => number!(dstport, u16),
                "rxsession" => number!(rxsession, u32),
                "txsession" => number!(txsession, u32),
                "ipv6" => ipv6 = Some(raw.parse::<OnOff>().map_err(|_| format!("invalid ipv6 value: {raw}"))?),
                "udp" => udp = Some(raw.parse::<OnOff>().map_err(|_| format!("invalid udp value: {raw}"))?),
                "cookie64" => cookie64 = Some(raw.parse::<OnOff>().map_err(|_| format!("invalid cookie64 value: {raw}"))?),
                "counter" => counter = Some(raw.parse::<OnOff>().map_err(|_| format!("invalid counter value: {raw}"))?),
                "pincounter" => pincounter = Some(raw.parse::<OnOff>().map_err(|_| format!("invalid pincounter value: {raw}"))?),
                "txcookie" => number!(txcookie, u64),
                "rxcookie" => number!(rxcookie, u64),
                "offset" => number!(offset, u32),
                other => return Err(format!("unsupported l2tpv3 option: {other}")),
            }
        }
        Ok(Self {
            id: id.ok_or_else(|| "l2tpv3 requires id=".to_string())?,
            src: src.ok_or_else(|| "l2tpv3 requires src=".to_string())?,
            dst: dst.ok_or_else(|| "l2tpv3 requires dst=".to_string())?,
            srcport,
            dstport,
            rxsession,
            txsession: txsession.ok_or_else(|| "l2tpv3 requires txsession=".to_string())?,
            ipv6,
            udp,
            cookie64,
            counter,
            pincounter,
            txcookie,
            rxcookie,
            offset,
        })
    }
}

impl ToCommand for Bridge {
    fn to_args(&self) -> Vec<String> {
        let mut args = vec!["bridge".to_string(), format!("id={}", self.id)];

        if let Some(br) = &self.bridge {
            args.push(format!("br={}", br));
        }
        if let Some(helper) = &self.helper {
            args.push(format!("helper={}", helper));
        }
        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for Bridge {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let backend = parts.next().ok_or_else(|| "empty netdev argument".to_string())?;
        if backend != "bridge" {
            return Err(format!("expected bridge backend, got {backend}"));
        }

        let mut id = None;
        let mut bridge = None;
        let mut helper = None;

        for part in parts {
            let (key, value) = part.split_once('=').ok_or_else(|| format!("invalid bridge option: {part}"))?;
            match key {
                "id" => id = Some(value.to_string()),
                "br" => bridge = Some(value.to_string()),
                "helper" => helper = Some(value.to_string()),
                other => return Err(format!("unsupported bridge option: {other}")),
            }
        }

        Ok(Self {
            id: id.ok_or_else(|| "bridge netdev requires id=".to_string())?,
            bridge,
            helper,
        })
    }
}

/// A host and port pair used by socket-based `-netdev` backends.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct HostAndPort {
    host: String,
    port: u16,
}
/// A host and optional port used by `listen=` socket endpoints.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct HostAndMaybePort {
    host: String,
    port: Option<u16>,
}
/// A `-netdev socket,...` backend using `listen=` and/or `connect=`.
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
        }
        if let Some(connection) = &self.connection {
            args.push(format!("connect={}:{}", connection.host, connection.port));
        }

        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for SocketRegular {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let backend = parts.next().ok_or_else(|| "empty netdev argument".to_string())?;
        if backend != "socket" {
            return Err(format!("expected socket backend, got {backend}"));
        }

        let mut id = None;
        let mut fd = None;
        let mut listen = None;
        let mut connection = None;

        for part in parts {
            let (key, value) = part.split_once('=').ok_or_else(|| format!("invalid socket option: {part}"))?;
            match key {
                "id" => id = Some(value.to_string()),
                "fd" => fd = Some(value.to_string()),
                "listen" => listen = Some(parse_host_and_maybe_port(value)?),
                "connect" => connection = Some(parse_host_and_port(value)?),
                "mcast" | "udp" | "localaddr" => return Err(format!("socket variant is not regular: {part}")),
                other => return Err(format!("unsupported socket option: {other}")),
            }
        }

        Ok(Self {
            id: id.ok_or_else(|| "socket netdev requires id=".to_string())?,
            fd,
            listen,
            connection,
        })
    }
}

/// A `-netdev socket,...` multicast backend using `mcast=`.
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
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let backend = parts.next().ok_or_else(|| "empty netdev argument".to_string())?;
        if backend != "socket" {
            return Err(format!("expected socket backend, got {backend}"));
        }

        let mut id = None;
        let mut fd = None;
        let mut mcast = None;
        let mut localaddr = None;

        for part in parts {
            let (key, value) = part.split_once('=').ok_or_else(|| format!("invalid socket option: {part}"))?;
            match key {
                "id" => id = Some(value.to_string()),
                "fd" => fd = Some(value.to_string()),
                "mcast" => mcast = Some(parse_host_and_port(value)?),
                "localaddr" => localaddr = Some(value.to_string()),
                "listen" | "connect" | "udp" => return Err(format!("socket variant is not multicast: {part}")),
                other => return Err(format!("unsupported socket option: {other}")),
            }
        }

        Ok(Self {
            id: id.ok_or_else(|| "socket netdev requires id=".to_string())?,
            fd,
            mcast,
            localaddr,
        })
    }
}

/// A `-netdev socket,...` UDP tunnel backend using `udp=`.
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
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let backend = parts.next().ok_or_else(|| "empty netdev argument".to_string())?;
        if backend != "socket" {
            return Err(format!("expected socket backend, got {backend}"));
        }

        let mut id = None;
        let mut fd = None;
        let mut udp = None;
        let mut localaddr = None;

        for part in parts {
            let (key, value) = part.split_once('=').ok_or_else(|| format!("invalid socket option: {part}"))?;
            match key {
                "id" => id = Some(value.to_string()),
                "fd" => fd = Some(value.to_string()),
                "udp" => udp = Some(parse_host_and_port(value)?),
                "localaddr" => localaddr = Some(parse_host_and_port(value)?),
                "listen" | "connect" | "mcast" => return Err(format!("socket variant is not udp tunnel: {part}")),
                other => return Err(format!("unsupported socket option: {other}")),
            }
        }

        Ok(Self {
            id: id.ok_or_else(|| "socket netdev requires id=".to_string())?,
            fd,
            udp,
            localaddr,
        })
    }
}

/// The `socket` backend variants supported by this crate.
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
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains(",mcast=") || s.starts_with("socket,mcast=") {
            return Ok(Self::Multicast(s.parse::<SocketMulticast>()?));
        }
        if s.contains(",udp=") || s.starts_with("socket,udp=") {
            return Ok(Self::UDPTunnel(s.parse::<SocketUdpTunnel>()?));
        }
        Ok(Self::SocketRegular(s.parse::<SocketRegular>()?))
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
    reconnect_ms: Option<u64>,
}

impl ToCommand for StreamOverTcp {
    fn to_args(&self) -> Vec<String> {
        let mut args = vec!["stream".to_string(), format!("id={}", self.id)];

        if let Some(server) = &self.server {
            args.push(format!("server={}", server.to_arg()));
        }
        args.push("addr.type=inet".to_string());
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
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let props = parse_netdev_props(s, "stream")?;
        ensure_prop_value(&props, "addr.type", "inet")?;
        Ok(Self {
            id: required_prop(&props, "id")?.to_string(),
            server: parse_optional_onoff(first_prop(&props, "server"))?,
            addr_host: required_prop(&props, "addr.host")?.to_string(),
            addr_port: required_prop(&props, "addr.port")?.parse::<u16>().map_err(|e| e.to_string())?,
            to: parse_optional_u16(first_prop(&props, "to"))?,
            numeric: parse_optional_onoff(first_prop(&props, "numeric"))?,
            keep_alive: parse_optional_onoff(first_prop(&props, "keep-alive"))?,
            mptcp: parse_optional_onoff(first_prop(&props, "mptcp"))?,
            addr_ipv4: parse_optional_onoff(first_prop(&props, "addr.ipv4"))?,
            addr_ipv6: parse_optional_onoff(first_prop(&props, "addr.ipv6"))?,
            reconnect_ms: parse_optional_u64(first_prop(&props, "reconnect-ms"))?,
        })
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct StreamOverUds {
    id: String,
    server: Option<OnOff>,
    addr_path: String,
    abstract_arg: Option<OnOff>,
    tight: Option<OnOff>,
    reconnect_ms: Option<u64>,
}

impl ToCommand for StreamOverUds {
    fn to_args(&self) -> Vec<String> {
        let mut args = vec!["stream".to_string(), format!("id={}", self.id)];

        if let Some(server) = &self.server {
            args.push(format!("server={}", server.to_arg()));
        }
        args.push("addr.type=unix".to_string());
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
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let props = parse_netdev_props(s, "stream")?;
        ensure_prop_value(&props, "addr.type", "unix")?;
        Ok(Self {
            id: required_prop(&props, "id")?.to_string(),
            server: parse_optional_onoff(first_prop(&props, "server"))?,
            addr_path: required_prop(&props, "addr.path")?.to_string(),
            abstract_arg: parse_optional_onoff(first_prop(&props, "abstract"))?,
            tight: parse_optional_onoff(first_prop(&props, "tight"))?,
            reconnect_ms: parse_optional_u64(first_prop(&props, "reconnect-ms"))?,
        })
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct StreamOverFd {
    id: String,
    server: Option<OnOff>,
    addr_str: String,
    reconnect_ms: Option<u64>,
}

impl ToCommand for StreamOverFd {
    fn to_args(&self) -> Vec<String> {
        let mut args = vec!["stream".to_string(), format!("id={}", self.id)];

        if let Some(server) = &self.server {
            args.push(format!("server={}", server.to_arg()));
        }
        args.push("addr.type=fd".to_string());
        args.push(format!("addr.str={}", self.addr_str));
        if let Some(reconnect_ms) = self.reconnect_ms {
            args.push(format!("reconnect-ms={}", reconnect_ms));
        }
        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for StreamOverFd {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let props = parse_netdev_props(s, "stream")?;
        ensure_prop_value(&props, "addr.type", "fd")?;
        Ok(Self {
            id: required_prop(&props, "id")?.to_string(),
            server: parse_optional_onoff(first_prop(&props, "server"))?,
            addr_str: required_prop(&props, "addr.str")?.to_string(),
            reconnect_ms: parse_optional_u64(first_prop(&props, "reconnect-ms"))?,
        })
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
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let props = parse_props(s)?;
        match first_prop(&props, "addr.type") {
            Some("inet") => Ok(Self::StreamOverTcp(s.parse::<StreamOverTcp>()?)),
            Some("unix") => Ok(Self::StreamOverUds(s.parse::<StreamOverUds>()?)),
            Some("fd") => Ok(Self::StreamOverFd(s.parse::<StreamOverFd>()?)),
            other => Err(format!("unsupported stream addr.type: {}", other.unwrap_or("<missing>"))),
        }
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
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let props = parse_netdev_props(s, "dgram")?;
        ensure_prop_value(&props, "remote.type", "inet")?;
        Ok(Self {
            id: required_prop(&props, "id")?.to_string(),
            remote_host: required_prop(&props, "remote.host")?.to_string(),
            remote_port: required_prop(&props, "remote.port")?.parse::<u16>().map_err(|e| e.to_string())?,
            local_host: first_prop(&props, "local.host").map(ToString::to_string),
        })
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
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let props = parse_netdev_props(s, "dgram")?;
        ensure_prop_value(&props, "remote.type", "inet")?;
        Ok(Self {
            id: required_prop(&props, "id")?.to_string(),
            remote_host: required_prop(&props, "remote.host")?.to_string(),
            remote_port: required_prop(&props, "remote.port")?.parse::<u16>().map_err(|e| e.to_string())?,
            local_str: first_prop(&props, "local.str").map(ToString::to_string),
        })
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct DgramSocket {
    id: String,
    local_host: String,
    local_port: u64,
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
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let props = parse_netdev_props(s, "dgram")?;
        ensure_prop_value(&props, "local.type", "inet")?;
        Ok(Self {
            id: required_prop(&props, "id")?.to_string(),
            local_host: required_prop(&props, "local.host")?.to_string(),
            local_port: required_prop(&props, "local.port")?.parse::<u64>().map_err(|e| e.to_string())?,
            remote_host: first_prop(&props, "remote.host").map(ToString::to_string),
            remote_port: parse_optional_u16(first_prop(&props, "remote.port"))?,
        })
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
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let props = parse_netdev_props(s, "dgram")?;
        ensure_prop_value(&props, "local.type", "unix")?;
        Ok(Self {
            id: required_prop(&props, "id")?.to_string(),
            local_path: PathBuf::from(required_prop(&props, "local.path")?),
            remote_path: first_prop(&props, "remote.path").map(PathBuf::from),
        })
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct DgramFd {
    id: String,
    local_str: String,
}

impl ToCommand for DgramFd {
    fn to_args(&self) -> Vec<String> {
        vec![["dgram".to_string(), format!("id={}", self.id), "local.type=fd".to_string(), format!("local.str={}", self.local_str)].join(DELIM_COMMA)]
    }
}

impl FromStr for DgramFd {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let props = parse_netdev_props(s, "dgram")?;
        ensure_prop_value(&props, "local.type", "fd")?;
        Ok(Self {
            id: required_prop(&props, "id")?.to_string(),
            local_str: required_prop(&props, "local.str")?.to_string(),
        })
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
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let props = parse_props(s)?;
        match (first_prop(&props, "local.type"), first_prop(&props, "remote.type")) {
            (Some("fd"), Some("inet")) => Ok(Self::DgramMulticastUdpFd(s.parse::<DgramMulticastUdpFd>()?)),
            (Some("fd"), _) => Ok(Self::DgramFd(s.parse::<DgramFd>()?)),
            (Some("unix"), _) => Ok(Self::DgramUds(s.parse::<DgramUds>()?)),
            (Some("inet"), _) => Ok(Self::DgramSocket(s.parse::<DgramSocket>()?)),
            (None, Some("inet")) => Ok(Self::DgramMulticast(s.parse::<DgramMulticast>()?)),
            _ => Err(format!("unsupported dgram backend: {s}")),
        }
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
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let props = parse_netdev_props(s, "vde")?;
        Ok(Self {
            id: required_prop(&props, "id")?.to_string(),
            sock: first_prop(&props, "sock").map(PathBuf::from),
            port: parse_optional_u16(first_prop(&props, "port"))?,
            group: first_prop(&props, "group").map(ToString::to_string),
            mode: first_prop(&props, "mode").map(ToString::to_string),
        })
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
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let props = parse_netdev_props(s, "netmap")?;
        Ok(Self {
            id: required_prop(&props, "id")?.to_string(),
            ifname: required_prop(&props, "ifname")?.to_string(),
            devname: first_prop(&props, "devname").map(ToString::to_string),
        })
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
    queues: Option<u64>,
    start_queue: Option<u64>,
    inhibit: Option<OnOff>,
    sock_fds: Option<Vec<String>>,
    map_path: Option<PathBuf>,
    map_start_index: Option<u64>,
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
        if let Some(map_path) = &self.map_path {
            args.push(format!("map-path={}", map_path.display()));
        }
        if let Some(map_start_index) = self.map_start_index {
            args.push(format!("map-start-index={}", map_start_index));
        }
        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for AfXdp {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let props = parse_netdev_props(s, "af-xdp")?;
        Ok(Self {
            id: required_prop(&props, "id")?.to_string(),
            ifname: required_prop(&props, "ifname")?.to_string(),
            mode: match first_prop(&props, "mode") {
                Some("native") => Some(NativeSkb::Native),
                Some("skb") => Some(NativeSkb::Skb),
                Some(other) => return Err(format!("invalid af-xdp mode: {other}")),
                None => None,
            },
            force_copy: parse_optional_onoff(first_prop(&props, "force-copy"))?,
            queues: parse_optional_u64(first_prop(&props, "queues"))?,
            start_queue: parse_optional_u64(first_prop(&props, "start-queue"))?,
            inhibit: parse_optional_onoff(first_prop(&props, "inhibit"))?,
            sock_fds: first_prop(&props, "sock-fds").map(|v| v.split(':').map(|part| part.to_string()).collect()),
            map_path: first_prop(&props, "map-path").map(PathBuf::from),
            map_start_index: parse_optional_u64(first_prop(&props, "map-start-index"))?,
        })
    }
}

/// A `-netdev passt,...` backend.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct Passt {
    id: String,
    path: Option<PathBuf>,
    quiet: Option<OnOff>,
    vhost_user: Option<OnOff>,
    mtu: Option<u64>,
    address: Option<String>,
    netmask: Option<String>,
    mac: Option<String>,
    gateway: Option<String>,
    interface: Option<String>,
    outbound: Option<String>,
    outbound_if4: Option<String>,
    outbound_if6: Option<String>,
    dns: Option<String>,
    search: Option<String>,
    fqdn: Option<String>,
    dhcp_dns: Option<OnOff>,
    dhcp_search: Option<OnOff>,
    map_host_loopback: Option<String>,
    map_guest_addr: Option<String>,
    dns_forward: Option<String>,
    dns_host: Option<String>,
    tcp: Option<OnOff>,
    udp: Option<OnOff>,
    icmp: Option<OnOff>,
    dhcp: Option<OnOff>,
    ndp: Option<OnOff>,
    dhcpv6: Option<OnOff>,
    ra: Option<OnOff>,
    freebind: Option<OnOff>,
    ipv4: Option<OnOff>,
    ipv6: Option<OnOff>,
    tcp_ports: Option<String>,
    udp_ports: Option<String>,
    param: Option<Vec<String>>,
}

impl ToCommand for Passt {
    fn to_args(&self) -> Vec<String> {
        let mut args = vec!["passt".to_string(), format!("id={}", self.id)];

        if let Some(path) = &self.path {
            args.push(format!("path={}", path.display()));
        }
        if let Some(quiet) = &self.quiet {
            args.push(format!("quiet={}", quiet.to_arg()));
        }
        if let Some(vhost_user) = &self.vhost_user {
            args.push(format!("vhost-user={}", vhost_user.to_arg()));
        }
        if let Some(mtu) = self.mtu {
            args.push(format!("mtu={}", mtu));
        }
        push_opt_string(&mut args, "address", &self.address);
        push_opt_string(&mut args, "netmask", &self.netmask);
        push_opt_string(&mut args, "mac", &self.mac);
        push_opt_string(&mut args, "gateway", &self.gateway);
        push_opt_string(&mut args, "interface", &self.interface);
        push_opt_string(&mut args, "outbound", &self.outbound);
        push_opt_string(&mut args, "outbound-if4", &self.outbound_if4);
        push_opt_string(&mut args, "outbound-if6", &self.outbound_if6);
        push_opt_string(&mut args, "dns", &self.dns);
        push_opt_string(&mut args, "search", &self.search);
        push_opt_string(&mut args, "fqdn", &self.fqdn);
        if let Some(dhcp_dns) = &self.dhcp_dns {
            args.push(format!("dhcp-dns={}", dhcp_dns.to_arg()));
        }
        if let Some(dhcp_search) = &self.dhcp_search {
            args.push(format!("dhcp-search={}", dhcp_search.to_arg()));
        }
        push_opt_string(&mut args, "map-host-loopback", &self.map_host_loopback);
        push_opt_string(&mut args, "map-guest-addr", &self.map_guest_addr);
        push_opt_string(&mut args, "dns-forward", &self.dns_forward);
        push_opt_string(&mut args, "dns-host", &self.dns_host);
        push_opt_onoff(&mut args, "tcp", &self.tcp);
        push_opt_onoff(&mut args, "udp", &self.udp);
        push_opt_onoff(&mut args, "icmp", &self.icmp);
        push_opt_onoff(&mut args, "dhcp", &self.dhcp);
        push_opt_onoff(&mut args, "ndp", &self.ndp);
        push_opt_onoff(&mut args, "dhcpv6", &self.dhcpv6);
        push_opt_onoff(&mut args, "ra", &self.ra);
        push_opt_onoff(&mut args, "freebind", &self.freebind);
        push_opt_onoff(&mut args, "ipv4", &self.ipv4);
        push_opt_onoff(&mut args, "ipv6", &self.ipv6);
        push_opt_string(&mut args, "tcp-ports", &self.tcp_ports);
        push_opt_string(&mut args, "udp-ports", &self.udp_ports);
        if let Some(params) = &self.param {
            for param in params {
                args.push(format!("param={}", param));
            }
        }

        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for Passt {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let props = parse_netdev_props(s, "passt")?;
        Ok(Self {
            id: required_prop(&props, "id")?.to_string(),
            path: first_prop(&props, "path").map(PathBuf::from),
            quiet: parse_optional_onoff(first_prop(&props, "quiet"))?,
            vhost_user: parse_optional_onoff(first_prop(&props, "vhost-user"))?,
            mtu: parse_optional_u64(first_prop(&props, "mtu"))?,
            address: first_prop(&props, "address").map(ToString::to_string),
            netmask: first_prop(&props, "netmask").map(ToString::to_string),
            mac: first_prop(&props, "mac").map(ToString::to_string),
            gateway: first_prop(&props, "gateway").map(ToString::to_string),
            interface: first_prop(&props, "interface").map(ToString::to_string),
            outbound: first_prop(&props, "outbound").map(ToString::to_string),
            outbound_if4: first_prop(&props, "outbound-if4").map(ToString::to_string),
            outbound_if6: first_prop(&props, "outbound-if6").map(ToString::to_string),
            dns: first_prop(&props, "dns").map(ToString::to_string),
            search: first_prop(&props, "search").map(ToString::to_string),
            fqdn: first_prop(&props, "fqdn").map(ToString::to_string),
            dhcp_dns: parse_optional_onoff(first_prop(&props, "dhcp-dns"))?,
            dhcp_search: parse_optional_onoff(first_prop(&props, "dhcp-search"))?,
            map_host_loopback: first_prop(&props, "map-host-loopback").map(ToString::to_string),
            map_guest_addr: first_prop(&props, "map-guest-addr").map(ToString::to_string),
            dns_forward: first_prop(&props, "dns-forward").map(ToString::to_string),
            dns_host: first_prop(&props, "dns-host").map(ToString::to_string),
            tcp: parse_optional_onoff(first_prop(&props, "tcp"))?,
            udp: parse_optional_onoff(first_prop(&props, "udp"))?,
            icmp: parse_optional_onoff(first_prop(&props, "icmp"))?,
            dhcp: parse_optional_onoff(first_prop(&props, "dhcp"))?,
            ndp: parse_optional_onoff(first_prop(&props, "ndp"))?,
            dhcpv6: parse_optional_onoff(first_prop(&props, "dhcpv6"))?,
            ra: parse_optional_onoff(first_prop(&props, "ra"))?,
            freebind: parse_optional_onoff(first_prop(&props, "freebind"))?,
            ipv4: parse_optional_onoff(first_prop(&props, "ipv4"))?,
            ipv6: parse_optional_onoff(first_prop(&props, "ipv6"))?,
            tcp_ports: first_prop(&props, "tcp-ports").map(ToString::to_string),
            udp_ports: first_prop(&props, "udp-ports").map(ToString::to_string),
            param: {
                let params = all_props(&props, "param").into_iter().map(ToString::to_string).collect::<Vec<_>>();
                (!params.is_empty()).then_some(params)
            },
        })
    }
}

/// A `-netdev vhost-user,...` backend.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct VhostUser {
    id: String,
    chardev: String,
    vhostforce: Option<OnOff>,
    queues: Option<u64>,
}

impl ToCommand for VhostUser {
    fn to_args(&self) -> Vec<String> {
        let mut args = vec!["vhost-user".to_string(), format!("id={}", self.id.to_string()), format!("chardev={}", self.chardev)];

        if let Some(vhostforce) = &self.vhostforce {
            args.push(format!("vhostforce={}", vhostforce.to_arg()));
        }
        if let Some(queues) = self.queues {
            args.push(format!("queues={}", queues));
        }
        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for VhostUser {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let backend = parts.next().ok_or_else(|| "empty netdev argument".to_string())?;
        if backend != "vhost-user" && backend != "type=vhost-user" {
            return Err(format!("expected vhost-user backend, got {backend}"));
        }

        let mut id = None;
        let mut chardev = None;
        let mut vhostforce = None;
        let mut queues = None;

        for part in parts {
            let (key, value) = part.split_once('=').ok_or_else(|| format!("invalid vhost-user option: {part}"))?;
            match key {
                "id" => id = Some(value.to_string()),
                "chardev" => chardev = Some(value.to_string()),
                "vhostforce" => vhostforce = Some(value.parse::<OnOff>().map_err(|_| format!("invalid vhostforce value: {value}"))?),
                "queues" => queues = Some(value.parse::<u64>().map_err(|e| e.to_string())?),
                other => return Err(format!("unsupported vhost-user option: {other}")),
            }
        }

        Ok(Self {
            id: id.ok_or_else(|| "vhost-user netdev requires id=".to_string())?,
            chardev: chardev.ok_or_else(|| "vhost-user netdev requires chardev=".to_string())?,
            vhostforce,
            queues,
        })
    }
}

/// A `-netdev vhost-vdpa,...` backend.
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
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let backend = parts.next().ok_or_else(|| "empty netdev argument".to_string())?;
        if backend != "vhost-vdpa" {
            return Err(format!("expected vhost-vdpa backend, got {backend}"));
        }

        let mut id = None;
        let mut vhostdev = None;
        let mut vhostfd = None;

        for part in parts {
            let (key, value) = part.split_once('=').ok_or_else(|| format!("invalid vhost-vdpa option: {part}"))?;
            match key {
                "id" => id = Some(value.to_string()),
                "vhostdev" => vhostdev = Some(PathBuf::from(value)),
                "vhostfd" => vhostfd = Some(value.to_string()),
                other => return Err(format!("unsupported vhost-vdpa option: {other}")),
            }
        }

        Ok(Self {
            id: id.ok_or_else(|| "vhost-vdpa netdev requires id=".to_string())?,
            vhostdev,
            vhostfd,
        })
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
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let props = parse_netdev_props(s, "vmnet-host")?;
        Ok(Self {
            id: required_prop(&props, "id")?.to_string(),
            isolated: parse_optional_onoff(first_prop(&props, "isolated"))?,
            net_uuid: first_prop(&props, "net_uuid").map(ToString::to_string),
            start_address: first_prop(&props, "start-address").map(ToString::to_string),
            end_address: first_prop(&props, "end-address").map(ToString::to_string),
            subnet_mask: first_prop(&props, "subnet-mask").map(ToString::to_string),
        })
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
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let props = parse_netdev_props(s, "vmnet-shared")?;
        Ok(Self {
            id: required_prop(&props, "id")?.to_string(),
            isolated: parse_optional_onoff(first_prop(&props, "isolated"))?,
            nat66_prefix: first_prop(&props, "nat66-prefix").map(ToString::to_string),
            start_address: first_prop(&props, "start-address").map(ToString::to_string),
            end_address: first_prop(&props, "end-address").map(ToString::to_string),
            subnet_mask: first_prop(&props, "subnet-mask").map(ToString::to_string),
        })
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
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let props = parse_netdev_props(s, "vmnet-bridged")?;
        Ok(Self {
            id: required_prop(&props, "id")?.to_string(),
            ifname: required_prop(&props, "ifname")?.to_string(),
            isolated: parse_optional_onoff(first_prop(&props, "isolated"))?,
        })
    }
}

/// A `-netdev hubport,...` backend.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct Hubport {
    id: String,
    hubid: u64,
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
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let backend = parts.next().ok_or_else(|| "empty netdev argument".to_string())?;
        if backend != "hubport" {
            return Err(format!("expected hubport backend, got {backend}"));
        }

        let mut id = None;
        let mut hubid = None;
        let mut netdev = None;

        for part in parts {
            let (key, value) = part.split_once('=').ok_or_else(|| format!("invalid hubport option: {part}"))?;
            match key {
                "id" => id = Some(value.to_string()),
                "hubid" => hubid = Some(value.parse::<u64>().map_err(|e| e.to_string())?),
                "netdev" => netdev = Some(value.to_string()),
                other => return Err(format!("unsupported hubport option: {other}")),
            }
        }

        Ok(Self {
            id: id.ok_or_else(|| "hubport netdev requires id=".to_string())?,
            hubid: hubid.ok_or_else(|| "hubport netdev requires hubid=".to_string())?,
            netdev,
        })
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum NetDev {
    /// Raw QMP JSON form, preserved verbatim.
    Json(String),
    User(User),
    Passt(Passt),
    L2tpv3(L2tpv3),
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
            NetDev::Json(json) => vec![json.clone()],
            NetDev::User(user) => user.to_args(),
            NetDev::Passt(passt) => passt.to_args(),
            NetDev::L2tpv3(l2tpv3) => l2tpv3.to_args(),
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
        if s.trim().starts_with('{') && s.trim().ends_with('}') {
            return Ok(Self::Json(s.to_string()));
        }
        if s.starts_with("user,") || s == "user" {
            return Ok(Self::User(s.parse::<User>()?));
        }
        if s.starts_with("passt,") || s == "passt" {
            return Ok(Self::Passt(s.parse::<Passt>()?));
        }
        if s.starts_with("l2tpv3,") || s == "l2tpv3" {
            return Ok(Self::L2tpv3(s.parse::<L2tpv3>()?));
        }
        if s.starts_with("tap,") || s == "tap" {
            return Ok(Self::Tap(s.parse::<Tap>()?));
        }
        if s.starts_with("bridge,") || s == "bridge" {
            return Ok(Self::Bridge(s.parse::<Bridge>()?));
        }
        if s.starts_with("socket,") || s == "socket" {
            return Ok(Self::Socket(s.parse::<Socket>()?));
        }
        if s.starts_with("stream,") || s == "stream" {
            return Ok(Self::Stream(s.parse::<Stream>()?));
        }
        if s.starts_with("dgram,") || s == "dgram" {
            return Ok(Self::Dgram(s.parse::<Dgram>()?));
        }
        if s.starts_with("vde,") || s == "vde" {
            return Ok(Self::Vde(s.parse::<Vde>()?));
        }
        if s.starts_with("netmap,") || s == "netmap" {
            return Ok(Self::Netmap(s.parse::<NetMap>()?));
        }
        if s.starts_with("af-xdp,") || s == "af-xdp" {
            return Ok(Self::AfXdp(s.parse::<AfXdp>()?));
        }
        if s.starts_with("vhost-user,") || s.starts_with("type=vhost-user,") || s == "vhost-user" || s == "type=vhost-user" {
            return Ok(Self::VhostUser(s.parse::<VhostUser>()?));
        }
        if s.starts_with("vhost-vdpa,") || s == "vhost-vdpa" {
            return Ok(Self::VhostVdpa(s.parse::<VhostVdpa>()?));
        }
        if s.starts_with("vmnet-host,") || s == "vmnet-host" {
            return Ok(Self::VmnetHost(s.parse::<VmnetHost>()?));
        }
        if s.starts_with("vmnet-shared,") || s == "vmnet-shared" {
            return Ok(Self::VmnetShared(s.parse::<VmnetShared>()?));
        }
        if s.starts_with("vmnet-bridged,") || s == "vmnet-bridged" {
            return Ok(Self::VmnetBridged(s.parse::<VmnetBridged>()?));
        }
        if s.starts_with("hubport,") || s == "hubport" {
            return Ok(Self::Hubport(s.parse::<Hubport>()?));
        }

        Err(format!("unsupported netdev backend: {s}"))
    }
}

fn parse_host_and_port(value: &str) -> Result<HostAndPort, String> {
    let (host, port) = value.rsplit_once(':').ok_or_else(|| format!("expected host:port, got {value}"))?;
    Ok(HostAndPort {
        host: host.to_string(),
        port: port.parse::<u16>().map_err(|e| e.to_string())?,
    })
}

fn parse_host_and_maybe_port(value: &str) -> Result<HostAndMaybePort, String> {
    if let Some((host, port)) = value.rsplit_once(':')
        && !port.is_empty()
    {
        return Ok(HostAndMaybePort {
            host: host.to_string(),
            port: Some(port.parse::<u16>().map_err(|e| e.to_string())?),
        });
    }

    Ok(HostAndMaybePort { host: value.to_string(), port: None })
}

fn parse_props(s: &str) -> Result<std::collections::BTreeMap<String, Vec<String>>, String> {
    let mut parts = s.split(DELIM_COMMA);
    let _backend = parts.next().ok_or_else(|| "empty netdev argument".to_string())?;
    let mut props = std::collections::BTreeMap::<String, Vec<String>>::new();

    for part in parts {
        let (key, value) = part.split_once('=').ok_or_else(|| format!("invalid netdev option: {part}"))?;
        props.entry(key.to_string()).or_default().push(value.to_string());
    }

    Ok(props)
}

fn parse_netdev_props(s: &str, backend_name: &str) -> Result<std::collections::BTreeMap<String, Vec<String>>, String> {
    let actual = s.split(DELIM_COMMA).next().ok_or_else(|| "empty netdev argument".to_string())?;
    if actual != backend_name {
        return Err(format!("expected {backend_name} backend, got {actual}"));
    }
    parse_props(s)
}

fn required_prop<'a>(props: &'a std::collections::BTreeMap<String, Vec<String>>, key: &str) -> Result<&'a str, String> {
    props
        .get(key)
        .and_then(|values| values.first())
        .map(|s| s.as_str())
        .ok_or_else(|| format!("missing required option: {key}"))
}

fn ensure_prop_value(props: &std::collections::BTreeMap<String, Vec<String>>, key: &str, expected: &str) -> Result<(), String> {
    let actual = required_prop(props, key)?;
    if actual == expected { Ok(()) } else { Err(format!("expected {key}={expected}, got {actual}")) }
}

fn first_prop<'a>(props: &'a std::collections::BTreeMap<String, Vec<String>>, key: &str) -> Option<&'a str> {
    props.get(key).and_then(|values| values.first()).map(|s| s.as_str())
}

fn all_props<'a>(props: &'a std::collections::BTreeMap<String, Vec<String>>, key: &str) -> Vec<&'a str> {
    props.get(key).map(|values| values.iter().map(|s| s.as_str()).collect()).unwrap_or_default()
}

fn push_opt_string(args: &mut Vec<String>, key: &str, value: &Option<String>) {
    if let Some(value) = value {
        args.push(format!("{key}={value}"));
    }
}

fn push_opt_onoff(args: &mut Vec<String>, key: &str, value: &Option<OnOff>) {
    if let Some(value) = value {
        args.push(format!("{key}={}", value.to_arg()));
    }
}

fn parse_optional_onoff(value: Option<&str>) -> Result<Option<OnOff>, String> {
    value.map(|raw| raw.parse::<OnOff>().map_err(|_| format!("invalid on/off value: {raw}"))).transpose()
}

fn parse_optional_u64(value: Option<&str>) -> Result<Option<u64>, String> {
    value.map(|raw| raw.parse::<u64>().map_err(|e| e.to_string())).transpose()
}

fn parse_optional_u16(value: Option<&str>) -> Result<Option<u16>, String> {
    value.map(|raw| raw.parse::<u16>().map_err(|e| e.to_string())).transpose()
}

fn parse_optional_ipv4(value: Option<&str>) -> Result<Option<Ipv4Addr>, String> {
    value.map(|raw| raw.parse::<Ipv4Addr>().map_err(|e| e.to_string())).transpose()
}

fn parse_optional_ipv6(value: Option<&str>) -> Result<Option<Ipv6Addr>, String> {
    value.map(|raw| raw.parse::<Ipv6Addr>().map_err(|e| e.to_string())).transpose()
}

fn parse_optional_qipv4net(value: Option<&str>) -> Result<Option<QIpv4Net>, String> {
    value
        .map(|raw| raw.parse::<ipnet::Ipv4Net>().map(|ip| QIpv4Net::builder().ip(ip).build()).map_err(|e| e.to_string()))
        .transpose()
}

fn parse_optional_qipv6net(value: Option<&str>) -> Result<Option<QIpv6Net>, String> {
    value
        .map(|raw| raw.parse::<ipnet::Ipv6Net>().map(|ip| QIpv6Net::builder().ip(ip).build()).map_err(|e| e.to_string()))
        .transpose()
}

fn parse_hostfwd(value: &str) -> Result<HostForward, String> {
    let (protocol, rest) = if let Some(rest) = value.strip_prefix("tcp:") {
        (Some(TcpUdp::Tcp), rest)
    } else if let Some(rest) = value.strip_prefix("udp:") {
        (Some(TcpUdp::Udp), rest)
    } else if let Some(rest) = value.strip_prefix("unix:") {
        (Some(TcpUdp::Unix), rest)
    } else {
        (None, value)
    };

    if matches!(protocol, Some(TcpUdp::Unix)) {
        let (hostpath, guest_range) = rest.split_once('-').ok_or_else(|| format!("invalid hostfwd: {value}"))?;
        let (guestaddr, guestport) = parse_guest_host_port(guest_range)?;
        return Ok(HostForward {
            protocol,
            hostaddr: None,
            hostport: None,
            hostpath: Some(hostpath.to_string()),
            guestaddr,
            guestport,
        });
    }

    let (host_range, guest_range) = rest.split_once('-').ok_or_else(|| format!("invalid hostfwd: {value}"))?;
    let (hostaddr, hostport) = parse_optional_host_port_range(host_range)?;
    let (guestaddr, guestport) = parse_guest_host_port(guest_range)?;

    Ok(HostForward {
        protocol,
        hostaddr,
        hostport: Some(hostport),
        hostpath: None,
        guestaddr,
        guestport,
    })
}

fn parse_optional_host_port_range(value: &str) -> Result<(Option<String>, u16), String> {
    if let Some((host, port)) = value.rsplit_once(':') {
        Ok(((!host.is_empty()).then_some(host.to_string()), port.parse::<u16>().map_err(|e| e.to_string())?))
    } else {
        Ok((None, value.parse::<u16>().map_err(|e| e.to_string())?))
    }
}

fn parse_guest_host_port(value: &str) -> Result<(Option<String>, u16), String> {
    if let Some((host, port)) = value.rsplit_once(':') {
        Ok(((!host.is_empty()).then_some(host.to_string()), port.parse::<u16>().map_err(|e| e.to_string())?))
    } else {
        Ok((None, value.parse::<u16>().map_err(|e| e.to_string())?))
    }
}

fn parse_guestfwd(value: &str) -> Result<GuestForward, String> {
    let mut parts = value.split(':');
    let proto = parts.next().ok_or_else(|| format!("invalid guestfwd: {value}"))?;
    if proto != "tcp" {
        return Err(format!("unsupported guestfwd protocol: {proto}"));
    }
    let server = parts.next().ok_or_else(|| format!("invalid guestfwd: {value}"))?.to_string();
    let port = parts.next().ok_or_else(|| format!("invalid guestfwd: {value}"))?.parse::<u16>().map_err(|e| e.to_string())?;
    let target = parts.collect::<Vec<_>>().join(":");
    if target.is_empty() {
        return Err(format!("invalid guestfwd target: {value}"));
    }

    let target = if let Some(device) = target.strip_prefix("device=") {
        GuestForwardTarget::Device(device.parse::<CharDev>().map_err(|e| format!("invalid guestfwd device: {e}"))?)
    } else if let Some(command) = target.strip_prefix("cmd:") {
        let mut tokens = command.split_whitespace();
        let cmd = tokens.next().ok_or_else(|| format!("invalid guestfwd command: {value}"))?.to_string();
        let args = tokens.map(|token| token.to_string()).collect();
        GuestForwardTarget::Cmd((cmd, args))
    } else {
        return Err(format!("unsupported guestfwd target: {target}"));
    };

    Ok(GuestForward { server, port, target })
}
