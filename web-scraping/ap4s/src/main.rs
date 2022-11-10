use std::sync::Arc;

use headless_chrome::{Browser, LaunchOptionsBuilder, Tab};
use regex::Regex;

#[allow(unused_must_use)]
fn main() {
    // let url =
    //     "https://www.google.com/search?q=AIにおける過学習の説明として、最も適切なものはどれか。";
    // println!("{:#?}", search_questions(url));
    // let result = search_questions(url);
    let class_id = String::from("IH14A219");
    let id = String::from("90223");
    let name = String::from("浜田知季");

    let browser = Browser::default().unwrap();
    let tab = browser.wait_for_initial_tab().unwrap();

    // navigate to google form website
    tab.navigate_to("https://docs.google.com/forms/d/e/1FAIpQLScRkNwFH-sXyyPK-pYyP8pfCpXo5-I1JyNzB0wo1F_9RXUoJQ/viewform");
    tab.wait_until_navigated();

    // enter the student infomation
    enter_student_info(&tab, &Student { class_id, id, name });

    // enter the collect.
    enter_answear(&tab);

    tab.find_element_by_xpath(r#"//*[@id="mG61Hd"]/div[2]/div/div[2]/div[4]/div/div/div[2]/div[1]/div/span/div/div[1]/label/div/div[2]/div/span"#)
        .unwrap()
        .click();

    // submit
    tab.find_element_by_xpath(xpath::SUBMIT).unwrap().click();

    find_website_links(
        "https://www.google.com/search?q=AIにおける過学習の説明として、最も適切なものはどれか。",
    );
}

#[allow(unused_must_use)]
fn enter_student_info(tab: &Arc<Tab>, student: &Student) {
    tab.find_element_by_xpath(&xpath::Student::ClassId.value())
        .unwrap()
        .type_into(&student.class_id);
    tab.find_element_by_xpath(&xpath::Student::Name.value())
        .unwrap()
        .type_into(&student.id);
    tab.find_element_by_xpath(&xpath::Student::Id.value())
        .unwrap()
        .type_into(&student.name);
}

#[allow(unused_must_use)]
fn enter_answear(tab: &Arc<Tab>) {
    let links = get_questions(tab)
        .iter()
        .map(|x| find_website_links(&format!("https://www.google.com/search?q={}", x)))
        .collect::<Vec<Vec<String>>>();
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

#[allow(unused_must_use)]
fn find_website_links(url: &str) -> Vec<String> {
    let browser = Browser::default().unwrap();
    let tab = browser.wait_for_initial_tab().unwrap();

    // navigate to past tests website
    tab.navigate_to(url);
    tab.wait_until_navigated();

    let rx = Regex::new(r#"^https?://(|www)\...-siken\.com.*$"#).unwrap();
    let els = tab.find_elements("a").unwrap();
    let attrs = els.iter().map(|x| x.get_attributes().unwrap().unwrap());
    let mut urls = vec![];
    attrs.for_each(|x| {
        x.iter().for_each(|x| {
            if rx.is_match(x) {
                urls.push(x.clone());
            }
        })
    });
    urls
}

#[allow(unused_must_use)]
fn get_questions(tab: &Arc<Tab>) -> Vec<String> {
    tab.find_elements_by_xpath(r#"//*[@class="M7eMe"]/span"#)
        .unwrap()
        .iter()
        .map(|x| {
            x.get_description().unwrap().children.unwrap()[0]
                .node_value
                .clone()
        })
        .collect()
}

mod xpath {

    pub const submit: &str = r#"//*[@id="mG61Hd"]/div[2]/div/div[3]/div[1]/div[1]/div/span"#;

    pub enum Student {
        Id,
        Name,
        ClassId,
    }

    impl Student {
        pub fn value(&self) -> String {
            let xpath = |x: i64| {
                format!("//*[@id=\"mG61Hd\"]/div[2]/div/div[2]/div[{}]/div/div/div[2]/div/div[1]/div/div[1]/input", x)
            };
            match *self {
                Student::Id => xpath(3),
                Student::Name => xpath(2),
                Student::ClassId => xpath(1),
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

pub enum Url {
    GoogleForm(String),
    GoogleSearch(String),
}

impl Url {
    pub fn value(&self) -> String {
        match *self {
            Url::GoogleSearch => |title| format!("https://www.google.com/search?q={}", title),
        }
    }
}
