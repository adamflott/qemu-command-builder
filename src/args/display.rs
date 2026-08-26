use crate::parsers::ARG_DISPLAY;
use std::path::PathBuf;
use std::str::FromStr;

use proptest_derive::Arbitrary;

use crate::common::{OnOff, YesNo};
use crate::to_command::{ToArg, ToCommand};

/// QEMU `-display` backend selection.
///
/// Each variant models one documented `-display` form from the bundled QEMU
/// option reference. `to_args()` emits raw `argv` values, while `FromStr`
/// accepts the single argument that follows `-display`.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum OnCoreEsOff {
    On,
    Core,
    Es,
    Off,
}

impl ToArg for OnCoreEsOff {
    fn to_arg(&self) -> &str {
        match self {
            OnCoreEsOff::On => "on",
            OnCoreEsOff::Core => "core",
            OnCoreEsOff::Es => "es",
            OnCoreEsOff::Off => "off",
        }
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum QemuDisplay {
    Spice {
        gl: Option<OnOff>,
    },
    Sdl {
        gl: Option<OnCoreEsOff>,
        grab_mod: Option<String>,
        show_cursor: Option<OnOff>,
        window_close: Option<OnOff>,
    },
    Gtk {
        clipboard: Option<OnOff>,
        fullscreen: Option<OnOff>,
        gl: Option<OnOff>,
        grab_on_hover: Option<OnOff>,
        show_tabs: Option<OnOff>,
        show_cursor: Option<OnOff>,
        window_close: Option<OnOff>,
        show_menubar: Option<OnOff>,
        zoom_to_fit: Option<OnOff>,
    },
    Vnc {
        vnc: String,
        optargs: Option<String>,
    },
    Curses {
        charset: Option<String>,
    },
    Cocoa {
        full_grab: Option<OnOff>,
        swap_opt_cmd: Option<OnOff>,
        show_cursor: Option<OnOff>,
        left_command_key: Option<OnOff>,
        full_screen: Option<OnOff>,
        zoom_to_fit: Option<OnOff>,
    },
    EglHeadless {
        rendernode: Option<PathBuf>,
    },
    Dbus {
        addr: Option<String>,
        p2p: Option<YesNo>,
        gl: Option<OnCoreEsOff>,
        rendernode: Option<PathBuf>,
        audiodev: Option<String>,
    },
    None,
}

impl ToCommand for QemuDisplay {
    fn command(&self) -> String {
        ARG_DISPLAY.to_string()
    }
    fn to_args(&self) -> Vec<String> {
        let mut args = vec![];
        match self {
            QemuDisplay::Spice { gl } => {
                args.push("spice-app".to_string());
                if let Some(gl) = gl {
                    args.push(format!("gl={}", gl.to_arg()));
                }
            }
            QemuDisplay::Sdl {
                gl,
                grab_mod,
                show_cursor,
                window_close,
            } => {
                args.push("sdl".to_string());
                if let Some(gl) = gl {
                    args.push(format!("gl={}", gl.to_arg()));
                }
                if let Some(grab_mod) = grab_mod {
                    args.push(format!("grab-mod={}", grab_mod));
                }
                if let Some(show_cursor) = show_cursor {
                    args.push(format!("show-cursor={}", show_cursor.to_arg()));
                }
                if let Some(window_close) = window_close {
                    args.push(format!("window-close={}", window_close.to_arg()));
                }
            }
            QemuDisplay::Gtk {
                clipboard,
                fullscreen,
                gl,
                grab_on_hover,
                show_tabs,
                show_cursor,
                window_close,
                show_menubar,
                zoom_to_fit,
            } => {
                args.push("gtk".to_string());
                if let Some(clipboard) = clipboard {
                    args.push(format!("clipboard={}", clipboard.to_arg()));
                }
                if let Some(fullscreen) = fullscreen {
                    args.push(format!("full-screen={}", fullscreen.to_arg()));
                }
                if let Some(gl) = gl {
                    args.push(format!("gl={}", gl.to_arg()));
                }
                if let Some(grab_on_hover) = grab_on_hover {
                    args.push(format!("grab-on-hover={}", grab_on_hover.to_arg()));
                }
                if let Some(show_tabs) = show_tabs {
                    args.push(format!("show-tabs={}", show_tabs.to_arg()));
                }
                if let Some(show_cursor) = show_cursor {
                    args.push(format!("show-cursor={}", show_cursor.to_arg()));
                }
                if let Some(window_close) = window_close {
                    args.push(format!("window-close={}", window_close.to_arg()));
                }
                if let Some(show_menubar) = show_menubar {
                    args.push(format!("show-menubar={}", show_menubar.to_arg()));
                }
                if let Some(zoom_to_fit) = zoom_to_fit {
                    args.push(format!("zoom-to-fit={}", zoom_to_fit.to_arg()));
                }
            }
            QemuDisplay::Vnc { vnc, optargs } => {
                args.push(format!("vnc={}", vnc.clone()));
                if let Some(optargs) = optargs {
                    args.push(optargs.clone());
                }
            }
            QemuDisplay::Curses { charset } => {
                args.push("curses".to_string());
                if let Some(charset) = charset {
                    args.push(format!("charset={}", charset));
                }
            }
            QemuDisplay::Cocoa {
                full_grab,
                swap_opt_cmd,
                show_cursor,
                left_command_key,
                full_screen,
                zoom_to_fit,
            } => {
                args.push("cocoa".to_string());
                if let Some(full_grab) = full_grab {
                    args.push(format!("full-grab={}", full_grab.to_arg()));
                }
                if let Some(swap_opt_cmd) = swap_opt_cmd {
                    args.push(format!("swap-opt-cmd={}", swap_opt_cmd.to_arg()));
                }
                if let Some(show_cursor) = show_cursor {
                    args.push(format!("show-cursor={}", show_cursor.to_arg()));
                }
                if let Some(left_command_key) = left_command_key {
                    args.push(format!("left-command-key={}", left_command_key.to_arg()));
                }
                if let Some(full_screen) = full_screen {
                    args.push(format!("full-screen={}", full_screen.to_arg()));
                }
                if let Some(zoom_to_fit) = zoom_to_fit {
                    args.push(format!("zoom-to-fit={}", zoom_to_fit.to_arg()));
                }
            }
            QemuDisplay::EglHeadless { rendernode } => {
                args.push("egl-headless".to_string());
                if let Some(rendernode) = rendernode {
                    args.push(format!("rendernode={}", rendernode.display()));
                }
            }
            QemuDisplay::Dbus { addr, p2p, gl, rendernode, audiodev } => {
                args.push("dbus".to_string());
                if let Some(addr) = addr {
                    args.push(format!("addr={}", addr));
                }
                if let Some(p2p) = p2p {
                    args.push(format!("p2p={}", p2p.to_arg()));
                }
                if let Some(gl) = gl {
                    args.push(format!("gl={}", gl.to_arg()));
                }
                if let Some(rendernode) = rendernode {
                    args.push(format!("rendernode={}", rendernode.display()));
                }
                if let Some(audiodev) = audiodev {
                    args.push(format!("audiodev={audiodev}"));
                }
            }
            QemuDisplay::None => {
                args.push("none".to_string());
            }
        }

        vec![args.join(",")]
    }
}

impl FromStr for QemuDisplay {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "none" {
            return Ok(Self::None);
        }
        if s == "spice-app" {
            return Ok(Self::Spice { gl: None });
        }
        if let Some(rest) = s.strip_prefix("spice-app,") {
            let mut gl = None;
            for (key, value) in display_options(rest)? {
                match key {
                    "gl" => gl = Some(parse_on_off(key, value)?),
                    other => return Err(format!("unsupported spice-app display option: {other}")),
                }
            }
            return Ok(Self::Spice { gl });
        }
        if s == "sdl" {
            return Ok(Self::Sdl {
                gl: None,
                grab_mod: None,
                show_cursor: None,
                window_close: None,
            });
        }
        if let Some(rest) = s.strip_prefix("sdl,") {
            let (mut gl, mut grab_mod, mut show_cursor, mut window_close) = (None, None, None, None);
            for (key, value) in display_options(rest)? {
                match key {
                    "gl" => gl = Some(parse_gl(value)?),
                    "grab-mod" => grab_mod = Some(value.to_string()),
                    "show-cursor" => show_cursor = Some(parse_on_off(key, value)?),
                    "window-close" => window_close = Some(parse_on_off(key, value)?),
                    other => return Err(format!("unsupported SDL display option: {other}")),
                }
            }
            return Ok(Self::Sdl {
                gl,
                grab_mod,
                show_cursor,
                window_close,
            });
        }
        if s == "gtk" {
            return Ok(Self::Gtk {
                clipboard: None,
                fullscreen: None,
                gl: None,
                grab_on_hover: None,
                show_tabs: None,
                show_cursor: None,
                window_close: None,
                show_menubar: None,
                zoom_to_fit: None,
            });
        }
        if let Some(rest) = s.strip_prefix("gtk,") {
            let mut display = Self::Gtk {
                clipboard: None,
                fullscreen: None,
                gl: None,
                grab_on_hover: None,
                show_tabs: None,
                show_cursor: None,
                window_close: None,
                show_menubar: None,
                zoom_to_fit: None,
            };
            let Self::Gtk {
                clipboard,
                fullscreen,
                gl,
                grab_on_hover,
                show_tabs,
                show_cursor,
                window_close,
                show_menubar,
                zoom_to_fit,
            } = &mut display
            else {
                unreachable!()
            };
            for option in rest.split(',') {
                let (key, value) = option.split_once('=').ok_or_else(|| format!("invalid GTK display option: {option}"))?;
                let value = value.parse::<OnOff>().map_err(|_| format!("invalid {key} value: {value}"))?;
                match key {
                    "clipboard" => *clipboard = Some(value),
                    "full-screen" => *fullscreen = Some(value),
                    "gl" => *gl = Some(value),
                    "grab-on-hover" => *grab_on_hover = Some(value),
                    "show-tabs" => *show_tabs = Some(value),
                    "show-cursor" => *show_cursor = Some(value),
                    "window-close" => *window_close = Some(value),
                    "show-menubar" => *show_menubar = Some(value),
                    "zoom-to-fit" => *zoom_to_fit = Some(value),
                    other => return Err(format!("unsupported GTK display option: {other}")),
                }
            }
            return Ok(display);
        }
        if s == "curses" {
            return Ok(Self::Curses { charset: None });
        }
        if let Some(rest) = s.strip_prefix("curses,") {
            let options = display_options(rest)?;
            if options.len() != 1 || options[0].0 != "charset" {
                return Err(format!("unsupported curses display options: {rest}"));
            }
            return Ok(Self::Curses {
                charset: Some(options[0].1.to_string()),
            });
        }
        if s == "cocoa" {
            return Ok(Self::Cocoa {
                full_grab: None,
                swap_opt_cmd: None,
                show_cursor: None,
                left_command_key: None,
                full_screen: None,
                zoom_to_fit: None,
            });
        }
        if let Some(rest) = s.strip_prefix("cocoa,") {
            let (mut full_grab, mut swap_opt_cmd, mut show_cursor, mut left_command_key, mut full_screen, mut zoom_to_fit) = (None, None, None, None, None, None);
            for (key, value) in display_options(rest)? {
                let value = parse_on_off(key, value)?;
                match key {
                    "full-grab" => full_grab = Some(value),
                    "swap-opt-cmd" => swap_opt_cmd = Some(value),
                    "show-cursor" => show_cursor = Some(value),
                    "left-command-key" => left_command_key = Some(value),
                    "full-screen" => full_screen = Some(value),
                    "zoom-to-fit" => zoom_to_fit = Some(value),
                    other => return Err(format!("unsupported Cocoa display option: {other}")),
                }
            }
            return Ok(Self::Cocoa {
                full_grab,
                swap_opt_cmd,
                show_cursor,
                left_command_key,
                full_screen,
                zoom_to_fit,
            });
        }
        if s == "egl-headless" {
            return Ok(Self::EglHeadless { rendernode: None });
        }
        if let Some(rest) = s.strip_prefix("egl-headless,") {
            let options = display_options(rest)?;
            if options.len() != 1 || options[0].0 != "rendernode" {
                return Err(format!("unsupported egl-headless display options: {rest}"));
            }
            return Ok(Self::EglHeadless {
                rendernode: Some(PathBuf::from(options[0].1)),
            });
        }
        if s == "dbus" {
            return Ok(Self::Dbus {
                addr: None,
                p2p: None,
                gl: None,
                rendernode: None,
                audiodev: None,
            });
        }
        if let Some(rest) = s.strip_prefix("dbus,") {
            let (mut addr, mut p2p, mut gl, mut rendernode, mut audiodev) = (None, None, None, None, None);
            for (key, value) in display_options(rest)? {
                match key {
                    "addr" => addr = Some(value.to_string()),
                    "p2p" => p2p = Some(value.parse::<YesNo>().map_err(|_| format!("invalid p2p value: {value}"))?),
                    "gl" => gl = Some(parse_gl(value)?),
                    "rendernode" => rendernode = Some(PathBuf::from(value)),
                    "audiodev" => audiodev = Some(value.to_string()),
                    other => return Err(format!("unsupported D-Bus display option: {other}")),
                }
            }
            return Ok(Self::Dbus { addr, p2p, gl, rendernode, audiodev });
        }
        if let Some(rest) = s.strip_prefix("vnc=") {
            let (vnc, optargs) = match rest.split_once(',') {
                Some((vnc, optargs)) => (vnc.to_string(), Some(optargs.to_string())),
                None => (rest.to_string(), None),
            };
            return Ok(Self::Vnc { vnc, optargs });
        }

        Err(format!("unsupported -display value: {s}"))
    }
}

fn display_options(value: &str) -> Result<Vec<(&str, &str)>, String> {
    value.split(',').map(|part| part.split_once('=').ok_or_else(|| format!("invalid display option: {part}"))).collect()
}

fn parse_on_off(key: &str, value: &str) -> Result<OnOff, String> {
    value.parse::<OnOff>().map_err(|_| format!("invalid {key} value: {value}"))
}

fn parse_gl(value: &str) -> Result<OnCoreEsOff, String> {
    match value {
        "on" => Ok(OnCoreEsOff::On),
        "core" => Ok(OnCoreEsOff::Core),
        "es" => Ok(OnCoreEsOff::Es),
        "off" => Ok(OnCoreEsOff::Off),
        _ => Err(format!("invalid gl value: {value}")),
    }
}
