import { assetDir } from "./paths";
import {join, parse} from "path"
import { assetEnum, file } from "../types/files";

export const firstToUpper = (str : string) => str.charAt(0).toUpperCase() + str.substr(1);
export const rawPathToEnum = (path : string) : assetEnum | null => {
    const newPath = path.replace(join(assetDir,"/"),"")
    const ext= parse(newPath).ext
    if(ext !== ".png" && ext !== ".ttf"){
        return null
    }
    
    console.log(assetDir,newPath)
    
    const type = ext===".png" ? "image" : "font"
    const name = newPath.replace(ext,"").split("/").map(firstToUpper).join("").split("_").map(firstToUpper).join("")
    return {
        path : newPath,
        type,
        name
    }
}
export const fileToAssetEnum = (asset : file) => ({
    file : asset,
    enum : rawPathToEnum(asset.path)
})