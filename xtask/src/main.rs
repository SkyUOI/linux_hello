use clap::Parser;

mod directories;
mod kcm;
mod options;
mod sh;

fn main() -> anyhow::Result<()> {
    let workspace_root = directories::workspace_directory()?;
    std::env::set_current_dir(&workspace_root)?;

    let args = options::Args::parse();

    match args.package {
        options::Package::Kcm { command } => match command {
            options::Command::Build {
                kernel,
                release,
                jobs,
            } => kcm::build::build(kernel, release, jobs),
            options::Command::Run {
                kernel,
                release,
                jobs,
            } => kcm::run::run(kernel, release, jobs),
            options::Command::Install { kernel, jobs } => kcm::install::install(kernel, jobs),
            options::Command::Prepare { kernel, jobs } => kcm::prepare::prepare(kernel, jobs),
            options::Command::Uninstall {} => kcm::uninstall::uninstall(),
        },
    }
}
