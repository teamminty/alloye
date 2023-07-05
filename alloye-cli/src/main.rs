#[derive(clap::Parser, Clone)]

struct Cli {
    command: SubCommand
}

#[derive(clap::Subcommand, clap::ValueEnum, Clone)]
enum SubCommand {
    // TODO
}

fn main() {
    
}