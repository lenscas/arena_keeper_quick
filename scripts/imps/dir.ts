import { assetDir } from "./paths";
import { mkdirSync } from "fs";
import {join} from "path"

export const safeDirCreation = (dirLoc : string) => {
    let last = assetDir
    dirLoc.split("/").forEach(v=>{
        last = join(last, v)
        try{
            mkdirSync(last)
        } catch(e){
            if(e.code == "EEXIST"){
                console.error("Skipping :  "+ last+ ". Already exists.")
            } else {
                throw(e)
            }
        }
    })
}