use anyhow::Ok;
use std::path::Path;
use std::{env, fs};

use crate::tui::app::ProjectConfig;

/// Default embedded Pulumi TypeScript template
const TYPESCRIPT_TEMPLATE: &str = include_str!("../../snippets/pulumi.webstackvzn.ts");
const DATABASE_CODE_TEMPLATE: &str = include_str!("../../snippets/database.ts");
const STORAGE_CODE_TEMPLATE: &str = include_str!("../../snippets/storage.ts");
const SERVICEBUS_CODE_TEMPLATE: &str = include_str!("../../snippets/service_bus.ts");
const CACHE_CODE_TEMPLATE: &str = include_str!("../../snippets/cache.ts");
const KEYVAULT_CODE_TEMPLATE: &str = include_str!("../../snippets/keyvault.ts");
const REGISTRY_CODE_TEMPLATE: &str = include_str!("../../snippets/container-registry.ts");

pub async fn create_entry_point(
    project_config: &ProjectConfig,
    output_dir: Option<&str>,
) -> Result<(), anyhow::Error> {
    println!("Starting to create entry point...");

    // Default the output directory to a subdirectory named after the app
    let output_dir: String = match output_dir {
        Some(dir) => dir.to_string(),
        _ => {
            let current_dir = env::current_dir()?.to_string_lossy().to_string();
            format!("{}/{}", current_dir, &project_config.resource_name)
        }
    };

    // Ensure the output directory exists
    if !Path::new(&output_dir).exists() {
        println!("Creating directory: {}", &output_dir);
        fs::create_dir_all(&output_dir)?;
    }

    // Replace placeholders in the template
    let mut modified_index =
        TYPESCRIPT_TEMPLATE.replace("{{project_name}}", &project_config.resource_name);

    let final_index_file =
        handle_additional_resources(&mut modified_index, &project_config.additional_resources)?;

    // Define the output file path
    let index_out_path = format!("{}/index.ts", &output_dir);

    // Write the modified template to the output file
    println!("Writing to: {}", &index_out_path);
    fs::write(&index_out_path, &final_index_file)?;

    println!(
        "index.ts created successfully with {} characters!",
        &modified_index.len()
    );
    Ok(())
}

fn handle_additional_resources(
    modified_index: &mut String,
    additional_resources: &[String],
) -> Result<String, anyhow::Error> {
    additional_resources
        .iter()
        .for_each(|res: &String| match res.to_lowercase().as_str() {
            "container registry" => modified_index.push_str(&format!("{}", REGISTRY_CODE_TEMPLATE)),
            "database" => modified_index.push_str(&format!("{}", DATABASE_CODE_TEMPLATE)),
            "cache" => modified_index.push_str(&format!("{}", CACHE_CODE_TEMPLATE)),
            "storage" => modified_index.push_str(&format!("{}", STORAGE_CODE_TEMPLATE)),
            "service bus" => modified_index.push_str(&format!("{}", SERVICEBUS_CODE_TEMPLATE)),
            "keyvault" => modified_index.push_str(&format!("{}", KEYVAULT_CODE_TEMPLATE)),
            _ => (),
        });
    Ok(modified_index.to_string())
}

pub fn generate_resource_snippet(
    resource: &str,
    name: Option<&str>,
) -> Result<String, anyhow::Error> {
    let raw_snippet = match resource.to_lowercase().as_str() {
        "containerregistry" => Ok(REGISTRY_CODE_TEMPLATE.to_string()),
        "database" => Ok(DATABASE_CODE_TEMPLATE.to_string()),
        "cache" => Ok(CACHE_CODE_TEMPLATE.to_string()),
        "storage" => Ok(STORAGE_CODE_TEMPLATE.to_string()),
        "servicebus" => Ok(SERVICEBUS_CODE_TEMPLATE.to_string()),
        "keyvault" => Ok(KEYVAULT_CODE_TEMPLATE.to_string()),
        _ => Ok(TYPESCRIPT_TEMPLATE.to_string()),
    };
    match name {
        Some(name) => Ok(raw_snippet?.replace("{{project_name}}", name)),
        _ => Ok(raw_snippet?.replace("{{project_name}}", "myapp")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_create_entry_point_with_default_output_dir() -> Result<(), anyhow::Error> {
        // Arrange
        let config = ProjectConfig {
            resource_name: "test_app".to_string(),
            environments: vec!["dev".to_string(), "prod".to_string()],
            additional_resources: vec!["database".to_string(), "keyvault".to_string()],
            owner_email: "".to_string(),
        };
        let temp_dir = tempdir()?; // Create a temporary directory
        let output_dir = temp_dir.path().to_str().unwrap(); // Convert path to a string
        let expected_file_path = format!("{}/index.ts", output_dir);
        let expected_content =
            TYPESCRIPT_TEMPLATE.replace("{{project_name}}", &config.resource_name);

        // Act
        create_entry_point(&config, Some(output_dir)).await?;

        // Assert
        // Check if the directory was created
        assert!(
            Path::new(output_dir).exists(),
            "Output directory was not created."
        );

        // Check if the file was created
        assert!(
            fs::metadata(&expected_file_path).is_ok(),
            "index.ts file was not created."
        );

        // Verify file contents
        let actual_content = fs::read_to_string(&expected_file_path)?;
        assert_eq!(
            actual_content, expected_content,
            "index.ts file content does not match the expected content."
        );

        // Cleanup: Temp directory is automatically cleaned up by `tempdir`
        Ok(())
    }

    #[tokio::test]
    async fn test_create_entry_point_with_custom_output_dir() -> Result<(), anyhow::Error> {
        // Arrange
        let config = ProjectConfig {
            resource_name: "test_app".to_string(),
            environments: vec!["dev".to_string(), "prod".to_string()],
            additional_resources: vec!["database".to_string(), "keyvault".to_string()],
            owner_email: "".to_string(),
        };
        let temp_dir = tempdir()?; // Create a temporary directory
        let output_dir = temp_dir.path().to_str().unwrap(); // Convert path to a string
        let expected_file_path = format!("{}/index.ts", output_dir);
        let expected_content =
            TYPESCRIPT_TEMPLATE.replace("{{project_name}}", &config.resource_name);

        // Act
        create_entry_point(&config, Some(output_dir)).await?;

        // Assert
        // Check if the directory was created
        assert!(
            Path::new(output_dir).exists(),
            "Custom output directory was not created."
        );

        // Check if the file was created
        assert!(
            fs::metadata(&expected_file_path).is_ok(),
            "index.ts file was not created in the custom directory."
        );

        // Verify file contents
        let actual_content = fs::read_to_string(&expected_file_path)?;
        assert_eq!(
            actual_content, expected_content,
            "index.ts file content does not match the expected content in the custom directory."
        );

        // Cleanup: Temp directory is automatically cleaned up by `tempdir`
        Ok(())
    }

    #[tokio::test]
    async fn test_handle_additional_resources() -> Result<(), anyhow::Error> {
        // Arrange
        let mut modified_index = TYPESCRIPT_TEMPLATE.to_string();
        let additional_resources = vec![
            "container registry".to_string(),
            "database".to_string(),
            "cache".to_string(),
            "storage".to_string(),
            "service bus".to_string(),
            "keyvault".to_string(),
        ];

        // Act
        let final_index = handle_additional_resources(&mut modified_index, &additional_resources)?;

        // Assert
        assert!(final_index.contains(REGISTRY_CODE_TEMPLATE));
        assert!(final_index.contains(DATABASE_CODE_TEMPLATE));
        assert!(final_index.contains(CACHE_CODE_TEMPLATE));
        assert!(final_index.contains(STORAGE_CODE_TEMPLATE));
        assert!(final_index.contains(SERVICEBUS_CODE_TEMPLATE));
        assert!(final_index.contains(KEYVAULT_CODE_TEMPLATE));

        Ok(())
    }
}
