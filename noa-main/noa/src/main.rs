extern crate init_sys;
extern crate noa_inst;
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    sync: bool,

    #[command(subcommand)]
    commands: Option<SubCommand>,
}

#[derive(Subcommand)]
enum SubCommand {
    Boot {
        #[command(subcommand)]
        boot: Boot,
    },
    Install {
        packages: Vec<String>,
        #[arg(short, long)]
        ask: bool,
        #[arg(short, long)]
        build: bool,
    },
}

#[derive(Subcommand)]
enum Boot {
    Start,
    Remove,
    Add,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match args {
        Cli { commands, sync } => {
            if sync {
                todo!("sync the package list")
            }

            match commands {
                Some(SubCommand::Boot { boot }) => match boot {
                    Boot::Add => todo!(),
                    Boot::Remove => todo!(),
                    Boot::Start => init_sys::boot_init::boot_linux().await,
                },
                Some(SubCommand::Install {
                    packages,
                    ask,
                    build,
                }) => {
                    noa_inst::install(&packages, ask, build);
                }

                None => {}
            }
        }
    }
}
