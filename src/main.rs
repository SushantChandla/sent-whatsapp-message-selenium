use std::time::Duration;
use thirtyfour::{prelude::*, support::sleep};
use tokio::{self};

/// How to use
///
/// Run the project:
/// ./geckodriver
/// cargo run
const SEND_TO: &str = "Sushant";
const MESSAGE: &str = "Hello";

const WAIT_SECONDS: u64 = 5;
const START_LINK: &str = "https://web.whatsapp.com/";

async fn wait() {
    sleep(Duration::new(WAIT_SECONDS, 0)).await;
}

#[tokio::main]
async fn main() {
    let mut caps = DesiredCapabilities::firefox();
    caps.add_firefox_arg("-profile").unwrap();
    caps.add_firefox_arg(
        // my default profile.
        "/Users/sushantchandla/Library/Application Support/Firefox/Profiles/uzmus0hw.default-release",
    )
    .unwrap();
    let driver = WebDriver::new("http://localhost:4444", &caps)
        .await
        .unwrap();

    driver.get(START_LINK).await.unwrap();
    open_chat(&driver).await;

    #[allow(unused_must_use)]
    {
        wait().await;
        wait().await;
        driver.quit().await;
    }
}

async fn open_chat(driver: &WebDriver) {
    let mut chat_options;
    loop {
        wait().await;
        chat_options = driver
            .find_elements(By::Css(format!("span[title='{SEND_TO}']").as_str()))
            .await
            .unwrap();
        if !chat_options.is_empty() {
            break;
        }
    }
    chat_options[0].click().await.unwrap();

    wait().await;

    let input = driver
        .find_element(By::XPath(
            "//*[@id='main']/footer/div[1]/div/span[2]/div/div[2]/div[1]/div/div[1]",
        ))
        .await
        .unwrap();

    for _ in 0..10 {
        for c in MESSAGE.chars() {
            input.send_keys(c).await.unwrap();
        }
        input.send_keys(Keys::Enter).await.unwrap();
    }
}
