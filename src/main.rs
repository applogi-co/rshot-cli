use std::error::Error;

use headless_chrome::protocol::cdp::Page;
use headless_chrome::Browser;

use clap::Parser;

pub fn screenshot_tab(url: String, width: u32, height: u32) -> Result<Vec<u8>, Box<dyn Error>>{
    let browser = Browser::default()?;
    // let tab = browser.wait_for_initial_tab()?;
    let tab = browser.new_tab_with_options(headless_chrome::protocol::cdp::Target::CreateTarget {
        url,
        width: Option::from(width),
        height: Option::from(height),
        browser_context_id: None,
        enable_begin_frame_control: None,
        new_window: Option::from(false),
        background: Option::from(false)
    })?;

    tab.wait_until_navigated()?;
    tab.wait_for_element("body")?;

    let result = tab
        .capture_screenshot(Page::CaptureScreenshotFormatOption::Png, None, None,true);


    let png_data = match result {
        Ok(data) => { data }
        Err(error) => {
            println!("Error: {:?}", error);
            Vec::new()
        }
    };


    return Ok(png_data);
}

// Simple program to capture a url screenshot
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    // URL to screenshot
    #[clap(short, long, value_parser)]
    url: String,

    // URL to screenshot
    #[clap(short, long, value_parser)]
    output: String,
    // browser window width
    #[clap(short, long, value_parser)]
    width: u32,

    // browser window height
    #[clap(short, long, value_parser)]
    height: u32

}

fn main() {
    let args = Args::parse();

    let image_data = screenshot_tab(args.url, args.width, args.height).expect("");

    std::fs::write(args.output, image_data).expect("Couldn't save image.");
}
