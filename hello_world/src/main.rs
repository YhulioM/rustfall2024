use std::io::{self, Read, Write};
use std::fs::File;

struct Car {
    model: String,
    year: u32,
}

impl Car{
    fn write_to_file(self: Car) {
        let mut file = File::create("user_info.txt").unwrap();
        writeln!(file,"{}",self.model).unwrap();
        writeln!(file,"{}",self.year.to_string()).unwrap();
    }

    fn from_file(path: &str) -> Car{
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let mut lines = contents.lines();
        let model = lines.next().unwrap().to_string();
        let year = lines.next().unwrap().parse().unwrap();

        Car {model,year}
    }
}

fn reading_from_console() {
    let mut buffer = String::new();

    print!("What's your car model? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let model = buffer.trim().to_string();
    buffer.clear();

    print!("What's the year? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let year = buffer.trim().parse().unwrap();

    let car = Car { model, year };

    car.write_to_file();
    println!("File created sucessfully ");
}

fn reading_from_file(){
    let car = Car::from_file("user_info.txt");
    println!("Car model: {}",car.model);
    println!("Car year: {}", car.year);
}

fn main(){
    reading_from_console();
    reading_from_file();
}