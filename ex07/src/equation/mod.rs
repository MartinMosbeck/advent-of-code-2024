pub struct Equation {
    result: u64,
    operands: Vec<u64>,
    num_operators: usize,
}

pub enum SolutionResult {
    SolveAble(u64),
    NonSolveAble
}

impl SolutionResult {
    pub fn to_string(&self) -> &str {
        match self {
            SolutionResult::SolveAble(_) => "==",
            SolutionResult::NonSolveAble => "!=",
        }
    }
}

#[derive(Clone, Debug, Copy)]
enum Operator {
    Plus,
    Multiply,
}

impl Operator {
    pub fn to_string(&self) -> &str {
        match self {
            Operator::Plus => "+",
            Operator::Multiply => "*",
        }
    }
}

impl Equation {
    pub fn from_numbers(numbers: Vec<u64>) -> Equation {
        let mut result: u64 = 0;
        let mut operands: Vec<u64> = Vec::new();

        for (i, number) in numbers.iter().enumerate() {
            if i == 0 {
                result = *number;
            } else {
                operands.push(*number);
            }
        }

        let num_operators = operands.len() - 1;

        Equation { result, operands, num_operators }
    }

    pub fn _print(&self) {
        let operands = self.operands.iter()
            .map(|num| num.to_string())
            .collect::<Vec<String>>()
            .join(" ");

        println!("{}: {} ", self.result, operands);
    }

    fn print_calculation(&self, operators:&Vec<Operator>, solvable: &SolutionResult) {
        print!("{} {} {}",
            self.result,
            solvable.to_string(),
            self.operands[0]
        );

        for (i, operator) in operators.iter().enumerate() {
            print!(" {} {}",
                operator.to_string(),
                self.operands[i + 1]
            );
        }
        println!("");
    }

    fn generate_operator_combinations(size: usize) -> Vec<Vec<Operator>> {
        let num_combinations = 1 << size; // 2^size

        (0..num_combinations)
            .map(|i| {
                (0..size)
                    .map(|j| {
                        if (i & (1 << j)) != 0 {
                            Operator::Multiply
                        } else {
                            Operator::Plus
                        }
                    })
                    .collect()
            })
            .collect()
    }

    pub fn try_solve(&self, verbose: bool) -> SolutionResult {
        let candidates = Self::generate_operator_combinations(self.num_operators);

        for candidate in &candidates {
            if self.is_solution(candidate) {
                if verbose {
                    self.print_calculation(
                        candidate,
                        &SolutionResult::SolveAble(self.result)
                    );
                }

                return SolutionResult::SolveAble(self.result)
            }
        }
        SolutionResult::NonSolveAble
    }

    fn is_solution(&self, operators:&Vec<Operator>) -> bool {
        let mut result_calculation: u64 = self.operands[0];

        for (i, operand) in self.operands[1..].iter().enumerate() {
            match operators[i] {
                Operator::Plus => result_calculation += *operand,
                Operator::Multiply => result_calculation *= *operand,
            }
        }

        result_calculation == self.result as u64
    }
}
