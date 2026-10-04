use clap::Parser;

mod build;
mod consts;
mod directories;
mod install;
mod options;
mod prepare;
mod run;
mod sh;

fn main() -> anyhow::Result<()> {
    let workspace_root = directories::workspace_directory()?;
    std::env::set_current_dir(&workspace_root)?;

    let args = options::Args::parse();

    match args.command {
        options::Command::Build { kernel, release, jobs } => build::build(kernel, release, jobs),
        options::Command::Run { kernel, release , jobs} => run::run(kernel, release, jobs),
        options::Command::Install { kernel , jobs} => install::install(kernel, jobs),
        options::Command::Prepare { kernel , jobs} => prepare::prepare(kernel, jobs),
    }
}
