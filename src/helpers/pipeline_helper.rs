use anyhow::Ok;
use std::{env, fs};

use crate::tui::app::ProjectConfig;

/// Default embedded `azure-pipelines.yaml` template
const PIPELINE_TEMPLATE: &str = include_str!("../../snippets/azure-pipelines.yaml");

/// Creates a pipeline YAML file for the given app name, saving it in the specified or default directory.
///
/// # Arguments
/// - `app_name`: The name of the application for which the pipeline YAML is being generated.
/// - `output_dir`: An optional directory where the YAML file will be saved. If not provided, it defaults to a subdirectory named after the app.
///
/// # Returns
/// - `Result<(), anyhow::Error>`: Returns a result indicating success or failure.
///
/// # Example
/// ```
/// let result = create_pipeline_yaml("myapp", None).await;
/// match result {
///     Ok(_) => println!("Pipeline YAML created."),
///     Err(e) => eprintln!("Error: {}", e),
/// }
/// ```
pub async fn create_pipeline_yaml(
    config: &ProjectConfig,
    output_dir: Option<&str>,
) -> Result<(), anyhow::Error> {
    println!("Creating pipeline YAML for {}", &config.resource_name);

    // Default the output directory to a subdirectory named after the app if not provided
    let output_dir: String = match output_dir {
        Some(dir) => dir.to_string(),
        None => {
            // If no output directory is provided, use the current directory and append the app name
            let current_dir = env::current_dir()?.to_string_lossy().to_string();
            format!("{}/{}", current_dir, &config.resource_name)
        }
    };

    // Ensure the output directory exists, creating it if necessary
    fs::create_dir_all(&output_dir)?;

    // Replace placeholders for `app_name` in the pipeline template (PIPELINE_TEMPLATE must be predefined)
    let modified_pipeline: String =
        PIPELINE_TEMPLATE.replace("{{project_name}}", &config.resource_name);

    // Construct the full file path for the pipeline YAML file
    let pipeline_path = format!("{}/azure-pipelines.yaml", output_dir);

    // Write the modified pipeline YAML content to the file
    fs::write(&pipeline_path, &modified_pipeline)
        .map_err(|err| anyhow::anyhow!("Failed to write to {}: {}", pipeline_path, err))?;

    // Output confirmation message with the path where the file was created
    println!("Pipeline YAML created successfully at '{}'.", pipeline_path);

    // Return a successful result
    Ok(())
}

#[tokio::test]
async fn test_create_pipeline_yaml() -> Result<(), anyhow::Error> {
    use tempfile::tempdir;

    // Arrange
    let config = ProjectConfig {
        resource_name: "myapp".to_string(),
        environments: vec!["dev".to_string(), "prod".to_string()],
        additional_resources: vec!["database".to_string(), "keyvault".to_string()],
        owner_email: "".to_string(),
    };
    let temp_dir = tempdir()?; // Create a temporary directory
    let output_dir = temp_dir.path().to_str().unwrap(); // Convert path to a string
    let expected_file_path = format!("{}/azure-pipelines.yaml", output_dir);
    let expected_content = PIPELINE_TEMPLATE.replace("{{project_name}}", &config.resource_name);

    // Act
    create_pipeline_yaml(&config, Some(output_dir)).await?;

    // Assert
    // Check if the file exists
    assert!(
        fs::metadata(&expected_file_path).is_ok(),
        "Pipeline file was not created."
    );

    // Verify file contents
    let actual_content = fs::read_to_string(&expected_file_path)?;
    assert_eq!(
        actual_content, expected_content,
        "Pipeline file content mismatch."
    );

    // Cleanup: Temp directory is automatically cleaned up
    Ok(())
}
