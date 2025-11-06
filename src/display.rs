use std::path::PathBuf;

use crate::common::{OnOff, YesNo};
use crate::to_command::{ToArg, ToCommand};

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
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

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
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
    },
    None,
}

impl ToCommand for QemuDisplay {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec![];

        cmd.push("-display".to_string());

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
            QemuDisplay::Dbus {
                addr,
                p2p,
                gl,
                rendernode,
            } => {
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
            }
            QemuDisplay::None => {
                args.push("none".to_string());
            }
        }
        cmd.push(args.join(","));
        cmd
    }
}
