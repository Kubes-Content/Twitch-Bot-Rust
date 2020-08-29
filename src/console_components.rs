use crate::logger::Logger;
use crate::browser::Browser;


pub struct ConsoleComponents<'life> {
    pub logger:&'life dyn Logger,
    pub browser:&'life dyn Browser
}

impl<'life> ConsoleComponents<'life> {
    pub fn new(new_logger:&'life dyn Logger, new_browser:&'life dyn Browser) -> ConsoleComponents<'life> {
        ConsoleComponents { logger: new_logger, browser: new_browser }
    }
}