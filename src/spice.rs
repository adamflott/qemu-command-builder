use crate::common::{AutoNeverAlways, OnOff, OnOffDefaultOff, OnOffDefaultOn};
use crate::parsers::DELIM_COMMA;
use crate::to_command::ToArg;
use crate::to_command::ToCommand;
use bon::Builder;
use proptest_derive::Arbitrary;
use std::path::PathBuf;
use std::str::FromStr;

pub(crate) const ARG_SPICE: &str = "-spice";

/// A SPICE channel name used by `tls-channel=` and `plaintext-channel=`.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum Channel {
    Main,
    Display,
    Cursor,
    Inputs,
    Record,
    Playback,
}

impl ToArg for Channel {
    fn to_arg(&self) -> &str {
        match self {
            Channel::Main => "main",
            Channel::Display => "display",
            Channel::Cursor => "cursor",
            Channel::Inputs => "inputs",
            Channel::Record => "record",
            Channel::Playback => "playback",
        }
    }
}
/// SPICE image compression mode for `image-compression=`.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Arbitrary)]
pub enum ImageCompression {
    AutoGlz,
    #[default]
    AutoLz,
    Quic,
    Glz,
    Lz,
    Off,
}

impl ToArg for ImageCompression {
    fn to_arg(&self) -> &str {
        match self {
            ImageCompression::AutoGlz => "auto_glz",
            ImageCompression::AutoLz => "auto_lz",
            ImageCompression::Quic => "quic",
            ImageCompression::Glz => "glz",
            ImageCompression::Lz => "lz",
            ImageCompression::Off => "off",
        }
    }
}
/// Ternary SPICE policy used by `streaming-video=`.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Arbitrary)]
pub enum OffAllFilter {
    #[default]
    Off,
    All,
    Filter,
}

impl ToArg for OffAllFilter {
    fn to_arg(&self) -> &str {
        match self {
            OffAllFilter::Off => "off",
            OffAllFilter::All => "all",
            OffAllFilter::Filter => "filter",
        }
    }
}
/// Enable the spice remote desktop protocol.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct Spice {
    /// Set the TCP port spice is listening on for plaintext channels.
    port: Option<u16>,

    /// Set the IP address spice is listening on. Default is any address.
    addr: Option<String>,

    /// Force using the specified IP version.
    ipv4: Option<OnOff>,
    ipv6: Option<OnOff>,
    unix: Option<OnOff>,

    /// Set the ID of the ``secret`` object containing the password
    /// you need to authenticate.
    password_secret: Option<String>,

    /// Require that the client use SASL to authenticate with the spice.
    /// The exact choice of authentication method used is controlled
    /// from the system / user's SASL configuration file for the 'qemu'
    /// service. This is typically found in /etc/sasl2/qemu.conf. If
    /// running QEMU as an unprivileged user, an environment variable
    /// SASL\_CONF\_PATH can be used to make it search alternate
    /// locations for the service config. While some SASL auth methods
    /// can also provide data encryption (eg GSSAPI), it is recommended
    /// that SASL always be combined with the 'tls' and 'x509' settings
    /// to enable use of SSL and server certificates. This ensures a
    /// data encryption preventing compromise of authentication
    /// credentials.
    sasl: Option<OnOff>,

    /// Allow client connects without authentication.
    disable_ticketing: Option<OnOff>,

    /// Disable copy paste between the client and the guest.
    disable_copy_paste: Option<OnOff>,

    /// Disable spice-vdagent based file-xfer between the client and the
    /// guest.
    disable_agent_file_xfer: Option<OnOff>,

    /// Set the TCP port spice is listening on for encrypted channels.
    tls_port: Option<u16>,

    /// Set the x509 file directory. Expects same filenames as -vnc
    /// $display,x509=$dir
    x509_dir: Option<PathBuf>,

    /// The x509 file names can also be configured individually.
    x509_key_file: Option<PathBuf>,
    x509_key_password: Option<PathBuf>,
    x509_cert_file: Option<PathBuf>,
    x509_cacert_file: Option<PathBuf>,
    x509_dh_key_file: Option<PathBuf>,

    /// Specify which ciphers to use.
    tls_ciphers: Option<String>,

    /// Force specific channel to be used with or without TLS
    /// encryption. The options can be specified multiple times to
    /// configure multiple channels. The special name "default" can be
    /// used to set the default mode. For channels which are not
    /// explicitly forced into one mode the spice client is allowed to
    /// pick tls/plaintext as he pleases.
    tls_channel: Option<Channel>,
    plaintext_channel: Option<Channel>,

    /// Configure image compression (lossless). Default is auto\_glz.
    image_compression: Option<ImageCompression>,

    /// Configure wan image compression (lossy for slow links). Default
    /// is auto.
    jpeg_wan_compression: Option<AutoNeverAlways>,
    zlib_glz_wan_compression: Option<AutoNeverAlways>,

    /// Configure video stream detection. Default is off.
    streaming_video: Option<OffAllFilter>,

    /// Enable/disable passing mouse events via vdagent. Default is on.
    agent_mouse: Option<OnOffDefaultOn>,

    /// Enable/disable audio stream compression (using celt 0.5.1).
    /// Default is on.
    playback_compression: Option<OnOffDefaultOn>,

    /// Enable/disable spice seamless migration. Default is off.
    seamless_migration: Option<OnOffDefaultOff>,

    /// Enable/disable OpenGL context. Default is off.
    gl: Option<OnOffDefaultOn>,

    /// DRM render node for OpenGL rendering. If not specified, it will
    /// pick the first available. (Since 2.9)
    rendernode: Option<PathBuf>,
}

impl ToCommand for Spice {
    fn command(&self) -> String {
        ARG_SPICE.to_string()
    }
    fn to_args(&self) -> Vec<String> {
        let mut args = vec![];
        if let Some(port) = &self.port {
            args.push(format!("port={}", port));
        }
        if let Some(addr) = &self.addr {
            args.push(format!("addr={}", addr));
        }
        if let Some(ipv4) = &self.ipv4 {
            args.push(format!("ipv4={}", ipv4.to_arg()));
        }
        if let Some(ipv6) = &self.ipv6 {
            args.push(format!("ipv6={}", ipv6.to_arg()));
        }
        if let Some(unix) = &self.unix {
            args.push(format!("unix={}", unix.to_arg()));
        }
        if let Some(password_secret) = &self.password_secret {
            args.push(format!("password-secret={}", password_secret));
        }
        if let Some(sasl) = &self.sasl {
            args.push(format!("sasl={}", sasl.to_arg()));
        }
        if let Some(disable_ticketing) = &self.disable_ticketing {
            args.push(format!("disable-ticketing={}", disable_ticketing.to_arg()));
        }
        if let Some(disable_copy_paste) = &self.disable_copy_paste {
            args.push(format!("disable-copy-paste={}", disable_copy_paste.to_arg()));
        }
        if let Some(disable_agent_file_xfer) = &self.disable_agent_file_xfer {
            args.push(format!("disable-agent-file-xfer={}", disable_agent_file_xfer.to_arg()));
        }
        if let Some(tls_port) = &self.tls_port {
            args.push(format!("tls-port={}", tls_port));
        }
        if let Some(x509_dir) = &self.x509_dir {
            args.push(format!("x509-dir={}", x509_dir.display()));
        }
        if let Some(x509_key_file) = &self.x509_key_file {
            args.push(format!("x509-key-file={}", x509_key_file.display()));
        }
        if let Some(x509_key_password) = &self.x509_key_password {
            args.push(format!("x509-key-password={}", x509_key_password.display()));
        }
        if let Some(x509_cert_file) = &self.x509_cert_file {
            args.push(format!("x509-cert-file={}", x509_cert_file.display()));
        }
        if let Some(x509_cacert_file) = &self.x509_cacert_file {
            args.push(format!("x509-cacert-file={}", x509_cacert_file.display()));
        }
        if let Some(x509_dh_key_file) = &self.x509_dh_key_file {
            args.push(format!("x509-dh-key-file={}", x509_dh_key_file.display()));
        }
        if let Some(tls_ciphers) = &self.tls_ciphers {
            args.push(format!("tls-ciphers={}", tls_ciphers));
        }
        if let Some(tls_channel) = &self.tls_channel {
            args.push(format!("tls-channel={}", tls_channel.to_arg()));
        }
        if let Some(plaintext_channel) = &self.plaintext_channel {
            args.push(format!("plaintext-channel={}", plaintext_channel.to_arg()));
        }
        if let Some(image_compression) = &self.image_compression {
            args.push(format!("image-compression={}", image_compression.to_arg()));
        }
        if let Some(jpeg_wan_compression) = &self.jpeg_wan_compression {
            args.push(format!("jpeg-wan-compression={}", jpeg_wan_compression.to_arg()));
        }
        if let Some(zlib_glz_wan_compression) = &self.zlib_glz_wan_compression {
            args.push(format!("zlib-glz-wan-compression={}", zlib_glz_wan_compression.to_arg()));
        }
        if let Some(streaming_video) = &self.streaming_video {
            args.push(format!("streaming-video={}", streaming_video.to_arg()));
        }
        if let Some(agent_mouse) = &self.agent_mouse {
            args.push(format!("agent-mouse={}", agent_mouse.to_arg()));
        }
        if let Some(playback_compression) = &self.playback_compression {
            args.push(format!("playback-compression={}", playback_compression.to_arg()));
        }
        if let Some(seamless_migration) = &self.seamless_migration {
            args.push(format!("seamless-migration={}", seamless_migration.to_arg()));
        }
        if let Some(gl) = &self.gl {
            args.push(format!("gl={}", gl.to_arg()));
        }
        if let Some(rendernode) = &self.rendernode {
            args.push(format!("rendernode={}", rendernode.display()));
        }

        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for Spice {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut value = Self::default();
        for part in s.split(DELIM_COMMA).filter(|part| !part.is_empty()) {
            let (key, raw) = part.split_once('=').ok_or_else(|| format!("invalid -spice option: {part}"))?;
            match key {
                "port" => value.port = Some(raw.parse::<u16>().map_err(|e| e.to_string())?),
                "addr" => value.addr = Some(raw.to_string()),
                "ipv4" => value.ipv4 = Some(raw.parse::<OnOff>().map_err(|_| format!("invalid ipv4 value: {raw}"))?),
                "ipv6" => value.ipv6 = Some(raw.parse::<OnOff>().map_err(|_| format!("invalid ipv6 value: {raw}"))?),
                "unix" => value.unix = Some(raw.parse::<OnOff>().map_err(|_| format!("invalid unix value: {raw}"))?),
                "password-secret" => value.password_secret = Some(raw.to_string()),
                "sasl" => value.sasl = Some(raw.parse::<OnOff>().map_err(|_| format!("invalid sasl value: {raw}"))?),
                "disable-ticketing" => value.disable_ticketing = Some(raw.parse::<OnOff>().map_err(|_| format!("invalid disable-ticketing value: {raw}"))?),
                "disable-copy-paste" => value.disable_copy_paste = Some(raw.parse::<OnOff>().map_err(|_| format!("invalid disable-copy-paste value: {raw}"))?),
                "disable-agent-file-xfer" => value.disable_agent_file_xfer = Some(raw.parse::<OnOff>().map_err(|_| format!("invalid disable-agent-file-xfer value: {raw}"))?),
                "tls-port" => value.tls_port = Some(raw.parse::<u16>().map_err(|e| e.to_string())?),
                "x509-dir" => value.x509_dir = Some(PathBuf::from(raw)),
                "x509-key-file" => value.x509_key_file = Some(PathBuf::from(raw)),
                "x509-key-password" => value.x509_key_password = Some(PathBuf::from(raw)),
                "x509-cert-file" => value.x509_cert_file = Some(PathBuf::from(raw)),
                "x509-cacert-file" => value.x509_cacert_file = Some(PathBuf::from(raw)),
                "x509-dh-key-file" => value.x509_dh_key_file = Some(PathBuf::from(raw)),
                "tls-ciphers" => value.tls_ciphers = Some(raw.to_string()),
                "tls-channel" => value.tls_channel = Some(parse_channel(raw)?),
                "plaintext-channel" => value.plaintext_channel = Some(parse_channel(raw)?),
                "image-compression" => value.image_compression = Some(parse_image_compression(raw)?),
                "jpeg-wan-compression" => value.jpeg_wan_compression = Some(parse_auto_never_always(raw)?),
                "zlib-glz-wan-compression" => value.zlib_glz_wan_compression = Some(parse_auto_never_always(raw)?),
                "streaming-video" => value.streaming_video = Some(parse_off_all_filter(raw)?),
                "agent-mouse" => value.agent_mouse = Some(raw.parse::<OnOffDefaultOn>().map_err(|_| format!("invalid agent-mouse value: {raw}"))?),
                "playback-compression" => value.playback_compression = Some(raw.parse::<OnOffDefaultOn>().map_err(|_| format!("invalid playback-compression value: {raw}"))?),
                "seamless-migration" => value.seamless_migration = Some(raw.parse::<OnOffDefaultOff>().map_err(|_| format!("invalid seamless-migration value: {raw}"))?),
                "gl" => value.gl = Some(raw.parse::<OnOffDefaultOn>().map_err(|_| format!("invalid gl value: {raw}"))?),
                "rendernode" => value.rendernode = Some(PathBuf::from(raw)),
                other => return Err(format!("unsupported -spice option: {other}")),
            }
        }
        Ok(value)
    }
}

fn parse_channel(value: &str) -> Result<Channel, String> {
    match value {
        "main" => Ok(Channel::Main),
        "display" => Ok(Channel::Display),
        "cursor" => Ok(Channel::Cursor),
        "inputs" => Ok(Channel::Inputs),
        "record" => Ok(Channel::Record),
        "playback" => Ok(Channel::Playback),
        _ => Err(format!("invalid spice channel: {value}")),
    }
}

fn parse_image_compression(value: &str) -> Result<ImageCompression, String> {
    match value {
        "auto_glz" => Ok(ImageCompression::AutoGlz),
        "auto_lz" => Ok(ImageCompression::AutoLz),
        "quic" => Ok(ImageCompression::Quic),
        "glz" => Ok(ImageCompression::Glz),
        "lz" => Ok(ImageCompression::Lz),
        "off" => Ok(ImageCompression::Off),
        _ => Err(format!("invalid image-compression value: {value}")),
    }
}

fn parse_off_all_filter(value: &str) -> Result<OffAllFilter, String> {
    match value {
        "off" => Ok(OffAllFilter::Off),
        "all" => Ok(OffAllFilter::All),
        "filter" => Ok(OffAllFilter::Filter),
        _ => Err(format!("invalid streaming-video value: {value}")),
    }
}

fn parse_auto_never_always(value: &str) -> Result<AutoNeverAlways, String> {
    match value {
        "auto" => Ok(AutoNeverAlways::Auto),
        "never" => Ok(AutoNeverAlways::Never),
        "always" => Ok(AutoNeverAlways::Always),
        _ => Err(format!("invalid auto/never/always value: {value}")),
    }
}
