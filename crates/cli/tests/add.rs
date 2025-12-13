mod helpers;

use {
    crate::helpers::{new_workspace, test_workspace},
    clap::Parser,
    heck::ToSnakeCase,
    std::{fs, path::Path},
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
    run_program(&project_dir, new_program_name).unwrap();
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

    // Programs should be generated
    let program_dir = project_dir
        .join("programs")
        .join(program_name.to_snake_case());
    assert!(program_dir.exists());

    // Handlers should be generated
    let handler_dir = program_dir.join("src").join("handlers");
    assert!(handler_dir.exists());

    // Contexts should be generated
    let context_dir = program_dir.join("src").join("contexts");
    assert!(context_dir.exists());
}

#[test]
fn add_handler() {
    let tmp_dir = TempDir::new("typhoon-test").unwrap();
    let program_name = "counter";
    let handler_name = "new-handler";
    let project_dir: std::path::PathBuf = new_workspace(
        &tmp_dir,
        "test-project",
        Some(program_name.to_string()),
        false,
    )
    .unwrap();
    run_handler(&project_dir, program_name, handler_name).unwrap();
    test_workspace(&project_dir).unwrap();

    // Handlers should be generated
    let handler_path = project_dir
        .join("programs")
        .join(program_name.to_snake_case())
        .join("src")
        .join("handlers")
        .join(format!("{}.rs", handler_name.to_snake_case()));
    assert!(
        handler_path.exists(),
        "Handler path does not exist: {}",
        handler_path.display()
    );

    // Contexts should be generated
    let context_path = project_dir
        .join("programs")
        .join(program_name.to_snake_case())
        .join("src")
        .join("contexts")
        .join(format!("{}.rs", handler_name.to_snake_case()));
    assert!(
        context_path.exists(),
        "Context path does not exist: {}",
        context_path.display()
    );

    // Router should be updated
    let lib_path = project_dir
        .join("programs")
        .join(program_name.to_snake_case())
        .join("src")
        .join("lib.rs");
    let lib_content = fs::read_to_string(lib_path.clone()).unwrap();
    assert!(
        lib_content.contains(&format!("    3 => {},", handler_name.to_snake_case())),
        "Router does not contain new instruction: {}",
        lib_content
    );
}

fn run_program(project_dir: &Path, program_name: &str) -> anyhow::Result<()> {
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

    typhoon_cli::add::program(path.clone(), name)?;

    Ok(())
}

fn run_handler(
    project_dir: &Path,
    program_name: &str,
    instruction_name: &str,
) -> anyhow::Result<()> {
    let args = vec![
        "typhoon",
        "add",
        "handler",
        program_name,
        instruction_name,
        "--path",
        project_dir.to_str().unwrap(),
    ];

    let expected_params = Commands::Add {
        subcommand: AddSubcommand::Handler {
            path: Some(project_dir.to_path_buf()),
            program: program_name.to_string(),
            name: instruction_name.to_string(),
        },
    };

    let cmd = Cli::parse_from(args);
    let Commands::Add {
        subcommand:
            AddSubcommand::Handler {
                path,
                program,
                name,
            },
    } = &cmd.command
    else {
        anyhow::bail!("Failed to parse command: {:?}", cmd.command);
    };

    assert_eq!(cmd.command, expected_params);

    typhoon_cli::add::handler(path.clone(), program, name)?;

    Ok(())
}
