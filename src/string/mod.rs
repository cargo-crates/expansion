#[macro_export]
macro_rules! string {
    ($e:expr) => { 
        $e.to_string()
    }
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn it_works() {
    // }
    #[test]
    fn test_macros() {
      // String
      assert_eq!(string!(String::from("hello")), String::from("hello"));
      // &str
      assert_eq!(string!("hello"), String::from("hello"));
      assert_eq!(string!(&"hello"), String::from("hello"));
      // slice
      assert_eq!(string!("hello"[..]), String::from("hello"));
      assert_eq!(string!(&"hello"[..]), String::from("hello"));
      // char
      assert_eq!(string!("hello".chars().next().unwrap()), String::from("h"));
      // i32 u32 i8 ...
      assert_eq!(string!(123), String::from("123"));
      assert_eq!(string!(-123), String::from("-123"));
      // f32 f64 ...
      assert_eq!(string!(1.23), String::from("1.23"));
      assert_eq!(string!(-1.23), String::from("-1.23"));
    }
}