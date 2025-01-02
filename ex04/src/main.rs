pub mod field;
use crate::field::Field;

fn get_filename() -> String {
    if USE_EXAMPLE {
        return String::from("example.txt");
    }
    String::from("input.txt")
}

const USE_EXAMPLE: bool = false;

fn main() {
    let mut field = Field::from_file(get_filename());
    field.setup_find_verbosity( if USE_EXAMPLE { true } else { false } );

    let pattern = r"XMAS|SAMX";
    let num_occurrences = field.find_occurrences(pattern);
    println!("Found XMAS {num_occurrences}");

    println!("------------------------------------------------------------");

    let num_occurrences = field.find_x_mas();
    println!("Found X-MAS {num_occurrences}");
}
