# rustcroissant

A Rust implementation for working with the [ML Commons Croissant](https://github.com/mlcommons/croissant) metadata format—a standardized way to describe machine learning datasets using JSON-LD.

[![Version](https://img.shields.io/badge/version-v0.1.2-blue)](https://github.com/beyondcivic/rustcroissant/releases/tag/v0.1.2)
[![Rust Version](https://img.shields.io/badge/Rust-1.88+-CE422B?logo=rust)](https://forge.rust-lang.org/channel-releases.html)
[![License](https://img.shields.io/badge/license-TBD-red)](LICENSE)

## Overview

Croissant is an open metadata standard designed to improve dataset documentation, searchability, and usage in machine learning workflows. This library simplifies the creation of Croissant-compatible metadata from CSV data sources by:

- Automatically inferring schema types from dataset content
- Generating complete, valid JSON-LD metadata
- Providing validation tools to ensure compatibility
- Supporting the full Croissant specification

This project provides both a command-line interface and a Rust library for converting CSV files to Croissant metadata format.

## Getting Started

### Prerequisites

- Rust 1.88 or later
- Nix 2.25.4 or later

### Installation

1. Clone the repository:

```bash
git clone https://github.com/beyondcivic/rustcroissant.git
cd rustcroissant
```

2. Prepare the environment using Nix flakes (recommended):

```bash
nix develop
```

### Building the Application

Build the project using Nix:

```bash
nix build
```

The resulting binary will be in the `result/bin/` directory.

✨ GOOD TO KNOW: The `nix build` command can be used instead of `cargo build` command, as it now uses Nix to manage dependencies and build the project.

### Running the Application

Run the CLI directly with Nix:

```bash
nix run
```

Or, specify arguments:

```bash
nix run . -- generate data.csv -o metadata.jsonld
```

✨GOOD TO KNOW: The `nix run` command can be used instead of `cargo run` command, as it now uses Nix to manage dependencies and run the project.

## Usage

### Command Line Interface

```bash
# Generate metadata with default output path
nix run . -- generate data.csv

# Specify output path
nix run . -- generate data.csv -o metadata.jsonld
```

### Using the Library in Your Rust Code

```rust
use rustcroissant::generate_metadata;

fn main() {
    let output_path = generate_metadata("data.csv", Some("dataset.jsonld"))
        .expect("Error generating metadata");
    println!("Metadata saved to: {}", output_path);
}
```

## Features

- Automatically infers field data types from CSV content
- Calculates SHA-256 hash for file verification
- Generates Croissant metadata in JSON-LD format
- Configurable output path

## Configuration

The application supports configuration through environment variables with the prefix `CROISSANT_`.

Currently, only `CROISSANT_OUTPUT_PATH` is supported to specify the output file path for generated metadata.

If no output path is provided explicitly, the default output path `metadata.jsonld` will be used.

## Usage Examples

### Generate metadata without validation

```bash
nix run . -- generate data.csv -o metadata.jsonld
```

### Generate metadata with validation

```bash
nix run . -- generate data.csv -o metadata.jsonld -v

Validation passed with no issues.
Croissant metadata generated and saved to: metadata.json
```

### Generate metadata with validation but without saving to a file

```bash
nix run . -- generate data.csv -v
Validation passed with no issues.
```

### Validate an existing metadata file

```bash
nix run . -- validate metadata.json
Validation passed with no issues.
```

### Example with issues

```
nix run . -- validate ./samples_jsonld/missing_fields.jsonld

Found the following 3 error(s) during the validation:
  -  [Metadata(mydataset) > FileObject(a-csv-table)] Property "https://schema.org/contentUrl" is mandatory, but does not exist.
  -  [Metadata(mydataset) > RecordSet(a-record-set) > Field(first-field)] The field does not specify a valid http://mlcommons.org/croissant/dataType, neither does any of its predecessor. Got:
  -  [Metadata(mydataset)] The current JSON-LD doesn't extend https://schema.org/Dataset.

Found the following 1 warning(s) during the validation:
  -  [Metadata(mydataset)] Property "http://purl.org/dc/terms/conformsTo" is recommended, but does not exist.
exit status 1
```

## Development

### Adding New Data Types

To add support for new data types, modify the `infer_data_type` function in `src/croissant/core.rs`:

```rust
fn infer_data_type(value: &str) -> &'static str {
    // Existing data type detection...

    // Add your new data type detection here
    if my_custom_type_detector(value) {
        return "sc:MyCustomType";
    }

    // Default to Text
    "sc:Text"
}
```

## License

TODO.

## Build Environment

Use Nix flakes to set up the build environment:

```bash
nix develop
```

Check the build arguments in your Nix flake or shell.nix file as needed.

Then build and run the project using:

```bash
nix build
nix
```
