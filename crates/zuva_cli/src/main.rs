use clap::Parser;

#[derive(Parser)]
#[command(name="zuva", version, about="Zuva language toolchain")]
struct Cli {}

fn main(){ let _=Cli::parse(); println!("Zuva CLI scaffold"); }
