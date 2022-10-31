use regex::Regex;
use urlencoding::decode;

fn main() {
    let url =
        "https://www.google.com/search?q=AIにおける過学習の説明として、最も適切なものはどれか。";
    let response = reqwest::blocking::get(url).unwrap().text().unwrap();
    // print!("{}", response);
    let document = scraper::Html::parse_document(&response);
    let title_selector = scraper::Selector::parse("a").unwrap();
    let rx = Regex::new(r"/url\?q=https://(|www)\...-siken\.com.*").unwrap();
    println!("{}", rx.is_match("/url?q=https://www.ap-siken.com/kakomon/04_aki/q4.html&amp;sa=U&amp;ved=2ahUKEwiVguawpYr7AhWKgVYBHb0BDdIQFnoECAsQAg&amp;usg=AOvVaw10iBt-XJl_Y-W5i7OzurTr"));
    let titles = document
        .select(&title_selector)
        .map(|x| {
            let href = decode(x.value().attr("href").unwrap())
                .unwrap()
                .into_owned();
            if rx.is_match(&href) && href.contains("&sa=") {
                href.replace("/url?q=", "")
                    .split_once("&sa=")
                    .unwrap()
                    .0
                    .to_string()
            } else {
                String::from("")
            }
        })
        .filter(|x| !x.is_empty());
    titles
        .zip(1..101)
        .for_each(|(item, number)| println!("{}. {}", number, item));
}
