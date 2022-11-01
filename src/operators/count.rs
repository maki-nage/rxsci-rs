use std::rc::Rc;
use std::sync::Arc;

use crate::{
    Item, Event, Source,
};

use crate::flextuple;


struct State {
    pub count: Vec<i32>,
}

/// counts the number of items 
/// schema must have one u64 field named value
pub fn count<
    I: 'static,
    //O: 'static,
    S,
>(
    schema: Rc<flextuple::FlexSchema>,
    reduce: bool,    
) -> Box<dyn Fn(S) -> Source<Rc<flextuple::FlexTuple>>>
where
    S: Into<Rc<Source<I>>>,
    //O: Into<i32>,
{
    let schema_clone = schema.clone();
    Box::new(move |source| {  // connect
        let source: Rc<Source<I>> = source.into();
        {
            let schema_clone1 = schema_clone.clone();
            move |event| {
                if let Event::Subscribe(sink, state_store) = event {
                    let state = state_store.create_state_i64("count");
                    {
                        let key = vec![0];
                        let mut s = state.borrow_mut();
                        s.create_key(&key);                        
                    }
                    let schema_clone2 = schema_clone1.clone();
                    source(
                        Event::Subscribe(                            
                            Rc::new({                                
                                move |event| {
                                    let schema_clone3 = schema_clone2.clone();
                                    match event {
                                        Event::PushItem(item) => {
                                            let mut s = state.borrow_mut();
                                            let value = *s.get(&item.key) + 1;
                                            s.set(&item.key, &Rc::new(value));
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
                                        },
                                        Event::PollItem => {
                                            panic!("source must not pull");
                                        },
                                        Event::Completed => {
                                            if reduce == true {
                                                let key = vec![0];
                                                let mut s = state.borrow_mut();
                                                let value = *s.get(&key);

                                                let mut ft = flextuple::FlexTuple::new(schema_clone3);
                                                ft.add_int64(value);
                                                sink(
                                                    Event::PushItem(Item {
                                                        key: key,
                                                        value: Rc::new(ft),
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