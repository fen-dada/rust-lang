pub trait Messager {
    fn send(&self, msg: &str);
}

pub struct Tracker<'a, T: Messager> {
    message: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> Tracker<'a, T>
where
    T: Messager,
{
    pub fn new(message: &'a T, max: usize) -> Tracker<'a, T> {
        Tracker {
            message,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.message.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.message
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.message
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::cell::RefCell;
    struct MockMessage {
        sent_message: RefCell<Vec<String>>,
    }

    impl MockMessage {
        fn new() -> MockMessage {
            MockMessage {
                sent_message: RefCell::new(vec![]),
            }
        }
    }

    impl Messager for MockMessage {
        fn send(&self, message: &str) {
            self.sent_message.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mockmessage = MockMessage::new();

        let limit = Tracker::new(&mockmessage, 100);

        limit.set_value(80);

        assert_eq!(mockmessage.sent_message.borrow().len(), 1);
    }
}
