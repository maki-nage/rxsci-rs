use std::rc::Rc;

use crate::{
    Item, Event, Source,
};


pub fn map<
    I: 'static,
    O: 'static,
    F: 'static,
    S,
>(
    f: F,
) -> Box<dyn Fn(S) -> Source<O>>
where
    F: Fn(I) -> O + Clone, // + Send + Sync,
    S: Into<Rc<Source<I>>>,
{
    Box::new(move |source| {  // connect
        println!("map entry");
        let source: Rc<Source<I>> = source.into();
        {
            println!("map0");
            let f = f.clone();
            println!("map1");
            move |event| {
                if let Event::Subscribe(sink, state_store) = event {
                    source(
                        Event::Subscribe(Rc::new(
                            {
                                let f = f.clone();
                                move |event| {
                                    match event {
                                        Event::PushItem(item) => {
                                            sink(
                                                Event::PushItem(Item {
                                                    key: item.key,
                                                    value: f(item.value),
                                                }),
                                            );
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