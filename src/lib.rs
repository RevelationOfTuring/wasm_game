// wasm_bindgen用于js与rust交互
use wasm_bindgen::prelude::*;
// wee_alloc是一个轻量的wasm内存分配器
use wee_alloc::WeeAlloc;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

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
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize) -> Self {
        Self {
            width,
            size: width * width,
            snake: Snake::new(13),
        }
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
        let head_index = self.snake_head_index();
        self.snake.body[0].0 = (head_index + 1) % self.size;
    }
}

struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
}

impl Snake {
    // 参数：出生点
    fn new(spawn_index: usize) -> Self {
        Self {
            body: vec![SnakeCell(spawn_index)],
        }
    }
}

// 注：每次改完rust代码都要 wasm-pack build生成wasm文件，这样js才可以调用到
