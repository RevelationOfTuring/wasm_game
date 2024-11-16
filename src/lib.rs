// wasm_bindgen用于js与rust交互
use wasm_bindgen::prelude::*;
// wee_alloc是一个轻量的wasm内存分配器
use wee_alloc::WeeAlloc;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

// 使用js提供的生成随机数的方法getRand(max)，module后面的字符串为getRand所在js的module路径
#[wasm_bindgen(module = "/www/utils/random.js")]
extern "C" {
    fn getRand(max: usize) -> usize;
}
// 从外部将js的alert方法import进来，这样wasm就可以在内部使用alert方法了
#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen] // 被wasm_bindgen修饰的方法才会被编译器认作是用于wasm的
pub fn hello(name: &str) {
    // alert为js的方法
    alert(name);
}

#[wasm_bindgen]
struct World {
    // 画布宽度
    width: usize,
    // 画布里有多少个格子
    size: usize,
    // 内嵌的蛇
    snake: Snake,
    // 蛋的坐标
    reward_cell: usize,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, spawn_index: usize) -> Self {
        let size = width * width;
        Self {
            width,
            size,
            snake: Snake::new(spawn_index),
            reward_cell: Self::gen_reward_cell(size),
        }
    }

    fn gen_reward_cell(max: usize) -> usize {
        // 使用js中提供的getRand()函数
        getRand(max)
    }

    pub fn reward_cell(&self) -> usize {
        self.reward_cell
    }

    pub fn width(&self) -> usize {
        self.width
    }

    // 蛇头的坐标
    pub fn snake_head_index(&self) -> usize {
        self.snake.body[0].0
    }

    // 更新蛇头位置
    pub fn update(&mut self) {
        let mut head_index = self.snake_head_index();
        let (col, row) = self.index_to_cell(head_index);
        // 获得移动后的行列新坐标
        let (col, row) = match self.snake.direction {
            Direction::Left => ((col - 1) % self.width, row),
            Direction::Right => ((col + 1) % self.width, row),
            Direction::Down => (col, (row + 1) % self.width),
            Direction::Up => (col, (row - 1) % self.width),
        };

        // 由新坐标获得蛇头的index
        head_index = self.cell_to_index(col, row);
        // 设置蛇头的新index
        self.set_snake_head(head_index);
    }

    fn set_snake_head(&mut self, index: usize) {
        self.snake.body[0].0 = index;
    }

    // 改变蛇头的移动方向
    pub fn change_snake_direction(&mut self, direction: Direction) {
        self.snake.direction = direction;
    }

    // 传入一个蛇头的index返回其在画布中的行列坐标
    fn index_to_cell(&self, index: usize) -> (usize, usize) {
        (index % self.width, index / self.width)
    }

    // 传入一个蛇头在画布中的行列坐标返回其index
    fn cell_to_index(&self, col: usize, row: usize) -> usize {
        row * self.width + col
    }
}

struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}

impl Snake {
    // 参数：出生点
    fn new(spawn_index: usize) -> Self {
        Self {
            body: vec![SnakeCell(spawn_index)],
            direction: Direction::Down, // 默认一开始向下
        }
    }
}

// 方向是js传进来的，所以要用wasm_bingen修饰
#[wasm_bindgen]
#[derive(PartialEq)] // 因为需要比较判断方向，所以要derive PartialEq
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// 注：每次改完rust代码都要 wasm-pack build生成wasm文件，这样js才可以调用到
