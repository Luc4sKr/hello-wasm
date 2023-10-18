import init, { add, hello_world } from "./pkg/hello_wasm.js";
await init();

console.log(add(2, 5))

const outputElement = document.getElementById("output");
const result = hello_world();
outputElement.textContent = result;