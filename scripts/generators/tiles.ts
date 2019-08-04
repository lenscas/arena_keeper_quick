import { readdirSync, writeFileSync, readFile, readFileSync } from "fs";
import { tileDir, generatedFolder, schemaFolder } from "../imps/paths";
import { tileEnum } from "../templates/tiles";
import { writeToMod } from "../imps/mod";
import { join } from "path";
import { firstToUpper } from "../imps/str";

export default (params : string[])=> {
    const tiles = readdirSync(join(tileDir,"images")).map(v=>v.replace(".png",""));
    const config = JSON.parse(readFileSync(join(tileDir,"tiles.json"),"utf-8"))
    const enums = tileEnum({images:tiles,conf:config})
    writeToMod(generatedFolder,"tiles")
    writeFileSync(join(generatedFolder,"tiles.rs"),enums,{encoding:"utf-8"})
    const schemaFile = join(schemaFolder,"conf.schema.json")
    const schema = JSON.parse(readFileSync(schemaFile,{encoding:"utf-8"}))
    schema.properties.speeds.properties = tiles.map(firstToUpper).reduce((v,f)=>{v[f] = {type:"number"}; return v},{})
    writeFileSync(schemaFile,JSON.stringify(schema,null,"  "))
    const tileSchemaFile = join(schemaFolder,"tiles.schema.json")
    const tileSchema = JSON.parse(readFileSync(tileSchemaFile,"utf-8"))
    tileSchema.properties.generateChances.properties = tiles
        .map(firstToUpper)
        .reduce(
            (v,f)=>{
                v[f] = {
                    type : "object",
                    additionalProperties : false,
                    properties : {
                        start : {
                            type : "number"
                        },
                        end : {
                            type : "number"
                        }
                    }
                }
                return v
            },
            {}
        )
    writeFileSync(tileSchemaFile,JSON.stringify(tileSchema,null,"  "))
    console.log("test:",schema.properties.speeds)
}