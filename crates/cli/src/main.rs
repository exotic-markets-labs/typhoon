use {
    clap::Parser,
    typhoon_cli::{new, Cli, Commands},
};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::New {
            name,
            path,
            force,
            typhoon_path,
        } => {
            new::execute(name, path, force, typhoon_path)?;
        }
    }

    Ok(())
}
