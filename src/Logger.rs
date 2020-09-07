pub trait Logger: Sync + 'static {
    fn write_line(&self, output:String);

    fn clear(&self);
}

#[derive(Clone, Default)]
pub struct DefaultLogger;

unsafe impl Send for DefaultLogger {}

unsafe impl Sync for DefaultLogger {}

impl Logger for DefaultLogger {

    fn write_line(&self, output: String) {
        println!("{}", output);
    }

    fn clear(&self) { }
}