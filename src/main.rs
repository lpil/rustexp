extern crate regex;
#[macro_use]
extern crate stdweb;

use regex::Regex;

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::document;
use stdweb::web::html_element::InputElement;

use stdweb::web::event::InputEvent;

fn main() {
    stdweb::initialize();

    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    let result = re.is_match("2014-01-01");

    console!(log, result);

    let pattern_input: InputElement = document()
        .query_selector(".rs-pattern-input")
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap();

    let subject_input: InputElement = document()
        .query_selector(".rs-subject-input")
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap();

    pattern_input.add_event_listener({
        let pattern_input = pattern_input.clone();
        let subject_input = subject_input.clone();

        move |_event: InputEvent| {
            run_regex(pattern_input.clone(), subject_input.clone());
        }
    });

    subject_input.add_event_listener({
        let pattern_input = pattern_input.clone();
        let subject_input = subject_input.clone();

        move |_event: InputEvent| {
            run_regex(pattern_input.clone(), subject_input.clone());
        }
    });

    stdweb::event_loop();
}

fn run_regex(pattern_input: InputElement, subject_input: InputElement) {
    let pattern: String = pattern_input.raw_value();
    let subject: String = subject_input.raw_value();

    let regex = match Regex::new(&pattern) {
        Ok(r) => r,
        error => {
            console!(log, format!("{:?}", error));
            return;
        }
    };

    for capture in regex.captures(&subject).iter() {
        console!(log, format!("{:?}", capture));
    }
}
