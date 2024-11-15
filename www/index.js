// 注：从https://webassembly.github.io/wabt/demo/wat2wasm/下载wasm文件
// test1.wasm的wat文件如下:
/* 
   (module
      (import "console" "log" (func $func1))
      (import "console" "error" (func $func2))
        (func (export "myfunc") (param i32 i32) (result i32)
          local.get 0
          local.get 1
          call $func1
          call $func2
           i32.add))
*/
// async function run() {
//     const importObject = {
//         console: {
//             log: () => {
//                 console.log("log info!");
//             },
//             error: () => {
//                 console.log("error info!");
//             }
//         }
//     }
//     // wasm调用js
//     const response = await fetch("test1.wasm");
//     const buffer = await response.arrayBuffer();
//     // debugger;
//     // 将js中的函数传入到wasm中
//     const wasm = await WebAssembly.instantiate(buffer, importObject);

//     // 从test1.wasm文件中获取方法 myfunc
//     const f = wasm.instance.exports.myfunc;
//     const res = f(100, 200);
//     // $ npm run dev后，打开浏览器输入：http://localhost:8080/，按F12打开console就可以看见30了
//     console.log(res);

//     // // js调用wasm
//     // const response = await fetch("test.wasm");
//     // const buffer = await response.arrayBuffer();
//     // const wasm = await WebAssembly.instantiate(buffer);

//     // // 从wasm文件中获取方法
//     // const addTwoFunction = wasm.instance.exports.addTwo;
//     // const res = addTwoFunction(10, 20);
//     // // $ npm run dev后，打开浏览器输入：http://localhost:8080/，按F12打开console就可以看见30了
//     // console.log(res);
// }

// run();

// init是wasm_game中自带的初始化函数
import init, { hello } from "wasm_game";

// 必须先init，然后在其回调中调用wasm中定义的函数hello
init().then(() => {
  hello("Michael Wang");
  console.log("Ok");
})