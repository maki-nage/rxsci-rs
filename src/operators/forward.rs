use std::rc::Rc;

use crate::{
    Event, Source,
};


pub fn forward<
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
                                            println!("PushItem");
                                            sink(
                                                Event::PushItem(data),
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