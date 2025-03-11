// // #[derive(Subcommand)]
// // pub enum Commands {
// //     /// Create a new project
// //     New {
// //         // <project_name> is required
// //         project_name: String,
// //         // --interactive is optional
// //         #[arg(long, help = "Run interactively", default_value_t = false)]
// //         interactive: bool,
// //         // --resource is optional, but can be provided
// //         #[arg(long, help = "Type of the resource")]
// //         resource: Option<String>,
// //         // --name is optional, used when resource is specified
// //         #[arg(long, help = "Name of the resource")]
// //         name: Option<String>,
// //     },
// //     /// Snippet management
// //     #[command(aliases = ["snippets"])] // Alias 'snippets' for 'snippet'
// //     Snippet {
// //         resource: Option<String>,
// //         #[arg(long, help = "Name of the snippet")]
// //         name: Option<String>,
// //         #[arg(long, help = "File to associate with the snippet")]
// //         file: Option<String>,
// //         #[command(subcommand)]
// //         subcommand: SnippetSubcommand,
// //     },
// //     /// Open configuration editor
// //     Config {
// //         #[arg(long, help = "Resource to configure")]
// //         resource: Option<String>,
// //     },
// // }
// // #[derive(Subcommand)]
// // pub enum SnippetSubcommand {
// //     /// List all snippets
// //     #[command(aliases = ["ls"])] // Alias 'ls' for 'list'
// //     List,
// // }
// // pub async fn run(command: Commands) {
// //     match command {
// //         Commands::New {
// //             project_name,
// //             interactive,
// //             resource,
// //             name: _,
// //         } => {
// //             println!("Creating project: {}", project_name);
// //             if interactive {
// //                 println!("Running interactively...");
// //            }
// //             // Call the project creation logic here, based on flags
// //             commands::project::new(
// //                 Some(&project_name), // Project name
// //                 None,                // Path, can be added if needed
// //                 resource.as_deref(), // Resource type (optional)
// //                 Some(interactive),   // Whether to run interactively
// //             )
// //             .await
// //             .unwrap(); // TODO: no unwrap (error handling here)
// //         }
// //         Commands::Snippet {
// //             subcommand,
// //             resource,
// //             name,
// //             file,
// //         } => match subcommand {
// //             SnippetSubcommand::List => {
// //                 commands::snippet::list().await.unwrap(); // TODO: no unwrap
// //             }
// //             _ => {
// //                 commands::snippet::new(name.as_deref(), file.as_deref(), resource.as_deref(), None)
// //                     .await
// //                     .unwrap(); // TODO: no unwrap
// //             }
// //         },
// //         Commands::Config { resource } => {
// //             commands::config::open_editor(resource.as_deref())
// //                 .await
// //                 .unwrap(); // TODO: no unwrap
// //         }
// //     }
// // }

// #[derive(Subcommand)]
// pub enum Commands {
//     /// Create a new project
//     New {
//         project_name: String,
//         #[arg(long, help = "Run interactively", default_value_t = false)]
//         interactive: bool,
//         #[arg(long, help = "Type of the resource")]
//         resource: Option<String>,
//         #[arg(long, help = "Name of the resource")]
//         name: Option<String>,
//     },
//     /// Snippet management (also available as 'snippets')
//     #[command(aliases = ["snippets"])] // Alias 'snippets' for 'snippet'
//     Snippet {
//         #[command(subcommand)]
//         subcommand: SnippetSubcommand,
//         #[arg(required = false, help = "Resource type for the snippet")]
//         resource: Option<String>,
//         // Optional flags
//         #[arg(long, help = "Name of the snippet")]
//         name: Option<String>,
//         #[arg(long, help = "Path for the output")]
//         path: Option<String>,
//     },
//     /// Open configuration editor
//     Config {
//         #[arg(long, help = "Resource to configure")]
//         resource: Option<String>,
//     },
// }
// #[derive(Subcommand)]
// pub enum SnippetSubcommand {
//     /// List all snippets (with alias 'ls')
//     #[command(aliases = ["ls", "l"])]
//     List,
//     #[command(aliases = ["n"])]
//     New {
//         #[arg(long, help = "Type of the resource")]
//         resource: Option<String>,
//         #[arg(long, help = "Name of the snippet")]
//         name: Option<String>,
//     },
// }
// pub async fn run(command: Commands) {
//     match command {
//         Commands::New {
//             project_name,
//             interactive,
//             resource,
//             name: _,
//         } => {
//             println!("Creating project: {}", project_name);
//             if interactive {
//                 println!("Running interactively...");
//             }
//             // Call the project creation logic here, based on flags
//             commands::project::new(
//                 Some(&project_name),
//                 None,
//                 resource.as_deref(),
//                 Some(interactive),
//             )
//             .await
//             .unwrap(); // TODO: no unwrap (error handling here)
//         }
//         Commands::Snippet {
//             subcommand,
//             resource,
//             name,
//             path,
//         } => {
//             match subcommand {
//                 SnippetSubcommand::List => {
//                     // Listing all available snippets
//                     println!("Listing all available snippets...");
//                     commands::snippet::list().await.unwrap(); // TODO: no unwrap
//                 }
//                 SnippetSubcommand::New {
//                     resource: new_resource,
//                     name: new_name,
//                 } => {
//                     if let Some(resource_type) = new_resource {
//                         // Handle snippet for specific resource
//                         println!("Creating snippet for resource: {}", resource_type);
//                         commands::snippet::new(
//                             new_name.as_deref(),
//                             path.as_deref(),
//                             Some(&resource_type),
//                             None,
//                         )
//                         .await
//                         .unwrap(); // TODO: no unwrap
//                     } else {
//                         println!("Error: Resource must be provided for creating a snippet.");
//                     }
//                 }
//             }
//         }
//         Commands::Config { resource } => {
//             commands::config::open_editor(resource.as_deref())
//                 .await
//                 .unwrap(); // TODO: no unwrap
//         }
//     }
// }

use clap::{Parser, Subcommand};

use crate::commands;

#[derive(Parser)]
#[command(name = "pulumimi")]
#[command(about = "A command-line tool for bootstrapping Pulumi projects")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

impl Cli {
    pub fn new() -> Self {
        Self::parse()
    }

    pub async fn run(self) {
        run(self.command).await;
    }
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start interactive mode
    Interactive {
        #[arg(long, help = "Suppress file output")]
        suppress_file: bool,
    },

    /// Create a new project
    New {
        project_name: String,
        #[arg(long, help = "Type of the resource")]
        resource: Option<String>,
    },

    /// Snippet management (also available as 'snippets')
    #[command(aliases = ["snippets"])] // Alias 'snippets' for 'snippet'
    Snippet {
        #[command(subcommand)]
        subcommand: SnippetSubcommand,
    },

    /// Open configuration editor
    Config {
        #[arg(long, help = "Resource to configure")]
        resource: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum SnippetSubcommand {
    /// List all snippets (with alias 'ls')
    #[command(aliases = ["ls", "l"])]
    List,

    /// Create a new snippet
    #[command(aliases = ["n"])]
    New {
        #[arg(long, help = "Type of the resource")]
        resource: String, // Resource is required for the 'new' subcommand
        #[arg(long, help = "Name of the snippet")]
        name: Option<String>,
    },
}

/// Run the appropriate command based on the CLI input
pub async fn run(command: Commands) {
    match command {
        // Handle the 'Interactive' subcommand
        Commands::Interactive { suppress_file } => {
            // TODO: Implement file output suppression
            if suppress_file {
                println!("Suppressing file output...");
            }
            commands::project::new(None, None, Some(true))
                .await
                .unwrap(); // TODO: no unwrap
        }
        // Handle the 'New' subcommand
        Commands::New {
            project_name,
            resource,
        } => {
            match resource {
                Some(resource_type) => {
                    commands::snippet::new(
                        Some(project_name.as_str()),
                        None, // Path is optional
                        Some(resource_type.as_str()),
                    )
                    .await
                    .unwrap(); // TODO: no unwrap
                }
                _ => {
                    commands::project::new(Some(&project_name), None, None)
                        .await
                        .unwrap(); // TODO: no unwrap (error handling here)
                }
            }
        }
        // Handle the 'Snippet' subcommand
        Commands::Snippet { subcommand } => {
            match subcommand {
                SnippetSubcommand::List => {
                    // println!("Listing all available snippets...");
                    commands::snippet::list().await.unwrap(); // TODO: no unwrap
                }
                SnippetSubcommand::New {
                    resource,
                    name: new_name,
                } => {
                    if !resource.is_empty() {
                        // Handle snippet for specific resource
                        println!("Creating snippet for resource: {}", resource);

                        commands::snippet::new(
                            new_name.as_deref(),
                            None, // Path is optional
                            Some(&resource),
                        )
                        .await
                        .unwrap(); // TODO: no unwrap
                    } else {
                        println!("Error: Resource must be provided for creating a snippet.");
                    }
                }
            }
        }
        // Handle the 'Config' subcommand
        Commands::Config { resource } => {
            commands::config::open_editor(resource.as_deref())
                .await
                .unwrap(); // TODO: no unwrap
        }
    }
}
