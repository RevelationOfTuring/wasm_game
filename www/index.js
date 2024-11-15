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
// import init, { hello } from "wasm_game";

// // 必须先init，然后在其回调中调用wasm中定义的函数hello
// init().then(() => {
//   hello("Michael Wang");
//   console.log("Ok");
// })

import init, { World } from "wasm_game";

// 必须先init，然后在其回调中调用wasm中定义的函数hello
init().then(() => {
  const fps = 10 // fps即每秒帧数，即蛇头的移动速度
  const CELL_SIZE = 20; // 一个小正方形的格的边长
  const world = World.new(16); // 画布每边有16个小方格
  const worldWidth = world.width();

  const canvas = document.getElementById("snake-world");
  const context = canvas.getContext("2d");
  canvas.width = worldWidth * CELL_SIZE;
  canvas.height = worldWidth * CELL_SIZE;

  // 绘制画布
  function drawWorld() {
    context.beginPath();

    // 绘制画布的竖线
    for (let x = 0; x < worldWidth + 1; x++) {
      context.moveTo(x * CELL_SIZE, 0);
      context.lineTo(x * CELL_SIZE, CELL_SIZE * worldWidth);
    }

    // 绘制画布的横线
    for (let y = 0; y < worldWidth + 1; y++) {
      context.moveTo(0, y * CELL_SIZE);
      context.lineTo(CELL_SIZE * worldWidth, y * CELL_SIZE);
    }
    context.stroke();
  }

  function drawSnake() {
    const snake_index = world.snake_head_index();
    // 蛇头在第几列（即x坐标）
    const row = snake_index % worldWidth;

    // 蛇头在第几行（即y坐标）
    const col = Math.floor(snake_index / worldWidth);


    context.beginPath();
    // 画蛇头矩形
    context.fillRect(
      // 起点横坐标
      row * CELL_SIZE,
      // 起点纵坐标
      col * CELL_SIZE,
      // 矩形横长
      CELL_SIZE,
      // 矩形竖高
      CELL_SIZE,
    );
    context.stroke();
  }

  function drawWorldAndSnake() {
    drawWorld();
    drawSnake();
  }

  function run() {
    setTimeout(() => {
      // 清理画布和蛇
      context.clearRect(
        0,
        0,
        canvas.width,
        canvas.height
      );

      // 更新蛇身子
      world.update();
      // 重新画画布和蛇
      drawWorldAndSnake();
      // 调用一个callback，即调用自己run函数
      requestAnimationFrame(run);
    }, 1000 / fps);// 1000/fps milliseconds刷新一次
  }

  drawWorldAndSnake();
  run();
})