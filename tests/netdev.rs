use pretty_assertions::assert_eq;
use qemu_command_builder::common::OnOff;
use qemu_command_builder::netdev::{Bridge, HostAndMaybePort, HostAndPort, Hubport, NetDev, Socket, SocketRegular, Tap, VhostVdpa};
use qemu_command_builder::to_command::ToCommand;
use std::path::PathBuf;
use std::str::FromStr;

#[test]
fn netdev_bridge_uses_br_key_and_round_trips() {
    let bridge = NetDev::Bridge(
        Bridge::builder()
            .id("n1".to_string())
            .bridge("qemubr0".to_string())
            .helper("/path/to/qemu-bridge-helper".to_string())
            .build(),
    );

    let rendered = bridge.to_args()[0].clone();
    assert_eq!("bridge,id=n1,br=qemubr0,helper=/path/to/qemu-bridge-helper", rendered);
    assert_eq!(bridge, NetDev::from_str(&rendered).unwrap());
}

#[test]
fn netdev_socket_regular_emits_connect_key_and_round_trips() {
    let socket = NetDev::Socket(Socket::SocketRegular(
        SocketRegular::builder()
            .id("n1".to_string())
            .listen(HostAndMaybePort::builder().host("".to_string()).port(1234).build())
            .connection(HostAndPort::builder().host("127.0.0.1".to_string()).port(4321).build())
            .build(),
    ));

    let rendered = socket.to_args()[0].clone();
    assert_eq!("socket,id=n1,listen=:1234,connect=127.0.0.1:4321", rendered);
    assert_eq!(socket, NetDev::from_str(&rendered).unwrap());
}

#[test]
fn netdev_socket_multicast_round_trips() {
    let rendered = "socket,id=n1,mcast=239.192.168.1:1102,localaddr=1.2.3.4";
    let parsed = NetDev::from_str(rendered).unwrap();

    assert_eq!(rendered, parsed.to_args()[0]);
    assert_eq!(parsed, NetDev::from_str(&parsed.to_args()[0]).unwrap());
}

#[test]
fn netdev_vhost_user_accepts_type_prefix_and_queues() {
    let parsed = NetDev::from_str("type=vhost-user,id=net0,chardev=chr0,vhostforce=on,queues=4").unwrap();

    assert_eq!("vhost-user,id=net0,chardev=chr0,vhostforce=on,queues=4", parsed.to_args()[0]);
    assert_eq!(parsed, NetDev::from_str(&parsed.to_args()[0]).unwrap());
}

#[test]
fn netdev_vhost_vdpa_round_trips() {
    let netdev = NetDev::VhostVdpa(
        VhostVdpa::builder()
            .id("net0".to_string())
            .vhostdev(PathBuf::from("/dev/vhost-vdpa-0"))
            .vhostfd("5".to_string())
            .build(),
    );

    let rendered = netdev.to_args()[0].clone();
    assert_eq!("vhost-vdpa,id=net0,vhostdev=/dev/vhost-vdpa-0,vhostfd=5", rendered);
    assert_eq!(netdev, NetDev::from_str(&rendered).unwrap());
}

#[test]
fn netdev_hubport_round_trips() {
    let netdev = NetDev::Hubport(Hubport::builder().id("hub0".to_string()).hubid(7).netdev("upstream0".to_string()).build());

    let rendered = netdev.to_args()[0].clone();
    assert_eq!("hubport,id=hub0,hubid=7,netdev=upstream0", rendered);
    assert_eq!(netdev, NetDev::from_str(&rendered).unwrap());
}

#[test]
fn netdev_tap_still_round_trips() {
    let netdev = NetDev::Tap(
        Tap::builder()
            .id("net0".to_string())
            .ifname("tap0".to_string())
            .script(qemu_command_builder::netdev::ScriptOrNot::None)
            .downscript(qemu_command_builder::netdev::ScriptOrNot::None)
            .vhost(OnOff::On)
            .queues(8)
            .build(),
    );

    let rendered = netdev.to_args()[0].clone();
    assert_eq!("tap,id=net0,ifname=tap0,script=no,downscript=no,vhost=on,queues=8", rendered);
    assert_eq!(netdev, NetDev::from_str(&rendered).unwrap());
}

#[test]
fn netdev_stream_tcp_round_trips() {
    let rendered = "stream,id=stream0,server=on,addr.type=inet,addr.host=127.0.0.1,addr.port=4444,to=9,numeric=off,keep-alive=on,mptcp=off,addr.ipv4=on,addr.ipv6=off,reconnect-ms=1000";
    let parsed = NetDev::from_str(rendered).unwrap();

    assert_eq!(rendered, parsed.to_args()[0]);
    assert_eq!(parsed, NetDev::from_str(&parsed.to_args()[0]).unwrap());
}

#[test]
fn netdev_dgram_multicast_udp_fd_round_trips() {
    let rendered = "dgram,id=dg0,remote.type=inet,remote.host=239.1.1.1,remote.port=1234,local.type=fd,local.str=fdset0";
    let parsed = NetDev::from_str(rendered).unwrap();

    assert_eq!(rendered, parsed.to_args()[0]);
    assert_eq!(parsed, NetDev::from_str(&parsed.to_args()[0]).unwrap());
}

#[test]
fn netdev_vmnet_shared_round_trips() {
    let rendered = "vmnet-shared,id=vm0,isolated=off,nat66-prefix=fd00::/64,start-address=192.168.64.1,end-address=192.168.64.254,subnet-mask=255.255.255.0";
    let parsed = NetDev::from_str(rendered).unwrap();

    assert_eq!(rendered, parsed.to_args()[0]);
    assert_eq!(parsed, NetDev::from_str(&parsed.to_args()[0]).unwrap());
}
