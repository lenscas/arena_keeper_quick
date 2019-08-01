import {readdirSync, statSync, writeFileSync, Dirent} from "fs"
import {assetDir,generatedFolder} from "../imps/paths"
import {firstToUpper, fileToAssetEnum} from "../imps/str"
import {template as enumTemplate,makeFile as enumMakeFile} from "../templates/assetEnum"
import {makeFolder, writeToMod} from "../imps/mod"
import * as path from "path"
import { templateImage, makeFile } from "../templates/loadAssets";
import { fileContainer, file } from "../types/files";

export default (params : string[]) => {
	const t = walk(assetDir)
	const g = flatten(t)
	const b = g.map(fileToAssetEnum).filter(v=>v.enum)
	const images = enumTemplate("Images",b.filter(v=>v.enum.type=="image").map(v=>v.enum.name))
	const fonts = enumTemplate("Fonts",b.filter(v=>v.enum.type=="font").map(v=>v.enum.name))
	const fileStr = enumMakeFile([images,fonts])
	const folder = path.join(generatedFolder,"assets")
	makeFolder("assets")
	writeToMod(folder,"loaded")
	writeFileSync(path.join(folder,"loaded.rs"),fileStr)
	writeToMod(folder,"to_load")
	console.log(b)
	const c = templateImage(
		b.map(
			v=>(
				{
					enumName : v.enum.name,
					fileName : v.file.path.replace(path.join(assetDir,"/"),""),
					enumContainer : v.enum.type==="font" ? "Fonts" : "Images",
					into : v.enum.type
				}
			)
		)
	)
	const assetStr = makeFile(c)
	writeFileSync(path.join(folder,"to_load.rs"),assetStr)
	console.log(c)

}



const walk = (paths : string) => {
	let files = readdirSync(paths, { withFileTypes: true })
	let t : fileContainer[] = files
		.map( (v : Dirent | string)=> {
			if(typeof v == "object"){
				v = v.name
			}
			const filePath = path.join(paths,v)
			return {
				isDir : statSync(filePath).isDirectory(),
				name : v,
				path : filePath
			}
		})
		.map(file => {
			if(!file.isDir){
				return {
					type : "file",
					file
				}
			} else {
				return {
					type : "dir",
					files : walk(file.path)
				}
			}
		})
	return t
}
const flatten = (files : fileContainer[]) :file[] => {
	const newFiles : file[] = []
	return files.reduce( (v,f) => {
		if(f.type=="file"){
			v.push(f.file)
			return v
		}
		const b = flatten(f.files)
		return v.concat(b)
	},newFiles)
}