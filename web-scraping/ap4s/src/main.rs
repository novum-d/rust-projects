use reqwest::Url;
use url::ParseError as UrlParseError;

fn main() {
    let url = "https://www.google.com/search?q=hoge&oq=hoge";
    let response = reqwest::blocking::get(url)
        .unwrap()
        .text_with_charset("utf-8")
        .unwrap()
        .replace('\\', "");
    let document = scraper::Html::parse_document(&response);
    let title_selector = scraper::Selector::parse(".yuRUbf a").unwrap();
    let titles = document.select(&title_selector).map(|x| {
        let href = x.value().attr("href").unwrap();
        if let Err(UrlParseError::RelativeUrlWithoutBase) = Url::parse(href) {
            href.replace("/url?q=", "")
        } else {
            String::from("")
        }
    });
    titles
        .zip(1..101)
        .for_each(|(item, number)| println!("{}. {}", number, item));
}
