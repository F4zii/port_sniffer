use crate::constants::VALID_FLAGS;

pub mod output_handler {
    pub fn print_data(output: Vec<u16>) {
        println!(); // separate the output
        for v in output {
            println!("{} is open", v);
        }
    }
}


pub mod input_handler {
    use crate::constants::VALID_FLAGS;

    pub fn is_flag_valid(flag: &str) -> bool {
        return VALID_FLAGS.iter().any(|s| *s == flag)
    }
}

