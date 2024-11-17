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
    // 下一个cell（用于 提高性能）
    next_cell: Option<SnakeCell>,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, spawn_index: usize) -> Self {
        let size = width * width;
        // spawn_index为蛇头坐标，3表示蛇的长度，即2个身体+1个头
        let snake = Snake::new(spawn_index, 3);
        Self {
            width,
            size,
            reward_cell: Self::gen_reward_cell(size, &snake.body),
            snake,
            next_cell: None,
        }
    }

    fn gen_reward_cell(max: usize, body: &Vec<SnakeCell>) -> usize {
        loop {
            // 使用js中提供的getRand()函数
            let reward_cell = getRand(max);
            // 要求刚开始的蛋不能在蛇身子上
            if !body.contains(&SnakeCell(reward_cell)) {
                return reward_cell;
            }
        }
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
        // let mut head_index = self.snake_head_index();
        // let (col, row) = self.index_to_cell(head_index);
        // // 获得移动后的行列新坐标
        // let (col, row) = match self.snake.direction {
        //     Direction::Left => ((col - 1) % self.width, row),
        //     Direction::Right => ((col + 1) % self.width, row),
        //     Direction::Down => (col, (row + 1) % self.width),
        //     Direction::Up => (col, (row - 1) % self.width),
        // };

        // // 由新坐标获得蛇头的index
        // head_index = self.cell_to_index(col, row);
        // // 设置蛇头的新index
        // self.set_snake_head(head_index);

        // 复制当前的蛇坐标数组
        let temp_body = self.snake.body.clone();
        match &self.next_cell {
            Some(next_cell) => {
                // 如果self.next_cell存在，说明之前刚刚触发过change_snake_direction
                // 设置蛇头为下一个cell
                self.snake.body[0] = next_cell.clone();
                // 清空self.next_cell
                self.next_cell = None;
            }
            None => {
                // 如果self.next_cell不存在，说明之前没有触发过change_snake_direction
                // 获得下一个cell
                self.snake.body[0] = self.gen_snake_next_cell(&self.snake.direction);
            }
        };

        let len: usize = self.snake_length();
        for i in 1..len {
            // 将原来身子中[0,n-2]的元素复制到现在的蛇头后面
            self.snake.body[i] = SnakeCell(temp_body[i - 1].0);
        }

        // 如果蛇头吃到蛋
        if self.reward_cell == self.snake_head_index() {
            if self.snake_length() < self.size {
                // 更新新的蛋(要求此时蛇的长度不能将整个size填满)
                self.reward_cell = Self::gen_reward_cell(self.size, &self.snake.body);
            } else {
                // 如果此时蛇已经将size填满，我们给前端一个常量123456789作为游戏胜利的提示
                self.reward_cell = 123456789;
            }
            // 蛇吃到果子，将原先自身最后一个cell也加入到body中，这样实现了身长的增加
            self.snake.body.push(SnakeCell(temp_body[len - 1].0));
        }
    }

    fn gen_snake_next_cell(&self, direction: &Direction) -> SnakeCell {
        let snake_head_index = self.snake_head_index();
        let row = snake_head_index / self.width;
        return match direction {
            Direction::Up => {
                // 如果是向上的话
                // 计算当前蛇头位置对应的上边界的点坐标
                let up_border_index = snake_head_index - row * self.width;
                if up_border_index == snake_head_index {
                    // 如果当前蛇头就在上边缘，那么下次应该从同一列的最下面出来
                    SnakeCell(self.size - self.width + up_border_index)
                } else {
                    // 如果当前蛇头不在上边缘，蛇头向上走一格
                    SnakeCell(snake_head_index - self.width)
                }
            }
            Direction::Down => {
                // 如果是向下的话
                // 计算当前蛇头位置对应的下边界的点坐标
                let down_border_index = snake_head_index + (self.width - row - 1) * self.width;
                if down_border_index == snake_head_index {
                    // 如果当前蛇头就在下边缘，那么下次应该从同一列的最上面出来
                    SnakeCell(self.width - (self.size - down_border_index))
                } else {
                    // 如果当前蛇头不在下边缘，蛇头向下走一格
                    SnakeCell(snake_head_index + self.width)
                }
            }
            Direction::Left => {
                // 如果是向左的话
                // 计算当前蛇头位置对应的左边界的点坐标
                let left_border_index = row * self.width;
                if left_border_index == snake_head_index {
                    // 如果当前蛇头就在左边缘，那么下次应该从同一列的最右面出来
                    SnakeCell(left_border_index + self.width - 1)
                } else {
                    // 如果当前蛇头不在左边缘，蛇头向左走一格
                    SnakeCell(snake_head_index - 1)
                }
            }
            Direction::Right => {
                // 如果是向右的话
                // 计算当前蛇头位置对应的右边界的点坐标
                let right_border_index = (row + 1) * self.width - 1;
                if right_border_index == snake_head_index {
                    // 如果当前蛇头就在右边缘，那么下次应该从同一列的最左面出来
                    SnakeCell(row * self.width)
                } else {
                    // 如果当前蛇头不在右边缘，蛇头向右走一格
                    SnakeCell(snake_head_index + 1)
                }
            }
        };
    }

    // fn set_snake_head(&mut self, index: usize) {
    //     self.snake.body[0].0 = index;
    // }

    // 改变蛇头的移动方向
    pub fn change_snake_direction(&mut self, direction: Direction) {
        // 如果当前的方向是a，不可立刻改变为a相反的方向
        let next_cell = self.gen_snake_next_cell(&direction);
        if next_cell == self.snake.body[1] {
            return;
        }

        self.snake.direction = direction;
        // 记录下一个位置的cell，为了提高wasm的性能（再执行update时，并不需要重新调用gen_snake_next_cell()计算）
        self.next_cell = Some(next_cell);
    }

    // // 传入一个蛇头的index返回其在画布中的行列坐标
    // fn index_to_cell(&self, index: usize) -> (usize, usize) {
    //     (index % self.width, index / self.width)
    // }

    // // 传入一个蛇头在画布中的行列坐标返回其index
    // fn cell_to_index(&self, col: usize, row: usize) -> usize {
    //     row * self.width + col
    // }

    // 返回一个SnakeCell的rust原生指针
    pub fn snake_cells(&self) -> *const SnakeCell {
        self.snake.body.as_ptr()
    }

    // 返回蛇身的长度
    pub fn snake_length(&self) -> usize {
        self.snake.body.len()
    }
}

#[derive(Clone, PartialEq)]
struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}

impl Snake {
    // 参数：出生点
    fn new(spawn_index: usize, size: usize) -> Self {
        let mut body = Vec::new();
        for i in 0..size {
            body.push(SnakeCell(spawn_index - i));
        }
        Self {
            body,
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
