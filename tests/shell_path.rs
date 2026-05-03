use pretty_assertions::assert_eq;
use qemu_command_builder::drive::Drive;
use qemu_command_builder::runwith::{RunWith, UserOrIds};
use qemu_command_builder::shell_path::ShellPath;
use qemu_command_builder::shell_string::ShellString;
use qemu_command_builder::to_command::ToCommand;
use std::str::FromStr;

#[test]
fn shell_path_args_remain_raw() {
    let drive = Drive::builder().file(ShellPath::from("/tmp/my disk.img")).build();
    assert_eq!(drive.to_command(), vec!["-drive".to_string(), "file=/tmp/my disk.img".to_string()]);
}

#[test]
fn shell_path_single_command_is_escaped() {
    let run_with = RunWith::builder().chroot(ShellPath::from("/tmp/my root")).user(UserOrIds::User(ShellString::from("vmuser"))).build();

    assert_eq!(run_with.to_single_command(), "-run-with 'chroot=/tmp/my root,user=vmuser'");
}

#[test]
fn shell_path_round_trips_with_spaces() {
    let parsed = RunWith::from_str("chroot=/tmp/my root,user=vmuser").unwrap();
    let expected = RunWith::builder().chroot(ShellPath::from("/tmp/my root")).user(UserOrIds::User(ShellString::from("vmuser"))).build();

    assert_eq!(parsed, expected);
}
