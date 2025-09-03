fn main() {
    let x = 5;
    println!("The value of x:{x}");
    let x = x + 1;
    println!("New value of x:{x}");
    {
        let x = x * 2;
        println!("Block value of x:{x}");
    }
    println!("last value of x:{x}");
}
