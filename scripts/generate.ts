const generatorName = process.argv[2]
const otherArgs = process.argv.filter((v,k)=> k > 2)
console.log(generatorName)
const generatorInUse = require("./generators/"+generatorName)
generatorInUse.default()
