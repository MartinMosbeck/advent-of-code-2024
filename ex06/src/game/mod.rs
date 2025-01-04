use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::playing_field::PlayingField;
use crate::utils::{Position, Direction, UP, direction_after_right_turn};

fn generate_buf_reader(filename: String) -> BufReader<File> {
    let file = File::open(filename).expect("File not openable");

    BufReader::new(file)
}

enum MoveResult {
    HitObstacle,
    LeftField,
    OK,
}

pub struct Game {
    field: PlayingField,
    player_position: Position,
    player_direction: Direction,
    verbosity: bool,
}

impl Game {
    pub fn from_file(filename: String) -> Game {
        println!("Using: {filename}");

        let mut player_position = Position { row: 0, col: 0 };
        let mut elements: Vec<Vec<char>> = Vec::new();
        let reader = generate_buf_reader(filename);
        for (index_row, line) in reader.lines().enumerate() {
            elements.push(line.unwrap().chars().collect());
            if let Some(index_col) = elements[index_row].iter().position(|&c| c == '^') {
                player_position = Position {
                    row: index_row,
                    col: index_col,
                };
            }
        }

        let num_visited: u32 = 1;
        elements[player_position.row][player_position.col] = 'X';

        let (num_rows, num_cols) = (elements.len(), elements[0].len());
        let field = PlayingField::new(
            num_rows,
            num_cols,
            elements,
            num_visited
        );

        Self {
            field,
            player_position,
            player_direction: UP,
            verbosity: false,
        }
    }

    pub fn setup_verbosity(&mut self, verbosity: bool) {
        self.verbosity = verbosity;
    }

    pub fn get_num_visited(&self) -> u32 {
        self.field.get_num_visited()
    }

    pub fn play(&mut self) {
        loop {
            if self.verbosity { self.field.print(); }

            match self.do_linear_move() {
                MoveResult::HitObstacle => self.do_turn_right(),
                MoveResult::LeftField => break,
                MoveResult::OK => panic!("Got OK at do_linear_move"),
            }
        }

        if self.verbosity { self.field.print(); }
    }

    fn do_turn_right(&mut self) {
        self.player_direction = direction_after_right_turn(self.player_direction);
    }

    fn do_linear_move(&mut self) -> MoveResult {
        loop {
            match self.do_step() {
                MoveResult::HitObstacle => return MoveResult::HitObstacle,
                MoveResult::LeftField => return MoveResult::LeftField,
                MoveResult::OK => continue,
            }
        }
    }

    fn do_step(&mut self) -> MoveResult {
        let (new_player_row, new_player_col) = PlayingField::calculate_move(
            &self.player_position,
            &self.player_direction
        );

        if ! self.field.is_valid_position(new_player_row, new_player_col) {
            return MoveResult::LeftField;
        }

        let new_position = Position {
            row: new_player_row as usize,
            col: new_player_col as usize,
        };

        if self.field.is_obstacle(&new_position) {
            return MoveResult::HitObstacle;
        }

        self.field.mark_position_as_visited(&new_position);
        self.player_position = new_position;
        return MoveResult::OK;
    }
}
