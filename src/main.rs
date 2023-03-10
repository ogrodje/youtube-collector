use std::time::Duration;
use ureq::{Agent, Response};
use error::YTError;
use scraper::{Html, Selector};

mod error;

pub struct YTClient {
    agent: Agent,
}

impl YTClient {
    pub fn from_agent(agent: Agent) -> Self { YTClient { agent } }

    fn response_as_string(response: ureq::Response) -> Result<String, YTError> {
        response.into_string()
            .map_err(|e| YTError::StringParsingError(e.to_string()))
    }

    fn body_to_document(body: &str) -> Result<Html, YTError> {
        Ok(Html::parse_document(body))
    }

    pub fn get_cookies(&mut self) -> Result<Html, YTError> {
        self.agent
            .get("https://youtube.com/@ogrodje")
            .set("Accept-Language", "en-US,en;q=0.9,en-GB;q=0.8")
            .call()
            .map_err(YTError::to_yt_error)
            .and_then(Self::response_as_string)
            .and_then(|body| Self::body_to_document(body.as_str()))
    }
}


fn main() {
    let mut client = YTClient::from_agent(
        ureq::AgentBuilder::new()
            .timeout_read(Duration::from_secs(2))
            .timeout_write(Duration::from_secs(2))
            .build()
    );

    client.get_cookies()
        .and_then(|html| {
            let s = Selector::parse("body").unwrap();

            for e in html.select(&s) {
                println!("x {}", e.value().name());
                println!("{}", e.inner_html());
            }

            // Ok(println!("ok..."));

            Ok(())
        })
        .expect("Boom");


    /*
    match agent.get("https://youtube.com/@ogrodje")
        .call() {
        Ok(response) => println!("{}", response.into_string().expect("yes!")),
        Err(e) => println!("boom. {}", e)
    }

     */
}
