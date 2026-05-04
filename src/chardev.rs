use std::path::PathBuf;
use std::str::FromStr;

use bon::Builder;
use proptest_derive::Arbitrary;

use crate::common::OnOff;
use crate::parsers::DELIM_COMMA;
use crate::to_command::{ToArg, ToCommand};

pub(crate) const ARG_CHARDEV: &str = "-chardev";

/// A QEMU `-chardev` backend.
///
/// The parser supports the canonical backend forms that this crate renders for
/// common QEMU character device backends such as `socket`, `stdio`, `file`,
/// `pipe`, `pty`, `hub`, and related simple backends.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct CharNull {
    id: String,
    mux: Option<OnOff>,
    logfile: Option<PathBuf>,
    logappend: Option<OnOff>,
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct CharSocketTcp {
    id: String,
    host: Option<String>,
    port: u16,
    to: Option<u16>,
    ipv4: Option<OnOff>,
    ipv6: Option<OnOff>,
    nodelay: Option<OnOff>,
    server: Option<OnOff>,
    wait: Option<OnOff>,
    telnet: Option<OnOff>,
    websocket: Option<OnOff>,
    reconnect_ms: Option<usize>,
    mux: Option<OnOff>,
    logfile: Option<PathBuf>,
    logappend: Option<OnOff>,
    tls_creds: Option<String>,
    tls_authz: Option<String>,
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct CharSocketUds {
    #[builder(into)]
    id: String,
    path: PathBuf,
    server: Option<OnOff>,
    wait: Option<OnOff>,
    telnet: Option<OnOff>,
    websocket: Option<OnOff>,
    reconnect_ms: Option<usize>,
    mux: Option<OnOff>,
    logfile: Option<PathBuf>,
    logappend: Option<OnOff>,
    abstract_opt: Option<OnOff>,
    tight: Option<OnOff>,
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum CharSocket {
    Tcp(CharSocketTcp),
    Uds(CharSocketUds),
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct CharUdp {
    id: String,
    host: Option<String>,
    port: u16,
    localaddr: Option<String>,
    localport: Option<u16>,
    ipv4: Option<OnOff>,
    ipv6: Option<OnOff>,
    mux: Option<OnOff>,
    logfile: Option<PathBuf>,
    logappend: Option<OnOff>,
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct CharMsMouse {
    id: String,
    mux: Option<OnOff>,
    logfile: Option<PathBuf>,
    logappend: Option<OnOff>,
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct CharHub {
    id: String,
    chardevs: Option<Vec<(usize, String)>>,
}
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct CharVc {
    id: String,
    width: Option<usize>,
    height: Option<usize>,
    cols: Option<usize>,
    rows: Option<usize>,
    mux: Option<OnOff>,
    logfile: Option<PathBuf>,
    logappend: Option<OnOff>,
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct CharRingBuf {
    id: String,
    size: Option<usize>,
    logfile: Option<PathBuf>,
    logappend: Option<OnOff>,
}
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct CharFile {
    id: String,
    path: PathBuf,
    input_path: Option<PathBuf>,
    mux: Option<OnOff>,
    logfile: Option<PathBuf>,
    logappend: Option<OnOff>,
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct CharPipe {
    id: String,
    path: PathBuf,
    mux: Option<OnOff>,
    logfile: Option<PathBuf>,
    logappend: Option<OnOff>,
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct CharWin32Console {
    id: String,
    mux: Option<OnOff>,
    logfile: Option<PathBuf>,
    logappend: Option<OnOff>,
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct CharWin32Serial {
    id: String,
    path: PathBuf,
    mux: Option<OnOff>,
    logfile: Option<PathBuf>,
    logappend: Option<OnOff>,
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct CharPty {
    id: String,
    path: Option<PathBuf>,
    mux: Option<OnOff>,
    logfile: Option<PathBuf>,
    logappend: Option<OnOff>,
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct CharStdio {
    id: String,
    mux: Option<OnOff>,
    signal: Option<OnOff>,
    logfile: Option<PathBuf>,
    logappend: Option<OnOff>,
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct CharBraille {
    id: String,
    mux: Option<OnOff>,
    logfile: Option<PathBuf>,
    logappend: Option<OnOff>,
}
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct CharSerial {
    id: String,
    path: PathBuf,
    mux: Option<OnOff>,
    logfile: Option<PathBuf>,
    logappend: Option<OnOff>,
}
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct CharParallel {
    id: String,
    path: PathBuf,
    mux: Option<OnOff>,
    logfile: Option<PathBuf>,
    logappend: Option<OnOff>,
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct CharSpice {
    id: String,
    name: String,
    debug: Option<String>,
    logfile: Option<PathBuf>,
    logappend: Option<OnOff>,
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum CharDev {
    Null(CharNull),
    Socket(CharSocket),
    Udp(CharUdp),
    MsMouse(CharMsMouse),
    Hub(CharHub),
    Vc(CharVc),
    RingBuf(CharRingBuf),
    File(CharFile),
    Pipe(CharPipe),
    Win32Console(CharWin32Console),
    Win32Serial(CharWin32Serial),
    Pty(CharPty),
    Stdio(CharStdio),
    Braille(CharBraille),
    Serial(CharSerial),
    Parallel(CharParallel),
    SpiceVmc(CharSpice),
    SpicePort(CharSpice),
}

impl CharDev {
    /// Returns the configured `id=` for the chardev backend.
    pub fn id(&self) -> &str {
        match self {
            CharDev::Null(n) => &n.id,
            CharDev::Socket(s) => match s {
                CharSocket::Tcp(t) => &t.id,
                CharSocket::Uds(u) => &u.id,
            },
            CharDev::Udp(u) => &u.id,
            CharDev::MsMouse(m) => &m.id,
            CharDev::Hub(h) => &h.id,
            CharDev::Vc(v) => &v.id,
            CharDev::RingBuf(r) => &r.id,
            CharDev::File(f) => &f.id,
            CharDev::Pipe(p) => &p.id,
            CharDev::Win32Console(w) => &w.id,
            CharDev::Win32Serial(ws) => &ws.id,
            CharDev::Pty(p) => &p.id,
            CharDev::Stdio(s) => &s.id,
            CharDev::Braille(b) => &b.id,
            CharDev::Serial(s) => &s.id,
            CharDev::Parallel(p) => &p.id,
            CharDev::SpiceVmc(s) => &s.id,
            CharDev::SpicePort(s) => &s.id,
        }
    }
}

impl ToCommand for CharDev {
    fn command(&self) -> String {
        ARG_CHARDEV.to_string()
    }

    fn to_args(&self) -> Vec<String> {
        let mut args = vec![];
        match self {
            CharDev::Null(null) => {
                args.push("null".to_string());
                args.push(format!("id={}", null.id));
                if let Some(mux) = &null.mux {
                    args.push(format!("mux={}", mux.to_arg()));
                }
                if let Some(logfile) = &null.logfile {
                    args.push(format!("logfile={}", logfile.display()));
                }
                if let Some(logappend) = &null.logappend {
                    args.push(format!("logappend={}", logappend.to_arg()));
                }
            }
            CharDev::Socket(socket) => match socket {
                CharSocket::Tcp(tcp) => {
                    args.push("socket".to_string());
                    args.push(format!("id={}", tcp.id));
                    if let Some(host) = &tcp.host {
                        args.push(format!("host={}", host));
                    }
                    args.push(format!("port={}", tcp.port));
                    if let Some(to) = &tcp.to {
                        args.push(format!("to={}", to));
                    }
                    if let Some(ipv4) = &tcp.ipv4 {
                        args.push(format!("ipv4={}", ipv4.to_arg()));
                    }
                    if let Some(ipv6) = &tcp.ipv6 {
                        args.push(format!("ipv6={}", ipv6.to_arg()));
                    }
                    if let Some(nodelay) = &tcp.nodelay {
                        args.push(format!("nodelay={}", nodelay.to_arg()));
                    }
                    if let Some(server) = &tcp.server {
                        args.push(format!("server={}", server.to_arg()));
                    }
                    if let Some(wait) = &tcp.wait {
                        args.push(format!("wait={}", wait.to_arg()));
                    }
                    if let Some(telnet) = &tcp.telnet {
                        args.push(format!("telnet={}", telnet.to_arg()));
                    }
                    if let Some(websocket) = &tcp.websocket {
                        args.push(format!("websocket={}", websocket.to_arg()));
                    }
                    if let Some(reconnect_ms) = &tcp.reconnect_ms {
                        args.push(format!("reconnect-ms={}", reconnect_ms));
                    }
                    if let Some(mux) = &tcp.mux {
                        args.push(format!("mux={}", mux.to_arg()));
                    }
                    if let Some(logfile) = &tcp.logfile {
                        args.push(format!("logfile={}", logfile.display()));
                    }
                    if let Some(logappend) = &tcp.logappend {
                        args.push(format!("logappend={}", logappend.to_arg()));
                    }
                    if let Some(tls_creds) = &tcp.tls_creds {
                        args.push(format!("tls-creds={}", tls_creds));
                    }
                    if let Some(tls_authz) = &tcp.tls_authz {
                        args.push(format!("tls-authz={}", tls_authz));
                    }
                }
                CharSocket::Uds(uds) => {
                    args.push("socket".to_string());
                    args.push(format!("id={}", uds.id));
                    args.push(format!("path={}", uds.path.display()));

                    if let Some(server) = &uds.server {
                        args.push(format!("server={}", server.to_arg()));
                    }
                    if let Some(wait) = &uds.wait {
                        args.push(format!("wait={}", wait.to_arg()));
                    }
                    if let Some(telnet) = &uds.telnet {
                        args.push(format!("telnet={}", telnet.to_arg()));
                    }
                    if let Some(websocket) = &uds.websocket {
                        args.push(format!("websocket={}", websocket.to_arg()));
                    }
                    if let Some(reconnect_ms) = &uds.reconnect_ms {
                        args.push(format!("reconnect-ms={}", reconnect_ms));
                    }
                    if let Some(mux) = &uds.mux {
                        args.push(format!("mux={}", mux.to_arg()));
                    }
                    if let Some(logfile) = &uds.logfile {
                        args.push(format!("logfile={}", logfile.display()));
                    }
                    if let Some(logappend) = &uds.logappend {
                        args.push(format!("logappend={}", logappend.to_arg()));
                    }
                    if let Some(abstract_opt) = &uds.abstract_opt {
                        args.push(format!("abstract={}", abstract_opt.to_arg()));
                    }
                    if let Some(tight) = &uds.tight {
                        args.push(format!("tight={}", tight.to_arg()));
                    }
                }
            },
            CharDev::Udp(udp) => {
                args.push("udp".to_string());
                args.push(format!("id={}", udp.id));

                if let Some(host) = &udp.host {
                    args.push(format!("host={}", host));
                }
                args.push(format!("port={}", udp.port));
                if let Some(localaddr) = &udp.localaddr {
                    args.push(format!("localaddr={}", localaddr));
                }
                if let Some(localport) = &udp.localport {
                    args.push(format!("localport={}", localport));
                }
                if let Some(ipv4) = &udp.ipv4 {
                    args.push(format!("ipv4={}", ipv4.to_arg()));
                }
                if let Some(ipv6) = &udp.ipv6 {
                    args.push(format!("ipv6={}", ipv6.to_arg()));
                }
                if let Some(mux) = &udp.mux {
                    args.push(format!("mux={}", mux.to_arg()));
                }
                if let Some(logfile) = &udp.logfile {
                    args.push(format!("logfile={}", logfile.display()));
                }
                if let Some(logappend) = &udp.logappend {
                    args.push(format!("logappend={}", logappend.to_arg()));
                }
            }
            CharDev::MsMouse(msmouse) => {
                args.push("msmouse".to_string());
                args.push(format!("id={}", msmouse.id));
                if let Some(mux) = &msmouse.mux {
                    args.push(format!("mux={}", mux.to_arg()));
                }
                if let Some(logfile) = &msmouse.logfile {
                    args.push(format!("logfile={}", logfile.display()));
                }
                if let Some(logappend) = &msmouse.logappend {
                    args.push(format!("logappend={}", logappend.to_arg()));
                }
            }
            CharDev::Hub(hub) => {
                args.push("hub".to_string());
                args.push(format!("id={}", hub.id));
                if let Some(chardevs) = &hub.chardevs {
                    for (n, chardev) in chardevs {
                        args.push(format!("chardevs.{}={}", n, chardev));
                    }
                }
            }
            CharDev::Vc(vc) => {
                args.push("vc".to_string());
                args.push(format!("id={}", vc.id));
                if let Some(width) = &vc.width {
                    args.push(format!("width={}", width));
                }
                if let Some(height) = &vc.height {
                    args.push(format!("height={}", height));
                }
                if let Some(cols) = &vc.cols {
                    args.push(format!("cols={}", cols));
                }
                if let Some(rows) = &vc.rows {
                    args.push(format!("rows={}", rows));
                }
                if let Some(mux) = &vc.mux {
                    args.push(format!("mux={}", mux.to_arg()));
                }
                if let Some(logvc) = &vc.logfile {
                    args.push(format!("logfile={}", logvc.display()));
                }
                if let Some(logappend) = &vc.logappend {
                    args.push(format!("logappend={}", logappend.to_arg()));
                }
            }
            CharDev::RingBuf(ringbuf) => {
                args.push("ringbuf".to_string());
                args.push(format!("id={}", ringbuf.id));
                if let Some(size) = &ringbuf.size {
                    args.push(format!("size={}", size));
                }
                if let Some(logfile) = &ringbuf.logfile {
                    args.push(format!("logfile={}", logfile.display()));
                }
                if let Some(logappend) = &ringbuf.logappend {
                    args.push(format!("logappend={}", logappend.to_arg()));
                }
            }
            CharDev::File(file) => {
                args.push("file".to_string());
                args.push(format!("id={}", file.id));
                args.push(format!("path={}", file.path.display()));
                if let Some(input_path) = &file.input_path {
                    args.push(format!("input-path={}", input_path.display()));
                }
                if let Some(mux) = &file.mux {
                    args.push(format!("mux={}", mux.to_arg()));
                }
                if let Some(logfile) = &file.logfile {
                    args.push(format!("logfile={}", logfile.display()));
                }
                if let Some(logappend) = &file.logappend {
                    args.push(format!("logappend={}", logappend.to_arg()));
                }
            }
            CharDev::Pipe(pipe) => {
                args.push("pipe".to_string());
                args.push(format!("id={}", pipe.id));
                args.push(format!("path={}", pipe.path.display()));
                if let Some(mux) = &pipe.mux {
                    args.push(format!("mux={}", mux.to_arg()));
                }
                if let Some(logpipe) = &pipe.logfile {
                    args.push(format!("logfile={}", logpipe.display()));
                }
                if let Some(logappend) = &pipe.logappend {
                    args.push(format!("logappend={}", logappend.to_arg()));
                }
            }
            CharDev::Win32Console(console) => {
                args.push("console".to_string());
                args.push(format!("id={}", console.id));
                if let Some(mux) = &console.mux {
                    args.push(format!("mux={}", mux.to_arg()));
                }
                if let Some(logconsole) = &console.logfile {
                    args.push(format!("logfile={}", logconsole.display()));
                }
                if let Some(logappend) = &console.logappend {
                    args.push(format!("logappend={}", logappend.to_arg()));
                }
            }
            CharDev::Win32Serial(serial) => {
                args.push("serial".to_string());
                args.push(format!("id={}", serial.id));
                args.push(format!("path={}", serial.path.display()));
                if let Some(mux) = &serial.mux {
                    args.push(format!("mux={}", mux.to_arg()));
                }
                if let Some(logserial) = &serial.logfile {
                    args.push(format!("logfile={}", logserial.display()));
                }
                if let Some(logappend) = &serial.logappend {
                    args.push(format!("logappend={}", logappend.to_arg()));
                }
            }
            CharDev::Pty(pty) => {
                args.push("pty".to_string());
                args.push(format!("id={}", pty.id));
                if let Some(path) = &pty.path {
                    args.push(format!("path={}", path.display()));
                }
                if let Some(mux) = &pty.mux {
                    args.push(format!("mux={}", mux.to_arg()));
                }
                if let Some(logpty) = &pty.logfile {
                    args.push(format!("logfile={}", logpty.display()));
                }
                if let Some(logappend) = &pty.logappend {
                    args.push(format!("logappend={}", logappend.to_arg()));
                }
            }
            CharDev::Stdio(stdio) => {
                args.push("stdio".to_string());
                args.push(format!("id={}", stdio.id));
                if let Some(mux) = &stdio.mux {
                    args.push(format!("mux={}", mux.to_arg()));
                }
                if let Some(signal) = &stdio.signal {
                    args.push(format!("signal={}", signal.to_arg()));
                }
                if let Some(stdio) = &stdio.logfile {
                    args.push(format!("logfile={}", stdio.display()));
                }
                if let Some(logappend) = &stdio.logappend {
                    args.push(format!("logappend={}", logappend.to_arg()));
                }
            }
            CharDev::Braille(braille) => {
                args.push("braille".to_string());
                args.push(format!("id={}", braille.id));
                if let Some(mux) = &braille.mux {
                    args.push(format!("mux={}", mux.to_arg()));
                }
                if let Some(braille) = &braille.logfile {
                    args.push(format!("logfile={}", braille.display()));
                }
                if let Some(logappend) = &braille.logappend {
                    args.push(format!("logappend={}", logappend.to_arg()));
                }
            }
            CharDev::Serial(serial) => {
                args.push("serial".to_string());
                args.push(format!("id={}", serial.id));
                args.push(format!("path={}", serial.path.display()));
                if let Some(mux) = &serial.mux {
                    args.push(format!("mux={}", mux.to_arg()));
                }
                if let Some(serial) = &serial.logfile {
                    args.push(format!("logfile={}", serial.display()));
                }
                if let Some(logappend) = &serial.logappend {
                    args.push(format!("logappend={}", logappend.to_arg()));
                }
            }
            CharDev::Parallel(parallel) => {
                args.push("parallel".to_string());
                args.push(format!("id={}", parallel.id));
                args.push(format!("path={}", parallel.path.display()));
                if let Some(mux) = &parallel.mux {
                    args.push(format!("mux={}", mux.to_arg()));
                }
                if let Some(parallel) = &parallel.logfile {
                    args.push(format!("logfile={}", parallel.display()));
                }
                if let Some(logappend) = &parallel.logappend {
                    args.push(format!("logappend={}", logappend.to_arg()));
                }
            }
            CharDev::SpiceVmc(spice) => {
                args.push("spicevmc".to_string());
                args.push(format!("id={}", spice.id));
                args.push(format!("name={}", spice.name));
                if let Some(debug) = &spice.debug {
                    args.push(format!("debug={}", debug));
                }
                if let Some(parallel) = &spice.logfile {
                    args.push(format!("logfile={}", parallel.display()));
                }
                if let Some(logappend) = &spice.logappend {
                    args.push(format!("logappend={}", logappend.to_arg()));
                }
            }
            CharDev::SpicePort(spice) => {
                args.push("spiceport".to_string());
                args.push(format!("id={}", spice.id));
                args.push(format!("name={}", spice.name));
                if let Some(debug) = &spice.debug {
                    args.push(format!("debug={}", debug));
                }
                if let Some(parallel) = &spice.logfile {
                    args.push(format!("logfile={}", parallel.display()));
                }
                if let Some(logappend) = &spice.logappend {
                    args.push(format!("logappend={}", logappend.to_arg()));
                }
            }
        }

        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for CharDev {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let backend = parts.next().ok_or_else(|| "empty chardev argument".to_string())?;

        match backend {
            "null" => parse_null_chardev(parts.collect()),
            "socket" => parse_socket_chardev(parts.collect()),
            "udp" => parse_udp_chardev(parts.collect()),
            "msmouse" => parse_msmouse_chardev(parts.collect()),
            "hub" => parse_hub_chardev(parts.collect()),
            "vc" => parse_vc_chardev(parts.collect()),
            "ringbuf" => parse_ringbuf_chardev(parts.collect()),
            "file" => parse_file_chardev(parts.collect()),
            "pipe" => parse_pipe_chardev(parts.collect()),
            "console" => parse_console_chardev(parts.collect()),
            "serial" => parse_serial_chardev(parts.collect()),
            "pty" => parse_pty_chardev(parts.collect()),
            "stdio" => parse_stdio_chardev(parts.collect()),
            "braille" => parse_braille_chardev(parts.collect()),
            "parallel" => parse_parallel_chardev(parts.collect()),
            "spicevmc" => parse_spice_chardev(parts.collect(), true),
            "spiceport" => parse_spice_chardev(parts.collect(), false),
            other => Err(format!("unsupported chardev backend: {other}")),
        }
    }
}

fn parse_null_chardev(parts: Vec<&str>) -> Result<CharDev, String> {
    let mut id = None;
    let mut mux = None;
    let mut logfile = None;
    let mut logappend = None;

    for part in parts {
        let (key, value) = part.split_once('=').ok_or_else(|| format!("invalid chardev null option: {part}"))?;
        match key {
            "id" => id = Some(value.to_string()),
            "mux" => mux = Some(value.parse::<OnOff>().map_err(|_| format!("invalid mux value: {value}"))?),
            "logfile" => logfile = Some(PathBuf::from(value)),
            "logappend" => logappend = Some(value.parse::<OnOff>().map_err(|_| format!("invalid logappend value: {value}"))?),
            other => return Err(format!("unsupported chardev null option: {other}")),
        }
    }

    Ok(CharDev::Null(CharNull {
        id: id.ok_or_else(|| "null chardev requires id=".to_string())?,
        mux,
        logfile,
        logappend,
    }))
}

fn parse_socket_chardev(parts: Vec<&str>) -> Result<CharDev, String> {
    let mut id = None;
    let mut host = None;
    let mut port = None;
    let mut path = None;
    let mut to = None;
    let mut ipv4 = None;
    let mut ipv6 = None;
    let mut nodelay = None;
    let mut server = None;
    let mut wait = None;
    let mut telnet = None;
    let mut websocket = None;
    let mut reconnect_ms = None;
    let mut mux = None;
    let mut logfile = None;
    let mut logappend = None;
    let mut tls_creds = None;
    let mut tls_authz = None;
    let mut abstract_opt = None;
    let mut tight = None;

    for part in parts {
        let Some((key, value)) = part.split_once('=') else {
            match part {
                "server" => {
                    server = Some(OnOff::On);
                    continue;
                }
                "nowait" => {
                    wait = Some(OnOff::Off);
                    continue;
                }
                "wait" => {
                    wait = Some(OnOff::On);
                    continue;
                }
                _ => return Err(format!("invalid chardev socket option: {part}")),
            }
        };
        match key {
            "id" => id = Some(value.to_string()),
            "host" => host = Some(value.to_string()),
            "port" => port = Some(value.parse::<u16>().map_err(|e| e.to_string())?),
            "path" => path = Some(PathBuf::from(value)),
            "to" => to = Some(value.parse::<u16>().map_err(|e| e.to_string())?),
            "ipv4" => ipv4 = Some(value.parse::<OnOff>().map_err(|_| format!("invalid ipv4 value: {value}"))?),
            "ipv6" => ipv6 = Some(value.parse::<OnOff>().map_err(|_| format!("invalid ipv6 value: {value}"))?),
            "nodelay" => nodelay = Some(value.parse::<OnOff>().map_err(|_| format!("invalid nodelay value: {value}"))?),
            "server" => server = Some(value.parse::<OnOff>().map_err(|_| format!("invalid server value: {value}"))?),
            "wait" => wait = Some(value.parse::<OnOff>().map_err(|_| format!("invalid wait value: {value}"))?),
            "telnet" => telnet = Some(value.parse::<OnOff>().map_err(|_| format!("invalid telnet value: {value}"))?),
            "websocket" => websocket = Some(value.parse::<OnOff>().map_err(|_| format!("invalid websocket value: {value}"))?),
            "reconnect-ms" => reconnect_ms = Some(value.parse::<usize>().map_err(|e| e.to_string())?),
            "mux" => mux = Some(value.parse::<OnOff>().map_err(|_| format!("invalid mux value: {value}"))?),
            "logfile" => logfile = Some(PathBuf::from(value)),
            "logappend" => logappend = Some(value.parse::<OnOff>().map_err(|_| format!("invalid logappend value: {value}"))?),
            "tls-creds" => tls_creds = Some(value.to_string()),
            "tls-authz" => tls_authz = Some(value.to_string()),
            "abstract" => abstract_opt = Some(value.parse::<OnOff>().map_err(|_| format!("invalid abstract value: {value}"))?),
            "tight" => tight = Some(value.parse::<OnOff>().map_err(|_| format!("invalid tight value: {value}"))?),
            other => return Err(format!("unsupported chardev socket option: {other}")),
        }
    }

    let id = id.ok_or_else(|| "socket chardev requires id=".to_string())?;
    if let Some(path) = path {
        return Ok(CharDev::Socket(CharSocket::Uds(CharSocketUds {
            id,
            path,
            server,
            wait,
            telnet,
            websocket,
            reconnect_ms,
            mux,
            logfile,
            logappend,
            abstract_opt,
            tight,
        })));
    }

    Ok(CharDev::Socket(CharSocket::Tcp(CharSocketTcp {
        id,
        host,
        port: port.ok_or_else(|| "tcp socket chardev requires port=".to_string())?,
        to,
        ipv4,
        ipv6,
        nodelay,
        server,
        wait,
        telnet,
        websocket,
        reconnect_ms,
        mux,
        logfile,
        logappend,
        tls_creds,
        tls_authz,
    })))
}

fn parse_udp_chardev(parts: Vec<&str>) -> Result<CharDev, String> {
    let mut id = None;
    let mut host = None;
    let mut port = None;
    let mut localaddr = None;
    let mut localport = None;
    let mut ipv4 = None;
    let mut ipv6 = None;
    let mut mux = None;
    let mut logfile = None;
    let mut logappend = None;

    for part in parts {
        let (key, value) = part.split_once('=').ok_or_else(|| format!("invalid chardev udp option: {part}"))?;
        match key {
            "id" => id = Some(value.to_string()),
            "host" => host = Some(value.to_string()),
            "port" => port = Some(value.parse::<u16>().map_err(|e| e.to_string())?),
            "localaddr" => localaddr = Some(value.to_string()),
            "localport" => localport = Some(value.parse::<u16>().map_err(|e| e.to_string())?),
            "ipv4" => ipv4 = Some(value.parse::<OnOff>().map_err(|_| format!("invalid ipv4 value: {value}"))?),
            "ipv6" => ipv6 = Some(value.parse::<OnOff>().map_err(|_| format!("invalid ipv6 value: {value}"))?),
            "mux" => mux = Some(value.parse::<OnOff>().map_err(|_| format!("invalid mux value: {value}"))?),
            "logfile" => logfile = Some(PathBuf::from(value)),
            "logappend" => logappend = Some(value.parse::<OnOff>().map_err(|_| format!("invalid logappend value: {value}"))?),
            other => return Err(format!("unsupported chardev udp option: {other}")),
        }
    }

    Ok(CharDev::Udp(CharUdp {
        id: id.ok_or_else(|| "udp chardev requires id=".to_string())?,
        host,
        port: port.ok_or_else(|| "udp chardev requires port=".to_string())?,
        localaddr,
        localport,
        ipv4,
        ipv6,
        mux,
        logfile,
        logappend,
    }))
}

fn parse_msmouse_chardev(parts: Vec<&str>) -> Result<CharDev, String> {
    let mut id = None;
    let mut mux = None;
    let mut logfile = None;
    let mut logappend = None;

    for part in parts {
        let (key, value) = part.split_once('=').ok_or_else(|| format!("invalid chardev msmouse option: {part}"))?;
        match key {
            "id" => id = Some(value.to_string()),
            "mux" => mux = Some(value.parse::<OnOff>().map_err(|_| format!("invalid mux value: {value}"))?),
            "logfile" => logfile = Some(PathBuf::from(value)),
            "logappend" => logappend = Some(value.parse::<OnOff>().map_err(|_| format!("invalid logappend value: {value}"))?),
            other => return Err(format!("unsupported chardev msmouse option: {other}")),
        }
    }

    Ok(CharDev::MsMouse(CharMsMouse {
        id: id.ok_or_else(|| "msmouse chardev requires id=".to_string())?,
        mux,
        logfile,
        logappend,
    }))
}

fn parse_hub_chardev(parts: Vec<&str>) -> Result<CharDev, String> {
    let mut id = None;
    let mut chardevs = Vec::new();

    for part in parts {
        let (key, value) = part.split_once('=').ok_or_else(|| format!("invalid chardev hub option: {part}"))?;
        if key == "id" {
            id = Some(value.to_string());
        } else if let Some(index) = key.strip_prefix("chardevs.") {
            chardevs.push((index.parse::<usize>().map_err(|e| e.to_string())?, value.to_string()));
        } else {
            return Err(format!("unsupported chardev hub option: {key}"));
        }
    }

    chardevs.sort_by_key(|(n, _)| *n);
    Ok(CharDev::Hub(CharHub {
        id: id.ok_or_else(|| "hub chardev requires id=".to_string())?,
        chardevs: (!chardevs.is_empty()).then_some(chardevs),
    }))
}

fn parse_vc_chardev(parts: Vec<&str>) -> Result<CharDev, String> {
    let mut id = None;
    let mut width = None;
    let mut height = None;
    let mut cols = None;
    let mut rows = None;
    let mut mux = None;
    let mut logfile = None;
    let mut logappend = None;

    for part in parts {
        let (key, value) = part.split_once('=').ok_or_else(|| format!("invalid chardev vc option: {part}"))?;
        match key {
            "id" => id = Some(value.to_string()),
            "width" => width = Some(value.parse::<usize>().map_err(|e| e.to_string())?),
            "height" => height = Some(value.parse::<usize>().map_err(|e| e.to_string())?),
            "cols" => cols = Some(value.parse::<usize>().map_err(|e| e.to_string())?),
            "rows" => rows = Some(value.parse::<usize>().map_err(|e| e.to_string())?),
            "mux" => mux = Some(value.parse::<OnOff>().map_err(|_| format!("invalid mux value: {value}"))?),
            "logfile" => logfile = Some(PathBuf::from(value)),
            "logappend" => logappend = Some(value.parse::<OnOff>().map_err(|_| format!("invalid logappend value: {value}"))?),
            other => return Err(format!("unsupported chardev vc option: {other}")),
        }
    }

    Ok(CharDev::Vc(CharVc {
        id: id.ok_or_else(|| "vc chardev requires id=".to_string())?,
        width,
        height,
        cols,
        rows,
        mux,
        logfile,
        logappend,
    }))
}

fn parse_ringbuf_chardev(parts: Vec<&str>) -> Result<CharDev, String> {
    let mut id = None;
    let mut size = None;
    let mut logfile = None;
    let mut logappend = None;

    for part in parts {
        let (key, value) = part.split_once('=').ok_or_else(|| format!("invalid chardev ringbuf option: {part}"))?;
        match key {
            "id" => id = Some(value.to_string()),
            "size" => size = Some(value.parse::<usize>().map_err(|e| e.to_string())?),
            "logfile" => logfile = Some(PathBuf::from(value)),
            "logappend" => logappend = Some(value.parse::<OnOff>().map_err(|_| format!("invalid logappend value: {value}"))?),
            other => return Err(format!("unsupported chardev ringbuf option: {other}")),
        }
    }

    Ok(CharDev::RingBuf(CharRingBuf {
        id: id.ok_or_else(|| "ringbuf chardev requires id=".to_string())?,
        size,
        logfile,
        logappend,
    }))
}

fn parse_file_chardev(parts: Vec<&str>) -> Result<CharDev, String> {
    let mut id = None;
    let mut path = None;
    let mut input_path = None;
    let mut mux = None;
    let mut logfile = None;
    let mut logappend = None;

    for part in parts {
        let (key, value) = part.split_once('=').ok_or_else(|| format!("invalid chardev file option: {part}"))?;
        match key {
            "id" => id = Some(value.to_string()),
            "path" => path = Some(PathBuf::from(value)),
            "input-path" => input_path = Some(PathBuf::from(value)),
            "mux" => mux = Some(value.parse::<OnOff>().map_err(|_| format!("invalid mux value: {value}"))?),
            "logfile" => logfile = Some(PathBuf::from(value)),
            "logappend" => logappend = Some(value.parse::<OnOff>().map_err(|_| format!("invalid logappend value: {value}"))?),
            other => return Err(format!("unsupported chardev file option: {other}")),
        }
    }

    Ok(CharDev::File(CharFile {
        id: id.ok_or_else(|| "file chardev requires id=".to_string())?,
        path: path.ok_or_else(|| "file chardev requires path=".to_string())?,
        input_path,
        mux,
        logfile,
        logappend,
    }))
}

fn parse_pipe_chardev(parts: Vec<&str>) -> Result<CharDev, String> {
    let mut id = None;
    let mut path = None;
    let mut mux = None;
    let mut logfile = None;
    let mut logappend = None;

    for part in parts {
        let (key, value) = part.split_once('=').ok_or_else(|| format!("invalid chardev pipe option: {part}"))?;
        match key {
            "id" => id = Some(value.to_string()),
            "path" => path = Some(PathBuf::from(value)),
            "mux" => mux = Some(value.parse::<OnOff>().map_err(|_| format!("invalid mux value: {value}"))?),
            "logfile" => logfile = Some(PathBuf::from(value)),
            "logappend" => logappend = Some(value.parse::<OnOff>().map_err(|_| format!("invalid logappend value: {value}"))?),
            other => return Err(format!("unsupported chardev pipe option: {other}")),
        }
    }

    Ok(CharDev::Pipe(CharPipe {
        id: id.ok_or_else(|| "pipe chardev requires id=".to_string())?,
        path: path.ok_or_else(|| "pipe chardev requires path=".to_string())?,
        mux,
        logfile,
        logappend,
    }))
}

fn parse_console_chardev(parts: Vec<&str>) -> Result<CharDev, String> {
    let mut id = None;
    let mut mux = None;
    let mut logfile = None;
    let mut logappend = None;

    for part in parts {
        let (key, value) = part.split_once('=').ok_or_else(|| format!("invalid chardev console option: {part}"))?;
        match key {
            "id" => id = Some(value.to_string()),
            "mux" => mux = Some(value.parse::<OnOff>().map_err(|_| format!("invalid mux value: {value}"))?),
            "logfile" => logfile = Some(PathBuf::from(value)),
            "logappend" => logappend = Some(value.parse::<OnOff>().map_err(|_| format!("invalid logappend value: {value}"))?),
            other => return Err(format!("unsupported chardev console option: {other}")),
        }
    }

    Ok(CharDev::Win32Console(CharWin32Console {
        id: id.ok_or_else(|| "console chardev requires id=".to_string())?,
        mux,
        logfile,
        logappend,
    }))
}

fn parse_serial_chardev(parts: Vec<&str>) -> Result<CharDev, String> {
    let mut id = None;
    let mut path = None;
    let mut mux = None;
    let mut logfile = None;
    let mut logappend = None;

    for part in parts {
        let (key, value) = part.split_once('=').ok_or_else(|| format!("invalid chardev serial option: {part}"))?;
        match key {
            "id" => id = Some(value.to_string()),
            "path" => path = Some(PathBuf::from(value)),
            "mux" => mux = Some(value.parse::<OnOff>().map_err(|_| format!("invalid mux value: {value}"))?),
            "logfile" => logfile = Some(PathBuf::from(value)),
            "logappend" => logappend = Some(value.parse::<OnOff>().map_err(|_| format!("invalid logappend value: {value}"))?),
            other => return Err(format!("unsupported chardev serial option: {other}")),
        }
    }

    Ok(CharDev::Serial(CharSerial {
        id: id.ok_or_else(|| "serial chardev requires id=".to_string())?,
        path: path.ok_or_else(|| "serial chardev requires path=".to_string())?,
        mux,
        logfile,
        logappend,
    }))
}

fn parse_pty_chardev(parts: Vec<&str>) -> Result<CharDev, String> {
    let mut id = None;
    let mut path = None;
    let mut mux = None;
    let mut logfile = None;
    let mut logappend = None;

    for part in parts {
        let (key, value) = part.split_once('=').ok_or_else(|| format!("invalid chardev pty option: {part}"))?;
        match key {
            "id" => id = Some(value.to_string()),
            "path" => path = Some(PathBuf::from(value)),
            "mux" => mux = Some(value.parse::<OnOff>().map_err(|_| format!("invalid mux value: {value}"))?),
            "logfile" => logfile = Some(PathBuf::from(value)),
            "logappend" => logappend = Some(value.parse::<OnOff>().map_err(|_| format!("invalid logappend value: {value}"))?),
            other => return Err(format!("unsupported chardev pty option: {other}")),
        }
    }

    Ok(CharDev::Pty(CharPty {
        id: id.ok_or_else(|| "pty chardev requires id=".to_string())?,
        path,
        mux,
        logfile,
        logappend,
    }))
}

fn parse_stdio_chardev(parts: Vec<&str>) -> Result<CharDev, String> {
    let mut id = None;
    let mut mux = None;
    let mut signal = None;
    let mut logfile = None;
    let mut logappend = None;

    for part in parts {
        let (key, value) = part.split_once('=').ok_or_else(|| format!("invalid chardev stdio option: {part}"))?;
        match key {
            "id" => id = Some(value.to_string()),
            "mux" => mux = Some(value.parse::<OnOff>().map_err(|_| format!("invalid mux value: {value}"))?),
            "signal" => signal = Some(value.parse::<OnOff>().map_err(|_| format!("invalid signal value: {value}"))?),
            "logfile" => logfile = Some(PathBuf::from(value)),
            "logappend" => logappend = Some(value.parse::<OnOff>().map_err(|_| format!("invalid logappend value: {value}"))?),
            other => return Err(format!("unsupported chardev stdio option: {other}")),
        }
    }

    Ok(CharDev::Stdio(CharStdio {
        id: id.ok_or_else(|| "stdio chardev requires id=".to_string())?,
        mux,
        signal,
        logfile,
        logappend,
    }))
}

fn parse_braille_chardev(parts: Vec<&str>) -> Result<CharDev, String> {
    let mut id = None;
    let mut mux = None;
    let mut logfile = None;
    let mut logappend = None;

    for part in parts {
        let (key, value) = part.split_once('=').ok_or_else(|| format!("invalid chardev braille option: {part}"))?;
        match key {
            "id" => id = Some(value.to_string()),
            "mux" => mux = Some(value.parse::<OnOff>().map_err(|_| format!("invalid mux value: {value}"))?),
            "logfile" => logfile = Some(PathBuf::from(value)),
            "logappend" => logappend = Some(value.parse::<OnOff>().map_err(|_| format!("invalid logappend value: {value}"))?),
            other => return Err(format!("unsupported chardev braille option: {other}")),
        }
    }

    Ok(CharDev::Braille(CharBraille {
        id: id.ok_or_else(|| "braille chardev requires id=".to_string())?,
        mux,
        logfile,
        logappend,
    }))
}

fn parse_parallel_chardev(parts: Vec<&str>) -> Result<CharDev, String> {
    let mut id = None;
    let mut path = None;
    let mut mux = None;
    let mut logfile = None;
    let mut logappend = None;

    for part in parts {
        let (key, value) = part.split_once('=').ok_or_else(|| format!("invalid chardev parallel option: {part}"))?;
        match key {
            "id" => id = Some(value.to_string()),
            "path" => path = Some(PathBuf::from(value)),
            "mux" => mux = Some(value.parse::<OnOff>().map_err(|_| format!("invalid mux value: {value}"))?),
            "logfile" => logfile = Some(PathBuf::from(value)),
            "logappend" => logappend = Some(value.parse::<OnOff>().map_err(|_| format!("invalid logappend value: {value}"))?),
            other => return Err(format!("unsupported chardev parallel option: {other}")),
        }
    }

    Ok(CharDev::Parallel(CharParallel {
        id: id.ok_or_else(|| "parallel chardev requires id=".to_string())?,
        path: path.ok_or_else(|| "parallel chardev requires path=".to_string())?,
        mux,
        logfile,
        logappend,
    }))
}

fn parse_spice_chardev(parts: Vec<&str>, is_vmc: bool) -> Result<CharDev, String> {
    let mut id = None;
    let mut name = None;
    let mut debug = None;
    let mut logfile = None;
    let mut logappend = None;

    for part in parts {
        let (key, value) = part.split_once('=').ok_or_else(|| format!("invalid chardev spice option: {part}"))?;
        match key {
            "id" => id = Some(value.to_string()),
            "name" => name = Some(value.to_string()),
            "debug" => debug = Some(value.to_string()),
            "logfile" => logfile = Some(PathBuf::from(value)),
            "logappend" => logappend = Some(value.parse::<OnOff>().map_err(|_| format!("invalid logappend value: {value}"))?),
            other => return Err(format!("unsupported chardev spice option: {other}")),
        }
    }

    let spice = CharSpice {
        id: id.ok_or_else(|| "spice chardev requires id=".to_string())?,
        name: name.ok_or_else(|| "spice chardev requires name=".to_string())?,
        debug,
        logfile,
        logappend,
    };

    Ok(if is_vmc { CharDev::SpiceVmc(spice) } else { CharDev::SpicePort(spice) })
}
