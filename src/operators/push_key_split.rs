use std::rc::Rc;

use crate::{
    Item, Event, Source,
};
use crate::flextuple;


pub fn push_key_split<
    T: 'static,
    S,
    F: 'static,
>(
    key_mapper: F,
) -> Box<dyn Fn(S) -> Source<T>>
where
    F: Fn(&T) -> Rc<flextuple::FlexTuple> + Clone, // + Clone, // + Send + Sync,
    S: Into<Rc<Source<T>>>,
{
    Box::new(move |source| {  // connect
        let source: Rc<Source<T>> = source.into();
        {
            let key_mapper = key_mapper.clone();
            move |event| {
                if let Event::Subscribe(sink, state_store) = event {
                    //let state = state_store.create_state_i64("push_key_split");
                    let state = state_store.create_state_ft("push_key_split");
                    source(
                        Event::Subscribe(                            
                            Rc::new({             
                                let key_mapper = key_mapper.clone();                   
                                move |event| {
                                    match event {
                                        Event::PushItem(item) => {
                                            let new_key = key_mapper(&item.value);
                                            let mut s = state.borrow_mut();                                            

                                            if let Some(k) = item.key.last() {
                                                let mut split_key = item.key.clone();
                                                split_key.push(0);

                                                match s.get_rc(*k) {
                                                    Some(current_key) => {
                                                        if *current_key != *new_key {
                                                            //s.set(*k, &Rc::new(*new_key));
                                                            s.set_rc(*k, &new_key.clone());
                                                            sink(Event::KeyCompleted(split_key.clone()));
                                                            sink(Event::KeyCreated(split_key.clone()));
                                                        }
                                                    },
                                                    None => { s.set_rc(*k, &new_key.clone()); }
                                                }

                                                sink(
                                                    Event::PushItem(Item {
                                                        key: split_key,
                                                        value: item.value,
                                                    }),
                                                );
                                            }
                                            else {
                                                panic!("push_key_split: not key");
                                            }
                                        },
                                        Event::KeyCreated(keys) => {
                                            if let Some(key) = keys.last() {
                                                let mut s = state.borrow_mut();
                                                (*s).create_key(*key);

                                                let mut split_key = keys.clone();
                                                split_key.push(0);
                                                sink(Event::KeyCreated(split_key));
                                                sink(Event::ForwardKeyCreated(keys, 0));
                                            }
                                        }
                                        Event::KeyCompleted(keys) => {
                                            if let Some(key) = keys.last() {
                                                let mut s = state.borrow_mut();

                                                let mut split_key = keys.clone();
                                                split_key.push(0);
                                                sink(Event::KeyCompleted(split_key));

                                                (*s).delete_key(*key);
                                                sink(Event::ForwardKeyCompleted(keys, 0));
                                            }
                                        }
                                        Event::ForwardKeyCreated(key, level) => {
                                            sink(
                                                Event::ForwardKeyCreated(key, level),
                                            );
                                        },
                                        Event::ForwardKeyCompleted(key, level) => {
                                            sink(
                                                Event::ForwardKeyCompleted(key, level)
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
                        state_store
                        ),
                    );
                }
            }
        }
        .into()
    })
}