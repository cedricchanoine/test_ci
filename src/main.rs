fn main() {
    println!("hello world");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1(){
        assert!(true);
    }

    #[test]
    fn test2(){
        assert!(true);
    }
}

