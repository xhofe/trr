use std::{
    collections::LinkedList,
    io,
    path::{Path, PathBuf},
};

use crate::cmd::{Args, Sort};

pub struct Tree {
    pub config: Args,
    pub paths: LinkedList<PathBuf>,
}

impl Tree {
    pub fn new(config: Args) -> Self {
        let mut paths = LinkedList::new();
        paths.push_back(PathBuf::from(config.path.clone()));
        Self { config, paths }
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
                std::process::exit(1);
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
                    Some(x) => Some(x),
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
                println!(
                    "{}{}── {}",
                    prefix,
                    prefix1,
                    path.file_name().unwrap().to_str().unwrap()
                );
                if path.is_dir() {
                    self.dfs(&path, depth + 1, prefix.clone() + prefix2 + "   ")?
                }
            }
        }
        Ok(())
    }
}