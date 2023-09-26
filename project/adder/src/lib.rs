#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {
    fn can_hold(&self, rec: &Rectangle) -> bool {
        self.width > rec.width && self.length > rec.length
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn if_can_hold() {
        let rec1 = Rectangle {
            width: 8,
            length: 7,
        };

        let rec2 = Rectangle {
            width: 6,
            length: 5,
        };

        assert!(rec1.can_hold(&rec2));
    }
}
