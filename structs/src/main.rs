#[derive(Debug)]
struct Rect(u32, u32);

impl Rect {
    fn area(&self) -> u32 {
        return self.0 * self.1;
    }
}

fn main() {

    let x = Rect(10, 20);
    
    println!("Hello, world! Area is {}", x.area());

    println!("Rect is {:?}", x);
}
