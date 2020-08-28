//use std::backtrace::Backtrace;

pub fn fail_safely(error_message:&str) {

    //println!("{:?}", Backtrace::new());

    println!(stringify!(error_message));


    assert!(false);
}