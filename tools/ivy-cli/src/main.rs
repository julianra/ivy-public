use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use ivy_devkit::{DevelopmentSigningKey, parse_manifest_yaml};

#[derive(Parser)]
#[command(name = "ivy", about = "Standalone Ivy Developer Kit CLI")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Validate a public Ivy module manifest.
    Validate { manifest: PathBuf },
    /// Generate a minimal standalone module skeleton.
    New {
        path: PathBuf,
        #[arg(long)]
        id: String,
        #[arg(long)]
        namespace: String,
    },
    /// Generate an explicitly development-only signing key.
    DevKeygen { output: PathBuf },
    /// Sign a file with a development-only key.
    DevSign {
        input: PathBuf,
        #[arg(long)]
        key: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Command::Validate { manifest } => validate(&manifest),
        Command::New {
            path,
            id,
            namespace,
        } => generate(&path, &id, &namespace),
        Command::DevKeygen { output } => keygen(&output),
        Command::DevSign { input, key, output } => sign(&input, &key, &output),
    }
}

fn validate(path: &Path) -> Result<()> {
    let text =
        fs::read_to_string(path).with_context(|| format!("could not read {}", path.display()))?;
    let manifest = parse_manifest_yaml(&text)?;
    println!(
        "valid Ivy development manifest: {} {} ({} provided capabilities)",
        manifest.id,
        manifest.version,
        manifest.capabilities.provides.len()
    );
    Ok(())
}

fn generate(path: &Path, id: &str, namespace: &str) -> Result<()> {
    if path.exists() {
        bail!("refusing to overwrite existing path {}", path.display());
    }
    fs::create_dir_all(path.join("src"))?;
    let domain = format!("community.{id}").replace('-', "_");
    let manifest = format!(
        "schema_version: 3\nid: {id}\nnamespace: {namespace}\nversion: 0.1.0\nlanguage: rust\ncapability_domains:\n  owns: [{domain}]\ncapabilities:\n  provides:\n    - id: {domain}.status\n      version: 1\n      kind: query\n      schema: schema:{domain}.status:v1\n      externally_invocable: false\n  requires: []\n  dynamic_requires: false\n"
    );
    parse_manifest_yaml(&manifest)?;
    fs::write(path.join("module.yaml"), manifest)?;
    fs::write(
        path.join("README.md"),
        format!("# {id}\n\nGenerated with the Ivy Developer Kit.\n"),
    )?;
    fs::write(
        path.join("src/main.rs"),
        "fn main() { println!(\"Replace this with your Ivy provider implementation.\"); }\n",
    )?;
    println!("created module skeleton at {}", path.display());
    Ok(())
}

fn keygen(output: &Path) -> Result<()> {
    if output.exists() {
        bail!("refusing to overwrite existing key {}", output.display());
    }
    let key = DevelopmentSigningKey::generate();
    fs::write(output, format!("{}\n", key.secret_base64()))?;
    set_private_permissions(output)?;
    println!(
        "created DEVELOPMENT-ONLY key at {}; never use it for official releases",
        output.display()
    );
    Ok(())
}

fn sign(input: &Path, key_path: &Path, output: &Path) -> Result<()> {
    let bytes = fs::read(input)?;
    let key = DevelopmentSigningKey::from_base64(&fs::read_to_string(key_path)?)?;
    let signature = key.sign(&bytes);
    fs::write(output, serde_json::to_vec_pretty(&signature)?)?;
    println!("wrote development signature to {}", output.display());
    Ok(())
}

#[cfg(unix)]
fn set_private_permissions(path: &Path) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;
    fs::set_permissions(path, fs::Permissions::from_mode(0o600))?;
    Ok(())
}

#[cfg(not(unix))]
fn set_private_permissions(_path: &Path) -> Result<()> {
    Ok(())
}
