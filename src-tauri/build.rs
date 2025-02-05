#[path = "src/util/compile_time_util.rs"]
mod compile_time_util;

use std::{env, fs};
use copy_to_output::copy_to_output;
use tauri_build::{try_build, Attributes, WindowsAttributes};
use compile_time_util::{convert_png_to_ico, convert_svg_to_png, kill_ghostifydpi, kill_goodbyedpi, stop_goodbyedpi_service, ICON_SIZES};

fn generate_sized_icons(src: &str, dst_prefix: &str, color: &str) {
    for size in ICON_SIZES {
        let dst = format!("./icons/generated/{dst_prefix}_{size}x{size}.png");
        convert_svg_to_png(src, dst.as_str(), color, size, size);
    }
}

fn generate_tray_icons(src: &str) {
    generate_sized_icons(src, "icon_light", "black");
    generate_sized_icons(src, "icon_dark", "white");
}

fn generate_title_bar_icons(src: &str) {
    let favicon = "../public/generated/icon.png";
    generate_sized_icons(src, "icon", "");
    convert_svg_to_png(src, favicon, "", 256, 256);
    convert_png_to_ico(favicon, "./icons/generated/icon.ico");
}

fn generate_icons() {
    let tray_icon_src = "../src/assets/icon-outline.svg";
    println!("cargo:rerun-if-changed={tray_icon_src}");
    generate_tray_icons(tray_icon_src);
    let title_bar_icon_src = "../src/assets/icon.svg";
    println!("cargo:rerun-if-changed={title_bar_icon_src}");
    generate_title_bar_icons(title_bar_icon_src);
}

fn build() {
    println!("cargo:rustc-cfg=runtime");
    let manifest = fs::read_to_string("./windows/manifest.xml").unwrap();
    let win_attributes = WindowsAttributes::new().app_manifest(manifest);
    let attributes = Attributes::new().windows_attributes(win_attributes);
    try_build(attributes).expect("Failed to run build script");
}

fn copy_goodbyedpi_to_bundle() {
    let build_type = &env::var("PROFILE").unwrap();
    let goodbyedpi_path = "../goodbyedpi";
    let folders_to_copy = ["x86", "x86_64", "licenses"];
    for folder in folders_to_copy {
        println!("cargo:rerun-if-changed={goodbyedpi_path}/{folder}/*");
        copy_to_output(&format!("{goodbyedpi_path}/{folder}"), build_type)
            .expect(&format!("Failed to copy {folder}"));
    }
}

fn main() {
	stop_goodbyedpi_service();
    kill_ghostifydpi();
    kill_goodbyedpi();
    generate_icons();
    build();
    copy_goodbyedpi_to_bundle();
}
