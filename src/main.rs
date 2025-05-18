use clap::{Parser, ValueEnum};
use include_dir::{Dir, include_dir};
use regex::Regex;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

/// Embed the entire templates directory
static TEMPLATE_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/templates");

#[derive(Parser)]
#[command(name = "pinocchio-init")]
#[command(about = "Initialize a new Pinocchio program.")]
struct Args {
    /// Name of the program
    pub program_name: String,

    /// Template to use (basic or full)
    #[arg(long, default_value = "basic")]
    pub template: TemplateKind,
}

#[derive(Debug, Clone, ValueEnum)]
enum TemplateKind {
    Basic,
    Full,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let target_dir = PathBuf::from(&args.program_name);

    if target_dir.exists() {
        eprintln!("Error: directory '{}' already exists.", args.program_name);
        std::process::exit(1);
    }

    // Select the template directory
    let template_name = match args.template {
        TemplateKind::Basic => "basic",
        TemplateKind::Full => "full",
    };

    println!("Initializing {}", args.program_name);
    println!("Using template: {}", template_name);

    let template_root = TEMPLATE_DIR
        .get_dir(template_name)
        .expect("Template directory not found");

    copy_dir(template_root, &target_dir)?;

    let normalized_program_name = normalize_program_name(&args.program_name);
    let random_program_id = Keypair::new().pubkey().to_string();
    replace_template_name(
        &target_dir,
        vec![
            ("__PROGRAM_NAME__", &args.program_name),
            ("__PROGRAM_NAME_NORMALIZED__", &normalized_program_name),
            ("__PROGRAM_ID__", &random_program_id),
        ],
    )?;

    println!("Initializing new Git repository");
    init_git(&target_dir)?;

    println!("{} initialized successfully", args.program_name);
    Ok(())
}

/// Recursively copy embedded template directory to disk.
fn copy_dir(dir: &Dir, dest: &Path) -> io::Result<()> {
    for file in dir.files() {
        // Remove the first component (the template folder name).
        let mut components = file.path().components();
        components.next();

        let relative_path: PathBuf = components.collect();
        let full_path = dest.join(relative_path);
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&full_path, file.contents())?;
    }

    for subdir in dir.dirs() {
        copy_dir(subdir, &dest)?;
    }

    Ok(())
}

/// Replace occurrences of old values with new values in all files under `dir`.
fn replace_template_name(dir: &Path, replacements: Vec<(&str, &str)>) -> io::Result<()> {
    for entry in WalkDir::new(dir).into_iter().filter_map(Result::ok) {
        if entry.file_type().is_file() {
            for (old, new) in &replacements {
                let re = Regex::new(&old).unwrap();
                let path = entry.path();
                let content = fs::read_to_string(path)?;
                let new_content = re.replace_all(&content, *new).to_string();
                fs::write(path, new_content)?;
            }
        }
    }

    Ok(())
}

/// Initialize a git repository in the target directory.
fn init_git(dir: &Path) -> io::Result<()> {
    Command::new("git")
        .arg("init")
        .current_dir(dir)
        .status()
        .expect("Failed to run git init");

    Ok(())
}

/// Normalize the program name.
fn normalize_program_name(name: &str) -> String {
    name.replace('-', "_")
}
