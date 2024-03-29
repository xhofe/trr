use std::{
    cmp::Ordering,
    fs::{self, DirEntry},
    io,
    path::Path,
};

use crate::cmd::{Args, Sort};
use colored::*;
use is_executable::IsExecutable;

pub struct Tree {
    pub config: Args,
    pub output: Option<Box<dyn io::Write>>,
}

impl Tree {
    pub fn new(config: Args) -> Self {
        let output = match config.output {
            Some(ref path) => Some(Box::new(fs::File::create(path).unwrap()) as Box<dyn io::Write>),
            None => None,
        };
        Self { config, output }
    }

    pub fn validate(&mut self) -> Result<(), String> {
        if self.config.version {
            self.config.sort = Some(Sort::Version);
        }
        if self.config.time {
            self.config.sort = Some(Sort::Mtime);
        }
        if self.config.change {
            self.config.sort = Some(Sort::Ctime);
        }
        if self.config.unsorted {
            self.config.sort = None;
        }
        if self.config.sort == Some(Sort::Version) {
            return Err("Sorting by version is not supported yet.".to_owned());
        }
        Ok(())
    }

    pub fn run(&mut self) {
        if let Err(e) = self.validate() {
            eprintln!("{}", e.red());
            return;
        }
        match self
            .filename(Path::new(&self.config.path.clone()))
            .or_else(|| {
                if self.config.path == "." {
                    Some(".".to_owned())
                } else {
                    None
                }
            }) {
            Some(filename) => self.println(&filename),
            None => {
                eprintln!("{}", "No files found.".red());
            }
        }
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
            let mut entries = dir
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
            self.sort(&mut entries);
            for (index, entry) in entries.iter().enumerate() {
                let path = entry.path();
                if !self.is_match(&path) {
                    continue;
                }
                let is_last = index == entries.len() - 1;
                let (prefix1, prefix2) = if is_last {
                    ("└", " ")
                } else {
                    ("├", "│")
                };
                self.filename(&path).and_then(|file_name| -> Option<()> {
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

    fn filename(&self, path: &Path) -> Option<String> {
        let mut file_name = match self.config.full_path {
            true => path.display().to_string(),
            false => path.file_name()?.to_str()?.to_owned(),
        };
        if self.config.quote {
            file_name = format!("\"{}\"", file_name);
        }
        let mut res = match path.is_symlink() {
            false => {
                if !self.config.color {
                    file_name
                } else if path.is_dir() {
                    file_name.cyan().to_string()
                } else if path.is_executable() {
                    file_name.green().bold().to_string()
                } else {
                    file_name.to_string()
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
        if self.config.size || self.config.human_size {
            res = format!(
                "[{}] {}",
                self.size_to_string(path.metadata().ok()?.len()),
                res
            );
        }
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

    fn sort(&self, entries: &mut Vec<DirEntry>) {
        entries.sort_by(|a, b| {
            if self.config.dirsfirst {
                if a.file_type().unwrap().is_dir() && !b.file_type().unwrap().is_dir() {
                    return Ordering::Less;
                } else if !a.file_type().unwrap().is_dir() && b.file_type().unwrap().is_dir() {
                    return Ordering::Greater;
                }
            }
            let order = match self.config.sort {
                Some(Sort::Mtime) => {
                    let a_mtime = a.path().metadata().unwrap().modified().unwrap();
                    let b_mtime = b.path().metadata().unwrap().modified().unwrap();
                    a_mtime.cmp(&b_mtime)
                }
                Some(Sort::Ctime) => {
                    let a_ctime = a.path().metadata().unwrap().accessed().unwrap();
                    let b_ctime = b.path().metadata().unwrap().accessed().unwrap();
                    a_ctime.cmp(&b_ctime)
                }
                Some(Sort::Name) => a
                    .file_name()
                    .to_str()
                    .unwrap()
                    .cmp(&b.file_name().to_str().unwrap()),
                Some(Sort::Size) => {
                    let a_size = a.path().metadata().unwrap().len();
                    let b_size = b.path().metadata().unwrap().len();
                    a_size.cmp(&b_size)
                }
                Some(Sort::Version) => Ordering::Equal,
                None => Ordering::Equal,
            };
            if self.config.reverse {
                order.reverse()
            } else {
                order
            }
        })
    }

    fn size_to_string(&self, size: u64) -> String {
        if !self.config.human_size {
            return format!("{}", size);
        }
        let mut size = size as f64;
        let scale = 1024_f64;
        let units = ["B", "K", "M", "G", "T", "P", "E", "Z", "Y"];
        let mut unit = 0;
        while size > scale && unit < units.len() - 1 {
            size /= scale;
            unit += 1;
        }
        format!("{:7.2}{}", size, units[unit])
    }

    fn is_match(&self, path: &Path) -> bool {
        if path.is_dir() || (self.config.pattern.is_none() && self.config.ignore.is_none()) {
            return true;
        }
        let is_ignore = self.config.ignore.is_some();
        let pattern = if is_ignore {
            self.config.ignore.as_ref().unwrap()
        } else {
            self.config.pattern.as_ref().unwrap()
        };
        let filename = path.file_name().unwrap().to_str().unwrap();
        let m = if self.config.ignore_case {
            filename.to_lowercase().contains(&pattern.to_lowercase())
        } else {
            filename.contains(pattern)
        };
        m ^ is_ignore
    }
}
