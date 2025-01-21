pub struct Args {
    root_dir: std::path::PathBuf,
    depth: usize,
    dirs_only: bool,
    include_hidden: bool,
}

impl Args {
    pub fn new(cwd: &str, depth: usize, dirs_only: bool, include_hidden: bool) -> std::io::Result<Args> {
        let root_dir = std::path::PathBuf::from(cwd);

        if !root_dir.exists() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Directory does not exist: {}", cwd),
            ));
        }

        Ok(Args {
            root_dir,
            depth: depth.max(1), // Ensure depth is at least 1
            dirs_only,
            include_hidden,
        })
    }
}

pub fn run(args: &Args) {
    println!(".");

    let mut dir_count = 1;
    let mut file_count = 0;

    walk(
        &args.root_dir,
        0,
        args.depth,
        args.dirs_only,
        args.include_hidden,
        &mut dir_count,
        &mut file_count,
    );

    println!("\n{} directories, {} files", dir_count, file_count);
}

fn walk(
    node: &std::path::PathBuf,
    indent_level: usize,
    max_indent: usize,
    dirs_only: bool,
    include_hidden: bool,
    dir_count: &mut usize,
    file_count: &mut usize,
) {
    if indent_level >= max_indent {
        return; // Stop recursion if max_indent is reached
    }

    if let Ok(entries) = std::fs::read_dir(node) {
        let mut entries: Vec<_> = entries.filter_map(|e| e.ok()).collect();
        entries.sort_by_key(|entry| entry.file_name()); // Sort entries alphabetically

        let total_entries = entries.len();
        for (i, entry) in entries.into_iter().enumerate() {
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy();

            // Skip hidden files if `include_hidden` is false
            if !include_hidden && file_name_str.starts_with('.') {
                continue;
            }

            // Determine the file type
            if let Ok(filetype) = entry.file_type() {
                // Build the indentation string
                let mut indent_str = String::new();
                for _ in 0..indent_level {
                    indent_str.push_str("│   ");
                }

                // Choose the appropriate branch character
                let branch = if i == total_entries - 1 { "└── " } else { "├── " };

                if filetype.is_dir() {
                    *dir_count += 1; // Increment directory count
                    let dir_str = file_name.to_str().unwrap();
                    println!("{}{}{}", indent_str, branch, dir_str);

                    // Recursively walk the directory
                    walk(
                        &entry.path(),
                        indent_level + 1,
                        max_indent,
                        dirs_only,
                        include_hidden,
                        dir_count,
                        file_count,
                    );
                } else if !dirs_only {
                    *file_count += 1; // Increment file count
                    let file_str = file_name.to_str().unwrap();
                    println!("{}{}{}", indent_str, branch, file_str);
                }
            } else {
                eprintln!("Could not determine file type for: {:?}", file_name);
            }
        }
    } else {
        eprintln!("Failed to read directory: {:?}", node);
    }
}
