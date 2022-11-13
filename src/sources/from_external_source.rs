//use pyo3::prelude::*;
use std::rc::Rc;

use crate::{
    Event, Source,
};


pub fn from_external_source<
    T: 'static,
    G: 'static,
>(
    external_source: G,
    //dispose: G,
//) -> Box<dyn Fn() -> Source<T>>
) -> Source<T>
where
    G: Fn(Event<T, T>) + Clone, // + Send + Sync,
{
    //Box::new(move || {  // connect
       ({
            move |event| {
                if let Event::Subscribe(sink, state_store) = event {
                    external_source(
                        Event::Subscribe(Rc::new(
                            {
                                move |event| {
                                    match event {
                                        Event::PushItem(data) => {
                                            sink(
                                                Event::PushItem(data),
                                            );
                                        },
                                        Event::KeyCreated(key) => {
                                            sink(Event::KeyCreated(key));
                                        },
                                        Event::KeyCompleted(key) => {
                                            sink(Event::KeyCompleted(key));
                                        },
                                        Event::PollItem => {
                                            panic!("source must not pull");
                                        },
                                        Event::Completed => {
                                            sink(Event::Completed);
                                        },
                                        _ => {
                                            panic!("Unexpected event");
                                        }
                                    }
                                }
                            }
                            .into()
                        ),
                        state_store)
                    );
                }
            }
        })
        .into()
    //})
}