use std::time;
use std::{
    fmt::{self},
    io,
    sync::Arc,
};

use headless_chrome::LaunchOptions;
use headless_chrome::{protocol::cdp::Page::CaptureScreenshotFormatOption::Png, Browser, Element};
use once_cell::sync::OnceCell;
use regex::Regex;
use urlencoding::encode;

use crate::xpath::{google_form, siken_dot_com};

static TAB: OnceCell<Arc<headless_chrome::Tab>> = OnceCell::new();

#[allow(unused_must_use)]
fn main() {
    let class_id = String::from("IH14A219");
    let id = String::from("90223");
    let name = String::from("浜田知季");
    let form_url = r#"https://docs.google.com/forms/d/e/1FAIpQLScRkNwFH-sXyyPK-pYyP8pfCpXo5-I1JyNzB0wo1F_9RXUoJQ/viewform"#;

    let option = LaunchOptions {
        headless: true,
        idle_browser_timeout: time::Duration::from_secs(200),
        ..Default::default()
    };
    let browser = Browser::new(option).unwrap();
    TAB.set(browser.wait_for_initial_tab().unwrap());

    println!("Now crawling...");

    // navigate to the google form website.
    TAB.get().unwrap().navigate_to(form_url);
    TAB.get().unwrap().wait_until_navigated();

    // type the student infomation.
    type_student_info(&Student { class_id, id, name });

    // type the collects.
    type_answer();

    // submit
    TAB.get()
        .unwrap()
        .find_element_by_xpath(google_form::SUBMIT)
        .unwrap()
        .click();

    let png = TAB
        .get()
        .unwrap()
        .capture_screenshot(Png, None, None, true)
        .unwrap();
    std::fs::write("./last.png", png);

    println!("...Done");
}

#[allow(unused_must_use)]
fn type_student_info(student: &Student) {
    TAB.get()
        .unwrap()
        .find_element_by_xpath(&google_form::Student::ClassId.to_string())
        .unwrap()
        .type_into(&student.class_id);
    TAB.get()
        .unwrap()
        .find_element_by_xpath(&google_form::Student::Name.to_string())
        .unwrap()
        .type_into(&student.id);
    TAB.get()
        .unwrap()
        .find_element_by_xpath(&google_form::Student::Id.to_string())
        .unwrap()
        .type_into(&student.name);
}

#[allow(unused_must_use)]
fn type_answer() {
    let answers = get_answers();
    for (i, ans) in (0_i32..).zip(answers.iter()) {
        match ans.to_owned() {
            ans if ans == Answers::A.to_string() => {
                TAB.get()
                    .unwrap()
                    .find_element_by_xpath(&google_form::Answers::A(i).to_string())
                    .unwrap()
                    .click();
            }
            ans if ans == Answers::I.to_string() => {
                TAB.get()
                    .unwrap()
                    .find_element_by_xpath(&google_form::Answers::I(i).to_string())
                    .unwrap()
                    .click();
            }
            ans if ans == Answers::U.to_string() => {
                TAB.get()
                    .unwrap()
                    .find_element_by_xpath(&google_form::Answers::U(i).to_string())
                    .unwrap()
                    .click();
            }
            ans if ans == Answers::E.to_string() => {
                TAB.get()
                    .unwrap()
                    .find_element_by_xpath(&google_form::Answers::E(i).to_string())
                    .unwrap()
                    .click();
            }
            _ => (),
        }
    }
}

#[allow(unused_must_use)]
fn type_answear(ans_xpath: String) {
    TAB.get()
        .unwrap()
        .find_element_by_xpath(&ans_xpath)
        .unwrap()
        .click();
}

#[allow(unused_must_use)]
fn get_answers() -> Vec<String> {
    let browser = Browser::default().unwrap();
    let search_tab = browser.wait_for_initial_tab().unwrap();

    let questions = get_questions();
    let urls_of_question = questions
        .iter()
        .map(|ans| {
            (
                ans.to_owned(),
                find_website_links(&Url::GoogleSearch(&encode(ans)).to_string(), &search_tab),
            )
        })
        .collect::<Vec<(String, Vec<String>)>>();

    let mut collects = Vec::new();
    let browser = Browser::default().unwrap();
    let siken_tab = browser.wait_for_initial_tab().unwrap();

    'top: for (i, uoq) in (0_i32..).zip(urls_of_question.iter()) {
        let collect_cnt = collects.len();
        for url in uoq.1.iter() {
            siken_tab.navigate_to(url);
            siken_tab.wait_until_navigated();

            let ans = get_node_value(
                &siken_tab
                    .find_element_by_xpath(&siken_dot_com::Question::Answer.to_string())
                    .unwrap(),
            );
            // println!("{}\n{}", rm_symbol(&ans), rm_symbol(&uoq.0)); // title diff
            if rm_symbol(&ans) == rm_symbol(&uoq.0) {
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
            println!("\nAnswer is not found!\nPlease search in the browser and choise an answear from the following numbers one to four.\nA browser with keywords searched from the question title will open...\n[Title]:\n {}", &uoq.0);
            webbrowser::open(&Url::GoogleSearch(&uoq.0).to_string());

            loop {
                println!("Please select and enter a number from the following.\nex). 1\n   1. ア\n   2. イ\n   3. ウ\n   4. エ");

                let mut input = String::new();
                io::stdin().read_line(&mut input);
                let num = input.trim().parse::<i32>().unwrap_or(0);
                match num {
                    1 => {
                        type_answear(google_form::Answers::A(i).to_string());
                        break;
                    }
                    2 => {
                        type_answear(google_form::Answers::I(i).to_string());
                        break;
                    }
                    3 => {
                        type_answear(google_form::Answers::U(i).to_string());
                        break;
                    }
                    4 => {
                        type_answear(google_form::Answers::E(i).to_string());
                        break;
                    }
                    _ => {
                        println!("You've typed an answer that is not in the options.\nPleaase type answer here again");
                    }
                }
            }
        }
    }
    collects
}

fn rm_symbol(ans: &str) -> String {
    let mut answer = ans.to_owned();
    answer.retain(|c| !r#"()ーも用有、，,？?。・ .;:"#.contains(c));
    answer
}

#[allow(unused_must_use)]
fn find_website_links(url: &str, search_tab: &Arc<headless_chrome::Tab>) -> Vec<String> {
    search_tab.navigate_to(url);
    search_tab.wait_until_navigated();

    let rx = Regex::new(r#"^https?://(|www)\...-siken\.com.*$"#).unwrap();
    let els = search_tab.find_elements("a").unwrap();
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
fn get_questions() -> Vec<String> {
    TAB.get()
        .unwrap()
        .find_elements_by_xpath(google_form::QUESTIONS)
        .unwrap()
        .iter()
        .map(get_node_value)
        .collect()
}

fn get_node_value(el: &Element) -> String {
    el.get_description().unwrap().children.unwrap()[0]
        .node_value
        .to_owned()
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
                            r#"//*[@id="mainCol"]/div[2]/div[2] | //*[@id="mainCol"]/div[2]/section | //*[@id="mainCol"]/div[2]/p[1] | //*[@id="mainCol"]/div[2]/article"#,
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
