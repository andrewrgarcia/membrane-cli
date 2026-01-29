use clap::{Parser, Subcommand};
use clap::builder::Styles;
use anyhow::Result;

mod core;
mod memfs;
mod sweep;
mod commands;
mod utils;


#[derive(Parser)]
#[command(
    name = "mem",
    version,
    styles = membrane_styles()
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}


#[derive(Subcommand)]
enum Commands {
    Init,
    Add { name: String },
    Show {
        project: Option<String>,
    },
    Set {
        project: String,
        key: String,
        value: Option<String>,
    },
    Rm { project: String },
    Keys {
        #[arg(long)]
        similar: bool,
    },
}



fn membrane_styles() -> Styles {
    Styles::styled()
        .header(
            anstyle::Style::new()
                .fg_color(Some(
                    anstyle::Color::Rgb(anstyle::RgbColor(255, 105, 180))
                ))
                .bold()
        )
        .usage(
            anstyle::Style::new()
                .fg_color(Some(
                    anstyle::Color::Rgb(anstyle::RgbColor(255, 105, 180))
                ))
        )
}

fn main() -> Result<()> {
    use std::env;
    use crate::utils::banner::print_wordmark;

    let args: Vec<String> = env::args().collect();

    // Show wordmark only on bare `mem`
    if args.len() == 1 {
        print_wordmark();
    }

    let cli = Cli::parse();

    match cli.command {
        Commands::Init => memfs::init_membrane(),
        Commands::Add { name } => commands::add::run(&name),
        Commands::Show { project } =>
            commands::show::run(project.as_deref()),
        Commands::Set { project, key, value } =>
            commands::set::run(&project, &key, value.as_deref()),
        Commands::Rm { project } =>
            commands::delete::run(&project),
        Commands::Keys { similar } =>
            commands::sweep_cmd::run(similar),
    }
}
