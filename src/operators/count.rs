use std::rc::Rc;
use std::sync::Arc;
use arc_swap::ArcSwap;

use crate::{
    Item, Event, Source,
};


struct State {
    pub count: Vec<i32>,
}

pub fn count<
    I: 'static,
    //O: 'static,
    S,
>(
    reduce: bool,
) -> Box<dyn Fn(S) -> Source<i32>>
where
    S: Into<Rc<Source<I>>>,
    //O: Into<i32>,
{
    Box::new(move |source| {  // connect
        let source: Rc<Source<I>> = source.into();
        {
            move |event| {
                if let Event::Subscribe(sink, state_store) = event {
                    //let state = State { count: vec![0]};
                    //let state = ArcSwap::from_pointee(0);
                    source(
                        Event::Subscribe(
                            Rc::new({
                                move |event| {
                                    match event {
                                        Event::PushItem(item) => {
                                            //state.count[0] += 1;
                                            //state.store(Arc::new(**state.load() + 1));
                                            if reduce == false {
                                                sink(
                                                    Event::PushItem(Item {
                                                        key: item.key,
                                                        //value: **state.load(),
                                                        value: 0,
                                                    }),
                                                );
                                            }
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
                        state_store
                        ),
                    );
                }
            }
        }
        .into()
    })
}