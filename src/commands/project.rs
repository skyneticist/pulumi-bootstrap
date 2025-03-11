use anyhow::Ok;
use std::env;
use std::fs;
use std::sync::Arc;

use crate::helpers::config_helper::create_config_files;
use crate::helpers::entrypoint_helper::create_entry_point;
use crate::helpers::pipeline_helper::create_pipeline_yaml;
use crate::tui::app::ProjectConfig;
use crate::tui::tui_main::tui_main;

/// Initializes a new Pulumi project by copying and modifying a template.
///
/// # Arguments
/// * `template_path` - Path to the custom template on disk (optional).
/// * `output_dir` - The directory where the generated Pulumi files should be written.
/// * `app_name` - The name of the project.
///
/// # Returns
/// Result<(), Box<dyn std::error::Error>>
pub async fn new(
    name: Option<&str>,
    output_dir: Option<&str>,
    interactive: Option<bool>,
) -> Result<(), anyhow::Error> {
    let config = match interactive {
        Some(true) => tui_main()?,
        Some(false) => ProjectConfig::default(name.ok_or(anyhow::Error::msg("Name is required"))?),
        _ => ProjectConfig::default(name.ok_or(anyhow::Error::msg("Name is required"))?),
    };

    // Default the output directory to a subdirectory named after the app
    let output_dir: String = match output_dir {
        Some(dir) => dir.to_string(),
        _ => {
            let current_dir = env::current_dir()?.to_string_lossy().to_string();
            format!("{}/{}", current_dir, &config.resource_name)
        }
    };

    // Ensure the new project directory exists
    fs::create_dir_all(&output_dir)?;

    // Atomic Reference Context is used here for shared async variables
    let output_dir_arc: Arc<String> = Arc::new(output_dir.clone());

    tokio::try_join!(
        create_pipeline_yaml(&config, Some(&output_dir_arc)),
        create_entry_point(&config, Some(&output_dir_arc)),
        create_config_files(&config, Some(&output_dir_arc)),
    )?;

    println!("Pulumi project initialized successfully at {}.", output_dir);
    Ok(())
}

#[tokio::test]
async fn test_new_with_default_output_dir() -> Result<(), anyhow::Error> {
    // Arrange
    let app_name = "default_test_app";
    let current_dir = env::current_dir()?.to_string_lossy().to_string();
    let expected_output_dir = format!("{}/{}", current_dir, app_name);

    // Act
    new(Some(app_name), None, Some(false)).await?;

    // Assert
    assert!(
        std::path::Path::new(&expected_output_dir).exists(),
        "Default output directory was not created."
    );

    // Cleanup
    fs::remove_dir_all(&expected_output_dir)?;

    Ok(())
}
