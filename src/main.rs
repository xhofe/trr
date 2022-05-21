use clap::Parser;

mod cmd;
fn main() {
    let args = cmd::Args::parse();
    println!("{:?}", args);
    
}
