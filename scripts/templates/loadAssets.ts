import { firstToUpper } from "../imps/str";

export const makeFile = (toExecute : string) => `
use quicksilver::Future;
use quicksilver::graphics::Image;
use super::loaded::AssetManager;
use quicksilver::Error;
use quicksilver::graphics::Font;
use super::loaded::Images;
use super::loaded::Fonts;

pub fn load_all() -> Box<Future<Item=AssetManager, Error=Error>>{
	let mut manager = AssetManager::new();
	Box::new(
		${toExecute}
	)
}
`
export const templateImage = (files: Array<{enumName : string, fileName : string,into : string, enumContainer: string}>) => {
	return files.map( (image,k) => {
		let basic = `
${firstToUpper(image.into)}::load("${image.fileName}").and_then(|v| {
	manager.insert_${image.into}(${image.enumContainer}::${image.enumName},v);
	Ok(manager)
})`
	if(k < files.length-1) {
		console.log("in here?")
		if(k==0){
			basic = basic + `.and_then(|mut manager|{`
		} else {
			basic = basic + `}).and_then(|mut manager|{`
		}
	}
	return basic
	}).join("") + "})"
}
/*
		Image::load("Human.png").and_then(|v|{
			manager.insert_image(Images::Human,v);
			Ok(manager)
		}).and_then(|mut manager| {
			Font::load("font.ttf").and_then(|v|{
				manager.insert_font(Fonts::BasicFont,v);
				Ok(manager)
			})
		})
*/