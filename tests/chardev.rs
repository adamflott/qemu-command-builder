use pretty_assertions::assert_eq;
use qemu_command_builder::chardev::{CharDev, CharHub, CharNull, CharPipe, CharSocket, CharSocketTcp, CharSocketUds, CharStdio};
use qemu_command_builder::common::OnOff;
use qemu_command_builder::to_command::ToCommand;
use std::path::PathBuf;
use std::str::FromStr;

#[test]
fn chardev_round_trips_socket_tcp() {
    let chardev = CharDev::Socket(CharSocket::Tcp(
        CharSocketTcp::builder()
            .id("mon0".to_string())
            .host("127.0.0.1".to_string())
            .port(4444)
            .server(OnOff::On)
            .wait(OnOff::Off)
            .build(),
    ));

    let rendered = chardev.to_args()[0].clone();

    assert_eq!("socket,id=mon0,host=127.0.0.1,port=4444,server=on,wait=off", rendered);
    assert_eq!(chardev, CharDev::from_str(&rendered).unwrap());
}

#[test]
fn chardev_socket_accepts_bare_server_nowait() {
    let parsed = CharDev::from_str("socket,id=charmonitor,path=/vms/vm1/run/qemu.monitor,server,nowait,logfile=/vms/vm1/run/qemu_mon.log,logappend=off").unwrap();

    let expected = CharDev::Socket(CharSocket::Uds(
        CharSocketUds::builder()
            .id("charmonitor")
            .path(PathBuf::from("/vms/vm1/run/qemu.monitor"))
            .server(OnOff::On)
            .wait(OnOff::Off)
            .logfile(PathBuf::from("/vms/vm1/run/qemu_mon.log"))
            .logappend(OnOff::Off)
            .build(),
    ));

    assert_eq!(expected, parsed);
    assert_eq!(
        "socket,id=charmonitor,path=/vms/vm1/run/qemu.monitor,server=on,wait=off,logfile=/vms/vm1/run/qemu_mon.log,logappend=off",
        parsed.to_args()[0]
    );
}

#[test]
fn chardev_pipe_includes_required_path() {
    let chardev = CharDev::Pipe(CharPipe::builder().id("pipe0".to_string()).path(PathBuf::from("/tmp/pipe")).mux(OnOff::Off).build());

    let rendered = chardev.to_args()[0].clone();

    assert_eq!("pipe,id=pipe0,path=/tmp/pipe,mux=off", rendered);
    assert_eq!(chardev, CharDev::from_str(&rendered).unwrap());
}

#[test]
fn chardev_pty_round_trips_optional_path() {
    let chardev = CharDev::Pty(qemu_command_builder::chardev::CharPty::builder().id("pty0".to_string()).path(PathBuf::from("/tmp/pty")).build());

    let rendered = chardev.to_args()[0].clone();

    assert_eq!("pty,id=pty0,path=/tmp/pty", rendered);
    assert_eq!(chardev, CharDev::from_str(&rendered).unwrap());
}

#[test]
fn chardev_hub_serializes_chardev_indexes_correctly() {
    let chardev = CharDev::Hub(CharHub::builder().id("hub0".to_string()).chardevs(vec![(0, "pty0".to_string()), (1, "vc0".to_string())]).build());

    let rendered = chardev.to_args()[0].clone();

    assert_eq!("hub,id=hub0,chardevs.0=pty0,chardevs.1=vc0", rendered);
    assert_eq!(chardev, CharDev::from_str(&rendered).unwrap());
}

#[test]
fn chardev_null_preserves_common_options() {
    let chardev = CharDev::Null(
        CharNull::builder()
            .id("null0".to_string())
            .mux(OnOff::On)
            .logfile(PathBuf::from("/tmp/null.log"))
            .logappend(OnOff::Off)
            .build(),
    );

    let rendered = chardev.to_args()[0].clone();

    assert_eq!("null,id=null0,mux=on,logfile=/tmp/null.log,logappend=off", rendered);
    assert_eq!(chardev, CharDev::from_str(&rendered).unwrap());
}

#[test]
fn chardev_stdio_still_round_trips() {
    let chardev = CharDev::Stdio(CharStdio::builder().id("serial0".to_string()).mux(OnOff::Off).signal(OnOff::Off).build());

    let rendered = chardev.to_args()[0].clone();

    assert_eq!("stdio,id=serial0,mux=off,signal=off", rendered);
    assert_eq!(chardev, CharDev::from_str(&rendered).unwrap());
}
