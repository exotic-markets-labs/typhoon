use {
    crate::{templates::Template, utils::is_valid_name},
    anyhow::{Context, Result},
    heck::ToKebabCase,
    std::{fs, path::PathBuf},
};

pub fn execute(
    project_name: String,
    program_name: Option<String>,
    path: Option<PathBuf>,
    force: bool,
    typhoon_path: Option<PathBuf>,
) -> Result<()> {
    // Validate project name
    if !is_valid_name(&project_name) {
        anyhow::bail!("Project name '{}' is not a valid name", project_name);
    }
    if !program_name.as_ref().is_none_or(|p| is_valid_name(p)) {
        anyhow::bail!(
            "Program name '{}' is not a valid name",
            program_name.unwrap_or_default()
        );
    }

    // Check if target directory already exists
    let target_path = path
        .unwrap_or_else(|| PathBuf::from("."))
        .join(project_name.to_kebab_case());
    if target_path.exists() && !force {
        anyhow::bail!("Directory '{}' already exists", target_path.display());
    }

    // Get template

    println!("Creating new Typhoon project '{}'...", project_name);
    println!("Location: {}", target_path.display());

    // Create project directory
    fs::create_dir_all(&target_path)
        .with_context(|| format!("Failed to create directory: {}", target_path.display()))?;

    // Generate project files
    Template::generate_workspace(&target_path, &project_name, program_name, typhoon_path)?;

    println!("\n✅ Project created successfully!");
    println!("\nNext steps:");
    println!("  cd {}", target_path.display());
    println!("  cargo build-sbf");

    Ok(())
}
