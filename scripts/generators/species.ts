import {readdirSync, writeFileSync, readFileSync, copyFileSync} from "fs"
import {speciesFolder, generatedFolder, assetDir} from "../imps/paths"
import * as path from "path"
import { createSpeciesEnum } from "../templates/species/speciesEnum";
import { makeFolder, writeToMod } from "../imps/mod";
import { createCharacterNames, createSpecieImages } from "../templates/species/specieCharacterNames";
import { rawPathToEnum } from "../imps/str";
import { safeDirCreation } from "../imps/dir";
import { Conf } from "../types/conf";
export default (params : string[] ) => {
    const folders = readdirSync(speciesFolder)
    console.log(folders)
    const folder = path.join(generatedFolder,"species")
    const names = folders.map(specieName => {
        const specieFolder = path.join(speciesFolder, specieName)
        const characterNames = readFileSync(path.join(specieFolder,"names"),{encoding:"utf-8"}).split("\n")
        const imageFolder = path.join(specieFolder,"images")
        const config = JSON.parse(readFileSync(path.join(specieFolder,"conf.json"),{encoding:"utf-8"})) as Conf
        if(!config.$schema){
            console.error(`No schema specified for: ${specieName}. "This species may cause problems`)
        }
        const images = readdirSync(imageFolder)
            .filter(V=>V!==".gitkeep")
            .map( v=>({path:path.join(imageFolder,v),name:v}))
            .map(v=> ({
                ...v,
                assetDir : path.join(assetDir,"generated",specieName,v.path.replace(path.join(specieFolder,"images"),""))
            }))
            .map( v=> ({
                ...rawPathToEnum(v.assetDir),
                fullPath : v.path,
                fileName : v.name
            })).filter(v=>v.name!==null)
        images.forEach(v=>{
            let newPath = path.join("generated",specieName)
            safeDirCreation(newPath)
            copyFileSync( v.fullPath,path.join(assetDir,newPath,v.fileName))
        })
        return {
            images: images.map(v=>v.name),
            specie : specieName,
            names : characterNames,
            config
        }
    })
    const enumStr = createSpeciesEnum(names.map(v=>({type:v.specie,config:v.config})))
    makeFolder("species")
    writeToMod(folder,"species")
    writeFileSync(path.join(folder,"species.rs"),enumStr)
    writeToMod(folder,"names")
    const nameStr = createCharacterNames(names)
    writeFileSync(path.join(folder,"names.rs"),nameStr)
    
    writeToMod(folder,"images")
    console.log(names)
    const imageStr = createSpecieImages(names)
    writeFileSync(path.join(folder,"images.rs"),imageStr)
    console.log(imageStr)
    
}