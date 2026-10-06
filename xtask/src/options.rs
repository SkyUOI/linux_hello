#[derive(clap::Parser)]
#[command(
    name = "xtask",
    override_usage = "cargo xtask <PACKAGE> <SUBCOMMAND> [OPTIONS]",
    subcommand_help_heading = "Package",
    disable_help_subcommand = true
)]
pub struct Args {
    #[command(subcommand)]
    pub package: Package,
}

#[derive(clap::Subcommand)]
pub enum Package {
    /// xtask for KCM Linux Hello (package `kcm`)
    #[command(
        override_usage = "cargo xtask kcm <SUBCOMMAND> [OPTIONS]",
        subcommand_help_heading = "Subcommands"
    )]
    Kcm {
        #[command(subcommand)]
        command: Command,
    },
}

#[derive(clap::Subcommand)]
pub enum Command {
    /// Prepare the kernel for building
    #[command(override_usage = "cargo xtask kcm prepare [OPTIONS]")]
    Prepare {
        /// Kernel to prepare for building. Defaults to noop
        #[arg(long, value_enum, default_value_t = Kernel::Noop)]
        kernel: Kernel,
        /// Number of jobs to use for building. 
        #[arg(short, long)]
        jobs: Option<u32>,
    },
    /// Build the kernel
    #[command(override_usage = "cargo xtask kcm build [OPTIONS]")]
    Build {
        /// Kernel to build. Defaults to noop.
        #[arg(long, value_enum, default_value_t = Kernel::Noop)]
        kernel: Kernel,
        /// Whether to build the kernel in release mode.
        #[arg(long)]
        release: bool,
        /// Number of jobs to use for building. 
        #[arg(short, long)]
        jobs: Option<u32>,
    },
    /// Run the kernel  
    #[command(override_usage = "cargo xtask kcm run [OPTIONS]")]
    Run {
        /// Kernel to run. Defaults to noop.
        #[arg(long, value_enum, default_value_t = Kernel::Noop)]
        kernel: Kernel,
        /// Whether to run the kernel in release mode.
        #[arg(long)]
        release: bool,
        /// Number of jobs to use for running. 
        #[arg(short, long)]
        jobs: Option<u32>,
    },
    /// Install the kernel to the system
    #[command(override_usage = "cargo xtask kcm install [OPTIONS]")]
    Install {
        /// Kernel to install. Defaults to noop.
        #[arg(long, value_enum, default_value_t = Kernel::Noop)]
        kernel: Kernel,
        /// Number of jobs to use for installation. 
        #[arg(short, long)]
        jobs: Option<u32>,
    },
    /// Uninstall the kernel from the system
    #[command(override_usage = "cargo xtask kcm uninstall")]
    Uninstall {},
}

#[derive(clap::ValueEnum, Clone)]
pub enum Kernel {
    Noop,
    FaceId,
}

impl Kernel {
    pub fn feature(&self) -> &'static str {
        match self {
            Kernel::Noop => "noop-kernel",
            Kernel::FaceId => "face-id-kernel",
        }
    }
}
