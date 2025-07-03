//
// Let's aggressively deny every lint that isn't already deny.
//
#![deny(absolute_paths_not_starting_with_crate)]
#![deny(box_pointers)]
#![deny(elided_lifetimes_in_paths)]
#![deny(explicit_outlives_requirements)]
#![deny(keyword_idents)]
#![deny(let_underscore_drop)]
#![deny(macro_use_extern_crate)]
#![deny(meta_variable_misuse)]
#![deny(missing_abi)]
#![deny(missing_copy_implementations)]
#![deny(missing_debug_implementations)]
#![deny(non_ascii_idents)]
#![deny(noop_method_call)]
#![deny(pointer_structural_match)]
#![deny(rust_2021_incompatible_closure_captures)]
#![deny(rust_2021_incompatible_or_patterns)]
#![deny(rust_2021_prefixes_incompatible_syntax)]
#![deny(rust_2021_prelude_collisions)]
#![deny(single_use_lifetimes)]
#![deny(trivial_casts)]
#![deny(trivial_numeric_casts)]
#![deny(unreachable_pub)]
#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unstable_features)]
#![deny(unused_crate_dependencies)]
#![deny(unused_extern_crates)]
#![deny(unused_import_braces)]
#![deny(unused_lifetimes)]
#![deny(unused_macro_rules)]
#![deny(unused_qualifications)]
#![deny(unused_results)]
#![deny(unused_tuple_struct_fields)]
#![deny(variant_size_differences)]
#![deny(anonymous_parameters)]
#![deny(array_into_iter)]
#![deny(asm_sub_register)]
#![deny(bad_asm_style)]
#![deny(bare_trait_objects)]
#![deny(break_with_label_and_loop)]
#![deny(clashing_extern_declarations)]
#![deny(coherence_leak_check)]
#![deny(confusable_idents)]
#![deny(const_evaluatable_unchecked)]
#![deny(const_item_mutation)]
#![deny(dead_code)]
#![deny(deprecated)]
#![deny(deprecated_where_clause_location)]
#![deny(deref_into_dyn_supertrait)]
#![deny(deref_nullptr)]
#![deny(drop_bounds)]
#![deny(duplicate_macro_attributes)]
#![deny(dyn_drop)]
#![deny(ellipsis_inclusive_range_patterns)]
#![deny(exported_private_dependencies)]
#![deny(for_loops_over_fallibles)]
#![deny(forbidden_lint_groups)]
#![deny(function_item_references)]
#![deny(illegal_floating_point_literal_pattern)]
#![deny(improper_ctypes)]
#![deny(improper_ctypes_definitions)]
#![deny(incomplete_features)]
#![deny(indirect_structural_match)]
#![deny(inline_no_sanitize)]
#![deny(invalid_doc_attributes)]
#![deny(invalid_value)]
#![deny(irrefutable_let_patterns)]
#![deny(large_assignments)]
#![deny(late_bound_lifetime_arguments)]
#![deny(legacy_derive_helpers)]
#![deny(mixed_script_confusables)]
#![deny(named_arguments_used_positionally)]
#![deny(no_mangle_generic_items)]
#![deny(non_camel_case_types)]
#![deny(non_fmt_panics)]
#![deny(non_shorthand_field_patterns)]
#![deny(non_snake_case)]
#![deny(non_upper_case_globals)]
#![deny(nontrivial_structural_match)]
#![deny(opaque_hidden_inferred_bound)]
#![deny(overlapping_range_endpoints)]
#![deny(path_statements)]
#![deny(private_in_public)]
#![deny(redundant_semicolons)]
#![deny(renamed_and_removed_lints)]
#![deny(repr_transparent_external_private_fields)]
#![deny(semicolon_in_expressions_from_macros)]
#![deny(special_module_name)]
#![deny(stable_features)]
#![deny(temporary_cstring_as_ptr)]
#![deny(trivial_bounds)]
#![deny(type_alias_bounds)]
#![deny(tyvar_behind_raw_pointer)]
#![deny(uncommon_codepoints)]
#![deny(unconditional_recursion)]
#![deny(unexpected_cfgs)]
#![deny(ungated_async_fn_track_caller)]
#![deny(uninhabited_static)]
#![deny(unknown_lints)]
#![deny(unnameable_test_items)]
#![deny(unreachable_code)]
#![deny(unreachable_patterns)]
#![deny(unstable_name_collisions)]
#![deny(unstable_syntax_pre_expansion)]
#![deny(unsupported_calling_conventions)]
#![deny(unused_allocation)]
#![deny(unused_assignments)]
#![deny(unused_attributes)]
#![deny(unused_braces)]
#![deny(unused_comparisons)]
#![deny(unused_doc_comments)]
#![deny(unused_features)]
#![deny(unused_imports)]
#![deny(unused_labels)]
#![deny(unused_macros)]
#![deny(unused_must_use)]
#![deny(unused_mut)]
#![deny(unused_parens)]
#![deny(unused_unsafe)]
#![deny(unused_variables)]
#![deny(warnings)]
#![deny(while_true)]
//
// And also every clippy lint
//
#![deny(clippy::all)]

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
