use super::structs::{Module, ModulesContainer, SpeciesConf, TilesConf};
use quicksilver::{
    combinators::join_all, graphics::Image, load_file, saving::SaveError, Error, Future,
};
use serde::de::Deserialize;
use std::{io::Read, path::Path};
use zip;

fn serde_err_to_quick_err<T>(res: serde_json::Result<T>) -> quicksilver::Result<T> {
    res.map_err(|v| Error::SaveError(SaveError::SerdeError(v)))
}
fn parse_json<'a, T>(buff: &'a [u8]) -> quicksilver::Result<T>
where
    T: Deserialize<'a>,
{
    serde_err_to_quick_err(serde_json::from_slice(buff))
}

pub fn get_all_mod_paths() -> impl Future<Item = Vec<String>, Error = Error> {
    load_file("./mods.json").and_then(|v| {
        println!("Got here");
        parse_json(&v)
    })
}

pub fn load_mod_info(path: &str) -> impl Future<Item = Module, Error = Error> {
    println!("got in load info? {}", path);
    let path = Path::new(&path).with_extension("zip");
    println!("{:?}", path);
    load_file(path).map(std::io::Cursor::new).map(|v| {
        println!("test?");
        let mut module = zip::read::ZipArchive::new(v).unwrap();
        let mut modu = Module::new();
        for i in 0..module.len() {
            let mut file = module.by_index(i).unwrap();

            let mut buff = Vec::new();
            file.read_to_end(&mut buff).unwrap();

            let file_name = Path::new(file.name());

            let conf_extension = Some(std::ffi::OsStr::new("json"));
            if file_name.starts_with("species") && file_name.extension() == conf_extension {
                let res: SpeciesConf = parse_json(&buff).unwrap();
                modu.set_species(res.name.clone(), res);
            } else if file_name.starts_with("tiles") && file_name.extension() == conf_extension {
                let res: TilesConf = parse_json(&buff).unwrap();
                modu.set_tiles(res);
            } else if file_name.extension() == Some(std::ffi::OsStr::new("png")) {
                let res = Image::from_bytes(&buff);
                match res {
                    Ok(img) => modu.add_image(file_name, img),
                    Err(x) => println!("Failed to load {:?} as image. Error : {:?}", file_name, x),
                }
            }
            println!("name: {}", file.name());
        }
        modu
    })
}

pub fn load_everything() -> impl Future<Item = ModulesContainer, Error = Error> {
    let mut container: ModulesContainer = Default::default();
    get_all_mod_paths()
        .and_then(|v| join_all(v.iter().map(|x| load_mod_info(x)).collect::<Vec<_>>()))
        .map(|mut v| {
            v.drain(0..v.len()).for_each(|x| container.add_module(x));
            container
        })
}
