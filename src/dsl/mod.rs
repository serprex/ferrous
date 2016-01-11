mod matchers;
mod assertions;

use dsl::assertions::*;
use dsl::matchers::*;

pub trait Assertion<A>: Sized {
    fn to<M: Matcher<A>>(self, matcher: M);

    fn not_to<M: Matcher<A>>(self, matcher: M);
}

pub trait Matcher<A> {
    fn matches(&self, actual: &A) -> bool;
    fn failure_message(&self, actual: &A) -> String;
    fn negated_failure_message(&self, actual: &A) -> String;
}


pub fn expect<A>(actual: &A) -> Expect<A> {
    Expect::new(actual)
}

// pub fn eventually<F, A>(f: F) -> Eventually<A> where F: Fn() -> A {
// }

// pub fn consistently<F, A>(f: F) -> Consistently<A> where F: Fn() -> A {
// }

pub fn equal<'a, E>(expected: &'a E) -> Equals<'a, E> {
    Equals::new(expected)
}