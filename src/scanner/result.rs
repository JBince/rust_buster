use console::style;
use std::fmt;

//Column width for result entries
const COL_WIDTH: usize = 15;

pub(super) struct ResultEntry {
    //Title used in the result entry
    pub status: String,

    //Value passed in via config/defaults
    pub method: String,

    //Length of response
    pub length: String,

    //Endpoint from response
    pub endpoint: String,

    //Endpoint for any redirects encountered
    //pub redirection: String,

    //Request Number
    pub request_number: String,
}

impl ResultEntry {
    pub fn new(
        status: u16,
        method: String,
        length: u16,
        endpoint: String,
        request_number: u16,
    ) -> Self {
        Self {
            status: status.to_string(),
            method: method.to_string(),
            length: length.to_string(),
            endpoint: endpoint.to_string(),
            request_number: request_number.to_string(),
            //redirection: String::new(),
        }
    }
}

impl fmt::Display for ResultEntry {
    //Display formatter for the given banner entry
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:\u{0020}<width$}{:\u{0020}<width$}{:\u{0020}<width$}{:\u{0020}<width$}{:\u{0020}<width$}",
            //Color based on category of status code
            self.request_number,
            match self.status.chars().next().unwrap() {
                '2' => style(&self.status).green(),
                '3' => style(&self.status).yellow(),
                '4' => style(&self.status).red(),
                _ => style(&self.status).blue(),
            },
            self.method,
            format!("{}c", self.length),
            self.endpoint,
            //style(&self.redirection).yellow(),
            width = COL_WIDTH
        )
    }
}
