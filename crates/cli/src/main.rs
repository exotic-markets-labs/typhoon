use {
    clap::Parser,
    typhoon_cli::{add, new, AddSubcommand, Cli, Commands},
};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::New {
            name,
            program,
            path,
            force,
            typhoon_path,
        } => {
            new::execute(name, program, path, force, typhoon_path)?;
        }
        Commands::Add { subcommand } => match subcommand {
            AddSubcommand::Program { path, name } => {
                add::program(path, &name)?;
            }
            AddSubcommand::Handler {
                path,
                program,
                name,
            } => {
                add::handler(path, &program, &name)?;
            }
        },
    }

    Ok(())
}
