fn main() {
    let url = "https://www.google.com/search?q=hoge&oq=hoge";
    let response = reqwest::blocking::get(url)
        .unwrap()
        .text()
        .unwrap()
        .replace('\\', "");
    let html = r#"
        <!doctype html>
    <html lang="ja">
    <head>
      <meta charset="UTF-8">
      <title>HTML Sample</title>
      <link rel="stylesheet" href="style.css">
      <script type="text/javascript" src="sample.js"></script>
    </head>
    <body>
      <div class="header">Header領域</div>
      <div class="main">
        <h1>見出し</h1>
        <p>コンテンツ</p>
        <img src="img/sample1.jpg">
      </div>
      <div class="yuRUbf">
        <span>Footer領域</span>
        <a href="https://google.com"><div>リンク</div></a>
      </div>
    </body>
    </html>
        "#;
    // println!("{:?}", &response);
    let document = scraper::Html::parse_document(&response);
    let title_selector = scraper::Selector::parse("a").unwrap();
    let titles = document
        .select(&title_selector)
        .map(|x| x.value().attr("href").unwrap())
        .filter(|x| x.contains("/url?q="))
        .map(|x| x.replace("/url?q=", ""));
    titles
        .zip(1..101)
        .for_each(|(item, number)| println!("{}. {}", number, item));
}
