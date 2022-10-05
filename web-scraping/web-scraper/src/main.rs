fn main() {
    let url = "https://www.imdb.com/search/title/?groups=top_100&sort=user_rating,desc&count=100";
    let response = reqwest::blocking::get(url).unwrap().text().unwrap();
    let document = scraper::Html::parse_document(&response);
}
