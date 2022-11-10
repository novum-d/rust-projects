use std::{
    fmt::{self},
    sync::Arc,
};

use headless_chrome::{Browser, Element, Tab};
use regex::Regex;

use crate::xpath::{google_form, siken_dot_com};

#[allow(unused_must_use)]
fn main() {
    let class_id = String::from("IH14A219");
    let id = String::from("90223");
    let name = String::from("浜田知季");

    let form_url = "https://docs.google.com/forms/d/e/1FAIpQLScRkNwFH-sXyyPK-pYyP8pfCpXo5-I1JyNzB0wo1F_9RXUoJQ/viewform";

    let browser = Browser::default().unwrap();
    let form_tab = browser.wait_for_initial_tab().unwrap();

    // navigate to google form website
    form_tab.navigate_to(form_url);
    form_tab.wait_until_navigated();

    // enter the student infomation
    enter_student_info(&form_tab, &Student { class_id, id, name });

    // enter the collect.
    enter_answear(&form_tab);

    // submit
    // tab.find_element_by_xpath(xpath::SUBMIT).unwrap().click();
}

#[allow(unused_must_use)]
fn enter_student_info(form_tab: &Arc<Tab>, student: &Student) {
    form_tab
        .find_element_by_xpath(&google_form::Student::ClassId.to_string())
        .unwrap()
        .type_into(&student.class_id);
    form_tab
        .find_element_by_xpath(&google_form::Student::Name.to_string())
        .unwrap()
        .type_into(&student.id);
    form_tab
        .find_element_by_xpath(&google_form::Student::Id.to_string())
        .unwrap()
        .type_into(&student.name);
}

#[allow(unused_must_use)]
fn enter_answear(form_tab: &Arc<Tab>) {
    get_answears(form_tab);
    let answers = vec![
        form_tab
            .find_elements_by_xpath(&google_form::Option::A.to_string())
            .unwrap(),
        form_tab
            .find_elements_by_xpath(&google_form::Option::I.to_string())
            .unwrap(),
        form_tab
            .find_elements_by_xpath(&google_form::Option::U.to_string())
            .unwrap(),
        form_tab
            .find_elements_by_xpath(&google_form::Option::E.to_string())
            .unwrap(),
    ];
}

#[allow(unused_must_use)]
fn get_answears(form_tab: &Arc<Tab>) {
    let links = get_questions(form_tab)
        .iter()
        .map(|title| find_website_links(&Url::GoogleSearch(title).to_string()))
        .collect::<Vec<Vec<String>>>();

    'top: for link in links.iter() {
        let browser = Browser::default().unwrap();
        let siken_tab = browser.wait_for_initial_tab().unwrap();
        for url in link.iter() {
            println!("{}", url);
            siken_tab.navigate_to(url);
            siken_tab.wait_until_navigated();
            let answear = get_node_value(
                &siken_tab
                    .find_element_by_xpath(&siken_dot_com::Question::Answear.to_string())
                    .unwrap(),
            );

            let collect = get_node_value(
                &siken_tab
                    .find_element_by_xpath(&siken_dot_com::Question::Collect.to_string())
                    .unwrap(),
            );
            println!("{}: {}", answear, collect)
        }
    }
}

#[allow(unused_must_use)]
fn find_website_links(url: &str) -> Vec<String> {
    let browser = Browser::default().unwrap();
    let browser_tab = browser.wait_for_initial_tab().unwrap();

    // navigate to past tests website
    browser_tab.navigate_to(url);
    browser_tab.wait_until_navigated();

    let rx = Regex::new(r#"^https?://(|www)\...-siken\.com.*$"#).unwrap();
    let els = browser_tab.find_elements("a").unwrap();
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
fn get_questions(form_tab: &Arc<Tab>) -> Vec<String> {
    form_tab
        .find_elements_by_xpath(r#"//*[@class="M7eMe"]/span"#)
        .unwrap()
        .iter()
        .map(get_node_value)
        .collect()
}

fn get_node_value(el: &Element) -> String {
    el.get_description().unwrap().children.unwrap()[0]
        .node_value
        .clone()
}

mod xpath {

    pub mod siken_dot_com {
        use std::fmt;

        pub enum Question {
            Answear,
            Collect,
        }
        impl fmt::Display for Question {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(
                    f,
                    "{}",
                    match *self {
                        Self::Answear =>
                            r#"//*[@id="mainCol"]/div[2]/div[2] | //*[@id="mainCol"]/div[2]/section | //*[@id="mainCol"]/div[2]/p[1]"#,
                        Self::Collect => r#"//*[@id="answerChar"]"#,
                    }
                )
            }
        }
    }

    pub mod google_form {
        use std::fmt;

        pub const SUBMIT: &str = r#"//*[@id="mG61Hd"]/div[2]/div/div[3]/div[1]/div[1]/div/span"#;

        pub enum Student {
            Id,
            Name,
            ClassId,
        }

        impl fmt::Display for Student {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(
                    f,
                    r#"//*[@id="mG61Hd"]/div[2]/div/div[2]/div[{}]/div/div/div[2]/div/div[1]/div/div[1]/input"#,
                    match &self {
                        Self::Id => 3,
                        Self::Name => 2,
                        Self::ClassId => 1,
                    }
                )
            }
        }

        pub enum Option {
            A,
            I,
            U,
            E,
        }

        impl fmt::Display for Option {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(
                    f,
                    r#"//*[@id="mG61Hd"]/div[2]/div/div[2]/div[4]/div/div/div[2]/div[1]/div/span/div/div[{}]/label/div/div[2]/div/span"#,
                    match *self {
                        Self::A => 1,
                        Self::I => 2,
                        Self::U => 3,
                        Self::E => 4,
                    }
                )
            }
        }
    }
}

struct Student {
    id: String,
    name: String,
    class_id: String,
}

pub enum Url<'a> {
    GoogleSearch(&'a str),
}

impl fmt::Display for Url<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Url::GoogleSearch(tltle) => write!(f, "https://www.google.com/search?q={}", tltle),
        }
    }
}
