#![allow(unreachable_code)]
fn main() {

    let mut count = 0u32;

    println!("Let's count until infinity!");
    //Infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("three");
            continue;
        }
        println!("{}", count);

        if count == 5 {
            println!("Ok, that's enough");
            
            break;
        }
    }

    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            break 'outer;
        }
        println!("This point will never be reached");
    }
    println!("Exited the outer loop");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("{:?}", result);


}
