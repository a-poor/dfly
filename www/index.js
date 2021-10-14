import * as wasm from "wasm-dfly";

// wasm.greet();
console.log(`1 + 2 = ${wasm.add(1,2)}`);

const data = [1,2,3,4,5];
console.log(`sum(${data}) = ${wasm.sum(data)}`);

const arr = wasm.Arr1D.new(data);
console.log(arr);
console.log(arr.len());
console.log(arr.sum());
console.log(arr.mean());
console.log(arr.copy());
