import init, { World } from "wasm_game";

// 必须先init，然后在其回调中调用wasm中定义的函数hello
init().then(() => {
  const fps = 10 // fps即每秒帧数，即蛇头的移动速度
  const CELL_SIZE = 20; // 一个小正方形的格的边长
  const world = World.new(16); // 画布每边有16个小方格
  const worldWidth = world.width();

  const canvas = <HTMLCanvasElement>document.getElementById("snake-world"); // <HTMLCanvasElement>为ts语法
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