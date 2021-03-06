use num_format::{Locale, ToFormattedString};
use std::path::PathBuf;

pub struct DirInfo {
    pub dir_count: usize,
    pub file_count: usize,
    pub size: usize,
}

impl DirInfo {
    pub fn new(dir_count: usize, file_count: usize, size: usize) -> Self {
        DirInfo {
            dir_count,
            file_count,
            size,
        }
    }

    pub fn file_count_formatted(&self) -> String {
        self.file_count.to_formatted_string(&Locale::en)
    }

    pub fn size_formatted(&self) -> String {
        let num = self.size / 1024_usize.pow(2);
        num.to_formatted_string(&Locale::en)
    }
}

pub fn get_folders(path: impl Into<PathBuf>, folder_name: &str) -> std::io::Result<Vec<String>> {
    fn walk(mut dir: std::fs::ReadDir, folder_name: &str) -> std::io::Result<Vec<String>> {
        dir.try_fold(Vec::new(), |mut acc: Vec<String>, file| {
            let file = file?;

            let size = match file.metadata()? {
                data if data.is_dir() => {
                    if file.file_name() == folder_name {
                        acc.push(file.path().display().to_string());
                        acc
                    } else {
                        acc.append(&mut walk(std::fs::read_dir(file.path())?, folder_name)?);
                        acc
                    }
                }
                _ => acc,
            };

            Ok(size)
        })
    }

    walk(std::fs::read_dir(path.into())?, folder_name)
}

pub fn dir_size(path: impl Into<PathBuf>) -> std::io::Result<DirInfo> {
    fn walk(mut dir: std::fs::ReadDir) -> std::io::Result<DirInfo> {
        dir.try_fold(DirInfo::new(0, 0, 0), |acc, file| {
            let file = file?;
            let size = match file.metadata()? {
                data if data.is_dir() => walk(std::fs::read_dir(file.path())?)?,
                data => DirInfo::new(1, 1, data.len() as usize),
            };

            Ok(DirInfo::new(
                acc.dir_count + 1,
                acc.file_count + size.file_count,
                acc.size + size.size,
            ))
        })
    }

    walk(std::fs::read_dir(path.into())?)
}
