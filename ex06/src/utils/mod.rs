pub struct Position {
    pub row: usize,
    pub col: usize,
}

pub type Direction = (i8, i8);
pub const UP: Direction = (-1, 0);
pub const DOWN: Direction = (1, 0);
pub const LEFT: Direction = (0, -1);
pub const RIGHT: Direction = (0, 1);

pub fn direction_after_right_turn(direction: Direction) -> Direction {
    if direction == UP {
        RIGHT
    } else if direction == RIGHT {
        DOWN
    } else if direction == DOWN {
        LEFT
    } else if direction == LEFT {
        UP
    } else {
        panic!("Unknown direction")
    }
}
