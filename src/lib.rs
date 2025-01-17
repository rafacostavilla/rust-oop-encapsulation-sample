pub struct AveragedCollection{
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {

    pub fn new() -> Self{
        AveragedCollection{
            list: vec![],
            average: 0 as f64,
        }
    }

    pub fn add(&mut self, value: i32){
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32>{
        let removed_item = self.list.pop();
        match removed_item {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64{
        self.average
    }

    fn update_average(&mut self){
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}