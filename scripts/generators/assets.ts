import {readdirSync, statSync, writeFileSync, Dirent} from "fs"
import {assetDir,generatedFolder} from "../imps/paths"
import {firstToUpper} from "../imps/str"
import {template as enumTemplate,makeFile as enumMakeFile} from "../templates/assetEnum"
import {makeFolder, writeToMod} from "../imps/mod"
import * as path from "path"
import { templateImage, makeFile } from "../templates/loadAssets";

export default (params : string[]) => {
	const t = walk(assetDir)
	const g = flatten(t)
	const b = g.map(v=> {
		v.path = v.path.replace(path.join(assetDir,"/"),"")
		return v
	}).map(v=> (
		{
			...v,
			ext : path.parse(v.path).ext
		}
	)).map(v=>({
		ext: v.ext,
		name : v.path.replace(v.ext,"").split("/").map(v=>firstToUpper(v)).join(""),
		file : v.path
	}))
	const images = enumTemplate("Images",b.filter(v=>v.ext==".png").map(v=>v.name))
	const fonts = enumTemplate("Fonts",b.filter(v=>v.ext==".ttf").map(v=>v.name))
	const fileStr = enumMakeFile([images,fonts])
	const folder = path.join(generatedFolder,"assets")
	makeFolder("assets")
	writeToMod(folder,"loaded")
	writeFileSync(path.join(folder,"loaded.rs"),fileStr)
	writeToMod(folder,"to_load")

	const c = templateImage(
		b.map(
			v=>(
				{
					enumName : v.name,
					fileName : v.file,
					enumContainer : v.ext==".ttf" ? "Fonts" : "Images",
					into : v.ext==".ttf" ? "font" : "image"
				}
			)
		)
	)
	const assetStr = makeFile(c)
	writeFileSync(path.join(folder,"to_load.rs"),assetStr)
	console.log(c)

}

type file = {
	isDir : boolean,
	name : string,
	path : string
}

type fileContainer = {
	type : "file",
	file : file
} | {
	type : "dir",
	files : Array<fileContainer>
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