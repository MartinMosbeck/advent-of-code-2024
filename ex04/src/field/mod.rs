use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn generate_buf_reader(filename: String) -> BufReader<File> {
    let file = File::open(filename).expect("File not openable");

    BufReader::new(file)
}

pub struct Field {
    elements: Vec<Vec<char>> ,
    num_rows: usize,
    num_cols: usize,
    verbosity: bool,
}

fn find_in_string(string: &String, regex: &Regex, 
        designation: &str, verbosity: bool) -> usize {
    let mut num_occurrences: usize = 0;

    let mut start = 0; 
    while let Some(mat) = regex.find(&string[start..]) {
        num_occurrences += 1;
        
        if verbosity {
            println!("- In {designation} found match at start position {}", mat.start());
        }
        
        start += mat.start() + 1; 
    }

    num_occurrences
}

impl Field {
    pub fn from_file(filename: String) -> Field{
        println!("Using: {filename}");
        
        let mut elements: Vec<Vec<char>> = Vec::new();
        let reader = generate_buf_reader(filename);
        for line in reader.lines() {
            elements.push(line.unwrap().chars().collect());
        }
        let (num_rows, num_cols) = (elements.len(), elements[0].len());

        Field { verbosity: false, elements, num_rows, num_cols }
    }

    pub fn from_matrix(matrix: Vec<Vec<char>>, size: usize) -> Field {
        Field { 
            verbosity: false,
            elements: matrix,
            num_rows: size,
            num_cols: size
        }
    }

    pub fn setup_find_verbosity(&mut self, verbosity: bool) {
        self.verbosity = verbosity;
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
    }

    fn debug_search_state_output(&self, designation: &str, num_matches: usize) {
        if self.verbosity { 
            println!("{designation} found {num_matches} matches"); 
        }
    }

    pub fn find_occurrences(&self, pattern: &str) -> usize {
        let regex = Regex::new(pattern).unwrap();
        let mut num_occurrences: usize = 0;

        if self.verbosity { self.print(); }

        num_occurrences += self.search_horizontal(&regex);
        num_occurrences += self.search_vertical(&regex);
        num_occurrences += self.search_diagonal(&regex);

        num_occurrences

    }

    fn search_horizontal(&self, regex: &Regex) -> usize {
        let mut num_occurrences: usize = 0;
        let search_designation = "Horizontal";
    
        for (i, row) in self.elements.iter().enumerate() {
            let line: String = row.iter().collect();
            let designation = format!("row {i}");
            num_occurrences += find_in_string(&line, regex, &designation, self.verbosity);
        }

        self.debug_search_state_output(search_designation, num_occurrences);
        num_occurrences
    }

    fn search_vertical(&self, regex: &Regex) -> usize {
        let mut num_occurrences: usize = 0;
        let search_designation = "Vertical";

        for j in 0..self.elements[0].len() {
            let col = self.assemble_col(j);
            let designation = format!("col {j}");
            num_occurrences += find_in_string(&col, regex, &designation, self.verbosity);
        }
        
        self.debug_search_state_output(search_designation, num_occurrences);
        num_occurrences
    }

    fn assemble_col(&self, column_index: usize) -> String{
       let mut col = String::new();
       for i in 0..self.elements.len() {
           col.push(self.elements[i][column_index]);
       }

       col
    }

    // Algorithm Explanation
    // Main Diagonals (Top-left to Bottom-right):
    // - Start from each element in the first row (0, j) where j ranges from 0 to num_cols - 1.
    // - Start from each element in the first column (i, 0) where i ranges from 1 to num_rows - 1 to avoid counting the top-left corner twice.
    // 
    // Anti-Diagonals (Top-right to Bottom-left):
    // - Start from each element in the first row (0, j) where j ranges from 0 to num_cols - 1.
    // - Start from each element in the last column (i, num_cols - 1) where i ranges from 1 to num_rows - 1 to avoid counting the top-right corner twice.
    fn search_diagonal(&self, regex: &Regex) -> usize {
        let mut num_occurrences: usize = 0;
        let search_designation = "Diagonal";

        num_occurrences += self.search_main_diagonals(regex);
        num_occurrences += self.search_anti_diagonals(regex);


        self.debug_search_state_output(search_designation, num_occurrences);
        num_occurrences
    }

    fn search_main_diagonals(&self, regex: &Regex) -> usize {
        let mut num_occurrences: usize = 0;

        for start_col in 0..self.num_cols {
            let diag = self.assemble_main_diag(0, start_col);
            let designation = format!("main diagonal starting at (0, {start_col})");
            num_occurrences += find_in_string(&diag, regex, &designation, self.verbosity);
        }
        for start_row in 1..self.num_rows {
            let designation = format!("main diagonal starting at ({start_row},0)");
            let diag = self.assemble_main_diag(start_row, 0);
            num_occurrences += find_in_string(&diag, regex, &designation, self.verbosity);
        }
        
        num_occurrences
    }

    fn search_anti_diagonals(&self, regex: &Regex) -> usize {
        let mut num_occurrences: usize = 0;

        for start_col in 0..self.num_cols {
            let designation = format!("anti diagonal starting at (0, {start_col})");
            let diag = self.assemble_anti_diag(0, start_col);
            num_occurrences += find_in_string(&diag, regex, &designation, self.verbosity);
        }
        for start_row in 1..self.num_rows {
            let designation = format!("anti diagonal starting at ({}, {})",
                start_row, self.num_cols - 1);
            let diag = self.assemble_anti_diag(start_row, self.num_cols - 1);
            num_occurrences += find_in_string(&diag, regex, &designation, self.verbosity);
        }
        
        num_occurrences
    }

    pub fn assemble_main_diag(&self, start_row: usize, start_col: usize) -> String {
        let mut diag = String::new();
        let mut x = start_row;
        let mut y = start_col;

        while x < self.num_rows && y < self.num_cols {
            diag.push(self.elements[x][y]);
            x += 1;
            y += 1;
        }

        diag
    }

    pub fn assemble_anti_diag(&self, start_row: usize, start_col: usize) -> String {
        let mut diag = String::new();
        let mut x = start_row;
        let mut y = start_col;

        while x < self.num_rows && y < self.num_cols {
            diag.push(self.elements[x][y]);
            x += 1;
            if y == 0 { break; }
            y -= 1;
        }

        diag
    }

    pub fn find_x_mas(&self) -> usize {
        let mut num_occurrences: usize = 0;

        let len_pattern = 3;
        let pattern = r"MAS|SAM";
        let regex = Regex::new(pattern).unwrap();

        
        for i in 0..(self.num_rows - (len_pattern - 1)) {
            for j in 0..(self.num_cols - (len_pattern - 1)) {
                let subfield = Field::from_matrix(self.get_sub_matrix(len_pattern, i, j), len_pattern);

                let main_diagonal = subfield.assemble_main_diag(0, 0);
                let designation = format!("Main diagonal starting ({}, {})", i, j);
                let num_match_main = find_in_string(
                    &main_diagonal, &regex, &designation, self.verbosity
                );

                let anti_diagonal = subfield.assemble_anti_diag(0, 2);
                let designation = format!("Anti diagonal starting ({}, {})", i, j + 2);
                let num_match_anti = find_in_string(
                    &anti_diagonal, &regex, &designation, self.verbosity
                );

                if (num_match_main >= 1) && (num_match_anti >= 1) {
                    num_occurrences += 1;
                }
            }
        }

        num_occurrences
    }
    
    fn get_sub_matrix(&self, size: usize, start_row: usize, 
                    start_col: usize) -> Vec<Vec<char>> {
        let mut subfield: Vec<Vec<char>> = Vec::new();

        for i in start_row..(start_row + size) {
            let mut row: Vec<char> = Vec::new();
            for j in start_col..(start_col + size) {
                row.push(self.elements[i][j]);
            }
            subfield.push(row);
        }
        
        subfield
    }

}
