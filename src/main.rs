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
enum KeysAction {
    Rename {
        old: String,
        new: String,

        #[arg(long)]
        project: Option<String>,
    },
}


#[derive(Subcommand)]
enum Commands {
    Init,
    Add { name: String },
    Show {
        project: Option<String>,

        #[arg(long)]
        sort: Option<String>,

        #[arg(long)]
        desc: bool,
    },
    Set { project: String, key: String, value: Option<String> },
    Unset { project: String, key: String },
    Rm { project: String },
    Keys {
        #[command(subcommand)]
        action: Option<KeysAction>,

        #[arg(long)]
        similar: bool,
    },
    Push {
        file: Option<String>,

        #[arg(long)]
        as_name: Option<String>,
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
        Commands::Show { project, sort, desc } =>
            commands::show::run(
                project.as_deref(),
                sort.as_deref(),
                desc,
            ),
        Commands::Set { project, key, value } =>
            commands::set::run(&project, &key, value.as_deref()),
        Commands::Unset { project, key } =>
            commands::unset::run(&project, &key),
        Commands::Rm { project } =>
            commands::delete::run(&project),
        Commands::Keys { action, similar } => {
            match action {
                Some(KeysAction::Rename { old, new, project }) =>
                    commands::keys_rename::run(&old, &new, project.as_deref()),
                None =>
                    commands::sweep_cmd::run(similar),
            }
        },
        Commands::Push { file, as_name } =>
            commands::push::run(file.as_deref(), as_name.as_deref()),
    }
}
