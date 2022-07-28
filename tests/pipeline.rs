use crate::operators::{map, Message, Source};
use crate::sources::from_iterable;
use crate::sinks::for_each;

#[cfg(test)]
mod tests {

    #[test]
    fn run_a_simple_pipeline() {
        let i = [1, 2, 3, 4];
        let source = from_iterable::from_iterable(i);
        let op = operators::map::map(Box::new(| i | { i+1 }));
        op(source);
        assert_eq!(1, 1)
    }
}