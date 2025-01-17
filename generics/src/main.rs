#[derive(Debug)]
struct Point<X1, Y1>{
    x: X1,
    y: Y1,
}

impl <X1, Y1> Point <X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2>{
        Point{
            x: self.x,
            y: other.y,
        }
    }
    
}
fn main() {
    let p1= Point { x: 5, y: 10.4 };
    let p2= Point { x: "Hello", y: 'c' };

    println!("{:?}", p1);
    println!("{:?}", p2);
}
