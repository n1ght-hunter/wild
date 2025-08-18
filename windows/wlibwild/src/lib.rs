use std::path::{Path, PathBuf};

use tracing::{info, warn};



pub fn run() {
    let files = vec![
        PathBuf::from("basic_objs/function.o"),
        PathBuf::from("basic_objs/main.o"),
    ];
    link_files(files);
}

fn link_files(files: Vec<PathBuf>) {
    info!("Starting to link files: {:?}", files);
}
