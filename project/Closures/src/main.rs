use std::thread;
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_blue > num_red {
            ShirtColor::Blue
        } else {
            ShirtColor::Red
        }
    }
}

fn main() {
    let list = vec![1, 2, 3];
    println!("before definition {:?}", list);

    thread::spawn(move || println!("from thread {:?}", list))
        .join()
        .unwrap();

    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let usr_pre1 = Some(ShirtColor::Blue);
    let giveaway1 = store.giveaway(usr_pre1);

    println!("user1 prefer {:?}, gets {:?}", usr_pre1, giveaway1);

    let usr_pre2 = None;
    let giveaway2 = store.giveaway(usr_pre2);

    println!("user2 prefer {:?}, gets {:?}", usr_pre2, giveaway2);
}
