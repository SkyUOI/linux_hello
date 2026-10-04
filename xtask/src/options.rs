#[derive(clap::Parser)]
#[command(name = "xtask")]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(clap::Subcommand)]
pub enum Command {
    Prepare {
        #[arg(long, value_enum, default_value_t = Kernel::Noop)]
        kernel: Kernel,
        #[arg(short, long)]
        jobs: Option<u32>,
    },
    Build {
        #[arg(long, value_enum, default_value_t = Kernel::Noop)]
        kernel: Kernel,
        #[arg(long)]
        release: bool,
        #[arg(short, long)]
        jobs: Option<u32>,
    },
    Run {
        #[arg(long, value_enum, default_value_t = Kernel::Noop)]
        kernel: Kernel,
        #[arg(long)]
        release: bool,
        #[arg(short, long)]
        jobs: Option<u32>,
    },
    Install {
        #[arg(long, value_enum, default_value_t = Kernel::Noop)]
        kernel: Kernel,
        #[arg(short, long)]
        jobs: Option<u32>,
    },
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
