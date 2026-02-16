mod parser;

use clap::Parser;
use std::fs;
use anyhow::{Context, Result};
use std::path::Path;
use dirs::home_dir;

#[derive(Parser)]
#[clap(
    author = "Edgar Torres <edgar.torres@ki.uni-stuttgart.de>",
    version = "1.0.0",
    about = "X-Gen - Advanced Experiment Structure Generator\n\nA CLI tool to generate consistent, well-structured experiment layouts\nfor researchers, data scientists, and developers.\n\nTemplates are loaded from ~/.x-gen/templates/ (custom) or embedded defaults.\n\nCreator: Edgar Torres <edgar.torres@ki.uni-stuttgart.de>"
)]
struct Args {
    name: String,

    #[clap(long="destination", short='d', default_value = "./")]
    destination: String,

    #[clap(long="template", short='t', default_value = "basic")]
    template: String,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    
    // Ensure user templates directory exists (auto-initialize)
    ensure_user_templates_exist()?;
    
    println!("Creating Experiment {} 🧪 -> {} 📂 using template: {}",
             args.name, args.destination, args.template);

    let root_path = Path::new(&args.destination).join(&args.name);
    fs::create_dir_all(&root_path).context("Failed to create destination directory")?;

    // Load the template structure
    let paths = parser::load_template(&args.template)
        .map_err(|e| anyhow::anyhow!("{}", e))?;

        let paths_count = paths.len();

    // Create all the directories and files from the template
    for path in paths {
        let full_path = root_path.join(&path);

        if path.ends_with('/') {
            // It's a directory
            fs::create_dir_all(&full_path)
                .context(format!("Failed to create directory: {:?}", full_path))?;
        } else {
            // It's a file
            let parent_dir = full_path.parent().unwrap();
            fs::create_dir_all(parent_dir)
                .context(format!("Failed to create parent directory for: {:?}", full_path))?;

            // Special handling for README.md - copy the template
            if path == "README.md" {
                // Try to read the template from user's custom templates first, then fall back to embedded
                let readme_content = load_readme_template(&args.template)
                    .unwrap_or_else(|_| {
                        eprintln!("Warning: README template not found, creating empty file");
                        String::new()
                    });
                
                // Replace placeholder in the template with the actual experiment name
                let content = readme_content.replace("[Experiment Name]", &args.name);
                
                fs::write(&full_path, content)
                    .context(format!("Failed to write to file: {:?}", full_path))?;
            } else {
                // Create other files as empty
                fs::File::create(&full_path)
                    .context(format!("Failed to create file: {:?}", full_path))?;
            }
        }
    }

    println!("✅ Experiment {} created successfully in {}!",
             args.name, root_path.display());
    println!("📁 Created {} items from template '{}'", paths_count, args.template);

    // Read and display the follow-up prompt if it exists
    if let Some(prompt_content) = load_follow_up_prompt(&args.template) {
        println!("\n📋 Follow-up Instructions:\n{}", prompt_content);
    } else {
        eprintln!("Warning: Follow-up prompt not found for template '{}'", args.template);
    }

    Ok(())
}

// Function to ensure user templates directory exists
fn ensure_user_templates_exist() -> Result<()> {
    if let Some(home_dir) = home_dir() {
        let templates_dir = home_dir.join(".x-gen").join("templates");
        fs::create_dir_all(&templates_dir)
            .context("Failed to create user templates directory")?;
    }
    Ok(())
}

// Function to load README template with fallback logic
fn load_readme_template(template_name: &str) -> Result<String> {
    // First, check user's custom templates
    if let Some(home_dir) = home_dir() {
        let user_readme_path = home_dir.join(".x-gen").join("templates").join(template_name).join("README.md");
        if user_readme_path.exists() {
            return fs::read_to_string(&user_readme_path)
                .context("Failed to read user README template");
        }
    }
    
    // Fall back to embedded default template
    match template_name {
        "basic" => {
            Ok(include_str!("../experiments/templates/basic/README.md").to_string())
        }
        _ => {
            Err(anyhow::anyhow!("README template not found for '{}'", template_name))
        }
    }
}

// Function to load follow-up prompt with fallback logic
fn load_follow_up_prompt(template_name: &str) -> Option<String> {
    // First, check user's custom templates
    if let Some(home_dir) = home_dir() {
        let user_prompt_path = home_dir.join(".x-gen").join("templates").join(template_name).join("FOLLOW_UP_PROMPT.md");
        if user_prompt_path.exists() {
            return fs::read_to_string(&user_prompt_path).ok();
        }
    }
    
    // Fall back to embedded default prompt
    match template_name {
        "basic" => {
            Some(include_str!("../experiments/templates/basic/FOLLOW_UP_PROMPT.md").to_string())
        }
        _ => None
    }
}
