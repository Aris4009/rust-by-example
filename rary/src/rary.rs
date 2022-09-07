#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        public_function();
        assert_eq!(1, 1);
    }

    #[test]
    fn t2() {
        private_function();
        assert_eq!(1, 1);
    }

    #[test]
    fn t3() {
        indirect_access();
        assert_eq!(1, 1);
    }
}

pub fn public_function() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}
