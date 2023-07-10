use serde::{Deserialize, Serialize};
use std::{thread, time};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiplyTask {
    pub one: i32,
    pub two: i32,
}

impl MultiplyTask {
    pub fn run(self) -> i32 {
        let duration = time::Duration::from_secs(20);
        thread::sleep(duration);
        return self.one * self.two;
    }
}
