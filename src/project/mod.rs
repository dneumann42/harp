use std::collections::HashMap;
use std::fs;
use std::fs::{create_dir_all, File, read_to_string, write};
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;
use directories::BaseDirs;
use rustyline::Config;
use serde_derive::{Deserialize, Serialize};
use crate::nodes::functions::Exp;
use crate::nodes::Node;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Project {
    config: Node,
}

impl Project {
    pub fn load_project(path: &String) -> Self {
        Project::make("", path)
    }

    pub fn name(&self) -> String {
        match &self.config {
            Node::Exp(Exp::Dict(cfg)) => {
                cfg["name"].to_string()
            }
            _ => "".to_string()
        }
    }

    pub fn path(&self) -> String {
        match &self.config {
            Node::Exp(Exp::Dict(xs)) => {
                xs["path"].to_string()
            }
            _ => "".to_string()
        }
    }

    pub fn files(&self) -> Vec<String> {
        let xs: Vec<_> = fs::read_dir(self.project_dir()).unwrap().map(|x| x.unwrap()).collect();
        xs.iter().map(|x| x.path().to_string_lossy().to_string()).collect()
    }

    pub fn path_buf(&self) -> PathBuf {
        PathBuf::from_str(self.path().as_str()).unwrap()
    }

    pub fn project_config_path(&self) -> PathBuf {
        self.path_buf().join("project.toml")
    }

    pub fn projects_cache_path(&self) -> PathBuf {
        if let Some(base) = BaseDirs::new() {
            base.data_local_dir().join("harp_projects").join(self.name())
        } else {
            panic!("Failed to get dir path");
        }
    }

    fn base_path() -> PathBuf {
        if let Some(base) = BaseDirs::new() {
            base.home_dir().to_path_buf()
        } else {
            panic!("Failed to get dir path");
        }
    }

    fn project_dir(&self) -> String {
        self.path_buf().to_string_lossy().to_string()
    }

    fn config_dir(&self) -> String {
        self.path_buf().join("project.toml").to_string_lossy().to_string()
    }

    fn default_config() -> Node {
        let mut cfg: HashMap<String, Node> = HashMap::new();
        cfg.insert("name".to_string(), Node::a(""));
        cfg.insert("author".to_string(), Node::s(""));
        cfg.insert("licence".to_string(), Node::s("MIT"));
        cfg.insert("version".to_string(), Node::a("v0.0.0"));
        cfg.insert("path".to_string(), Node::s(Project::base_path().to_string_lossy().to_string()));
        cfg.insert("cache_path".to_string(), Node::s(""));
        Node::Exp(Exp::Dict(cfg))
    }

    pub fn save(&self) {
        if let Some(base) = BaseDirs::new() {
            create_dir_all(self.projects_cache_path()).expect("Failed to create project directory or already exists");
            create_dir_all(self.project_dir()).expect("Failed to create project directory or already exists");
            let s = toml::to_string_pretty(&self.config).expect("Failed to serialize to toml");
            let mut f = File::create(self.config_dir()).unwrap();
            write(self.config_dir(), s).unwrap();
        }
    }

    pub fn load(&mut self) {
        let contents = read_to_string(self.config_dir()).expect("Failed to read project config");
        self.config = toml::from_str(contents.as_str()).expect("Failed to parse toml");
    }

    fn new() -> Self {
        Default::default()
    }

    pub fn make<S: ToString>(name: S, path: S) -> Self {
        let mut cfg = Project::default_config();
        match &mut cfg {
            Node::Exp(Exp::Dict(cfg)) => {
                cfg.insert("name".to_string(), Node::a(name));
                cfg.insert("path".to_string(), Node::s(path));
                Self {
                    config: Node::Exp(Exp::Dict(cfg.clone()))
                }
            }
            _ => panic!()
        }
    }
}

impl Default for Project {
    fn default() -> Self {
        Self {
            config: Project::default_config(),
        }
    }
}