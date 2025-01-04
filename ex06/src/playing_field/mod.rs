use crate::utils::{Position, Direction};

pub struct PlayingField {
    num_rows: usize,
    num_cols: usize,
    elements: Vec<Vec<char>>,
    num_visited: u32,
}

impl PlayingField {
    pub fn new(num_rows: usize, num_cols: usize, elements: Vec<Vec<char>>,
            num_visited: u32) -> PlayingField {
        Self {
            num_rows,
            num_cols,
            elements,
            num_visited,
        }
    }

    pub fn print(&self) {
        print!("  ");
        for i in 0..self.num_cols {
            print!("{i}");
        }

        println!("");
        for (i, line) in self.elements.iter().enumerate() {
            let string: String = line.iter().collect();
            println!("{i} {string}");
        }
        println!("");
    }

    pub fn is_obstacle(&self, position: &Position) -> bool {
        return self.elements[position.row][position.col] == '#';
    }

    pub fn mark_position_as_visited(&mut self, position: &Position) {
        if self.elements[position.row][position.col] != 'X' {
            self.elements[position.row][position.col] = 'X';
            self.num_visited += 1;
        }
    }

    pub fn is_valid_position(&self, row: isize, col: isize) -> bool {
        if row < 0 || row > self.num_rows as isize - 1 ||
           col < 0 || col > self.num_rows as isize - 1 {
            return false;
        }

        return true;
    }

    pub fn calculate_move(position: &Position, step: &Direction) -> (isize, isize) {
        return (
            position.row as isize + step.0 as isize,
            position.col as isize + step.1 as isize
        )
    }

    pub fn get_num_visited(&self) -> u32 {
        self.num_visited
    }
}
