use std::fmt;

#[derive(Debug)]
struct MinMax(i64,i64);

//implement 'Display' for 'MinMax'
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    real: f64,
    img: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}",self.real, self.img)
    }
}

struct List(Vec<i32>) ;

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;

        write!(f,"[")?;

        for(count,v) in vec.iter().enumerate() {
            if count!=0 { write!(f,", ")?;}
            write!(f,"{}:{}",count, v)?;
        }
        write!(f, "]")
    }
}

fn main() {
    let minmax = MinMax(0,14);
    println!{"Compare structures."};
    println!("Debug {:?}", minmax);
    println!("Display:{}", minmax);

    let big_range = MinMax(-300,300);
    let small_range = MinMax(-3,3);

    println!("The big range is {big} and  the small range is {small}.",small = small_range, big = big_range);
    let point = Point2D {img: 3.3, real: 7.2};

    println!("Compare points:");
    println!("Display: {} + {}i",point.real, point.img);
    println!("Debug: {:?}", point);

    let v = List(vec![1,2,3]);
    println!("{}",v);
}
