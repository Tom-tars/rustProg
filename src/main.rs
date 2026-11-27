use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Rust");
    let mut arr: Vec<usize> = vec![];
    for i in 0..=20usize {
        arr.push(i.pow(4));
        println!("{}", arr[i].to_string());
    }
    let x = || println!("arr {:?}", arr);
    x();
    let point = Point::new(5f64, 3f64);
    println!("{}", point.d_to_o());
    let line = Line::new(3f64, 4f64, 5f64);
    println!("{}", line.d_to_p(&point));
    println!("{}", 5f64.sqrt());
    let file: String = fs::read_to_string("./src/main.rs")?;
    let all: Vec<&str> = file.matches("main").collect();
    println!("{:?}", all);
    Ok(())
}

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
    fn d_to_o(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}
 
struct Line {
    a: f64, // ← 改成 f64
    b: f64,
    c: f64,
}

impl Line {
    fn new(a: f64, b: f64, c: f64) -> Self {
        Line { a, b, c }
    }
    fn d_to_p(&self, &Point { x, y }: &Point) -> f64 {
        (self.a * x + self.b * y + self.c).abs() / (self.a * self.a + self.b * self.b).sqrt()
    }
}
