import init, { World, Direction, GameStatus } from "wasm_game";
import { getRand } from './utils/random';

// 必须先init，然后在其回调中调用wasm中定义的函数hello
init().then(wasm => {
  const fps = 2 // fps即每秒帧数，即蛇头的移动速度
  const CELL_SIZE = 30; // 一个小正方形的格的边长
  const WORLD_WIDTH = 4; // 画布每边有16个小方格
  const SNAKE_SPWAN_INDEX = getRand(WORLD_WIDTH * WORLD_WIDTH); // 蛇的出生位置随机
  const world = World.new(WORLD_WIDTH, SNAKE_SPWAN_INDEX);
  const worldWidth = world.width();

  // 
  const gameStatus = document.getElementById("game-status");
  const gameControlButton = document.getElementById("game-control-button");

  const canvas = <HTMLCanvasElement>document.getElementById("snake-world"); // <HTMLCanvasElement>为ts语法
  const context = canvas.getContext("2d");
  canvas.width = worldWidth * CELL_SIZE;
  canvas.height = worldWidth * CELL_SIZE;


  // 给按钮添加点击事件
  gameControlButton.addEventListener("click", () => {
    const status = world.game_status();
    if (status == undefined) {
      // rust中的None就是js中的undefined
      gameControlButton.textContent = "In the game ...";
      world.start_game();
      run();
    } else {
      // 重新刷新当前页面
      location.reload();
    }
  })

  // 监听键盘发出的事件
  document.addEventListener("keydown", e => {
    switch (e.code) {
      case "ArrowUp":
        world.change_snake_direction(Direction.Up);
        break;
      case "ArrowDown":
        world.change_snake_direction(Direction.Down);
        break;
      case "ArrowLeft":
        world.change_snake_direction(Direction.Left);
        break;
      case "ArrowRight":
        world.change_snake_direction(Direction.Right);
        break;
    }
  })

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
    // 获取蛇身子的坐标数组
    const snakeCells = new Uint32Array(
      wasm.memory.buffer, // wasm里面自带的memory buffer
      world.snake_cells(), // wasm中蛇身子数组的起始指针
      world.snake_length()  // wasm中蛇身子数组的长度
    );


    context.beginPath();
    const snakeHead = snakeCells[0];

    // cellIndex为迭代出的蛇身的每一个坐标，i为该元素在wasm蛇身坐标数组中的索引
    snakeCells.filter((cellIndex, i) =>
      // 当蛇头撞到蛇身上时，不要在蛇头位置再渲染蛇身
      !(i > 0 && cellIndex == snakeHead)
    ).forEach((cellIndex, i) => {
      // x坐标
      const row = cellIndex % worldWidth;
      // y坐标
      const col = Math.floor(cellIndex / worldWidth);

      // 设置蛇的颜色（头是灰色，身子是黑色）
      context.fillStyle = i !== 0 ? "#000000" : "#787878";
      // 画蛇身矩形
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
    })

    context.stroke();
  }

  // 画蛋
  function drawReward() {
    const index = world.reward_cell();
    // if (index === 123456789) {
    //   // 如果单的index为123456789，表示蛇身子已经占满了全部格子，宣告胜利
    //   alert("Win the game!");
    // }
    // 蛋在第几列（即x坐标）
    const row = index % worldWidth;

    // 蛋在第几行（即y坐标）
    const col = Math.floor(index / worldWidth);

    context.beginPath();
    // 设置蛋的颜色（红）
    context.fillStyle = "#FF0000";
    // 画蛋矩形
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

  function displayGameStatusInfo() {
    gameStatus.textContent = world.game_status_info();
  }

  function drawWorldAndSnake() {
    drawWorld();
    drawSnake();
    drawReward();
    displayGameStatusInfo();
  }

  function run() {
    const status = world.game_status();
    // gameControlButton.textContent = world.snake_length().toString();
    if (status === GameStatus.Won || status === GameStatus.Lost) {
      // 如果游戏已经失败或成功，就停止游戏
      gameControlButton.textContent = "play again?";
      return;
    }

    setTimeout(() => {
      // 清理画布和蛇
      context.clearRect(
        0,
        0,
        canvas.width,
        canvas.height
      );

      // 更新蛇头
      world.update();
      // 重新画画布和蛇
      drawWorldAndSnake();
      // 调用一个callback，即调用自己run函数
      requestAnimationFrame(run);
    }, 1000 / fps);// 1000/fps milliseconds刷新一次
  }

  drawWorldAndSnake();
})