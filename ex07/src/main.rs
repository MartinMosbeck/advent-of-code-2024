mod equation;
mod equation_solver;

use crate::equation_solver::EquationSolver;

fn get_filename() -> String {
    if USE_EXAMPLE {
        return String::from("example.txt");
    }
    String::from("input.txt")
}

const USE_EXAMPLE: bool = true;

fn main() {
    let mut solver = EquationSolver::from_file(get_filename());
    solver.setup_verbosity(if USE_EXAMPLE { true } else { false });
    solver.solve();
    println!("Num solvable: {}", solver.get_num_solved());
    println!("Sum solvable: {}", solver.get_sum_solved());
}
