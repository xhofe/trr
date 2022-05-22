use std::{
    collections::LinkedList,
    fs, io,
    path::{Path, PathBuf},
};

use crate::cmd::Args;
use colored::*;
use is_executable::IsExecutable;

pub struct Tree {
    pub config: Args,
    pub output: Option<Box<dyn io::Write>>,
    pub paths: LinkedList<PathBuf>,
}

impl Tree {
    pub fn new(config: Args) -> Self {
        let mut paths = LinkedList::new();
        paths.push_back(PathBuf::from(config.path.clone()));
        let output = match config.output {
            Some(ref path) => Some(Box::new(fs::File::create(path).unwrap()) as Box<dyn io::Write>),
            None => None,
        };
        Self {
            config,
            paths,
            output,
        }
    }

    #[allow(unused)]
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }

    pub fn run(&mut self) {
        match self.dfs(Path::new(&self.config.path.clone()), 0, "".to_owned()) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }

    fn dfs(&mut self, dir: &Path, depth: usize, prefix: String) -> io::Result<()> {
        if self.config.level.is_some() && depth >= self.config.level.unwrap() {
            return Ok(());
        }
        if dir.is_dir() {
            let entries = dir
                .read_dir()?
                .filter_map(|x| match x.ok() {
                    Some(x) => {
                        if self.config.all || !x.file_name().to_str().unwrap().starts_with(".") {
                            if !self.config.directories || x.file_type().unwrap().is_dir() {
                                Some(x)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                    None => None,
                })
                .collect::<Vec<_>>();
            for (index, entry) in entries.iter().enumerate() {
                let path = entry.path();
                let is_last = index == entries.len() - 1;
                let (prefix1, prefix2) = if is_last {
                    ("└", " ")
                } else {
                    ("├", "│")
                };
                self.file_name(&path).and_then(|file_name| -> Option<()> {
                    self.println(&format!("{}{}── {}", prefix, prefix1, file_name));
                    Some(())
                });
                if path.is_dir() {
                    self.dfs(&path, depth + 1, prefix.clone() + prefix2 + "   ")?
                } else {
                    if self.config.follow_links {
                        let path = path.read_link();
                        if path.is_ok() {
                            self.dfs(&path.unwrap(), depth + 1, prefix.clone() + prefix2 + "   ")?
                        }
                    }
                }
            }
        } else {
            eprintln!("invalid path: {}", dir.display());
        }
        Ok(())
    }

    fn file_name(&self, path: &Path) -> Option<String> {
        let file_name = path.file_name()?.to_str()?.to_owned();
        let res = match path.is_symlink() {
            false => {
                if !self.config.color {
                    file_name
                } else if path.is_dir() {
                    file_name.cyan().to_string()
                } else if path.is_executable() {
                    file_name.green().bold().to_string()
                } else {
                    file_name.yellow().bold().to_string()
                }
            }
            true => {
                let link_name = path.read_link().ok()?.to_str()?.to_owned();
                if !self.config.color {
                    format!("{} -> {}", file_name, link_name)
                } else {
                    format!("{} -> {}", file_name.blue(), link_name.red().bold())
                }
            }
        };
        Some(res)
    }

    fn println(&mut self, s: &str) {
        if let Some(ref mut o) = self.output {
            match writeln!(o, "{}", s) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("{}", e);
                    std::process::exit(1);
                }
            }
        } else {
            println!("{}", s);
        }
    }
}
