fn main() {
    println!("Hello, world!");

    let x = five();
    another_function(x);
}

fn five() -> i32 { 
    5
}

fn another_function(x : i32) {
    println!("Another function printing {}.", x);
}

