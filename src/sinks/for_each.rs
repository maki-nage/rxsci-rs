use std::rc::Rc;

use crate::{
    Event, Source,
};

pub fn for_each<
    T: 'static,
    F: 'static,
    S,
>(
    f: F,
) -> Box<dyn Fn(S)>
where
    F: Fn(T) + Clone, //  + Send + Sync,
    S: Into<Rc<Source<T>>>,
{
    Box::new(move |source| { // connect
        let source: Rc<Source<T>> = source.into();
        {
            let f = f.clone();
            source(
                Event::Subscribe(Rc::new(
                    {
                        let f = f.clone();
                        move |event| {
                            match event {
                                Event::PushItem(item) => {
                                    f(item.value);
                                },
                                Event::PollItem => {
                                    panic!("sink must not pull");
                                },
                                Event::Completed => {
                                    print!("completed");
                                },
                                _ => {
                                    panic!("Unexpected event");
                                }
                            }
                        }
                    }
                    .into(),
                )),
            );
        }
        .into()
    })
}