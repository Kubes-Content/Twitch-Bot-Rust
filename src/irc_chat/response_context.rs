#[derive(Clone)]
pub struct ResponseContext<'l, TParser> {
    pub parser: &'l TParser,
    response_received: String,
    responses_to_send: Vec<String>,
}

impl<'l, TParser> ResponseContext<'l, TParser> {
    pub fn new(parser: &TParser, response_received: String) -> ResponseContext<TParser> {
        ResponseContext {
            parser,
            response_received,
            responses_to_send: Vec::new(),
        }
    }

    pub fn get_received_response(&self) -> String {
        self.response_received.clone()
    }

    pub fn get_responses_to_reply_with(&self) -> Vec<String> {
        self.responses_to_send.clone()
    }

    pub fn add_response_to_reply_with(&mut self, reply: String) {
        self.responses_to_send.push(reply);
    }
}
