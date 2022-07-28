//use pyo3::prelude::*;
//use std::rc::Rc;
use std::{
    iter::IntoIterator,
};

use crate::{
    Item, Event, Source,
};


pub fn from_iterable<
    T: 'static,
    I: 'static,
>(
    iter: I,
//) -> Box<dyn Fn() -> Source<T>>
) -> Source<T>
where
//    S: Into<Rc<Source<I>>>,
    //T: Send + Sync,
    I: IntoIterator<Item = T> + Clone + Copy,
    //<I as IntoIterator>::IntoIter,
{
    //Box::new(move || {  // connect
       ({
            move |event| {
                if let Event::Subscribe(sink, state_store) = event {
                    let iter = iter.clone().into_iter();
                    for i in iter {
                        sink(
                            Event::PushItem(Item {
                                key: vec![0],
                                value: i,
                            })
                        );
                    }
                    sink(Event::Completed);
                }
            }
        })
        .into()
    //})
}