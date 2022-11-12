use std::{
    fmt::{self},
    io,
    sync::Arc,
};

use headless_chrome::{
    protocol::cdp::Page::CaptureScreenshotFormatOption::Png, Browser, Element, Tab,
};
use regex::Regex;

use crate::xpath::{google_form, siken_dot_com};

#[allow(unused_must_use)]
fn main() {
    let class_id = String::from("IH14A219");
    let id = String::from("90223");
    let name = String::from("浜田知季");
    let form_url = r#"https://docs.google.com/forms/d/e/1FAIpQLScRkNwFH-sXyyPK-pYyP8pfCpXo5-I1JyNzB0wo1F_9RXUoJQ/viewform"#;

    let browser = Browser::default().unwrap();
    let form_tab = browser.wait_for_initial_tab().unwrap();

    println!("Now crawling...");

    // navigate to the google form website.
    form_tab.navigate_to(form_url);
    form_tab.wait_until_navigated();

    // type the student infomation.
    type_student_info(&form_tab, &Student { class_id, id, name });

    // type the collects.
    type_answer(&form_tab);

    // submit
    form_tab
        .find_element_by_xpath(google_form::SUBMIT)
        .unwrap()
        .click();

    let png = form_tab.capture_screenshot(Png, None, None, true).unwrap();
    std::fs::write("./last.png", png);

    println!("...Done");
}

#[allow(unused_must_use)]
fn type_student_info(form_tab: &Arc<Tab>, student: &Student) {
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
fn type_answer(form_tab: &Arc<Tab>) {
    let answers = get_answers(form_tab);
    for (i, ans) in (0_i32..).zip(answers.iter()) {
        match ans.to_owned() {
            ans if ans == Answers::A.to_string() => {
                form_tab
                    .find_element_by_xpath(&google_form::Answers::A(i).to_string())
                    .unwrap()
                    .click();
            }
            ans if ans == Answers::I.to_string() => {
                form_tab
                    .find_element_by_xpath(&google_form::Answers::I(i).to_string())
                    .unwrap()
                    .click();
            }
            ans if ans == Answers::U.to_string() => {
                form_tab
                    .find_element_by_xpath(&google_form::Answers::U(i).to_string())
                    .unwrap()
                    .click();
            }
            ans if ans == Answers::E.to_string() => {
                form_tab
                    .find_element_by_xpath(&google_form::Answers::E(i).to_string())
                    .unwrap()
                    .click();
            }
            _ => (),
        }
    }
}

#[allow(unused_must_use)]
fn get_answers(form_tab: &Arc<Tab>) -> Vec<String> {
    println!("{:#?}", get_questions(form_tab));
    let links = get_questions(form_tab)
        .iter()
        .map(|ans| {
            // println!("{}", ans.to_owned());
            let tp = (
                ans.to_owned(),
                find_website_links(&Url::GoogleSearch(ans).to_string()),
            );
            // println!("{}", ans.to_owned());
            tp
        })
        .collect::<Vec<(String, Vec<String>)>>();
    let mut collects = Vec::new();
    let browser = Browser::default().unwrap();
    let siken_tab = browser.wait_for_initial_tab().unwrap();

    'top: for (i, link) in (0_i32..).zip(links.iter()) {
        let collect_cnt = collects.len();
        for url in link.1.iter() {
            siken_tab.navigate_to(url);
            siken_tab.wait_until_navigated();
            let ans = get_node_value(
                &siken_tab
                    .find_element_by_xpath(&siken_dot_com::Question::Answer.to_string())
                    .unwrap(),
            );
            println!("{}", collect_cnt);
            if rm_symbol(&ans) == rm_symbol(&link.0) {
                let collect = get_node_value(
                    &siken_tab
                        .find_element_by_xpath(&siken_dot_com::Question::Collect.to_string())
                        .unwrap(),
                );
                println!("{}. {}: {}", i + 1, ans, collect);
                collects.push(collect);
                continue 'top;
            }
        }

        if collects.len() != collect_cnt + 1 {
            println!("Answer is not found. Please search and type an answear.");
            let mut input = String::new();
            loop {
                io::stdin().read_line(&mut input);
                match input.trim().to_owned() {
                    input
                        if input == Answers::A.to_string()
                            || input == Answers::I.to_string()
                            || input == Answers::U.to_string()
                            || input == Answers::E.to_string() =>
                    {
                        break;
                    }
                    _ => {
                        println!("You've typed an answer that is not in the options.\nPleaase type answer here again")
                    }
                }
            }
        }
    }
    collects
}

fn rm_symbol(ans: &str) -> String {
    let mut answer = ans.to_owned();
    answer.retain(|c| !r#"(),、，？?。・ .;:"#.contains(c));
    answer
}

#[allow(unused_must_use)]
fn find_website_links(url: &str) -> Vec<String> {
    let browser = Browser::default().unwrap();
    let browser_tab = browser.wait_for_initial_tab().unwrap();

    browser_tab.navigate_to(url);
    browser_tab.wait_until_navigated();

    let rx = Regex::new(r#"^https?://(|www)\...-siken\.com.*$"#).unwrap();
    let els = browser_tab.find_elements("a").unwrap();
    let attrs = els.iter().map(|x| x.get_attributes().unwrap().unwrap());
    let mut urls = vec![];
    attrs.for_each(|url| {
        url.iter().for_each(|x| {
            if rx.is_match(x) {
                urls.push(x.to_owned());
            }
        })
    });
    urls
}

#[allow(unused_must_use)]
fn get_questions(form_tab: &Arc<Tab>) -> Vec<String> {
    form_tab
        .find_elements_by_xpath(google_form::QUESTIONS)
        .unwrap()
        .iter()
        .map(get_node_value)
        .collect()
}

fn get_node_value(el: &Element) -> String {
    let node_value = el.get_description().unwrap().children.unwrap()[0]
        .node_value
        .to_owned();
    println!("{:#?}", node_value);
    node_value
}

mod xpath {

    pub mod siken_dot_com {
        use std::fmt;

        pub enum Question {
            Answer,
            Collect,
        }
        impl fmt::Display for Question {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(
                    f,
                    "{}",
                    match *self {
                        Self::Answer =>
                            r#"//*[@id="mainCol"]/div[2]/div[2] | //*[@id="mainCol"]/div[2]/section | //*[@id="mainCol"]/div[2]/p[1]"#,
                        Self::Collect => r#"//*[@id="answerChar"]"#,
                    }
                )
            }
        }
    }

    pub mod google_form {
        use std::fmt;

        pub const QUESTIONS: &str = r#"//*[@class="M7eMe"]/span"#;
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

        pub enum Answers {
            A(i32),
            I(i32),
            U(i32),
            E(i32),
        }

        impl fmt::Display for Answers {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let i = match *self {
                    Self::A(x) => (x, 1),
                    Self::I(x) => (x, 2),
                    Self::U(x) => (x, 3),
                    Self::E(x) => (x, 4),
                };
                write!(
                    f,
                    r#"//*[@id="mG61Hd"]/div[2]/div/div[2]/div[{}]/div/div/div[2]/div[1]/div/span/div/div[{}]/label"#,
                    i.0 + 4,
                    i.1
                )
            }
        }
    }
}

enum Answers {
    A,
    I,
    U,
    E,
}

impl fmt::Display for Answers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::A => "ア",
                Self::I => "イ",
                Self::U => "ウ",
                Self::E => "エ",
            }
        )
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
            Url::GoogleSearch(title) => write!(f, r#"https://www.google.com/search?q={}"#, title),
        }
    }
}
