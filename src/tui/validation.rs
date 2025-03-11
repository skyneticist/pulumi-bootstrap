use super::app::App;

pub fn validate_inputs(app: &App) -> Result<(), String> {
    if app.config.resource_name.is_empty() {
        return Err("Resource name cannot be empty.".to_string());
    }
    if app.config.owner_email.is_empty() {
        return Err("Owner email cannot be empty.".to_string());
    }
    if app.config.environments.len() < 1 {
        return Err("At least one environment must be selected".to_string());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_inputs() {
        let mut app = App::new();
        app.config.resource_name = "test".to_string();
        app.config.owner_email = "".to_owned();
        app.config.environments = vec!["dev".to_string()];
        assert_eq!(
            validate_inputs(&app),
            Err("Owner email cannot be empty.".to_string())
        );

        
    }
}
