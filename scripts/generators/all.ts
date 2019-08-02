export default (params : string[]) => {
    console.log("generating species")
    require("./species").default([]);
    console.log("generating assets")
    require("./assets").default([]);
    
}