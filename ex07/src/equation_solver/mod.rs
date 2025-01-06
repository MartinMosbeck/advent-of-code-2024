use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::equation::Equation;
use crate::equation::SolutionResult;

fn generate_buf_reader(filename: String) -> BufReader<File> {
    let file = File::open(filename).expect("File not openable");

    BufReader::new(file)
}

pub struct EquationSolver {
    equations: Vec<Equation>,
    verbosity: bool,
    num_solved: u32,
    sum_solved: u64,
}

impl EquationSolver {

    fn line_to_vec_u32(line: &String) -> Vec<u64> {
        let numbers: Vec<u64> = line
            .split_whitespace()                      
            .map(|s| s.replace(":", ""))            
            .map(|s| s.parse::<u64>()
            .map_err(|e| println!("Failed to parse '{}': {:?}", s, e)))
            .filter_map(Result::ok)                
            .collect();                           

        numbers
    }

    pub fn setup_verbosity(&mut self, verbosity: bool) {
        self.verbosity = verbosity;
    }

    pub fn get_num_solved(&self) -> u32 {
        self.num_solved
    }

    pub fn get_sum_solved(&self) -> u64 {
        self.sum_solved
    }

    pub fn from_file(filename: String) -> EquationSolver {
        println!("Using: {filename}");

        let mut equations: Vec<Equation> = Vec::new();

        let reader = generate_buf_reader(filename);
        for line in reader.lines() {
            let line = line.unwrap();
            let numbers = EquationSolver::line_to_vec_u32(&line);
            let equation = Equation::from_numbers(numbers);
            equations.push(equation);
        }

        EquationSolver { 
            equations, 
            verbosity: false,
            num_solved: 0,
            sum_solved: 0,
        }
    }

    pub fn solve(&mut self) {
        for equation in &self.equations {
            match equation.try_solve(self.verbosity) {
                SolutionResult::SolveAble(result) => {
                    self.num_solved += 1;
                    self.sum_solved += result;
                },
                SolutionResult::NonSolveAble => (),
            }
        }
    }
}
