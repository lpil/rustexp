use fancy_regex::Regex as FancyRegex;
use regex::Regex;
use std::fmt::Write;

use wasm_bindgen::prelude::*;
use web_sys::{Event, HtmlElement, HtmlInputElement, HtmlTextAreaElement};

fn main() {
    // Input containing pattern from which we construct the regex
    let pattern_input: HtmlTextAreaElement =
        get_element_by_query_selector(".rs-pattern-input").unwrap();

    // Input containing string on which we run the regex
    let subject_input: HtmlTextAreaElement =
        get_element_by_query_selector(".rs-subject-input").unwrap();

    // `pre` in which we render the results
    let output_pre: HtmlElement = get_element_by_query_selector(".rs-output").unwrap();

    // `input` for choosing between `regex` and `fancy-regex`
    let fancy_regex: HtmlInputElement = get_element_by_query_selector(".fancy-regex").unwrap();

    // `body` element
    let body: HtmlElement = get_element_by_query_selector("body").unwrap();

    let input_event_closure = Closure::<dyn FnMut(_)>::new({
        let pattern_input = pattern_input.clone();
        let subject_input = subject_input.clone();
        let output_pre = output_pre.clone();
        let fancy_regex = fancy_regex.clone();
        let body = body.clone();

        move |_: Event| {
            run_regex(
                pattern_input.clone(),
                subject_input.clone(),
                output_pre.clone(),
                fancy_regex.clone(),
                body.clone(),
            );
        }
    });
    pattern_input
        .add_event_listener_with_callback("input", input_event_closure.as_ref().unchecked_ref())
        .unwrap();
    subject_input
        .add_event_listener_with_callback("input", input_event_closure.as_ref().unchecked_ref())
        .unwrap();
    fancy_regex
        .add_event_listener_with_callback("change", input_event_closure.as_ref().unchecked_ref())
        .unwrap();

    input_event_closure.forget();

    run_regex(pattern_input, subject_input, output_pre, fancy_regex, body);
}

fn run_regex(
    pattern_input: HtmlTextAreaElement,
    subject_input: HtmlTextAreaElement,
    output_pre: HtmlElement,
    fancy_regex: HtmlInputElement,
    body: HtmlElement,
) {
    let pattern: String = pattern_input.value();
    let subject: String = subject_input.value();
    let use_fancy_regex = fancy_regex.checked();

    body.set_class_name(if use_fancy_regex {
        "fancy-regex"
    } else {
        "regex"
    });

    // We don't want to do anything if there is no pattern.
    // It will match anything
    if pattern.is_empty() {
        output_pre.set_text_content(Some(""));
        return;
    }

    let formatted = if !use_fancy_regex {
        let regex = match Regex::new(&pattern) {
            // If the pattern doesn't compile into a regex
            // render the error and halt.
            Err(e) => {
                output_pre.set_text_content(Some(&format!("{:?}", e)));
                return;
            }
            Ok(re) => re,
        };

        format_captures(regex, &subject)
    } else {
        let regex = match FancyRegex::new(&pattern) {
            // If the pattern doesn't compile into a regex
            // render the error and halt.
            Err(e) => {
                output_pre.set_text_content(Some(&format!("{:?}", e)));
                return;
            }
            Ok(re) => re,
        };

        format_captures_fancy(regex, &subject)
    };
    output_pre.set_text_content(Some(&formatted));
}

fn format_captures(regex: Regex, subject: &str) -> String {
    let mut buffer = String::new();

    for captures in regex.captures_iter(subject) {
        writeln!(&mut buffer, "Some(Captures({{").unwrap();

        for (i, cap) in captures.iter().enumerate() {
            match cap {
                Some(cap) => {
                    writeln!(&mut buffer, "    {}: Some({:?}),", i, cap.as_str()).unwrap();
                }
                None => {
                    writeln!(&mut buffer, "    {}: None,", i).unwrap();
                }
            }
        }

        writeln!(&mut buffer, "}})),").unwrap();
    }

    if buffer.is_empty() {
        buffer.push_str("None")
    }
    buffer
}

fn format_captures_fancy(regex: FancyRegex, subject: &str) -> String {
    let mut buffer = String::new();

    for captures in regex.captures_iter(subject) {
        writeln!(&mut buffer, "Some(Captures({{").unwrap();

        for cap in captures.iter() {
            for (i, c) in cap.iter().flatten().enumerate() {
                writeln!(&mut buffer, "    {}: Some({:?}),", i, c.as_str()).unwrap();
            }
        }

        writeln!(&mut buffer, "}})),").unwrap();
    }

    if buffer.is_empty() {
        buffer.push_str("None")
    }
    buffer
}

fn document() -> web_sys::Document {
    web_sys::window()
        .expect_throw("Can't find the global Window")
        .document()
        .expect_throw("Can't find document")
}

fn get_element_by_query_selector<T: JsCast>(selector: &str) -> Option<T> {
    document().query_selector(selector).ok()??.dyn_into().ok()
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
