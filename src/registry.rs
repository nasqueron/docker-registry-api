use serde_derive::{Deserialize, Serialize};
use std::fs::{DirEntry, read_dir};
use std::io::Result as IOResult;
use std::path::Path;

/*   -------------------------------------------------------------
     Registry
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

/// Represents a Docker registry
#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize)]
pub struct Registry {
    #[serde(skip_serializing)]
    pub directory: String,

    pub repositories_count: i32
}

impl Registry {
    const DEFAULT_LOCATION: &'static str = "/var/lib/registry";

    pub fn new (directory: String) -> Self {
        let mut registry = Registry {
            directory,
            repositories_count: 0,
        };

        registry.update_stats();

        registry
    }

    pub fn with_default_location () -> Self {
        Self::new(String::from(Self::DEFAULT_LOCATION))
    }

    pub fn update_stats (&mut self) {
        self.repositories_count = self.count_repositories();
    }

    pub fn count_repositories (&self) -> i32 {
        let path_name = format!("{}/docker/registry/v2/repositories", self.directory);
        let path = Path::new(&path_name);

        if path.exists() && path.is_dir() {
            match count_subdirectories(path) {
                Ok(n) => n as i32,
                Err(e) => {
                    error!(target: "api", "Can't count registry directories: {}", e);

                    0
                }
            }
        } else {
            error!(target: "api",
                   "Registry path doesn't exist or isn't a directory: {}",
                   path_name);

            0
        }
    }
}


/*   -------------------------------------------------------------
     File system helper functions
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

fn count_subdirectories (dir: &Path) -> IOResult<usize> {
    let count = read_dir(dir)?
        .filter(|entry| is_entry_sub_directory(entry))
        .count();

    Ok(count)
}

fn is_entry_sub_directory(entry: &IOResult<DirEntry>) -> bool {
    match entry {
        Ok(e) => e.path().is_dir(),
        Err(_) => false,
    }
}
