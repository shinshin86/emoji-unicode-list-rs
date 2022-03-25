use scraper::{Html, Selector};
use serde::Serialize;

fn fetch_html() -> Result<String, Box<dyn std::error::Error>> {
    let url = "https://ja.wikipedia.org/wiki/Unicode%E3%81%AEEmoji%E3%81%AE%E4%B8%80%E8%A6%A7";
    let body = reqwest::blocking::get(url)?.text()?;

    Ok(body)
}

#[derive(Debug, Serialize)]
struct Emoji {
    unicode: String,
    name: String,
}

fn main() {
    let html = fetch_html().unwrap();
    let document = Html::parse_document(&html);
    let unicode_selector_str = "table.sortable.wikitable > tbody > tr > td:nth-child(2)";
    let unicode_selector = Selector::parse(unicode_selector_str).unwrap();
    let mut unicode_list = Vec::new();

    for element in document.select(&unicode_selector) {
        if let Some(unicode) = element.text().next() {
            unicode_list.push(unicode);
        }
    }

    let name_selector_str = "table.sortable.wikitable > tbody > tr > td:nth-child(3)";
    let name_selector = Selector::parse(name_selector_str).unwrap();
    let mut names = Vec::new();

    for element in document.select(&name_selector) {
        if let Some(name) = element.text().next() {
            names.push(name);
        }
    }

    assert_eq!(unicode_list.len(), names.len());

    let mut emoji_list: Vec<Emoji> = Vec::new();
    for (i, unicode) in unicode_list.iter().enumerate() {
        emoji_list.push(Emoji {
            unicode: unicode.to_string(),
            name: names[i].to_string().replace("\n", ""),
        })
    }

    let result_str = serde_json::to_string(&emoji_list).unwrap();
    println!("{}", result_str);
}
