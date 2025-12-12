use {
    crate::templates::Template,
    heck::{ToKebabCase, ToSnakeCase},
    std::{
        fs,
        path::{Path, PathBuf},
    },
    toml_edit::{value, DocumentMut},
};

pub fn program(project_dir: Option<PathBuf>, program: &str) -> anyhow::Result<()> {
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
    Template::generate_program(&project_dir, program)?;

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

pub fn handler(path: Option<PathBuf>, program: &str, instruction: &str) -> anyhow::Result<()> {
    let project_dir = path.unwrap_or_else(|| PathBuf::from("."));
    if !project_dir.exists() {
        anyhow::bail!("No Typhoon workspace found at '{}'", project_dir.display());
    }

    let handler_dir = project_dir
        .join("handlers")
        .join(instruction.to_snake_case());
    if handler_dir.exists() {
        anyhow::bail!(
            "Handler '{}' already exists in the Typhoon workspace",
            instruction
        );
    }

    // Generate handler files
    Template::generate_handler(&project_dir, program, instruction)?;

    update_mod_file(
        &project_dir
            .join("programs")
            .join(program.to_snake_case())
            .join("src")
            .join("contexts")
            .join("mod.rs"),
        &instruction.to_snake_case(),
    )?;
    update_mod_file(
        &project_dir
            .join("programs")
            .join(program.to_snake_case())
            .join("src")
            .join("handlers")
            .join("mod.rs"),
        &instruction.to_snake_case(),
    )?;
    println!("\n✅ Handler added successfully!");

    Ok(())
}

fn update_mod_file(mod_path: &Path, mod_name: &str) -> anyhow::Result<()> {
    let mod_content = fs::read_to_string(mod_path)?;
    let mut mod_doc = mod_content.split("\n").collect::<Vec<&str>>();
    let new_line = format!("mod {};", mod_name.to_snake_case());
    mod_doc.insert(0, new_line.as_str());

    // Find last 'use' line and insert 'pub use *' after it
    let mut mod_doc = mod_doc.clone();
    let last_use_idx = mod_doc
        .iter()
        .rposition(|line| line.trim_start().starts_with("use "));
    let use_line = format!("pub use {}::*;", mod_name.to_snake_case());
    if let Some(idx) = last_use_idx {
        mod_doc.insert(idx + 1, &use_line);
    } else {
        // if no use found, insert after the first line
        mod_doc.insert(1, &use_line);
    }
    fs::write(mod_path, mod_doc.join("\n"))?;

    Ok(())
}
