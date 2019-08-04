import {normalize} from "path"
export const currentFolder = __dirname.split("/")
export const appendFile = (...args: string[]) => normalize(currentFolder.map(v=>v).join("/")+"/"+args.join("/"))
export const generatedFolder = appendFile("..","..","src","generated")
export const assetDir = appendFile("..", "..","static")
export const speciesFolder = appendFile("..","..","species")
export const generatedAssetsFolder = appendFile("..", "..","static","generated")
export const tileDir = appendFile("..","..","tiles")
export const schemaFolder = appendFile("..","types")