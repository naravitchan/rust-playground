fn hello(s: String) {
    println!("{} 1", s);
}

fn hello2(s: impl Into<String>) {
    println!("{} 2", s.into());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_work() {
        let s = "Hello".to_string();
        hello(s.clone());
        println!("{}", s);

        let s = "Hello".to_string();
        hello2(s.clone());
        println!("{}", s);

        let s = "borrow";
        hello2(s);
        println!("{}", s);
    }
}
