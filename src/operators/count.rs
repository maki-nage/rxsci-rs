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
                    let state = state_store.create_state_i32("count");
                    {
                        let key = vec![0];
                        let mut s = state.borrow_mut();
                        s.create_key(&key);                        
                    }
                    source(
                        Event::Subscribe(                            
                            Rc::new({
                                move |event| {
                                    match event {
                                        Event::PushItem(item) => {
                                            let mut s = state.borrow_mut();
                                            let value = *s.get(&item.key) + 1;
                                            s.set(&item.key, &Rc::new(value));
                                            if reduce == false {
                                                sink(
                                                    Event::PushItem(Item {
                                                        key: item.key,
                                                        value: value,
                                                    }),
                                                );
                                            }
                                        },
                                        Event::PollItem => {
                                            panic!("source must not pull");
                                        },
                                        Event::Completed => {
                                            if reduce == true {
                                                let key = vec![0];
                                                let mut s = state.borrow_mut();
                                                let value = *s.get(&key);
                                                sink(
                                                    Event::PushItem(Item {
                                                        key: key,
                                                        value: value,
                                                    }),
                                                );
                                            }
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