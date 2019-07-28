
use std::path::Path;
use quicksilver::lifecycle::Asset;
use quicksilver::graphics::Image;

pub fn load<P: 'static +  AsRef<Path>>(image: P) -> Asset<Image> {
    Asset::new(Image::load(image))
}