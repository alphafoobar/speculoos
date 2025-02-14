use super::{AssertionFailure, DescriptiveSpec, Spec};

use std::borrow::Borrow;

pub trait StrAssertions {
    fn starts_with<'r, E: Borrow<&'r str>>(&mut self, expected: E);
    fn ends_with<'r, E: Borrow<&'r str>>(&mut self, expected: E);
    fn contains<'r, E: Borrow<&'r str>>(&mut self, expected: E);
    fn does_not_contain<'r, E: Borrow<&'r str>>(&mut self, expected: E);
    fn is_empty(&mut self);
}

impl<'s> StrAssertions for Spec<'s, &'s str> {
    /// Asserts that the subject `&str` starts with the provided `&str`.
    ///
    /// ```rust
    /// # use speculoos::prelude::*;
    /// assert_that(&"Hello").starts_with(&"H");
    /// ```
    fn starts_with<'r, E: Borrow<&'r str>>(&mut self, expected: E) {
        let subject = self.subject;
        starts_with(self, subject, expected);
    }

    /// Asserts that the subject `&str` ends with the provided `&str`.
    ///
    /// ```rust
    /// # use speculoos::prelude::*;
    /// assert_that(&"Hello").ends_with(&"o");
    /// ```
    fn ends_with<'r, E: Borrow<&'r str>>(&mut self, expected: E) {
        let subject = self.subject;
        ends_with(self, subject, expected);
    }

    /// Asserts that the subject `&str` contains the provided `&str`.
    ///
    /// ```rust
    /// # use speculoos::prelude::*;
    /// assert_that(&"Hello").contains(&"e");
    /// ```
    fn contains<'r, E: Borrow<&'r str>>(&mut self, expected: E) {
        let subject = self.subject;
        contains(self, subject, expected);
    }

    /// Asserts that the subject `&str` contains the provided `&str`.
    ///
    /// ```rust
    /// # use speculoos::prelude::*;
    /// assert_that(&"Hello").contains(&"e");
    /// ```
    fn does_not_contain<'r, E: Borrow<&'r str>>(&mut self, expected: E) {
        let subject = self.subject;
        does_not_contain(self, subject, expected);
    }

    /// Asserts that the subject `&str` is empty.
    ///
    /// ```rust
    /// # use speculoos::prelude::*;
    /// assert_that(&"").is_empty();
    /// ```
    fn is_empty(&mut self) {
        let subject = self.subject;
        is_empty(self, subject);
    }
}

impl<'s> StrAssertions for Spec<'s, String> {
    /// Asserts that the subject `String` starts with the provided `&str`.
    ///
    /// ```rust
    /// # use speculoos::prelude::*;
    /// assert_that(&"Hello".to_owned()).starts_with(&"H");
    /// ```
    fn starts_with<'r, E: Borrow<&'r str>>(&mut self, expected: E) {
        let subject = &self.subject;
        starts_with(self, subject, expected);
    }

    /// Asserts that the subject `String` ends with the provided `&str`.
    ///
    /// ```rust
    /// # use speculoos::prelude::*;
    /// assert_that(&"Hello".to_owned()).ends_with(&"o");
    /// ```
    fn ends_with<'r, E: Borrow<&'r str>>(&mut self, expected: E) {
        let subject = &self.subject;
        ends_with(self, subject, expected);
    }

    /// Asserts that the subject `String` contains the provided `&str`.
    ///
    /// ```rust
    /// # use speculoos::prelude::*;
    /// assert_that(&"Hello".to_owned()).contains(&"e");
    /// ```
    fn contains<'r, E: Borrow<&'r str>>(&mut self, expected: E) {
        let subject = &self.subject;
        contains(self, subject, expected);
    }

    /// Asserts that the subject `String` does not contain the provided `&str`.
    ///
    /// ```rust
    /// # use speculoos::prelude::*;
    /// assert_that(&"Hello".to_owned()).does_not_contain(&"x");
    /// ```
    fn does_not_contain<'r, E: Borrow<&'r str>>(&mut self, expected: E) {
        let subject = &self.subject;
        does_not_contain(self, subject, expected);
    }

    /// Asserts that the subject `String` is empty.
    ///
    /// ```rust
    /// # use speculoos::prelude::*;
    /// assert_that(&"".to_owned()).is_empty();
    /// ```
    fn is_empty(&mut self) {
        let subject = &self.subject;
        is_empty(self, subject);
    }
}

fn starts_with<'r, 's, S: DescriptiveSpec<'s>, E: Borrow<&'r str>>(
    spec: &'s S,
    subject: &str,
    expected: E,
) {
    let borrowed_expected = expected.borrow();

    if !subject.starts_with(borrowed_expected) {
        AssertionFailure::from_spec(spec)
            .with_expected(format!("string starting with <{:?}>", borrowed_expected))
            .with_actual(format!("<{:?}>", subject))
            .fail();
    }
}

fn ends_with<'r, 's, S: DescriptiveSpec<'s>, E: Borrow<&'r str>>(
    spec: &'s S,
    subject: &str,
    expected: E,
) {
    let borrowed_expected = expected.borrow();

    if !subject.ends_with(borrowed_expected) {
        AssertionFailure::from_spec(spec)
            .with_expected(format!("string ending with <{:?}>", borrowed_expected))
            .with_actual(format!("<{:?}>", subject))
            .fail();
    }
}

fn contains<'r, 's, S: DescriptiveSpec<'s>, E: Borrow<&'r str>>(
    spec: &'s S,
    subject: &str,
    expected: E,
) {
    let borrowed_expected = expected.borrow();

    if !subject.contains(borrowed_expected) {
        AssertionFailure::from_spec(spec)
            .with_expected(format!("string containing <{:?}>", borrowed_expected))
            .with_actual(format!("<{:?}>", subject))
            .fail();
    }
}

fn does_not_contain<'r, 's, S: DescriptiveSpec<'s>, E: Borrow<&'r str>>(
    spec: &'s S,
    subject: &str,
    expected: E,
) {
    let borrowed_expected = expected.borrow();

    if subject.contains(borrowed_expected) {
        AssertionFailure::from_spec(spec)
            .with_expected(format!("string not containing <{:?}>", borrowed_expected))
            .with_actual(format!("<{:?}>", subject))
            .fail();
    }
}

fn is_empty<'s, S: DescriptiveSpec<'s>>(spec: &'s S, subject: &str) {
    if !subject.is_empty() {
        AssertionFailure::from_spec(spec)
            .with_expected("an empty string".to_string())
            .with_actual(format!("<{:?}>", subject))
            .fail();
    }
}

#[cfg(test)]
mod tests {

    use super::super::prelude::*;

    #[test]
    fn should_allow_multiple_borrow_forms_for_str() {
        let value = "Hello";
        assert_that(&value).starts_with("H");
        assert_that(&value).starts_with(&mut "H");
        assert_that(&value).starts_with(&"H");

        assert_that(&value).ends_with("o");
        assert_that(&value).ends_with(&mut "o");
        assert_that(&value).ends_with(&"o");

        assert_that(&value).contains("l");
        assert_that(&value).contains(&mut "l");
        assert_that(&value).contains(&"l");
    }

    #[test]
    fn should_not_panic_if_str_starts_with_value() {
        let value = "Hello";
        assert_that(&value).starts_with(&"H");
    }

    #[test]
    #[should_panic(expected = "\n\texpected: string starting with <\"A\">\
                   \n\t but was: <\"Hello\">")]
    fn should_panic_if_str_does_not_start_with_value() {
        let value = "Hello";
        assert_that(&value).starts_with(&"A");
    }

    #[test]
    fn should_not_panic_if_str_ends_with_value() {
        let value = "Hello";
        assert_that(&value).ends_with(&"o");
    }

    #[test]
    #[should_panic(expected = "\n\texpected: string ending with <\"A\">\n\t but was: <\"Hello\">")]
    fn should_panic_if_str_does_not_end_with_value() {
        let value = "Hello";
        assert_that(&value).ends_with(&"A");
    }

    #[test]
    fn should_not_panic_if_str_contains_value() {
        let value = "Hello";
        assert_that(&value).contains(&"l");
    }

    #[test]
    fn should_not_panic_if_str_does_not_contains_value() {
        let value = "Hello";
        assert_that(&value).does_not_contain(&"x");
    }

    #[test]
    #[should_panic(
        expected = "\n\texpected: string not containing <\"l\">\n\t but was: <\"Hello\">"
    )]
    fn should_panic_if_str_contains_value() {
        let value = "Hello";
        assert_that(&value).does_not_contain(&"l");
    }

    #[test]
    #[should_panic(expected = "\n\texpected: string containing <\"A\">\n\t but was: <\"Hello\">")]
    fn should_panic_if_str_does_not_contain_value() {
        let value = "Hello";
        assert_that(&value).contains(&"A");
    }

    #[test]
    fn should_not_panic_if_str_is_empty() {
        let value = "";
        assert_that(&value).is_empty();
    }

    #[test]
    #[should_panic(expected = "\n\texpected: an empty string\n\t but was: <\"Hello\">")]
    fn should_panic_if_str_is_not_empty() {
        let value = "Hello";
        assert_that(&value).is_empty();
    }

    #[test]
    fn should_allow_multiple_borrow_forms_for_string() {
        let value = "Hello".to_owned();
        assert_that(&value).starts_with("H");
        assert_that(&value).starts_with(&mut "H");
        assert_that(&value).starts_with(&"H");

        assert_that(&value).ends_with("o");
        assert_that(&value).ends_with(&mut "o");
        assert_that(&value).ends_with(&"o");

        assert_that(&value).contains("l");
        assert_that(&value).contains(&mut "l");
        assert_that(&value).contains(&"l");
    }

    #[test]
    fn should_not_panic_if_string_starts_with_value() {
        let value = "Hello".to_owned();
        assert_that(&value).starts_with(&"H");
    }

    #[test]
    #[should_panic(expected = "\n\texpected: string starting with <\"A\">\
                   \n\t but was: <\"Hello\">")]
    fn should_panic_if_string_does_not_start_with_value() {
        let value = "Hello".to_owned();
        assert_that(&value).starts_with(&"A");
    }

    #[test]
    fn should_not_panic_if_string_ends_with_value() {
        let value = "Hello".to_owned();
        assert_that(&value).ends_with(&"o");
    }

    #[test]
    #[should_panic(expected = "\n\texpected: string ending with <\"A\">\n\t but was: <\"Hello\">")]
    fn should_panic_if_string_does_not_end_with_value() {
        let value = "Hello".to_owned();
        assert_that(&value).ends_with(&"A");
    }

    #[test]
    fn should_not_panic_if_string_contains_value() {
        let value = "Hello".to_owned();
        assert_that(&value).contains(&"l");
    }

    #[test]
    #[should_panic(expected = "\n\texpected: string containing <\"A\">\n\t but was: <\"Hello\">")]
    fn should_panic_if_string_does_not_contain_value() {
        let value = "Hello".to_owned();
        assert_that(&value).contains(&"A");
    }

    #[test]
    fn should_not_panic_if_string_is_empty() {
        let value = "".to_owned();
        assert_that(&value).is_empty();
    }

    #[test]
    #[should_panic(expected = "\n\texpected: an empty string\n\t but was: <\"Hello\">")]
    fn should_panic_if_string_is_not_empty() {
        let value = "Hello".to_owned();
        assert_that(&value).is_empty();
    }
}
