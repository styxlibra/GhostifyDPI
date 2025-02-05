#[path = "shared_util.rs"]
mod shared_util;

use std::{env, fs};
use std::path::Path;
use image::{EncodableLayout, ImageFormat, ImageReader};
use resvg::{tiny_skia, usvg};

pub use shared_util::*;

fn create_parent_dir(dst: &str) {
    let folder = Path::new(dst).parent().unwrap();
    fs::create_dir_all(folder).unwrap();
}

pub fn convert_svg_to_png(src: &str, dst: &str, color: &str, width: u16, height: u16) {
    let contents = fs::read_to_string(src).unwrap();
    let svg_str = if color.is_empty() { contents } else { contents.replace("currentColor", color) };
    let svg_data = svg_str.as_bytes();
    let options = usvg::Options::default();
    let svg_tree = usvg::Tree::from_data(&svg_data, &options).unwrap();
    let target_size = usvg::Size::from_wh(width.into(), height.into()).unwrap();
    let pixmap_size = svg_tree.size().scale_to(target_size);
    let sx = pixmap_size.width() / svg_tree.size().width();
    let sy = pixmap_size.height() / svg_tree.size().height();
    let transform = tiny_skia::Transform::from_scale(sx, sy);
    let mut pixmap = tiny_skia::Pixmap::new(width.into(), height.into()).unwrap();
    resvg::render(&svg_tree, transform, &mut pixmap.as_mut());
    let data = pixmap.encode_png().unwrap();
    let existing = fs::read(dst).unwrap_or_default();
    if data == existing { return; }
    create_parent_dir(dst);
    log(format!("Converting from {src} to {dst}, size {width}x{height}"));
    fs::write(dst, data).unwrap();
}

pub fn convert_png_to_ico(src: &str, dst: &str) {
    let temp_dst_path = env::temp_dir().join("icon.ico");
    let temp_dst = temp_dst_path.as_os_str().to_str().unwrap();
    create_parent_dir(temp_dst);
    let contents = ImageReader::open(src).unwrap().decode().unwrap();
    contents.save_with_format(temp_dst, ImageFormat::Ico).unwrap();
    let data = fs::read(temp_dst).unwrap();
    fs::remove_file(temp_dst).unwrap();
    let existing = fs::read(dst).unwrap_or_default();
    if data == existing.as_bytes() { return; }
    let width = contents.width();
    let height = contents.height();
    create_parent_dir(dst);
    log(format!("Converting from {src} to {dst}, size {width}x{height}"));
    fs::write(dst, data).unwrap();
}

pub fn kill_ghostifydpi() {
    execute(vec!["taskkill.exe", "/f", "/t", "/im", "ghostifydpi.exe"]);
}

pub fn stop_goodbyedpi_service() {
    execute(vec!["sc.exe", "stop", "goodbyedpi"]);
    execute(vec!["sc.exe", "delete", "goodbyedpi"]);
}
