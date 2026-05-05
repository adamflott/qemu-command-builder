use qemu_command_builder::parser::parse_qemu_command_line;
use qemu_command_builder::to_command::ToCommand;

fn main() {
    for arg in std::env::args().skip(1) {
        let cmd = std::fs::read_to_string(arg).unwrap();
        match parse_qemu_command_line(&cmd) {
            Ok(q) => {
                println!("cmd: {}", cmd);
                println!("data: {:#?}", q);
                println!("single command: {:#?}", q.to_single_command());
                println!("args: {:#?}", q.to_args());
                println!("command: {:#?}", q.to_command());
            }
            Err(err) => {
                eprintln!("{:?}", err);
            }
        }
    }
}
