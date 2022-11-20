use std::rc::Rc;

use crate::{
    Item, Event, Source,
};

use crate::flextuple;

/// counts the number of items 
/// schema must have one u64 field named value
pub fn count<
    //I: 'static + std::fmt::Debug,
    //O: 'static,
    S,
>(
    schema: Rc<flextuple::FlexSchema>,
    reduce: bool,    
) -> Box<dyn Fn(S) -> Source<Rc<flextuple::FlexTuple>>>
where
    S: Into<Rc<Source<Rc<flextuple::FlexTuple>>>>,
    //O: Into<i32>,
{
    let schema_clone = schema.clone();
    Box::new(move |source| {  // connect
        let source: Rc<Source<Rc<flextuple::FlexTuple>>> = source.into();
        {
            let schema_clone1 = schema_clone.clone();
            move |event| {
                if let Event::Subscribe(sink, state_store) = event {
                    let state = state_store.create_state_i64("count");
                    let schema_clone2 = schema_clone1.clone();
                    source(
                        Event::Subscribe(                            
                            Rc::new({                                
                                move |event| {
                                    let schema_clone3 = schema_clone2.clone();
                                    match event {
                                        Event::PushItem(item) => {
                                            let mut s = state.borrow_mut();
                                            if let Some(key) = item.key.last() {
                                                let value = match s.get(*key) {
                                                    Some(value) => {
                                                        value + 1
                                                    },
                                                    None => 1
                                                };
                                                s.set(*key, value);
                                                if reduce == false {
                                                    let mut ft = flextuple::FlexTuple::new(schema_clone3);
                                                    ft.add_int64(value);
                                                    sink(
                                                        Event::PushItem(Item {
                                                            key: item.key,
                                                            value: Rc::new(ft),
                                                        }),
                                                    );
                                                }
                                            }
                                        },
                                        Event::PollItem => {
                                            panic!("source must not pull");
                                        },
                                        Event::KeyCreated(keys) => {
                                            let mut s = state.borrow_mut();
                                            if let Some(key) = keys.last() {
                                                (*s).create_key(*key);
                                            }
                                            sink(Event::KeyCreated(keys));
                                        },
                                        Event::KeyCompleted(keys) => {
                                            let mut s = state.borrow_mut();
                                            if let Some(key) = keys.last() {
                                                let k = key.clone();
                                                if reduce == true {
                                                    let value = match s.get(*key) {
                                                        Some(value) => value,
                                                        None => 0,
                                                    };
                                                    let mut ft = flextuple::FlexTuple::new(schema_clone3);
                                                    ft.add_int64(value);
                                                    sink(
                                                        Event::PushItem(Item {
                                                            key: keys.clone(),
                                                            value: Rc::new(ft),
                                                        }),
                                                    );
                                                }                                                
                                                (*s).delete_key(k);
                                            }
                                            sink(Event::KeyCompleted(keys));
                                        },
                                        other => {
                                            sink(other);
                                            //panic!("Unexpected event: {:?}", other);
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