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
    walk(&args.root_dir, 0, args.depth, args.dirs_only, args.include_hidden);
}

fn walk(node: &std::path::PathBuf, indent_level: usize, max_indent: usize, dirs_only: bool, include_hidden: bool) {
    if indent_level >= max_indent {
        return; // Stop recursion if max_indent is reached
    }

    if let Ok(entries) = std::fs::read_dir(node) {
        for entry_result in entries {
            match entry_result {
                Ok(entry) => {
                    let file_name = entry.file_name();
                    let file_name_str = file_name.to_string_lossy();

                    // Skip hidden files if `include_hidden` is false
                    if !include_hidden && file_name_str.starts_with('.') {
                        continue;
                    }

                    // Determine the file type
                    if let Ok(filetype) = entry.file_type() {
                        // Print the current entry with indentation
                        for _ in 0..indent_level {
                            print!(" ");
                        }

                        if filetype.is_dir() {
                            println!("{:?}", file_name);
                            // Recursively walk into the directory
                            walk(&entry.path(), indent_level + 1, max_indent, dirs_only, include_hidden);
                        } else if !dirs_only {
                            println!("{:?}", file_name);
                        }
                    } else {
                        eprintln!("Could not determine file type for: {:?}", file_name);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to process entry: {}", e);
                }
            }
        }
    } else {
        eprintln!("Failed to read directory: {:?}", node);
    }
}
