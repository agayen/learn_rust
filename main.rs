fn main() {
    println!("Hello, world!");
    test_block()
    another_function()

    let x = plus_one(5);
    println!("The value of x is: {x}");
}

fn another_function() {
    println!("Another function.");
}

fn test_block(){
    let y = {
        let x = 3;
        x + 1
    }; // y = 4

    println!("The value of y is: {y}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}