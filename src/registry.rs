use lazy_static::lazy_static;
use log::error;
use regex::Regex;
use serde_derive::{Deserialize, Serialize};
use std::fs::{DirEntry, File, read_dir};
use std::io::{Read, Result as IOResult};
use std::path::Path;

/*   -------------------------------------------------------------
     Registry
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

/// Represents a Docker registry
#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Clone)]
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
        let path_name = self.get_repositories_path();
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

    pub fn get_repositories_path (&self) -> String {
        format!("{}/docker/registry/v2/repositories", self.directory)
    }

    pub fn get_repository (&self, repository_name: &str) -> Option<Repository> {
        if !Repository::is_valid_name(repository_name) {
            return None
        }

        let path = Path::new(&self.get_repositories_path()).join(repository_name);

        let directory = match path.as_path().to_str() {
            Some(name) => String::from(name),
            None => { return None; }
        };

        let mut repository = Repository {
            directory,
            name: String::from(repository_name),
            tags: Vec::new(),
        };

        repository.update_tags();

        Some(repository)
    }
}

/*   -------------------------------------------------------------
     Repository
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

/// Represents a repository from the Docker registry
#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize)]
pub struct Repository {
    #[serde(skip_serializing)]
    pub directory: String,

    pub name: String,
    pub tags: Vec<Tag>,
}

impl Repository {
    pub fn exists(&self) -> bool {
        let path = Path::new(&self.directory);

        path.exists() && path.is_dir()
    }

    pub fn is_valid_name(name: &str) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new("^/?[a-zA-Z0-9_-]+$").unwrap();
        }

        RE.is_match(name) && name.len() <= 30
    }

    pub fn update_tags(&mut self) {
        let path = Path::new(&self.directory).join("_manifests/tags");

        let tag_names = get_subdirectories_names(&path);
        self.tags = tag_names.iter()
            .map(|name| Tag {
                name: name.clone(),
                hash: self.get_hash_for_tag(&name).unwrap_or(String::new()),
            })
            .collect();
    }

    fn get_hash_for_tag(&self, tag_name: &str) -> IOResult<String> {
        let mut buffer = String::new();

        let path = Path::new(&self.directory)
            .join("_manifests/tags")
            .join(tag_name)
            .join("current/link");

        let mut f = File::open(path)?;
        f.read_to_string(&mut buffer)?;

        buffer = Tag::clean_tag(&buffer);

        Ok(buffer)
    }
}

/*   -------------------------------------------------------------
     Tag
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

/// Represents a repository tag
#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Clone)]
pub struct Tag {
    pub name: String,
    pub hash: String,
}

impl Tag {
    pub fn clean_tag (tag: &str) -> String {
        let fragments: Vec<&str> = tag.split(":").collect();

        if fragments.len() == 1 {
            String::from(tag)
        } else {
            String::from(fragments[1])
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

fn get_entry_name(entry: &IOResult<DirEntry>) -> String {
    match entry {
        Ok(e) => match e.file_name().into_string() {
            Ok(name) => String::from(name),
            Err(_) => String::new(),
        }
        Err(_) => String::new(),
    }
}

fn get_subdirectories_names (dir: &Path) -> Vec<String> {
    match std::fs::read_dir(dir) {
        Ok(iterator) => {
            iterator
                .filter(|entry| is_entry_sub_directory(entry))
                .map(|entry| get_entry_name(&entry))
                .filter(|name| name != "")
                .collect::<Vec<_>>()
        },
        Err(_) => {
            error!("Can't get subdirectories of {:?}", dir);

            Vec::new()
        }
    }
}
