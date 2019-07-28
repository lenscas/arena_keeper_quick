import {normalize} from "path"
export const currentFolder = __dirname.split("/")
export const appendFile = (...args: string[]) => normalize(currentFolder.map(v=>v).join("/")+"/"+args.join("/"))
export const generatedFolder = appendFile("..","..","src","generated")
export const assetDir = (appendFile("..", "..","static"))