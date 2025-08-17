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
        )
        .subcommand(
            Command::new("generate")
                .about("Generate Croissant metadata from a CSV file")
                .arg(clap::Arg::new("input")
                    .help("Input CSV file")
                    .required(true)
                    .index(1)
                )
                .arg(clap::Arg::new("output")
                    .short('o')
                    .long("output")
                    .help("Output JSON-LD file")
                    .required(false)
                    .value_name("FILE")
                )
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
        Some(("generate", sub_m)) => {
            let input = sub_m
                .get_one::<String>("input")
                .expect("Input CSV required");
            let output = sub_m.get_one::<String>("output");
            let input_path = std::path::Path::new(input);
            let output_path = output.map(|o| std::path::Path::new(o));

            match rustcroissant::croissant::generate::generate_metadata_from_csv(
                input_path,
                output_path,
            ) {
                Ok(_) => {
                    if let Some(o) = output {
                        println!("Croissant metadata generated and saved to: {}", o);
                    } else {
                        println!("Croissant metadata generated.");
                    }
                }
                Err(e) => {
                    eprintln!("Error generating metadata: {}", e);
                    std::process::exit(1);
                }
            }
        }
        _ => {
            // This shouldn't happen with subcommand_required, but handle it anyway
            println!("Unknown command. Use --help for usage information.");
        }
    }
}
