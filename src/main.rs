use ansi_term::{Color, Style};
use clap::Parser;
use headless_chrome::Browser;
use std::error::Error;

#[derive(Parser)]
struct Cli {
    keywords: String,
}

fn main() {
    let keywords = Cli::parse().keywords;
    let input = format!("https://www.google.com/search?q={}", keywords);
    browse(input).unwrap();
}

fn browse(input: String) -> Result<(), Box<dyn Error>> {
    let browser = Browser::default()?;

    let tab = browser.wait_for_initial_tab()?;

    tab.navigate_to(&input)?.wait_until_navigated()?;

    let div_elements = tab.wait_for_elements("div.tF2Cxc")?;

    let mut output = "\n".to_string();
    for div_element in div_elements {
        let mut title = "".to_string();
        let mut url = "".to_string();
        let mut overview = "".to_string();

        let h3_element = div_element.find_element("h3.LC20lb");
        if h3_element.is_ok() {
            title = h3_element?.get_inner_text()?;
            url = div_element.find_element("a")?.get_attributes()?.unwrap()[1].to_string();
        }
        let overview_div = div_element.find_element("div.VwiC3b");
        if overview_div.is_ok() {
            overview = overview_div?.get_inner_text()?;
        }

        let title = Style::new().fg(Color::Yellow).bold().paint(title);
        let url = Style::new().fg(Color::Green).paint(url);
        if overview != "" {
            overview += "\n";
        }
        output += format!("{}\n{}\n{}\n", title, url, overview).as_str();
    }

    println!("{}", output);

    Ok(())
}
