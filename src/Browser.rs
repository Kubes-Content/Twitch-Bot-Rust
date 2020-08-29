use url::Url;
use std::io::Result;
use std::process::ExitStatus;


pub trait Browser {
    fn launch_url(&self, url:Url) -> Result<ExitStatus> ;
}

pub struct DefaultBrowser { }

impl Browser for DefaultBrowser {
    fn launch_url(&self, url:Url) -> Result<ExitStatus> {
        open::that(url.as_ref())
    }
}

impl DefaultBrowser {

    /*pub async fn wait_for_url_prefix_to_change_from(&self, prefix:&str) {
        const POLL_RATE:u64 = 10;

        while self.get_current_address().starts_with(prefix) {
            println!("'{0}' does not start with '{1}'", self.get_current_address(), prefix);
            delay_for(Duration::from_millis(POLL_RATE)).await;
        }
    }*/
}