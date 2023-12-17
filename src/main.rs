mod cli;
mod sysinfo;

fn main() {
    let command = std::env::args().nth(1).expect("No command given");

    cli::handle(command.as_str());
}
