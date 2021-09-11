use eg_diesel::*;
use std::io;
fn main() {
    loop {
        print_commands();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if let Ok(command) = parse_command(&input) {
            run_command(&command);
        } else {
            continue;
        }
    }
}
