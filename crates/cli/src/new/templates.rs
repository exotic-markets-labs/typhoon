use {
    anyhow::{Context, Result},
    handlebars::Handlebars,
    heck::{ToKebabCase, ToPascalCase, ToSnakeCase, ToTitleCase},
    serde::Serialize,
    std::{
        fs,
        path::{Path, PathBuf},
    },
};

#[derive(Serialize)]
struct TemplateContext {
    project_name_snake: String,
    project_name_kebab: String,
    project_name_title: String,
    project_name_pascal: String,
    typhoon_version: String,
    typhoon_builder_version: String,
    typhoon_idl_version: String,
}

pub struct Template {
    #[allow(dead_code)]
    name: String,
    files: Vec<TemplateFile>,
}

struct TemplateFile {
    path_template: String,
    content_template: String,
}

impl Template {
    pub fn generate(
        target_dir: &Path,
        project_name: &str,
        typhoon_path: Option<PathBuf>,
    ) -> Result<()> {
        let (typhoon_version, typhoon_builder_version, typhoon_idl_version) =
            if let Some(typhoon_path) = typhoon_path {
                // Convert typhoon_path to an absolute path
                let typhoon_path = typhoon_path.canonicalize().with_context(|| {
                    format!("Failed to canonicalize path: {}", typhoon_path.display())
                })?;
                (
                    format!(
                        "{{ path = \"{}\" }}",
                        typhoon_path.join("crates").join("lib").to_string_lossy()
                    ),
                    format!(
                        "{{ path = \"{}\" }}",
                        typhoon_path
                            .join("crates")
                            .join("instruction-builder")
                            .to_string_lossy()
                    ),
                    format!(
                        "{{ path = \"{}\" }}",
                        typhoon_path
                            .join("crates")
                            .join("idl-generator")
                            .to_string_lossy()
                    ),
                )
            } else {
                (
                    format!("\"{}\"", env!("CARGO_PKG_VERSION")).to_string(),
                    format!("\"{}\"", env!("CARGO_PKG_VERSION")).to_string(),
                    format!("\"{}\"", env!("CARGO_PKG_VERSION")).to_string(),
                )
            };

        let ctx = TemplateContext {
            project_name_snake: project_name.to_snake_case(),
            project_name_kebab: project_name.to_kebab_case(),
            project_name_title: project_name.to_title_case(),
            project_name_pascal: project_name.to_pascal_case(),
            typhoon_version,
            typhoon_builder_version,
            typhoon_idl_version,
        };

        let mut handlebars = Handlebars::new();
        handlebars.set_strict_mode(true);

        for file in &Self::workspace(project_name).files {
            // Render file path template
            let file_path = handlebars
                .render_template(&file.path_template, &ctx)
                .with_context(|| {
                    format!(
                        "Failed to render file path template: {}",
                        file.path_template
                    )
                })?;

            // Render file content template
            let file_content = handlebars
                .render_template(&file.content_template, &ctx)
                .with_context(|| {
                    format!(
                        "Failed to render file content template: {}",
                        file.path_template
                    )
                })?;

            let full_path = target_dir.join(&file_path);

            // Create parent directories if needed
            if let Some(parent) = full_path.parent() {
                fs::create_dir_all(parent)
                    .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
            }

            fs::write(&full_path, file_content)
                .with_context(|| format!("Failed to write file: {}", full_path.display()))?;
        }

        Ok(())
    }

    fn workspace(project_name: &str) -> Self {
        Self {
            name: "workspace".to_string(),
            files: vec![
                TemplateFile {
                    path_template: "Cargo.toml".to_string(),
                    content_template: include_str!("../../templates/Cargo.toml.template")
                        .to_string(),
                },
                TemplateFile {
                    path_template: format!("programs/{}/Cargo.toml", project_name.to_snake_case())
                        .to_string(),
                    content_template: include_str!("../../templates/programs/Cargo.toml.template")
                        .to_string(),
                },
                TemplateFile {
                    path_template: format!("programs/{}/build.rs", project_name.to_snake_case())
                        .to_string(),
                    content_template: include_str!("../../templates/programs/build.rs.template")
                        .to_string(),
                },
                TemplateFile {
                    path_template: format!("programs/{}/src/lib.rs", project_name.to_snake_case())
                        .to_string(),
                    content_template: include_str!("../../templates/programs/src/lib.rs.template")
                        .to_string(),
                },
                TemplateFile {
                    path_template: format!(
                        "programs/{}/src/state/mod.rs",
                        project_name.to_snake_case()
                    )
                    .to_string(),
                    content_template: include_str!(
                        "../../templates/programs/src/state/mod.rs.template"
                    )
                    .to_string(),
                },
                TemplateFile {
                    path_template: format!(
                        "programs/{}/src/state/counter.rs",
                        project_name.to_snake_case()
                    )
                    .to_string(),
                    content_template: include_str!(
                        "../../templates/programs/src/state/counter.rs.template"
                    )
                    .to_string(),
                },
                TemplateFile {
                    path_template: format!(
                        "programs/{}/src/handlers/mod.rs",
                        project_name.to_snake_case()
                    )
                    .to_string(),
                    content_template: include_str!(
                        "../../templates/programs/src/handlers/mod.rs.template"
                    )
                    .to_string(),
                },
                TemplateFile {
                    path_template: format!(
                        "programs/{}/src/handlers/initialize.rs",
                        project_name.to_snake_case()
                    )
                    .to_string(),
                    content_template: include_str!(
                        "../../templates/programs/src/handlers/initialize.rs.template"
                    )
                    .to_string(),
                },
                TemplateFile {
                    path_template: format!(
                        "programs/{}/src/handlers/close.rs",
                        project_name.to_snake_case()
                    )
                    .to_string(),
                    content_template: include_str!(
                        "../../templates/programs/src/handlers/close.rs.template"
                    )
                    .to_string(),
                },
                TemplateFile {
                    path_template: format!(
                        "programs/{}/src/handlers/increment.rs",
                        project_name.to_snake_case()
                    )
                    .to_string(),
                    content_template: include_str!(
                        "../../templates/programs/src/handlers/increment.rs.template"
                    )
                    .to_string(),
                },
                TemplateFile {
                    path_template: format!(
                        "programs/{}/src/contexts/mod.rs",
                        project_name.to_snake_case()
                    )
                    .to_string(),
                    content_template: include_str!(
                        "../../templates/programs/src/contexts/mod.rs.template"
                    )
                    .to_string(),
                },
                TemplateFile {
                    path_template: "tests/Cargo.toml".to_string(),
                    content_template: include_str!("../../templates/tests/Cargo.toml.template")
                        .to_string(),
                },
                TemplateFile {
                    path_template: "tests/src/lib.rs".to_string(),
                    content_template: include_str!("../../templates/tests/src/lib.rs.template")
                        .to_string(),
                },
                TemplateFile {
                    path_template: "tests/tests/integration.rs".to_string(),
                    content_template: include_str!(
                        "../../templates/tests/tests/integration.rs.template"
                    )
                    .to_string(),
                },
            ],
        }
    }
}
