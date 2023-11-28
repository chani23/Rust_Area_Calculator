use std::io;
use std::f32::consts::PI;

struct Rectangle {
    width: f32, 
    height: f32,
}

impl Rectangle {
    fn new(width: f32, height: f32) -> Rectangle{
        Rectangle { 
            width, 
            height, 
        }
    }
    fn area(&self) -> f32 {
        self.width * self.height
    }
    fn show(&self) {
        println!("Area of {}x{} is {}", self.width, self.height, self.area());
    }
}

struct Triangle {
    width: f32,
    height: f32,
}

impl Triangle {
    fn new(width: f32, height: f32) -> Triangle{
        Triangle { 
            width, 
            height, 
        }
    }
    fn area(&self) -> f32 {
        self.width * self.height * 0.5
    }
    fn show(&self) {
        println!("Area of Triangle {}x{} is {}", self.width, self.height, self.area());
    }
}

struct Circle{
    radius: f32,
}

impl Circle{
    fn new(radius:f32) -> Circle{
        Circle {
            radius,
         }
    }
    fn area(&self) -> f32{
        let radius = self.radius as f32;
        radius.powf(2.0)* PI
    }
    fn show(&self){
        println!("Area of Circle pi*{}^2 is {}",self.radius,self.area());
    }
}


fn input(ino: &str) -> f32 {
    println!("{}",ino);
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    s.trim().parse().expect("Please input valid number")
}


fn main() {

    loop {
        println!("Welcome to the Area Calculator");
        println!("r: Rectangle // t: Triangle // c: Circle // q: for end");
        let mut ty = String::new();
        io::stdin().read_line(&mut ty)
            .expect("Failed to read line");
        let ty = ty.trim();
        
        match ty {
            "q" => {
                println!("See you later..");
                break; // Exit the loop if 'q' is entered
            }
            "r" => {
                println!("Welclome to the Rectangle calculator");
                let w = input("Write Width:");
                let h = input("Write Height");
                let r = Rectangle::new(w,h);
                r.show();
            }
            "t" => {
                println!("Welclome to the Triangle calculator");
                let w = input("Write Width:");
                let h = input("Write Height");
                let r = Triangle::new(w,h);
                r.show();
            }
            "c" => {
                println!("Welclome to the Circle calculator");
                let l = input("Write Radius:");
                let r = Circle::new(l);
                r.show();
            }
            _ => print!("bye")
        }

        
    }
}