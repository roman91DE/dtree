mod dtree;

fn main() {
    let args = dtree::Args::new("/Users/roman/Repos/dtree", 3, false, true).expect("Couldn't read arguments");
    dtree::run(&args);
}
