pub mod output_handler {

    pub fn print_data(output: Vec<u16>) {
        println!(); // separate the output
        for v in output {
            println!("{} is open", v);
        }
    }
}


