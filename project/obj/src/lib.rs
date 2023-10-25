pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    fn add(&mut self, value:i32) {
        self.list.push(value);
        self.update_average();
    }

    fn remove(&mut self) -> Option<i32> {
        let item = self.list.pop();

        match item {
            Some(value) => {
                self.update_average();
                Some(value)
            }

            None => None,
        }
    } 

    pub fn average(&self) -> f64 {
        self.average
    } 

    fn update_average(&mut self) {
        let sum: i32 = self.list.iter().sum();

        self.average = sum as f64 / self.list.len() as f64;
    }
}
