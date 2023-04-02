use regex::Regex;
use std::fmt::Write;

use wasm_bindgen::prelude::*;
use web_sys::{HtmlElement, HtmlTextAreaElement, InputEvent};

fn main() {
    // Input containing pattern from which we construct the regex
    let pattern_input: HtmlTextAreaElement = document()
        .query_selector(".rs-pattern-input")
        .unwrap()
        .unwrap()
        .dyn_into()
        .unwrap();

    // Input containing string on which we run the regex
    let subject_input: HtmlTextAreaElement = document()
        .query_selector(".rs-subject-input")
        .unwrap()
        .unwrap()
        .dyn_into()
        .unwrap();

    // `pre` in which we render the results
    let output_pre: HtmlElement = document()
        .query_selector(".rs-output")
        .unwrap()
        .unwrap()
        .dyn_into()
        .unwrap();

    let input_event_closure = Closure::<dyn FnMut(_)>::new({
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
    pattern_input
        .add_event_listener_with_callback("input", input_event_closure.as_ref().unchecked_ref())
        .unwrap();
    subject_input
        .add_event_listener_with_callback("input", input_event_closure.as_ref().unchecked_ref())
        .unwrap();
    input_event_closure.forget();
}

fn run_regex(
    pattern_input: HtmlTextAreaElement,
    subject_input: HtmlTextAreaElement,
    output_pre: HtmlElement,
) {
    let pattern: String = pattern_input.value();
    let subject: String = subject_input.value();

    // We don't want to do anything if there is no pattern.
    // It will match anything
    if pattern == String::from("") {
        output_pre.set_text_content(Some(""));
        return;
    }

    let regex = match Regex::new(&pattern) {
        // If the pattern doesn't compile into a regex
        // render the error and halt.
        Err(e) => {
            output_pre.set_text_content(Some(&format!("{:?}", e)));
            return;
        }
        Ok(re) => re,
    };

    let formatted = format_captures(regex, &subject);
    output_pre.set_text_content(Some(&formatted));
}

fn format_captures(regex: regex::Regex, subject: &str) -> String {
    let mut buffer = String::new();

    for captures in regex.captures_iter(subject) {
        write!(&mut buffer, "Some(Captures({{\n").unwrap();

        for (i, cap) in captures.iter().enumerate() {
            write!(
                &mut buffer,
                "    {}: Some({:?}),\n",
                i,
                cap.unwrap().as_str()
            )
            .unwrap();
        }

        write!(&mut buffer, "}})),\n").unwrap();
    }

    if buffer == "" {
        String::from("None")
    } else {
        buffer
    }
}

pub fn document() -> web_sys::Document {
    web_sys::window()
        .expect_throw("Can't find the global Window")
        .document()
        .expect_throw("Can't find document")
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
