use std::collections::HashSet;
use std::fs::{create_dir_all, File, read_to_string, write};
use std::path::PathBuf;
use directories::BaseDirs;
use toml::ser::Error;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct HarpAppConfig {
    project_history: HashSet<String>,
}

#[derive(Debug)]
pub enum ConfigErr {
    Path(String),
    CreateDir(String),
    SerializationErr(String),
    DeserializeErr(String),
    IOError(String),
}

impl HarpAppConfig {
    pub fn new() -> Self {
        Self {
            project_history: HashSet::new()
        }
    }

    pub fn add_project(&mut self, path: &String) {
        self.project_history.insert(path.to_owned());
    }

    pub fn projects(&self) -> Vec<String> {
        self.project_history.iter().map(|s| s.to_owned()).collect()
    }

    pub fn app_config_path() -> Option<PathBuf> {
        match BaseDirs::new() {
            Some(xs) => Some(xs.config_local_dir().join("harp").to_path_buf()),
            None => None
        }
    }

    pub fn app_config_file_path() -> Option<PathBuf> {
        HarpAppConfig::app_config_path().map(|f| f.join("config.toml"))
    }

    pub fn save(&self) -> Result<(), ConfigErr> {
        let path = HarpAppConfig::app_config_file_path().ok_or(ConfigErr::Path("Failed to build path.".to_string()))?;
        let cfg_dir = HarpAppConfig::app_config_path().ok_or(ConfigErr::Path("Failed to build path".to_string()))?;
        let _ = create_dir_all(cfg_dir.as_path()).map_err(|e| ConfigErr::CreateDir(e.to_string()))?;
        let res = toml::to_string_pretty(&self).map_err(|e| ConfigErr::SerializationErr(e.to_string()))?;
        let _ = write(&path.as_path(), res).map_err(|e| ConfigErr::IOError(e.to_string()))?;
        Ok(())
    }

    pub fn load() -> Result<HarpAppConfig, ConfigErr> {
        if let Some(path) = HarpAppConfig::app_config_file_path() {
            if path.as_path().exists() {
                let contents = read_to_string(path.as_path()).map_err(|e| ConfigErr::IOError(e.to_string()))?;
                let cfg = toml::from_str(contents.as_str()).map_err(|e| ConfigErr::DeserializeErr(e.to_string()))?;
                Ok(cfg)
            } else {
                Ok(Self::new())
            }
        } else {
            Err(ConfigErr::Path("Failed to build path.".to_string()))
        }
    }
}

