struct Rect {
    width: i32,
    height: i32,
}

impl Rect {
    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn perimeter(&self, num: i32) -> i32 {
        2 * (self.width + self.height)
    }

    fn debug() -> i32 {
        return 1;
    }
}

fn main() {
    let rect1 = Rect {
        width: 10,
        height: 20,
    };

    println!("area is {}", rect1.area());
    println!("perimeter is {}", rect1.perimeter(1));
    println!("debug is {}", Rect::debug());
}

