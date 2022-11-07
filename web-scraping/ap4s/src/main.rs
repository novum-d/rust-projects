use std::sync::Arc;

use headless_chrome::{Browser, LaunchOptionsBuilder, Tab};
use regex::Regex;
use urlencoding::decode;

#[allow(unused_must_use)]
fn main() {
    // let url =
    //     "https://www.google.com/search?q=AIにおける過学習の説明として、最も適切なものはどれか。";
    // println!("{:#?}", search_questions(url));
    // let result = search_questions(url);
    let class_id = String::from("IH14A219");
    let id = String::from("90223");
    let name = String::from("浜田知季");

    let options = LaunchOptionsBuilder::default()
        .window_size(Some((1920, 1080)))
        .build()
        .expect("Fail to build");
    let browser = Browser::new(options).unwrap();
    let tab = browser.wait_for_initial_tab().unwrap();

    // navigate to google form website
    tab.navigate_to("https://docs.google.com/forms/d/e/1FAIpQLScRkNwFH-sXyyPK-pYyP8pfCpXo5-I1JyNzB0wo1F_9RXUoJQ/viewform");
    tab.wait_until_navigated();

    // enter the student infomation
    enter_student_info(&tab, &Student { class_id, id, name });

    tab.find_element_by_xpath(r#"//*[@id="mG61Hd"]/div[2]/div/div[2]/div[4]/div/div/div[2]/div[1]/div/span/div/div[1]/label/div/div[2]/div/span"#)
        .unwrap()
        .click();

    // submit
    tab.find_element_by_xpath(r#"//*[@id="mG61Hd"]/div[2]/div/div[3]/div[1]/div[1]/div/span"#)
        .unwrap()
        .click();
    search_questions(
        "https://www.google.com/search?q=AIにおける過学習の説明として、最も適切なものはどれか。",
    );
    // let png = tab
    //     .capture_screenshot(
    //         headless_chrome::protocol::cdp::Page::CaptureScreenshotFormatOption::Png,
    //         None,
    //         None,
    //         true,
    //     )
    //     .unwrap();
    // std::fs::write("./page.png", png).unwrap();
}

#[allow(unused_must_use)]
fn search_questions(url: &str) -> Vec<String> {
    let browser = Browser::default().unwrap();
    let tab = browser.wait_for_initial_tab().unwrap();

    // navigate to past tests website
    tab.navigate_to(url);
    tab.wait_until_navigated();

    let links = tab.find_elements("a").unwrap();
    let rx = Regex::new(r"/url\?q=https?://(|www)\...-siken\.com.*").unwrap();

    println!("----------------------------------------------------------");
    links.iter().map(|x| {
        println!("{:?}", x);
        ""
    });

    // "//*[@id="rso"]/div[1]/div/div/div[1]/div/div/div[1]/div/a"
    let response = reqwest::blocking::get(url).unwrap().text().unwrap();
    let document = scraper::Html::parse_document(&response);
    let title_selector = scraper::Selector::parse("a").unwrap();
    let rx = Regex::new(r"/url\?q=https?://(|www)\...-siken\.com.*").unwrap();
    document
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
        .filter(|x| !x.is_empty())
        .collect()
}

#[allow(unused_must_use)]
fn enter_student_info(tab: &Arc<Tab>, student: &Student) {
    tab.find_element_by_xpath(&xpath::Form::ClassId.value())
        .unwrap()
        .type_into(&student.class_id);
    tab.find_element_by_xpath(&xpath::Form::Name.value())
        .unwrap()
        .type_into(&student.id);
    tab.find_element_by_xpath(&xpath::Form::Id.value())
        .unwrap()
        .type_into(&student.name);
}

#[allow(unused_must_use)]
fn enter_answear(tab: &Arc<Tab>) {
    let answers = vec![
        tab.find_elements_by_xpath(&xpath::Answear::A.value())
            .unwrap(),
        tab.find_elements_by_xpath(&xpath::Answear::I.value())
            .unwrap(),
        tab.find_elements_by_xpath(&xpath::Answear::U.value())
            .unwrap(),
        tab.find_elements_by_xpath(&xpath::Answear::E.value())
            .unwrap(),
    ];
}

mod xpath {

    pub enum Form {
        Id,
        Name,
        ClassId,
    }

    impl Form {
        pub fn value(&self) -> String {
            let xpath = |x: i64| {
                format!("//*[@id=\"mG61Hd\"]/div[2]/div/div[2]/div[{}]/div/div/div[2]/div/div[1]/div/div[1]/input", x)
            };
            match *self {
                Form::Id => xpath(3),
                Form::Name => xpath(2),
                Form::ClassId => xpath(1),
            }
        }
    }

    pub enum Answear {
        A,
        I,
        U,
        E,
    }

    impl Answear {
        pub fn value(&self) -> String {
            let xpath = |x: i64| {
                format!(
                    r#"//*[@id="mG61Hd"]/div[2]/div/div[2]/div[4]/div/div/div[2]/div[1]/div/span/div/div[{}]/label/div/div[2]/div/span"#,
                    x
                )
            };
            match *self {
                Answear::A => xpath(1),
                Answear::I => xpath(2),
                Answear::U => xpath(3),
                Answear::E => xpath(4),
            }
        }
    }
}

struct Student {
    id: String,
    name: String,
    class_id: String,
}
