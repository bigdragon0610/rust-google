use ansi_term::{Color, Style};
use clap::Parser;
use headless_chrome::Browser;
use std::{sync::Arc, thread};

#[derive(Parser)]
struct Cli {
    keywords: String,
}

fn main() {
    let keywords = Cli::parse().keywords;
    let input = format!("https://www.google.com/search?q={}", keywords);
    browse(input);
}

fn browse(input: String) {
    let browser = Browser::default().unwrap();

    let tab = browser.wait_for_initial_tab().unwrap();

    tab.navigate_to(&input).unwrap();

    let mut completed_element_ids: Vec<u32> = Vec::new();

    let rc_cnt = Arc::strong_count(&tab);
    let tab_clone = tab.clone();
    thread::spawn(move || {
        tab_clone.wait_until_navigated().unwrap();
    });

    loop {
        let mut is_last = false;
        if Arc::strong_count(&tab) == rc_cnt {
            is_last = true;
        }

        let div_elements = tab.wait_for_elements("div.tF2Cxc").unwrap();
        for div_element in div_elements {
            let id = div_element.backend_node_id;
            if completed_element_ids.contains(&id) {
                continue;
            }

            let h3_element = div_element.find_element("h3.LC20lb");
            if h3_element.is_err() {
                break;
            }
            let title = h3_element.unwrap().get_inner_text().unwrap();

            let a_element = div_element.find_element("a");
            if a_element.is_err() {
                break;
            }
            let url = a_element.unwrap().get_attributes().unwrap().unwrap()[1].to_string();

            let mut overview = String::new();
            let overview_div = div_element.find_element("div.VwiC3b");
            if overview_div.is_ok() {
                overview = overview_div.unwrap().get_inner_text().unwrap();
            }

            let title = Style::new().fg(Color::Yellow).bold().paint(title);
            let url = Style::new().fg(Color::Green).paint(url);
            if overview != "" {
                overview += "\n";
            }
            println!("{}\n{}\n{}", title, url, overview);
            completed_element_ids.push(id);
        }
        if is_last {
            break;
        }
    }
}
