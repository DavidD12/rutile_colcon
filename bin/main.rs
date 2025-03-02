use clap::Parser;
use log::{error, info};
use rutile_colcon::generate;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// package name
    #[arg(short, long)]
    package_name: String,
    /// destination directory
    #[arg(short, long)]
    directory: String,
    /// verbose level
    #[arg(short, long, default_value_t = 1)]
    verbose: u8,
}

fn main() {
    let args = Args::parse();
    if args.verbose > 0 {
        //
        if std::env::var("RUST_LOG").is_err() {
            unsafe { std::env::set_var("RUST_LOG", "info") }
        }
        env_logger::init();
    }
    //------------------------- Generate -------------------------
    let folder = format!("{}/{}/", args.directory, args.package_name);
    if let Err(e) = generate(&args.package_name, &folder) {
        error!("{}", e);
        std::process::exit(1);
    }
    info!("rutile package generated ({})", args.directory);
}
