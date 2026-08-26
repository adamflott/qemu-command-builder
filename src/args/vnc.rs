use crate::common::OnOff;
use crate::parsers::ARG_VNC;
use crate::parsers::DELIM_COMMA;
use crate::to_command::ToArg;
use crate::to_command::ToCommand;
use bon::Builder;
use proptest_derive::Arbitrary;
use std::path::PathBuf;
use std::str::FromStr;

/// A VNC server endpoint for `-vnc`.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum VNCDisplay {
    To(u64),
    Network { host: Option<String>, display: u64 },
    Unix(PathBuf),
    None,
}

/// VNC display sharing policy for `share=`.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum AllowExclusiveForceSharedIgnore {
    AllowExclusive,
    ForceShared,
    Ignore,
}

impl ToArg for AllowExclusiveForceSharedIgnore {
    fn to_arg(&self) -> &str {
        match self {
            AllowExclusiveForceSharedIgnore::AllowExclusive => "allow-exclusive",
            AllowExclusiveForceSharedIgnore::ForceShared => "force-shared",
            AllowExclusiveForceSharedIgnore::Ignore => "ignore",
        }
    }
}
/// Configure the QEMU VNC server.
///
/// `Display` and `FromStr` round-trip the canonical comma-separated
/// `-vnc` argument forms that this crate models.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct VNC {
    display: VNCDisplay,

    /// Connect to a listening VNC client via a "reverse" connection.
    /// The client is specified by the display. For reverse network
    /// connections (host:d,``reverse``), the d argument is a TCP port
    /// number, not a display number.    
    reverse: Option<OnOff>,

    /// Opens an additional TCP listening port dedicated to VNC
    /// Websocket connections. If a bare websocket option is given, the
    /// Websocket port is 5700+display. An alternative port can be
    /// specified with the syntax ``websocket``\ =port.
    ///
    /// If host is specified connections will only be allowed from this
    /// host. It is possible to control the websocket listen address
    /// independently, using the syntax ``websocket``\ =host:port.
    ///
    /// Websocket could be allowed over UNIX domain socket, using the syntax
    /// ``websocket``\ =unix:path, where path is the location of a unix socket
    /// to listen for connections on.
    ///
    /// If no TLS credentials are provided, the websocket connection
    /// runs in unencrypted mode. If TLS credentials are provided, the
    /// websocket connection requires encrypted client connections.
    websocket: Option<String>,

    /// Require that password based authentication is used for client
    /// connections.
    ///
    /// The password must be set separately using the ``set_password``
    /// command in the :ref:`QEMU monitor`. The
    /// syntax to change your password is:
    /// ``set_password <protocol> <password>`` where <protocol> could be
    /// either "vnc" or "spice".
    ///
    /// If you would like to change <protocol> password expiration, you
    /// should use ``expire_password <protocol> <expiration-time>``
    /// where expiration time could be one of the following options:
    ///     now, never, +seconds or UNIX time of expiration, e.g. +60 to
    /// make password expire in 60 seconds, or 1335196800 to make
    /// password expire on "Mon Apr 23 12:00:00 EDT 2012" (UNIX time for
    /// this date and time).
    ///
    /// You can also use keywords "now" or "never" for the expiration
    /// time to allow <protocol> password to expire immediately or never
    /// expire.
    password: Option<OnOff>,

    /// Require that password based authentication is used for client
    /// connections, using the password provided by the ``secret``
    /// object identified by ``secret-id``.
    password_secret: Option<String>,

    /// Provides the ID of a set of TLS credentials to use to secure the
    /// VNC server. They will apply to both the normal VNC server socket
    /// and the websocket socket (if enabled). Setting TLS credentials
    /// will cause the VNC server socket to enable the VeNCrypt auth
    /// mechanism. The credentials should have been previously created
    /// using the ``-object tls-creds`` argument.
    tls_creds: Option<String>,

    /// Provides the ID of the QAuthZ authorization object against which
    /// the client's x509 distinguished name will validated. This object
    /// is only resolved at time of use, so can be deleted and recreated
    /// on the fly while the VNC server is active. If missing, it will
    /// default to denying access.
    tls_authz: Option<String>,

    /// Require that the client use SASL to authenticate with the VNC
    /// server. The exact choice of authentication method used is
    /// controlled from the system / user's SASL configuration file for
    /// the 'qemu' service. This is typically found in
    /// /etc/sasl2/qemu.conf. If running QEMU as an unprivileged user,
    /// an environment variable SASL\_CONF\_PATH can be used to make it
    /// search alternate locations for the service config. While some
    /// SASL auth methods can also provide data encryption (eg GSSAPI),
    /// it is recommended that SASL always be combined with the 'tls'
    /// and 'x509' settings to enable use of SSL and server
    /// certificates. This ensures a data encryption preventing
    /// compromise of authentication credentials. See the
    /// :ref:`VNC security` section in the System Emulation Users Guide
    /// for details on using SASL authentication.    
    sasl: Option<OnOff>,

    /// Provides the ID of the QAuthZ authorization object against which
    /// the client's SASL username will validated. This object is only
    /// resolved at time of use, so can be deleted and recreated on the
    /// fly while the VNC server is active. If missing, it will default
    /// to denying access.
    sasl_authz: Option<String>,

    /// Legacy method for enabling authorization of clients against the
    /// x509 distinguished name and SASL username. It results in the
    /// creation of two ``authz-list`` objects with IDs of
    /// ``vnc.username`` and ``vnc.x509dname``. The rules for these
    /// objects must be configured with the HMP ACL commands.
    ///
    /// This option is deprecated and should no longer be used. The new
    /// ``sasl-authz`` and ``tls-authz`` options are a replacement.
    acl: Option<OnOff>,

    /// Enable lossy compression methods (gradient, JPEG, ...). If this
    /// option is set, VNC client may receive lossy framebuffer updates
    /// depending on its encoding settings. Enabling this option can
    /// save a lot of bandwidth at the expense of quality.   
    lossy: Option<OnOff>,

    /// Disable adaptive encodings. Adaptive encodings are enabled by
    /// default. An adaptive encoding will try to detect frequently
    /// updated screen regions, and send updates in these regions using
    /// a lossy encoding (like JPEG). This can be really helpful to save
    /// bandwidth when playing videos. Disabling adaptive encodings
    /// restores the original static behavior of encodings like Tight.
    non_adaptive: Option<OnOff>,

    /// Set display sharing policy. 'allow-exclusive' allows clients to
    /// ask for exclusive access. As suggested by the rfb spec this is
    /// implemented by dropping other connections. Connecting multiple
    /// clients in parallel requires all clients asking for a shared
    /// session (vncviewer: -shared switch). This is the default.
    /// 'force-shared' disables exclusive client access. Useful for
    /// shared desktop sessions, where you don't want someone forgetting
    /// specify -shared disconnect everybody else. 'ignore' completely
    /// ignores the shared flag and allows everybody connect
    /// unconditionally. Doesn't conform to the rfb spec but is
    /// traditional QEMU behavior.
    share: Option<AllowExclusiveForceSharedIgnore>,

    /// Set keyboard delay, for key down and key up events, in
    /// milliseconds. Default is 10. Keyboards are low-bandwidth
    /// devices, so this slowdown can help the device and guest to keep
    /// up and not lose events in case events are arriving in bulk.
    /// Possible causes for the latter are flaky network connections, or
    /// scripts for automated testing.
    key_delay_ms: Option<u64>,

    /// Use the specified audiodev when the VNC client requests audio
    /// transmission. When not using an -audiodev argument, this option
    /// must be omitted, otherwise is must be present and specify a
    /// valid audiodev.
    audiodev: Option<String>,

    /// Permit the remote client to issue shutdown, reboot or reset power
    /// control requests.
    power_control: Option<OnOff>,
}

impl ToCommand for VNC {
    fn command(&self) -> String {
        ARG_VNC.to_string()
    }
    fn to_args(&self) -> Vec<String> {
        let mut args = vec![];
        match &self.display {
            VNCDisplay::To(l) => {
                args.push(format!("to={}", l));
            }
            VNCDisplay::Network { host, display } => {
                if let Some(host) = host {
                    args.push(format!("{}:{}", host, display));
                } else {
                    args.push(format!(":{}", display));
                }
            }
            VNCDisplay::Unix(path) => {
                args.push(format!("unix={}", path.display()));
            }
            VNCDisplay::None => {
                args.push("none".to_string());
            }
        }

        if let Some(reverse) = &self.reverse {
            args.push(format!("reverse={}", reverse.to_arg()));
        }
        if let Some(websocket) = &self.websocket {
            args.push(format!("websocket={}", websocket));
        }
        if let Some(password) = &self.password {
            args.push(format!("password={}", password.to_arg()));
        }
        if let Some(password_secret) = &self.password_secret {
            args.push(format!("password-secret={}", password_secret));
        }
        if let Some(tls_creds) = &self.tls_creds {
            args.push(format!("tls-creds={}", tls_creds));
        }
        if let Some(tls_authz) = &self.tls_authz {
            args.push(format!("tls-authz={}", tls_authz));
        }
        if let Some(sasl) = &self.sasl {
            args.push(format!("sasl={}", sasl.to_arg()));
        }
        if let Some(sasl_authz) = &self.sasl_authz {
            args.push(format!("sasl-authz={}", sasl_authz));
        }
        if let Some(acl) = &self.acl {
            args.push(format!("acl={}", acl.to_arg()));
        }
        if let Some(lossy) = &self.lossy {
            args.push(format!("lossy={}", lossy.to_arg()));
        }
        if let Some(non_adaptive) = &self.non_adaptive {
            args.push(format!("non-adaptive={}", non_adaptive.to_arg()));
        }
        if let Some(share) = &self.share {
            args.push(format!("share={}", share.to_arg()));
        }
        if let Some(key_delay_ms) = &self.key_delay_ms {
            args.push(format!("key-delay-ms={}", key_delay_ms));
        }
        if let Some(audiodev) = &self.audiodev {
            args.push(format!("audiodev={}", audiodev));
        }
        if let Some(power_control) = &self.power_control {
            args.push(format!("power-control={}", power_control.to_arg()));
        }

        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for VNC {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(DELIM_COMMA);
        let display = parse_display(parts.next().ok_or_else(|| "empty -vnc argument".to_string())?)?;
        let mut value = VNC::builder().display(display).build();

        for part in parts {
            let (key, raw) = part.split_once('=').ok_or_else(|| format!("invalid -vnc option: {part}"))?;
            match key {
                "reverse" => value.reverse = Some(raw.parse::<OnOff>().map_err(|_| format!("invalid reverse value: {raw}"))?),
                "websocket" => value.websocket = Some(raw.to_string()),
                "password" => value.password = Some(raw.parse::<OnOff>().map_err(|_| format!("invalid password value: {raw}"))?),
                "password-secret" => value.password_secret = Some(raw.to_string()),
                "tls-creds" => value.tls_creds = Some(raw.to_string()),
                "tls-authz" => value.tls_authz = Some(raw.to_string()),
                "sasl" => value.sasl = Some(raw.parse::<OnOff>().map_err(|_| format!("invalid sasl value: {raw}"))?),
                "sasl-authz" => value.sasl_authz = Some(raw.to_string()),
                "acl" => value.acl = Some(raw.parse::<OnOff>().map_err(|_| format!("invalid acl value: {raw}"))?),
                "lossy" => value.lossy = Some(raw.parse::<OnOff>().map_err(|_| format!("invalid lossy value: {raw}"))?),
                "non-adaptive" => value.non_adaptive = Some(raw.parse::<OnOff>().map_err(|_| format!("invalid non-adaptive value: {raw}"))?),
                "share" => value.share = Some(parse_share(raw)?),
                "key-delay-ms" => value.key_delay_ms = Some(raw.parse::<u64>().map_err(|e| e.to_string())?),
                "audiodev" => value.audiodev = Some(raw.to_string()),
                "power-control" => value.power_control = Some(raw.parse::<OnOff>().map_err(|_| format!("invalid power-control value: {raw}"))?),
                other => return Err(format!("unsupported -vnc option: {other}")),
            }
        }
        Ok(value)
    }
}

fn parse_display(value: &str) -> Result<VNCDisplay, String> {
    if let Some(limit) = value.strip_prefix("to=") {
        return Ok(VNCDisplay::To(limit.parse::<u64>().map_err(|e| e.to_string())?));
    }
    if let Some(path) = value.strip_prefix("unix:") {
        return Ok(VNCDisplay::Unix(PathBuf::from(path)));
    }
    if value == "none" {
        return Ok(VNCDisplay::None);
    }
    if let Some((host, display)) = value.rsplit_once(':') {
        let display = display.parse::<u64>().map_err(|e| e.to_string())?;
        let host = if host.is_empty() { None } else { Some(host.to_string()) };
        return Ok(VNCDisplay::Network { host, display });
    }
    Err(format!("unsupported vnc display: {value}"))
}

fn parse_share(value: &str) -> Result<AllowExclusiveForceSharedIgnore, String> {
    match value {
        "allow-exclusive" => Ok(AllowExclusiveForceSharedIgnore::AllowExclusive),
        "force-shared" => Ok(AllowExclusiveForceSharedIgnore::ForceShared),
        "ignore" => Ok(AllowExclusiveForceSharedIgnore::Ignore),
        _ => Err(format!("invalid share value: {value}")),
    }
}
