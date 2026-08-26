use pretty_assertions::assert_eq;
use qemu_command_builder::QemuInstanceForX86_64;
use qemu_command_builder::args::display::QemuDisplay;
use qemu_command_builder::args::smbios::Smbios;
use qemu_command_builder::args::spice::{Channel, ImageCompression, OffAllFilter, Spice};
use qemu_command_builder::args::vnc::{AllowExclusiveForceSharedIgnore, VNC, VNCDisplay};
use qemu_command_builder::common::{AutoNeverAlways, OnOff, OnOffDefaultOff, OnOffDefaultOn};
use qemu_command_builder::to_command::ToCommand;
use std::path::PathBuf;
use std::str::FromStr;

#[test]
fn smbios_additional_types_round_trip() {
    let type3 = Smbios::from_str("type=3,manufacturer=Acme,version=1.0,serial=123,asset=asset0,sku=sku0").unwrap();
    let type17 = Smbios::from_str("type=17,loc_pfx=DIMM,bank=BANK0,manufacturer=Acme,serial=abcd,asset=asset1,part=part1,speed=3200").unwrap();
    let type41 = Smbios::from_str("type=41,designation=net0,kind=ethernet,instance=1,pcidev=0000:00:02.0").unwrap();

    assert_eq!("type=3,manufacturer=Acme,version=1.0,serial=123,asset=asset0,sku=sku0", type3.to_args()[0]);
    assert_eq!("type=17,loc_pfx=DIMM,bank=BANK0,manufacturer=Acme,serial=abcd,asset=asset1,part=part1,speed=3200", type17.to_args()[0]);
    assert_eq!("type=41,designation=net0,kind=ethernet,instance=1,pcidev=0000:00:02.0", type41.to_args()[0]);
}

#[test]
fn spice_round_trips_supported_properties() {
    let spice = Spice::builder()
        .port(5900)
        .addr("127.0.0.1".to_string())
        .tls_port(5901)
        .disable_ticketing(OnOff::On)
        .x509_cacert_file(PathBuf::from("/etc/pki/ca-cert.pem"))
        .tls_channel(Channel::Main)
        .plaintext_channel(Channel::Display)
        .image_compression(ImageCompression::Quic)
        .jpeg_wan_compression(AutoNeverAlways::Always)
        .streaming_video(OffAllFilter::Filter)
        .agent_mouse(OnOffDefaultOn::Off)
        .playback_compression(OnOffDefaultOn::On)
        .seamless_migration(OnOffDefaultOff::On)
        .gl(OnOffDefaultOn::Off)
        .rendernode(PathBuf::from("/dev/dri/renderD128"))
        .build();

    let rendered = spice.to_args()[0].clone();
    assert_eq!(
        "port=5900,addr=127.0.0.1,disable-ticketing=on,tls-port=5901,x509-cacert-file=/etc/pki/ca-cert.pem,tls-channel=main,plaintext-channel=display,image-compression=quic,jpeg-wan-compression=always,streaming-video=filter,agent-mouse=off,playback-compression=on,seamless-migration=on,gl=off,rendernode=/dev/dri/renderD128",
        rendered
    );
    assert_eq!(spice, Spice::from_str(&rendered).unwrap());
}

#[test]
fn vnc_round_trips_network_display_and_options() {
    let vnc = VNC::builder()
        .display(VNCDisplay::Network {
            host: Some("localhost".to_string()),
            display: 0,
        })
        .reverse(OnOff::Off)
        .websocket("5700".to_string())
        .password(OnOff::On)
        .share(AllowExclusiveForceSharedIgnore::ForceShared)
        .key_delay_ms(25)
        .audiodev("audio0".to_string())
        .power_control(OnOff::On)
        .build();

    let rendered = vnc.to_args()[0].clone();
    assert_eq!(
        "localhost:0,reverse=off,websocket=5700,password=on,share=force-shared,key-delay-ms=25,audiodev=audio0,power-control=on",
        rendered
    );
    assert_eq!(vnc, VNC::from_str(&rendered).unwrap());
}

#[test]
fn qemu_instance_parses_vnc() {
    let cmd = "/usr/bin/qemu-system-x86_64 -vnc localhost:0,password=on -nodefaults";
    let parsed = QemuInstanceForX86_64::from_str(cmd).unwrap();
    assert_eq!(cmd, parsed.to_single_command());
}

#[test]
fn qemu_instance_parses_spice() {
    let cmd = "/usr/bin/qemu-system-x86_64 -spice port=5900,disable-ticketing=on -nodefaults";
    let parsed = QemuInstanceForX86_64::from_str(cmd).unwrap();
    assert_eq!(cmd, parsed.to_single_command());
}

#[test]
fn gtk_display_round_trips_qemu_11_1_clipboard() {
    let display = QemuDisplay::from_str("gtk,clipboard=on,full-screen=off").unwrap();
    assert_eq!(vec!["gtk,clipboard=on,full-screen=off"], display.to_args());
}

#[test]
fn display_backends_round_trip_all_structured_forms() {
    for value in [
        "spice-app,gl=on",
        "sdl,gl=core,grab-mod=ctrl-alt,show-cursor=on,window-close=off",
        "curses,charset=UTF-8",
        "cocoa,full-grab=on,swap-opt-cmd=off,show-cursor=on,left-command-key=off,full-screen=on,zoom-to-fit=off",
        "egl-headless,rendernode=/dev/dri/renderD128",
        "dbus,addr=unix:path=/tmp/qemu-dbus,p2p=yes,gl=es,rendernode=/dev/dri/renderD128,audiodev=audio0",
    ] {
        let parsed = QemuDisplay::from_str(value).unwrap();
        assert_eq!(value, parsed.to_args()[0]);
        assert_eq!(parsed, QemuDisplay::from_str(&parsed.to_args()[0]).unwrap());
    }
}
