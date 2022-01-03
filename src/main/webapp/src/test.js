const  _ = require("loadsh")


let debouncedFunc = _.throttle((x)=>{
    console.log(x)
},1000);

for (let i = 0; i < 1111111111111111; i++) {
    debouncedFunc(i)
}