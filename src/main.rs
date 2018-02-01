use std::env;
use std::fs::File;
use std::io::Read;

mod plate;
use plate::Plate;

mod machine;
use machine::Machine;

mod operator;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1].clone();
    print_info_text(&format!("Loading program from {}...", file_name));
    let program = read_program(file_name.to_string());
    print_info_text(&String::from("Program successfully loaded"));
    let mut machine = Machine::new(program);
    print_info_text(&String::from("Starting machine..."));
    machine.run();
}

fn print_info_text(str: &str) {
    print_delimiter();
    println!("{}", str);
    print_delimiter();
}

fn print_delimiter() {
    println!("=======================================");
}

fn read_program(file_name: String) -> Vec<Plate> {
    let mut file = File::open(file_name).expect("Cannot open file");
    let mut program : Vec<Plate> = vec![];

    while {
        let mut buffer = [0u8;4];
        let read_bytes = file.read(&mut buffer).expect("Cannot read file");
        if read_bytes > 0 && read_bytes < 4 {
            panic!("Read less then 4 bytes!");
        }
        if read_bytes != 0 {
            let plate = Plate::new(buffer);
            program.push(plate);
        }
        read_bytes != 0
    } {}

    program
}
