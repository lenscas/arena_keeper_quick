import * as fs from "fs"
import { generatedFolder } from "./paths";
import {join} from "path"

export const writeToMod = (location : string,toAdd : string) => {
	const modFile = location + "/mod.rs"
	const fullToAdd = "pub mod " + toAdd+";\n"
	if (fs.existsSync(modFile)) {
		if(fs.readFileSync(modFile,"utf-8").indexOf(fullToAdd) === -1 ){
			fs.appendFileSync(modFile,fullToAdd)
		}

	} else {
		fs.writeFileSync(modFile,fullToAdd)
	}
}
export const makeFolder =(path : string) => {

	const b = [generatedFolder].concat(path.split("/")).map((v,k)=>({val:v,key:k}))
	const p : Array<{path:string, name?:string}> = []
	const c = b.reduce((v,f)=>{
		let toAdd = ""
		const item = v[v.length-1]
		if(item){
			toAdd = item.path
		}
		let path = join(toAdd,f.val)
		const toPush = {
			path
		}
		if(item){
			toAdd = item.path
		}
		if(b.length > f.key + 1){
			toPush["name"] = b[f.key+1].val
		}
		v.push(toPush)
		return v
	},p)
	c.map(v=>{
		try{
			fs.mkdirSync(v.path)
		}
		catch(e){
			if(e.code == "EEXIST"){
				console.error("Skipping :  "+ v.path+ ". Already exists.")
			} else {
				throw(e)
			}

		}
		if(v.name){
			writeToMod(v.path,v.name)
		}

	})
}