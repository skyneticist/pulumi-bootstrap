use anyhow::Error;
use anyhow::Ok;
use serde_yaml;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use crate::tui::app::ProjectConfig;

/// Represents the structure of the configuration, including settings and environment metadata.
#[derive(serde::Serialize)]
struct Config {
    config: BTreeMap<String, String>, // Key-value settings for Azure configurations.
    environment: Vec<String>,         // Metadata related to the environment (e.g., 'meta').
}

pub struct AzureSubscription {
    env: String,
    id: String,
    _name: String,
    tier: String,
    virtual_subnets: Vec<String>,
}

/// Generates a configuration for a specific environment, preserving the order of keys.
///
/// # Arguments
/// - `env`: The name of the environment (e.g., "dev", "test").
/// - `location`: The Azure region (e.g., "eastus2").
/// - `subscription_id`: The Azure subscription ID.
/// - `app_name`: The name of the app to dynamically configure the settings.
///
/// # Returns
/// - A `Config` struct representing the generated configuration with ordered keys.
fn generate_config(infra_config: &ProjectConfig, azure_subscription: &AzureSubscription) -> Config {
    let mut config: BTreeMap<String, String> = BTreeMap::new();

    // Static Azure location
    let location: &str = "eastus2";

    let app_name = &infra_config.resource_name;
    let owner_email = &infra_config.owner_email;
    // Populate config with environment-specific settings
    config.insert(format!("{}:owneremail", app_name), owner_email.to_owned());
    config.insert(
        format!("{}:virtualSubnetworks", app_name),
        azure_subscription.virtual_subnets.join(","),
    );
    config.insert(
        format!("{}:tier", app_name),
        azure_subscription.tier.to_string(),
    );
    config.insert(
        format!("{}:subscriptionId", app_name),
        azure_subscription.id.to_string(),
    );
    config.insert(format!("{}:location", app_name), location.to_string());
    config.insert(
        format!("{}:env", app_name),
        azure_subscription.env.to_string(),
    );
    config.insert(format!("{}:app", app_name), app_name.to_string());
    config.insert(format!("azure-native:location"), location.to_string());

    // Define metadata for the environment
    let environment: Vec<String> = vec!["meta".to_string()];

    // Return populated Config struct instance
    Config {
        config,
        environment,
    }
}

/// Writes a YAML configuration to a file.
///
/// # Arguments
/// - `config`: The `Config` struct to be serialized.
/// - `output_path`: The path where the YAML file should be written.
///
/// # Returns
/// - `Result<(), Box<dyn std::error::Error>>`: Indicates success or failure.
fn write_config_to_file(config: &Config, output_path: &str) -> Result<(), Error> {
    // Serialize the configuration into a YAML string
    let yaml_string = serde_yaml::to_string(config)?;

    // Remove the leading '---' if it exists
    let cleaned_yaml = yaml_string
        .trim_start_matches("---")
        .trim_start()
        .to_string();

    // Ensure the output directory exists
    let output_path = Path::new(output_path);
    if let Some(parent_dir) = output_path.parent() {
        fs::create_dir_all(parent_dir)?;
    }

    fs::write(output_path, cleaned_yaml)?;
    Ok(())
}

pub fn get_subscription_info(environment: &str) -> AzureSubscription {
    // TODO: These values should come from .env
    match environment {
        "dev" => AzureSubscription {
            env: "dev".to_owned(),
            id: "1234-5678-91011".to_owned(),
            _name: "dev01".to_owned(),
            tier: "bronze".to_owned(),
            virtual_subnets: vec!["".to_string(), "".to_string()],
        },
        "test" => AzureSubscription {
            env: "test".to_owned(),
            id: "1234-5678-91011".to_owned(),
            _name: "nonprod01".to_owned(),
            tier: "silver".to_owned(),
            virtual_subnets: vec!["".to_string(), "".to_string()],
        },
        "stage" => AzureSubscription {
            env: "stage".to_owned(),
            id: "1234-5678-91011".to_owned(),
            _name: "preprod01".to_owned(),
            tier: "gold".to_owned(),
            virtual_subnets: vec!["".to_string(), "".to_string()],
        },
        "prod" => AzureSubscription {
            env: "prod".to_owned(),
            id: "1234-5678-91011".to_owned(),
            _name: "prod01".to_owned(),
            tier: "platinum".to_owned(),
            virtual_subnets: vec!["".to_string(), "".to_string()],
        },
        "" => {
            return AzureSubscription {
                env: "default".to_owned(),
                id: "default-id".to_owned(),
                _name: "default-name".to_owned(),
                tier: "default-tier".to_owned(),
                virtual_subnets: vec!["default-subnet".to_owned()],
            }
        }
        _ => panic!("Must pass an environment!"),
    }
}

/// Generates and writes configurations for multiple environments.
///
/// This function creates default configurations for "dev", "test", "stage", and "prod",
/// and writes them to individual YAML files in the `output_dir` directory.
///
/// # Arguments
/// - `app_name`: The name of the app to dynamically configure the settings.
/// - `output_dir`: The directory where configuration files will be written.
///
/// # Returns
/// - `Result<(), Box<dyn std::error::Error>>`: Indicates success or failure.
pub async fn create_config_files(
    config: &ProjectConfig,
    output_dir: Option<&str>,
) -> Result<(), Error> {
    // Define default environments
    let environments = ["dev", "test", "stage", "prod"]; // get from .env file, with the rest of Azure Subscription values

    // Generate and write configuration files for each environment
    for env in environments.iter() {
        let azure_subscription: AzureSubscription = get_subscription_info(env);
        let config = generate_config(config, &azure_subscription);
        let output_path = format!(
            "{}/{}.yaml",
            output_dir.unwrap_or("."), // Default to current directory if not provided
            env
        );
        write_config_to_file(&config, &output_path)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    /// Helper function to clean up test files after testing.
    fn cleanup_test_files(output_dir: &str) {
        if Path::new(output_dir).exists() {
            fs::remove_dir_all(output_dir).unwrap();
        }
    }

    #[test]
    fn test_generate_config() {
        let azure_subscription = AzureSubscription {
            env: "dev".to_string(),
            id: "1234-5678-91011".to_string(),
            _name: "dev01".to_string(),
            tier: "bronze".to_string(),
            virtual_subnets: vec!["subnet1".to_string(), "subnet2".to_string()],
        };

        let config = ProjectConfig {
            resource_name: "test_app".to_string(),
            environments: vec!["dev".to_string()],
            additional_resources: vec![],
            owner_email: "owner@example.com".to_string(),
        };

        let config = generate_config(&config, &azure_subscription);

        assert_eq!(
            config.config.get("test_app:virtualSubnetworks").unwrap(),
            "subnet1,subnet2"
        );
        assert_eq!(config.config.get("test_app:tier").unwrap(), "bronze");
        assert_eq!(
            config.config.get("test_app:subscriptionId").unwrap(),
            "1234-5678-91011"
        );
        assert_eq!(config.config.get("test_app:location").unwrap(), "eastus2");
        assert_eq!(config.config.get("test_app:env").unwrap(), "dev");
        assert_eq!(config.environment, vec!["meta".to_string()]);
    }

    #[test]
    fn test_write_config_to_file() {
        let config = Config {
            config: BTreeMap::from([
                ("key1".to_string(), "value1".to_string()),
                ("key2".to_string(), "value2".to_string()),
            ]),
            environment: vec!["meta".to_string()],
        };

        let output_path = "test_output/test_config.yaml";
        let output_dir = "test_output";

        // Ensure clean test setup
        cleanup_test_files(output_dir);

        // Write config to file and validate it exists
        write_config_to_file(&config, output_path).unwrap();
        assert!(Path::new(output_path).exists());

        // Validate file contents
        let written_content = fs::read_to_string(output_path).unwrap();
        assert!(written_content.contains("key1: value1"));
        assert!(written_content.contains("key2: value2"));

        // Clean up
        cleanup_test_files(output_dir);
    }

    #[test]
    fn test_get_subscription_info() {
        let dev_subscription = get_subscription_info("dev");
        assert_eq!(dev_subscription.env, "dev");
        assert_eq!(dev_subscription.id, "1234-5678-91011");
        assert_eq!(dev_subscription.tier, "bronze");

        let prod_subscription = get_subscription_info("prod");
        assert_eq!(prod_subscription.env, "prod");
        assert_eq!(prod_subscription.id, "1234-5678-91011");
        assert_eq!(prod_subscription.tier, "platinum");
    }

    #[tokio::test]
    async fn test_create_config_files() {
        let config = ProjectConfig {
            resource_name: "test_app".to_string(),
            environments: vec![
                "dev".to_string(),
                "test".to_string(),
                "stage".to_string(),
                "prod".to_string(),
            ],
            additional_resources: vec![],
            owner_email: "owner@example.com".to_string(),
        };
        let output_dir = "test_configs";

        // Ensure clean test setup
        cleanup_test_files(output_dir);

        // Generate configuration files
        create_config_files(&config, Some(output_dir))
            .await
            .unwrap();

        // Validate generated files
        let expected_files = ["dev.yaml", "test.yaml", "stage.yaml", "prod.yaml"];
        for file in expected_files.iter() {
            let path = format!("{}/{}", output_dir, file);
            assert!(Path::new(&path).exists());

            // Check that the file contains expected data
            let content = fs::read_to_string(path).unwrap();
            assert!(content.contains("test_app:env"));
            assert!(content.contains("test_app:location"));
        }

        // Clean up
        cleanup_test_files(output_dir);
    }

    #[test]
    #[should_panic(expected = "Must pass an environment!")]
    fn test_get_subscription_info_invalid_env() {
        get_subscription_info("invalid_env");
    }
}
