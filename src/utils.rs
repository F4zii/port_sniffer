pub mod output_handler {

    pub fn print_data(output: Vec<u16>) {
        for v in output {
            println!("{} is open", v);
        }
    }
}


