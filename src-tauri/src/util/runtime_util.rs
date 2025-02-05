use std::env::current_exe;
use std::path::PathBuf;

pub fn current_exe_dir() -> PathBuf {
    current_exe().unwrap().parent().unwrap().to_path_buf()
}
