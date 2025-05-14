use clap::Command;

// Import your version module
use rustcroissant::version;

fn main() {
    // Setup command line argument parsing
    let app = Command::new("rustcroissant")
        .about("Tools for generating and validating Croissant metadata")
        .version(version::get_version().version)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("version")
                .about("Print the version information")
                .long_about("Print the version, git hash, and build time information of the rustcroissant tool")
        );

    // Parse arguments and handle commands
    let matches = app.get_matches();

    match matches.subcommand() {
        Some(("version", _)) => {
            // Print version information
            let v = version::get_version();
            println!("{} version {}", v.app_name, v.version);
            println!("Git commit: {}", v.git_hash);
            println!("Built on: {}", v.build_time);
        }
        _ => {
            // This shouldn't happen with subcommand_required, but handle it anyway
            println!("Unknown command. Use --help for usage information.");
        }
    }
}
