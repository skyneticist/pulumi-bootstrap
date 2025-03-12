use std::fs;

use anyhow::Ok;
use colored::*;

use crate::helpers::entrypoint_helper::generate_resource_snippet;

/// Creates a new resource snippet.
pub async fn new(
    name: Option<&str>,
    output_path: Option<&str>,
    resource: Option<&str>,
) -> Result<(), anyhow::Error> {
    // Create directory
    match output_path {
        Some(path) => fs::create_dir_all(path)?,
        _ => fs::create_dir_all("_snippet")?,
    }

    let snippet = generate_resource_snippet(resource.unwrap(), name)?;
    println!("{}", snippet);
    fs::write(
        format!("{}/{}.ts", output_path.unwrap(), name.unwrap()),
        snippet,
    )?;
    Ok(())
}

/// Lists the available resource snippets.
pub async fn list() -> Result<(), anyhow::Error> {
    println!(
        "{}",
        "+-----------------+\n\
         | Resource       |\n\
         +-----------------+\n\
         | Database       |\n\
         | Keyvault       |\n\
         | Storage        |\n\
         | ServiceBus     |\n\
         | CosmosDB       |\n\
         | AppService     |\n\
         | FunctionApp    |\n\
         +-----------------+"
            .cyan()
    );
    println!(
        "{}",
        "Usage: `pulumimi snippet --resource / -r Database`\n".yellow()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_with_all_args() -> Result<(), anyhow::Error> {
        // Arrange
        let name = Some("test_name");
        let file = Some("test_file");
        let resource = Some("test_resource");

        // Act
        let result = new(name, file, resource).await;

        // Assert
        assert!(result.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_new_with_default_args() -> Result<(), anyhow::Error> {
        // Act
        let result = new(None, None, None).await;

        // Assert
        assert!(result.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_list() -> Result<(), anyhow::Error> {
        // Act
        let result = list().await;

        // Assert
        assert!(result.is_ok());
        Ok(())
    }
}
