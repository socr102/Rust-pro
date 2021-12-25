use std::fmt;

#[derive(Debug)]
struct Circle {
    radius: i32
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius is {}",self.radius)
    }
}


fn main() {
    let circle = Circle {radius: 8};
    println!("{}", circle.to_string());
    println!("{}", circle);

    let parsed : i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();
    println!("{:?}",parsed+turbo_parsed);
}
