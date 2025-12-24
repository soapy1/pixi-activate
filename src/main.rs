use std::path::PathBuf;
use std::fs;
use serde::{Serialize, Deserialize};
use directories::UserDirs;
use pixi::{cli::shell, config::WorkspaceConfig};

use clap::{Parser};


#[derive(Parser, Debug)]
#[command(name="pixi-activate", version, about = "Activate a pixi-registered environment")]
struct Cli {
    /// Name of the environment to activate
    #[arg(long, short)]
    name: Option<String>,

    /// The path to `pixi.toml`, `pyproject.toml`, or the project directory
    #[arg(long)]
    manifest_path: Option<PathBuf>,

    /// The environment to activate in the shell
    #[arg(long, short)]
    environment: Option<String>,
}

// Define a struct that matches the structure of your JSON objects
#[derive(Debug, Serialize, Deserialize)]
struct RegisteredEnvironment {
    name: String,
    path: String,
}

fn environment_registry() -> PathBuf {
    let user_dirs = UserDirs::new().expect("Could not determine user directories");
    let register_dir = user_dirs.home_dir().join(".pixi/register");
    fs::create_dir_all(&register_dir).expect("Could not create register directory");
    return register_dir.join("environments.json");
}

fn get_manifest_path_from_name(name: &str) -> Option<PathBuf> {
    let registry_path = environment_registry();
    let data = fs::read_to_string(&registry_path).unwrap_or_else(|_| "[]".to_string());
    let envs: Vec<RegisteredEnvironment> = serde_json::from_str(&data).unwrap();
    
    return envs.iter()
        .find(|env| env.name == name)
        .map(|env| PathBuf::from(&env.path))
}

fn activate_environment(path: PathBuf, environment: Option<String>) {
    let shell_args = shell::Args::new(
        environment = environment,
        workspace_config = WorkspaceConfig::new(manifest_path = path),
    );
    return shell::execute(shell_args);
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if (cli.manifest_path.is_none() & cli.name.is_none()) | (cli.manifest_path.is_some() & cli.name.is_some() ) {
        eprintln!("Must provide only one of `name` or `manifest-path`");
        std::process::exit(1);
    }

    if let Some(name) = cli.name {
        let manifest_path = match get_manifest_path_from_name(&name) {
            Some(path) => path,
            None => {
                eprintln!("Error: Environment name '{}' not found in registry", name);
                std::process::exit(1);
            }
        };
        activate_environment(manifest_path, cli.environment);
    }

    if let Some(manifest_path) = cli.manifest_path{
        match fs::canonicalize(manifest_path) {
            Ok(absolute_path) => {
                activate_environment(absolute_path, cli.environment);
            }
            Err(e) => {
                eprintln!("Error canonicalizing path: {}", e);
                std::process::exit(1);
            }
        }
    }

    std::process::exit(0);
}
