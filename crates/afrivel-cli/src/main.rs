//! Binaire `afrivel` — CLI globale du framework (scaffolding, codegen, dev, délégation runtime).

#![forbid(unsafe_code)]

mod cli;
mod cmd_dev;
mod cmd_make;
mod cmd_new;
mod cmd_runtime;
mod context;
mod manifest;
mod templates;
mod ui;
mod writer;

use clap::Parser;

use cli::{Cli, Command};
use ui::{CliResult, Ui};

fn main() {
    let cli = Cli::parse();
    let ui = Ui::new(&cli.globals);
    let env = templates::environment();

    let result: CliResult = match &cli.command {
        Command::New { name } => {
            cmd_new::run(&ui, &env, name, cli.globals.force, cli.globals.dry_run)
        }
        Command::MakeModule { name, model } => {
            cmd_make::run(&ui, &env, &cli.globals, name, model.as_deref())
        }
        Command::ModuleList => cmd_runtime::module_list(&ui),
        Command::Dev { port } => cmd_dev::run(&ui, *port),
        Command::Completion { shell } => cmd_runtime::completion(*shell),
        Command::Runtime(args) => cmd_runtime::delegate(args),
    };

    if let Err(e) = result {
        if !e.message.is_empty() {
            eprintln!("{e}");
        }
        std::process::exit(e.code);
    }
}
