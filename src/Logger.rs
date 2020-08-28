pub trait Logger {
    fn write_line(&self, output:&str);

    fn clear(&self);
}

pub struct DefaultLogger {

}

impl Logger for DefaultLogger {

    fn write_line(&self, output: &str) {
        println!("{}", output);
    }

    fn clear(&self) {
        unimplemented!()
    }
}