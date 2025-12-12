use {
    clap::Parser,
    heck::ToKebabCase,
    std::{path::PathBuf, process::Command},
    tempdir::TempDir,
    typhoon_cli::{Cli, Commands},
};

#[test]
fn new() {
    let tmp_dir = TempDir::new("typhoon-test").unwrap();
    let project_dir = run(&tmp_dir, "test-project", false).unwrap();
    test(&project_dir).unwrap();

    // IDL should be generated
    let idl_dir = project_dir.join("target/idls");
    assert!(idl_dir.exists());
}

#[test]
fn new_force() {
    let tmp_dir = TempDir::new("typhoon-test").unwrap();
    run(&tmp_dir, "test-project", false).unwrap();

    assert!(run(&tmp_dir, "test-project", false)
        .unwrap_err()
        .to_string()
        .contains("already exists"));
    run(&tmp_dir, "test-project", true).unwrap();
}

fn run(tmp_dir: &TempDir, project_name: &str, force: bool) -> anyhow::Result<PathBuf> {
    let mut args = vec![
        "typhoon",
        "new",
        project_name,
        "--path",
        tmp_dir.path().to_str().unwrap(),
        "--typhoon-path",
        "../../",
    ];

    if force {
        args.push("--force");
    }

    let expected_params = Commands::New {
        name: project_name.to_string(),
        path: Some(tmp_dir.path().to_path_buf()),
        force,
        typhoon_path: Some(PathBuf::from("../../")),
    };

    let cmd = Cli::parse_from(args);
    let Commands::New {
        name,
        path,
        force,
        typhoon_path,
    } = &cmd.command;

    assert_eq!(cmd.command, expected_params);

    typhoon_cli::new::execute(name.clone(), path.clone(), *force, typhoon_path.clone())?;

    let project_dir = tmp_dir.path().join(name.to_kebab_case());

    assert!(project_dir.exists());

    Ok(project_dir)
}

fn test(project_dir: &PathBuf) -> anyhow::Result<()> {
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
