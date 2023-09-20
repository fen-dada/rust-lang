struct Point<T> {
    x: T,
    y: T,
}
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if largest < item {
            largest = item;
        }
    }

    largest
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        // T doesnt have a copy trait
        &self.x
    }
}

impl Point<f32> {
    fn dis_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt() // temp value cant reference
    }
}

fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let l = largest(&v);

    println!("{}", l);

    let m = vec!['r', 't', 'y', 'v'];

    let ll = largest(&m);

    println!("{}", ll);
}
