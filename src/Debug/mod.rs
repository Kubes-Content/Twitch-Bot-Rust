//use std::backtrace::Backtrace;

#[deprecated]
pub fn fail_safely(error_message:&str) {

    //println!("{:?}", Backtrace::new());

    println!("{}", error_message);


    assert!(false);
}