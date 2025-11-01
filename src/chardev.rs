use std::path::PathBuf;

use bon::Builder;

use crate::common::OnOff;
use crate::to_command::{ToArg, ToCommand};

#[derive(Default, Builder)]
pub struct CharNull {
    id: String,
}

#[derive(Default, Builder)]
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

#[derive(Default, Builder)]
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

pub enum CharSocket {
    Tcp(CharSocketTcp),
    Uds(CharSocketUds),
}

#[derive(Default, Builder)]
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

#[derive(Default, Builder)]
pub struct CharMsMouse {
    id: String,
    mux: Option<OnOff>,
    logfile: Option<PathBuf>,
    logappend: Option<OnOff>,
}

#[derive(Default, Builder)]
pub struct CharHub {
    id: String,
    chardevs: Option<Vec<(usize, String)>>,
}
#[derive(Default, Builder)]
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

#[derive(Default, Builder)]
pub struct CharRingBuf {
    id: String,
    size: Option<usize>,
    logfile: Option<PathBuf>,
    logappend: Option<OnOff>,
}
#[derive(Default, Builder)]
pub struct CharFile {
    id: String,
    path: PathBuf,
    input_path: Option<PathBuf>,
    mux: Option<OnOff>,
    logfile: Option<PathBuf>,
    logappend: Option<OnOff>,
}

#[derive(Default, Builder)]
pub struct CharPipe {
    id: String,
    mux: Option<OnOff>,
    logfile: Option<PathBuf>,
    logappend: Option<OnOff>,
}

#[derive(Default, Builder)]
pub struct CharWin32Console {
    id: String,
    mux: Option<OnOff>,
    logfile: Option<PathBuf>,
    logappend: Option<OnOff>,
}

#[derive(Default, Builder)]
pub struct CharWin32Serial {
    id: String,
    path: PathBuf,
    mux: Option<OnOff>,
    logfile: Option<PathBuf>,
    logappend: Option<OnOff>,
}

#[derive(Default, Builder)]
pub struct CharPty {
    id: String,
    mux: Option<OnOff>,
    logfile: Option<PathBuf>,
    logappend: Option<OnOff>,
}

#[derive(Default, Builder)]
pub struct CharStdio {
    id: String,
    mux: Option<OnOff>,
    signal: Option<OnOff>,
    logfile: Option<PathBuf>,
    logappend: Option<OnOff>,
}

#[derive(Default, Builder)]
pub struct CharBraille {
    id: String,
    mux: Option<OnOff>,
    logfile: Option<PathBuf>,
    logappend: Option<OnOff>,
}
#[derive(Default, Builder)]
pub struct CharSerial {
    id: String,
    path: PathBuf,
    mux: Option<OnOff>,
    logfile: Option<PathBuf>,
    logappend: Option<OnOff>,
}
#[derive(Default, Builder)]
pub struct CharParallel {
    id: String,
    path: PathBuf,
    mux: Option<OnOff>,
    logfile: Option<PathBuf>,
    logappend: Option<OnOff>,
}

#[derive(Default, Builder)]
pub struct CharSpice {
    id: String,
    name: String,
    debug: Option<String>,
    logfile: Option<PathBuf>,
    logappend: Option<OnOff>,
}

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
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec!["-chardev".to_string()];

        let mut args = vec![];
        match self {
            CharDev::Null(null) => {
                args.push("null".to_string());
                args.push(format!("id={}", null.id));
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
                    for (chardev, n) in chardevs {
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

        cmd.push(args.join(","));

        cmd
    }
}
