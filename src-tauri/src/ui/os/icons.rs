use include_dir::{include_dir, Dir};
use std::{collections::HashMap, sync::LazyLock};
use tauri::image::Image;

use crate::util::ICON_SIZES;

static ICON_SOURCES: Dir = include_dir!("./icons/generated");
const EMPTY_IMAGE: Image = Image::new(&[], 0, 0);

pub struct IconSet<'a> {
    icon: Image<'a>,
    icon_dark: Image<'a>,
    icon_light: Image<'a>,
}

impl<'a> IconSet<'a> {
    pub const fn new() -> Self {
        IconSet {
            icon: EMPTY_IMAGE,
            icon_dark: EMPTY_IMAGE,
            icon_light: EMPTY_IMAGE,
        }
    }

    pub fn icon_dark(&self) -> Image {
        self.icon_dark.clone()
    }

    pub fn icon_light(&self) -> Image {
        self.icon_light.clone()
    }
}

pub static ICONS: LazyLock<HashMap<u16, IconSet>> = LazyLock::new(|| {
    let mut map = HashMap::with_capacity(ICON_SIZES.len());
    for size in ICON_SIZES {
        let mut icon_set = IconSet::new();
        for theme in ["icon", "icon_dark", "icon_light"] {
            let file = format!("{theme}_{size}x{size}.png");
            let contents = ICON_SOURCES.get_file(file).unwrap().contents();
            let image = Image::from_bytes(contents).unwrap();
            match theme {
                "icon" => icon_set.icon = image,
                "icon_dark" => icon_set.icon_dark = image,
                "icon_light" => icon_set.icon_light = image,
                _ => panic!("Unexpected theme {theme}"),
            }
        }
        map.insert(size, icon_set);
    }
    map
});
