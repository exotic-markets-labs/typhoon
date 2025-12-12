mod helpers;

use {
    crate::helpers::{new_workspace, test_workspace},
    clap::Parser,
    heck::ToSnakeCase,
    std::path::Path,
    tempdir::TempDir,
    typhoon_cli::{AddSubcommand, Cli, Commands},
};

#[test]
fn add_program() {
    let tmp_dir = TempDir::new("typhoon-test").unwrap();
    let program_name = "counter";
    let new_program_name = "new-counter";
    let project_dir: std::path::PathBuf = new_workspace(
        &tmp_dir,
        "test-project",
        Some(program_name.to_string()),
        false,
    )
    .unwrap();
    run(&project_dir, new_program_name).unwrap();
    test_workspace(&project_dir).unwrap();

    // IDLs should be generated
    let old_idl_path =
        project_dir.join(format!("target/idls/{}.json", program_name.to_snake_case()));
    let new_idl_path = project_dir.join(format!(
        "target/idls/{}.json",
        new_program_name.to_snake_case()
    ));
    assert!(old_idl_path.exists());
    assert!(new_idl_path.exists());
}

fn run(project_dir: &Path, program_name: &str) -> anyhow::Result<()> {
    let args = vec![
        "typhoon",
        "add",
        "program",
        program_name,
        "--path",
        project_dir.to_str().unwrap(),
    ];

    let expected_params = Commands::Add {
        subcommand: AddSubcommand::Program {
            path: Some(project_dir.to_path_buf()),
            name: program_name.to_string(),
        },
    };

    let cmd = Cli::parse_from(args);
    let Commands::Add {
        subcommand: AddSubcommand::Program { path, name },
    } = &cmd.command
    else {
        anyhow::bail!("Failed to parse command: {:?}", cmd.command);
    };

    assert_eq!(cmd.command, expected_params);

    typhoon_cli::add::program(path.clone(), name.clone())?;

    Ok(())
}
