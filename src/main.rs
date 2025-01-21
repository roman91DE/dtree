use clap::{Arg, ArgAction, Command};
mod dtree;

fn main() {
    let matches = Command::new("dtree")
        .version("1.0")
        .author("Roman Höhn <rohoehn123@gmail.com>")
        .about("A tree-like directory listing program")
        .arg(
            Arg::new("directory")
                .help("The directory to list")
                .default_value(".")
                .index(1),
        )
        .arg(
            Arg::new("level")
                .short('L')
                .long("level")
                .help("Limit the depth of the directory tree")
                .default_value("2")
                .value_parser(clap::value_parser!(usize)),
        )
        .arg(
            Arg::new("show_hidden")
                .short('a')
                .long("all")
                .help("Show hidden files")
                .action(ArgAction::SetTrue),
        )
        // .arg(
        //     Arg::new("full_path")
        //         .short('f')
        //         .long("full-path")
        //         .help("Print the full path prefix for each file")
        //         .action(ArgAction::SetTrue),
        // )
        .get_matches();

    // Parsing values
    let directory = matches.get_one::<String>("directory").unwrap();
    let level = *matches.get_one::<usize>("level").unwrap_or(&2);
    let show_hidden = matches.get_flag("show_hidden");
    let full_path = matches.get_flag("full_path");

    // Initialize dtree::Args
    let args = dtree::Args::new(directory, level, show_hidden, full_path)
        .expect("Couldn't parse arguments");

    // Run the program
    dtree::run(&args);
}
