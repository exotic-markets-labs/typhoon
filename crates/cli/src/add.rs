use std::{fs, path::PathBuf};

use heck::{ToKebabCase, ToSnakeCase};
use toml_edit::{value, DocumentMut};

use crate::templates::Template;

pub fn program(project_dir: Option<PathBuf>, program: String) -> anyhow::Result<()> {
    // Check if project directory already exists
    let project_dir = project_dir.unwrap_or_else(|| PathBuf::from("."));
    if !project_dir.exists() {
        anyhow::bail!("No Typhoon workspace found at '{}'", project_dir.display());
    }

    // Check if program already exists
    let program_dir = project_dir.join("programs").join(program.to_snake_case());
    if program_dir.exists() {
        anyhow::bail!(
            "Program '{}' already exists in the Typhoon workspace",
            program
        );
    }

    // Validate project name
    if program.is_empty() {
        anyhow::bail!("Program name cannot be empty");
    }

    println!("Adding program {} to the Typhoon workspace...", program);
    println!("Location: {}", project_dir.display());

    // Generate program files
    Template::generate_program(&project_dir, &program)?;

    // Update Cargo.toml
    let workspace_toml_path = project_dir.join("Cargo.toml");
    let workspace = fs::read_to_string(&workspace_toml_path)?;
    let mut workspace_doc = workspace.parse::<DocumentMut>()?;
    workspace_doc["workspace"]["dependencies"][program.to_kebab_case()]["path"] =
        value(format!("programs/{}", program.to_snake_case()));
    fs::write(workspace_toml_path, workspace_doc.to_string())?;

    let tests_toml_path = project_dir.join("tests").join("Cargo.toml");
    let tests = fs::read_to_string(&tests_toml_path)?;
    let mut tests_doc = tests.parse::<DocumentMut>()?;
    tests_doc["package"]["metadata"]["typhoon"]["builder-dependencies"][program.to_snake_case()]
        ["path"] = value(format!("../programs/{}", program.to_snake_case()));
    tests_doc["dev-dependencies"][program.to_kebab_case()]["workspace"] = value(true);
    fs::write(tests_toml_path, tests_doc.to_string())?;

    println!("\n✅ Program added successfully!");

    Ok(())
}

pub fn handler(path: Option<PathBuf>, program: String, instruction: String) -> anyhow::Result<()> {
    Ok(())
}
