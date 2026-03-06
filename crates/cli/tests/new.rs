mod helpers;

use {
    crate::helpers::{new_workspace, test_workspace},
    heck::ToSnakeCase,
    temp_dir::TempDir,
};

#[test]
fn new() {
    let project_name = "test-project";
    let tmp_dir = TempDir::new().unwrap();
    let project_dir = new_workspace(&tmp_dir, project_name, None, false).unwrap();
    test_workspace(&project_dir).unwrap();

    // IDL should be generated
    let idl_path = project_dir.join(format!("target/idls/{}.json", project_name.to_snake_case()));
    assert!(idl_path.exists());

    // Program should be generated
    let program_dir = project_dir.join(format!("programs/{}/src", project_name.to_snake_case()));
    assert!(program_dir.exists());
}

#[test]
fn new_with_program_name() {
    let project_name = "test-project";
    let program_name = "counter";
    let tmp_dir = TempDir::new().unwrap();
    let project_dir = new_workspace(
        &tmp_dir,
        project_name,
        Some(program_name.to_string()),
        false,
    )
    .unwrap();
    test_workspace(&project_dir).unwrap();

    // IDL should be generated
    let idl_path = project_dir.join(format!("target/idls/{}.json", program_name.to_snake_case()));
    assert!(idl_path.exists());

    // Program should be generated
    let program_dir = project_dir.join(format!("programs/{}/src", program_name.to_snake_case()));
    assert!(program_dir.exists());
}

#[test]
fn new_force() {
    let tmp_dir = TempDir::new().unwrap();
    new_workspace(&tmp_dir, "test-project", Some("counter".to_string()), false).unwrap();

    assert!(
        new_workspace(&tmp_dir, "test-project", Some("counter".to_string()), false)
            .unwrap_err()
            .to_string()
            .contains("already exists")
    );
    new_workspace(&tmp_dir, "test-project", Some("counter".to_string()), true).unwrap();
}

#[test]
fn new_invalid_name() {
    let tmp_dir = TempDir::new().unwrap();
    let test_names = vec![
        "", "_", "1", "123_123", "mod", "fn", "pub", "use", "struct", "enum", "impl", "self",
        "super", "crate",
    ];
    for name in test_names {
        assert!(
            new_workspace(&tmp_dir, name, None, false).is_err(),
            "Expected error for invalid name: {}",
            name
        );
    }
}
