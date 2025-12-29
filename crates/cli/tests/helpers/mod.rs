use {
    clap::Parser,
    heck::ToKebabCase,
    std::{path::PathBuf, process::Command},
    tempdir::TempDir,
    typhoon_cli::{Cli, Commands},
};

pub fn new_workspace(
    tmp_dir: &TempDir,
    project_name: &str,
    program_name: Option<String>,
    force: bool,
) -> anyhow::Result<PathBuf> {
    let mut args = vec![
        "typhoon",
        "new",
        project_name,
        "--path",
        tmp_dir.path().to_str().unwrap(),
        "--typhoon-path",
        "../../",
    ];

    if let Some(program_name) = &program_name {
        args.push("--program");
        args.push(program_name);
    }

    if force {
        args.push("--force");
    }

    let expected_params = Commands::New {
        name: project_name.to_string(),
        program: program_name.clone(),
        path: Some(tmp_dir.path().to_path_buf()),
        force,
        typhoon_path: Some(PathBuf::from("../../")),
    };

    let cmd = Cli::parse_from(args);
    let Commands::New {
        name,
        program,
        path,
        force,
        typhoon_path,
    } = &cmd.command
    else {
        anyhow::bail!("Failed to parse command: {:?}", cmd.command);
    };

    assert_eq!(cmd.command, expected_params);

    typhoon_cli::new::execute(
        name.clone(),
        program.clone(),
        path.clone(),
        *force,
        typhoon_path.clone(),
    )?;

    let project_dir = tmp_dir.path().join(name.to_kebab_case());

    assert!(project_dir.exists());

    Ok(project_dir)
}

pub fn test_workspace(project_dir: &PathBuf) -> anyhow::Result<()> {
    // Build the project
    let output = Command::new("cargo")
        .current_dir(project_dir)
        .arg("build-sbf")
        .output()?;

    assert!(
        output.status.success(),
        "Failed to build project: {}",
        String::from_utf8(output.stderr)?
    );

    // Run the integration test
    let output = Command::new("cargo")
        .current_dir(project_dir)
        .arg("test")
        .output()?;

    assert!(
        output.status.success(),
        "Failed to run integration test: {}",
        String::from_utf8(output.stderr)?
    );

    // Check the output
    let stdout = String::from_utf8(output.stdout)?;
    assert!(
        stdout.contains("test result: ok"),
        "Integration test failed: {}",
        stdout
    );

    Ok(())
}
