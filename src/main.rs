extern crate regex;
#[macro_use]
extern crate stdweb;

use regex::Regex;
use std::fmt::Write;
use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::{document, HtmlElement};
use stdweb::web::html_element::InputElement;

use stdweb::web::event::InputEvent;

fn main() {
    stdweb::initialize();

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

    let output_pre: HtmlElement = document()
        .query_selector(".rs-output")
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap();

    pattern_input.add_event_listener({
        let pattern_input = pattern_input.clone();
        let subject_input = subject_input.clone();
        let output_pre = output_pre.clone();

        move |_event: InputEvent| {
            run_regex(
                pattern_input.clone(),
                subject_input.clone(),
                output_pre.clone(),
            );
        }
    });

    subject_input.add_event_listener({
        let pattern_input = pattern_input.clone();
        let subject_input = subject_input.clone();
        let output_pre = output_pre.clone();

        move |_event: InputEvent| {
            run_regex(
                pattern_input.clone(),
                subject_input.clone(),
                output_pre.clone(),
            );
        }
    });

    stdweb::event_loop();
}

fn run_regex(pattern_input: InputElement, subject_input: InputElement, output_pre: HtmlElement) {
    let pattern: String = pattern_input.raw_value();
    let subject: String = subject_input.raw_value();

    if pattern == String::from("") {
        output_pre.set_text_content("");
        return;
    }

    let regex = match Regex::new(&pattern) {
        Ok(re) => re,
        Err(e) => {
            output_pre.set_text_content(&format!("{:?}", e));
            return;
        }
    };

    let captures = regex.captures(&subject);
    let formatted = format_captures(captures);
    output_pre.set_text_content(&formatted);
    console!(log, formatted);
}

fn format_captures(opt: Option<regex::Captures>) -> String {
    match opt {
        None => String::from("None"),
        Some(captures) => {
            let mut buffer = String::from("Some(Captures({\n");

            for (i, cap) in captures.iter().enumerate() {
                write!(&mut buffer, "    {}: ", i).unwrap();
                match cap {
                    None => write!(&mut buffer, "None,\n",).unwrap(),
                    Some(m) => write!(&mut buffer, "Some({:?}),\n", m.as_str()).unwrap(),
                }
            }
            buffer.push_str("}))");
            buffer
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    #[test]
    fn test_format_captures_none() {
        let captures = None;
        assert_eq!("None", format_captures(captures));
    }

    #[test]
    fn test_format_captures_some() {
        let regex = Regex::new("..(..).(..)").unwrap();
        let subject = "0OobodoO0";
        let captures = regex.captures(subject);
        let expected = r#"Some(Captures({
    0: Some("0Oobodo"),
    1: Some("ob"),
    2: Some("do"),
}))"#;
        assert_eq!(expected, format_captures(captures));
    }
}
