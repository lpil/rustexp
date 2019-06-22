extern crate regex;
// #[macro_use]
extern crate stdweb;

use regex::Regex;
use std::fmt::Write;

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::{document, HtmlElement};
use stdweb::web::html_element::{InputElement, TextAreaElement};
use stdweb::web::event::InputEvent;

fn main() {
    stdweb::initialize();

    // Input containing pattern from which we construct the regex
    let pattern_input: InputElement = document()
        .query_selector(".rs-pattern-input")
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap();

    // Input containing string on which we run the regex
    let subject_input: TextAreaElement = document()
        .query_selector(".rs-subject-input")
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap();

    // `pre` in which we render the results
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

fn run_regex(pattern_input: InputElement, subject_input: TextAreaElement, output_pre: HtmlElement) {
    let pattern: String = pattern_input.raw_value();
    let subject: String = subject_input.value();

    // We don't want to do anything if there is no pattern.
    // It will match anything
    if pattern == String::from("") {
        output_pre.set_text_content("");
        return;
    }

    let regex = match Regex::new(&pattern) {
        // If the pattern doesn't compile into a regex
        // render the error and halt.
        Err(e) => {
            output_pre.set_text_content(&format!("{:?}", e));
            return;
        }
        Ok(re) => re,
    };

    let formatted = format_captures(regex, &subject);
    output_pre.set_text_content(&formatted);
}

fn format_captures(regex: regex::Regex, subject: &str) -> String {
    let mut buffer = String::new();

    for captures in regex.captures_iter(subject) {
        write!(&mut buffer, "Some(Captures({{\n").unwrap();

        for (i, cap) in captures.iter().enumerate() {
            write!(&mut buffer, "    {}: Some({:?}),\n", i, cap.unwrap().as_str()).unwrap();
        }

        write!(&mut buffer, "}})),\n").unwrap();
    }

    if buffer == "" {
        String::from("None")
    } else {
        buffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    #[test]
    fn test_format_captures_no_matches() {
        let regex = Regex::new("foo").unwrap();
        let subject = "bar";
        assert_eq!("None", format_captures(regex, subject));
    }

    #[test]
    fn test_format_captures_no_captures() {
        let regex = Regex::new("foo").unwrap();
        let subject = "foobar";
        let expected = r#"Some(Captures({
    0: Some("foo"),
})),
"#;
        assert_eq!(expected, format_captures(regex, subject));
    }

    #[test]
    fn test_format_captures_some() {
        let regex = Regex::new("..(..).(..)").unwrap();
        let subject = "0OobodoO0";
        let expected = r#"Some(Captures({
    0: Some("0Oobodo"),
    1: Some("ob"),
    2: Some("do"),
})),
"#;
        assert_eq!(expected, format_captures(regex, subject));
    }

    #[test]
    fn test_format_captures_some_2() {
        let regex = Regex::new("(.)(.)(.)(.)(.)(.)(.)(.)(.)(.)").unwrap();
        let subject = "1234567890";
        let expected = r#"Some(Captures({
    0: Some("1234567890"),
    1: Some("1"),
    2: Some("2"),
    3: Some("3"),
    4: Some("4"),
    5: Some("5"),
    6: Some("6"),
    7: Some("7"),
    8: Some("8"),
    9: Some("9"),
    10: Some("0"),
})),
"#;
        assert_eq!(expected, format_captures(regex, subject));
    }

    #[test]
    fn test_format_captures_all() {
        let regex = Regex::new(r#"\b(.)at"#).unwrap();
        let subject = "cat hat no pat no don't sit on that";
        let expected = r#"Some(Captures({
    0: Some("cat"),
    1: Some("c"),
})),
Some(Captures({
    0: Some("hat"),
    1: Some("h"),
})),
Some(Captures({
    0: Some("pat"),
    1: Some("p"),
})),
"#;
        assert_eq!(expected, format_captures(regex, subject));
    }
}
