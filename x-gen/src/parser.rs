use std::fs;
// use std::path::Path;

// Simple function to extract depth and path from a tree line
fn extract_depth_and_path(line: &str) -> (usize, String) {
    // Skip empty lines
    if line.trim().is_empty() {
        return (0, String::new());
    }
    
    // Remove tree characters and count indentation
    let mut clean_line = line.replace("├── ", "");
    clean_line = clean_line.replace("└── ", "");
    clean_line = clean_line.replace("│   ", "    "); // Replace vertical bar with 4 spaces
    
    // Count leading spaces to determine depth (every 4 spaces = 1 level)
    let trimmed = clean_line.trim_start();
    let depth = (clean_line.len() - trimmed.len()) / 4;
    
    // Extract path part (before any comment), preserving trailing slash for directories
    let path_part_raw = trimmed.split('#').next().unwrap_or("").trim();
    let path_part = path_part_raw.to_string();
    
    (depth, path_part)
}

// Parse the tree structure
pub fn parse_tree_structure(tree_str: &str) -> Vec<String> {
    let mut paths = Vec::new();
    let mut stack: Vec<(usize, String)> = Vec::new(); // (depth, path)
    let mut root_dir_name = String::new(); // Track the root directory name to exclude it
    let mut first_item = true; // Flag to identify the root directory

    for line in tree_str.lines() {
        let (depth, path_part) = extract_depth_and_path(line);
        
        if path_part.is_empty() {
            continue;
        }
        
        // Pop items from stack that are at the same or deeper level
        while let Some(&(stack_depth, _)) = stack.last() {
            if stack_depth >= depth {
                stack.pop();
            } else {
                break;
            }
        }
        
        // Calculate full path
        let full_path = if let Some((_, parent_path)) = stack.last() {
            if parent_path.ends_with('/') {
                format!("{}{}", parent_path, path_part)
            } else {
                format!("{}/{}", parent_path, path_part)
            }
        } else {
            path_part.clone()
        };
        
        // Track the root directory name (first item at depth 0 that ends with '/')
        if first_item && depth == 0 && path_part.ends_with('/') {
            root_dir_name = path_part.clone();
            first_item = false;
        }
        
        // Only add non-empty paths
        if !full_path.is_empty() {
            // Check if it's a directory (has trailing slash) but don't remove it yet for comparison
            let is_directory = full_path.ends_with('/') && full_path != "/";
            let clean_path_for_comparison = if is_directory {
                full_path.trim_end_matches('/').to_string()
            } else {
                full_path.clone()
            };
            
            // Skip the root directory name itself
            if clean_path_for_comparison != root_dir_name {
                // Push the original path (with slash if it's a directory)
                paths.push(full_path.clone());
            }
        }
        
        stack.push((depth, full_path));
    }
    
    paths
}

use dirs::home_dir;

// Function to load and parse a template file
pub fn load_template(template_name: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // First, check if user has custom template in ~/.x-gen/templates/
    if let Some(home_dir) = home_dir() {
        let user_template_path = home_dir.join(".x-gen").join("templates").join(template_name).join(format!("{}.toml", template_name));
        
        if user_template_path.exists() {
            // Load from user's custom templates
            let content = fs::read_to_string(&user_template_path)?;
            return parse_tree_structure_from_content(&content);
        }
    }
    
    // Fall back to embedded default templates
    match template_name {
        "basic" => {
            let content = include_str!("../experiments/templates/basic/basic.toml");
            // Extract just the structure part from the embedded content
            let structure_content = extract_structure_from_content(content)?;
            Ok(parse_tree_structure(&structure_content))
        }
        _ => {
            Err(format!("Template '{}' not found. Either use a built-in template or create a custom one in ~/.x-gen/templates/", template_name).into())
        }
    }
}

// Helper function to parse tree structure from content (refactored from original load_template)
fn parse_tree_structure_from_content(content: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let structure_content = extract_structure_from_content(content)?;
    // Parse the tree structure
    Ok(parse_tree_structure(&structure_content))
}

// Helper function to extract structure content from TOML content
fn extract_structure_from_content(content: &str) -> Result<String, Box<dyn std::error::Error>> {
    let lines: Vec<&str> = content.lines().collect();
    let mut in_structure = false;
    let mut structure_content = String::new();

    for line in lines {
        if line.trim().starts_with("structure = \"\"\"") {
            in_structure = true;
            // Skip the line with structure = """ and continue to the next line
            continue;
        } else if in_structure && line.trim() == "\"\"\"" {
            // End of structure - break out of the loop
            break;
        } else if in_structure {
            structure_content.push_str(line);
            structure_content.push('\n');
        }
    }

    if structure_content.is_empty() {
        return Err("Structure not found in template file".into());
    }

    // Remove the last newline if present
    if structure_content.ends_with('\n') {
        structure_content.pop();
    }

    Ok(structure_content)
}