use std::rc::Rc;

use crate::{
    Item, Event, Source,
};


pub fn pop_key<
    T: 'static,
    S,
>() -> Box<dyn Fn(S) -> Source<T>>
where
    S: Into<Rc<Source<T>>>,
{
    Box::new(move |source| {  // connect
        let source: Rc<Source<T>> = source.into();
        {
            move |event| {
                if let Event::Subscribe(sink, state_store) = event {
                    source(
                        Event::Subscribe(Rc::new(
                            {
                                move |event| {
                                    match event {                                        
                                        Event::PushItem(data) => {
                                            let mut key = data.key.clone();
                                            key.pop();
                                            sink(
                                                Event::PushItem(Item {
                                                    key: key,
                                                    value: data.value,
                                                }),
                                            );
                                        },
                                        Event::PollItem => {
                                            panic!("source must not pull");
                                        },
                                        Event::Completed => {
                                            sink(Event::Completed);
                                        },
                                        Event::ForwardKeyCreated(key) => {
                                            sink(
                                                Event::KeyCreated(key),
                                            );
                                        },
                                        Event::ForwardKeyCompleted(key) => {
                                            sink(
                                                Event::KeyCompleted(key),
                                            );
                                        },
                                        // drop key events
                                        Event::KeyCreated(_) => {},
                                        Event::KeyCompleted(_) => {},
                                        _ => {
                                            panic!("Unexpected event");
                                        }
                                    }
                                }
                            }
                            .into(),
                        ),
                        state_store),
                    );
                }
            }
        }
        .into()
    })
}