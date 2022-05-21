use clap::Parser;

mod cmd;
mod tree;
fn main() {
    let args = cmd::Args::parse();
    let mut tree = tree::Tree::new(args);
    tree.run();
}
