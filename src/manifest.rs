use std::{
    collections::HashMap,
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
};

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::copy;

static BASE_DIR: Lazy<PathBuf> = Lazy::new(|| {
    dirs::config_dir()
        .map(|buf| buf.join("dot"))
        .expect("Failed to get default path")
});

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct Manifest {
    #[serde(skip)]
    path: PathBuf,
    #[serde(flatten)]
    pub(crate) entries: HashMap<String, Entry>,
}

impl Default for Manifest {
    fn default() -> Self {
        let path = BASE_DIR.join(".dot.toml");
        let entries = HashMap::new();
        Self { path, entries }
    }
}

impl Manifest {
    pub(crate) fn load_from_disk(manifest: Option<PathBuf>) -> Result<Self, String> {
        let path = match manifest {
            Some(m) => m,
            None => BASE_DIR.join(".dot.toml"),
        };

        ensure_exists(&path);
        let mut manifest: Manifest = fs::read_to_string(path.as_path())
            .ok()
            .and_then(|contents| toml::from_str(&contents).ok())
            .unwrap_or_default();

        manifest.path = path;

        Ok(manifest)
    }

    pub fn entry<S: AsRef<str>>(&self, name: S) -> Option<&Entry> {
        self.entries.get(name.as_ref())
    }

    pub fn drop_entry<S: AsRef<str>>(&mut self, name: S) {
        self.entries.remove(name.as_ref());
        self.save();
    }

    pub fn add_entry<S: AsRef<str>>(&mut self, name: S, path: PathBuf) {
        if self.entries.contains_key(name.as_ref()) {
            return;
        }

        let local = BASE_DIR.join(name.as_ref());
        copy(&path, &local, path.is_dir());
        self.entries.insert(
            name.as_ref().to_owned(),
            Entry {
                dir: path.is_dir(),
                path,
                local_file: local,
            },
        );
        self.save();
    }

    pub fn save(&mut self) {
        ensure_exists(&self.path);
        let contents = toml::to_string_pretty(self).expect("Failed to serialize to string");
        let mut f = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.path)
            .expect("Failed to open manifest file for writing.");

        f.write_all(contents.as_bytes())
            .expect("Failed to save manifest contents to disk.");
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct Entry {
    pub local_file: PathBuf,
    pub path: PathBuf,
    pub dir: bool,
}

fn ensure_exists(path: &PathBuf) {
    match path.parent() {
        Some(parent) => {
            fs::create_dir_all(parent).expect("Failed to create manifest directory hierarchy.")
        }
        None => {}
    }

    _ = OpenOptions::new()
        .write(true)
        .create(true)
        .open(path);
}
