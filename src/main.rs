mod manifest;

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use manifest::Entry;
use crate::manifest::Manifest;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    mode: AppMode,

    /// Path to manifest file (defaults to $CONFIG_DIR/dot/.dot.toml)
    #[arg(long, short)]
    manifest: Option<PathBuf>
}

#[derive(Subcommand, Debug)]
enum AppMode {
    /// Install dotfiles to the manifest repo
    #[clap(alias = "t")]
    Track {
        name: String,
        path: PathBuf,
    },
    /// Purge dotfiles from the manifest repo
    #[clap(alias = "u")]
    Untrack {
        name: String,
    },
    /// Copy dotfiles from the manifest repo to disk
    #[clap(alias = "e")]
    Export {
        name: String,
    },
    /// Copy dotfiles from disk to the manifest repo
    #[clap(alias = "i")]
    Import {
        name: String,
    },
    List,
}

fn main() {
    let args = Args::parse();
    let mut manifest = match Manifest::load_from_disk(args.manifest) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Error while parsing manifest: {e}");
            return;
        }
    };

    match args.mode {
        AppMode::Track { name, path } => track(&mut manifest, name, path),
        AppMode::Untrack { name } => untrack(&mut manifest, name),
        AppMode::Export { name } => export(&mut manifest, name),
        AppMode::Import { name } => import(&mut manifest, name),
        AppMode::List => list(&manifest),
    }
}

fn track(manifest: &mut Manifest, name: String, path: PathBuf) {
    manifest.add_entry(name, path);
}

fn untrack(manifest: &mut Manifest, name: String) {
    manifest.drop_entry(name);
}

fn export(manifest: &mut Manifest, name: String) {
    let Entry {
        local_file: src,
        path: dest,
        dir,
    } = manifest.entry(name).expect("No entry found.");

    copy(src, dest, *dir);
}

fn import(manifest: &mut Manifest, name: String) {
    let Entry {
        local_file: dest,
        path: src,
        dir
    } = manifest.entry(name).expect("No entry found.");

    copy(src, dest, *dir);
}

fn list(manifest: &Manifest) {
    let mut keys: Vec<String> = manifest.entries.keys().cloned().collect();
    keys.sort_by(|a, b| a.to_lowercase().partial_cmp(&b.to_lowercase()).unwrap());
    
    for entry in keys {
        println!("{}", entry);
    }
}

pub(crate) fn copy(src: &PathBuf, dest: &PathBuf, dir: bool) {
    if dir {
        todo!();
    }

    println!("{src:?} -> {dest:?}");
    std::fs::copy(src, dest).unwrap();
}
