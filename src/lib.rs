//pub use crate::map::map;
//use crate::sources::from_iterable::from_iterable;
pub use crate::core::*;
use never::Never;
use std::rc::Rc;
use std::cell::RefCell;

use env_logger;

mod core;
//mod map;
mod operators;
mod sources;
//mod sinks;
mod state;

//type Operator<T> = dyn Fn(core::Source<T>) -> core::Source<T>;
type OperatorConnectFn<T> = Box<dyn Fn(core::Source<T>) -> core::Source<T>>;

pub struct Operator {
    op: OperatorConnectFn<i32>
}


pub struct StateStore {
    store: Rc<dyn state::state::StateStore>
}

pub struct SourceOp {
    src: core::Source<i32>
}

pub struct SourceSubscription {
    sink: Rc<core::Callbag<i32, i32>>,
    user_index: i64,
}

impl Operator {
    pub fn new(op: OperatorConnectFn<i32>) -> Self {
        Operator {
            op: op,            
        }
    }
}

impl StateStore {
    pub fn new(store: Rc<dyn state::state::StateStore>) -> Self {
        StateStore {
            store: store,
        }
    }
}

impl SourceOp {
    pub fn new(src: core::Source<i32>) -> Self {
        SourceOp {
            src: src,
        }
    }
}

impl SourceSubscription {
    pub fn new(sink: Rc<core::Callbag<i32, i32>>, user_index: i64) -> Self {
        SourceSubscription {
            sink: sink,
            user_index: user_index,
        }
    }

    pub fn on_next(&self, i: i32) {
        (self.sink)(core::Event::PushItem(core::Item {
            key: vec![0],
            value: i,
        }));
    }
}

pub struct Pipeline{
    value: u32,
    ops: Vec<Box<Operator>>,
}

impl Pipeline {
    pub fn new() -> Self {
        Pipeline {
            value: 42,
            ops: vec!()
        }
    }

    pub fn add_operator(&mut self, op: Box<Operator>) {
        self.ops.push(op);
    }

    fn compose(&self, source: Source<i32>) -> Callbag<Never, i32> {
        println!("{:p}", &source);
        let mut callbag = source;
        

        for op in &self.ops {
            println!("op: {:p}", op.op);
            callbag = (op.op)(callbag);
        }
        callbag
    }

    pub fn run(&self, source: Box<SourceOp>) {
        let i = [1, 2, 3, 4];
        println!("{:p}", &i);
        //let source = sources::from_iterable::from_iterable(i);
        let source = source.src;
        println!("created source");
        let callbag = self.compose(source);
        /*
        let sink = sinks::for_each::for_each(Box::new(|i| {
            println!("{}", i);
        }));
        sink(callbag);
        */
        println!("run out") 
    }

    pub fn subscribe(&self, source: Box<SourceOp>, state: Rc<dyn state::state::StateStore>, on_next: fn(i32, i32), index: i32) {
        println!("subscribe");
        let source = source.src;
        let callbag = self.compose(source);
        let sink = operators::forward::forward();
        let push_fn = sink(callbag);
        println!("created subscribe sink");
        push_fn(
            core::Event::Subscribe(Rc::new(
                {
                    move |event| {
                        match event {
                            Event::PushItem(item) => {
                                println!("PushItem in subscribe: {}, {}", index, item.value);
                                on_next(index, item.value);
                                println!("done");
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
            ), state),
        );
    }
}

#[no_mangle]
pub extern "C" fn map(f: fn(i32, i32) -> i32, index: i32) -> *const Operator {
    // c_void
    let op = operators::map::map(Box::new(move | i | {
        f(index, i)
    }));
    Box::into_raw(Box::new(Operator::new(op)))
}

/*
no_mangle]
pub extern "C" fn for_each() -> *const Operator {
    let op = sinks::for_each::for_each(Box::new(|i| {
        println!("{}", i);
    }));
    Box::into_raw(Box::new(Operator::new(op)))
}
*/

#[no_mangle]
pub extern "C" fn from_external_source(f: fn(i32, i64, *const SourceSubscription), index: i64) -> *const SourceOp {
    // c_void
    let src = sources::from_external_source::from_external_source(Box::new(move | e : Event<i32, i32>| {
        //if let Event::Subscribe(sink) = event {
        //}
        match e {
            Event::Subscribe(sink, state_store) => {
                let mut s = Box::new(SourceSubscription::new(sink, index));
                let raw_s = Box::into_raw(s);
                println!("receive subscribe from_external_source");
                f(0, index, raw_s);
            }
            _ => {

            }
        };
    }));
    Box::into_raw(Box::new(SourceOp::new(src)))
}

#[no_mangle]
pub extern "C" fn external_source_on_next(p_source: *mut SourceSubscription, i: i32) {
    let mut source = unsafe { &mut (*p_source) };
    source.on_next(i);
}


#[no_mangle]
pub extern "C" fn to_external_sink(f: fn(i: i32), index: i32) -> *const Operator {
    // c_void
    let op = operators::map::map(Box::new(move | i | { i }));
    Box::into_raw(Box::new(Operator::new(op)))
}


#[no_mangle]
pub extern "C" fn create_memory_state_store() -> *const StateStore {
//*const state::state::StateStore {
    let mut p = Rc::new(state::state::MemoryStateStore::new());
    let ss = Rc::new(StateStore::new(p));
    //Rc::into_raw(p) as *const Rc<dyn state::state::StateStore> as *const libc::c_void
    Rc::into_raw(ss)
}


#[no_mangle]
pub extern "C" fn create_pipeline() -> *mut Pipeline {
    let p = Box::new(Pipeline::new());
    Box::into_raw(p)
}

#[no_mangle]
pub extern "C" fn pipeline_add_operator(pipeline_ptr: *mut Pipeline, 
                                        op_ptr: *mut Operator) {
    println!("enter");
    if pipeline_ptr.is_null() {
        log::error!("provided pipeline is NULL");
        //return ptr::null();
        return;
    }
    println!("pipeline addr: {:p}", pipeline_ptr);
    let mut pipeline = unsafe { &mut (*pipeline_ptr) };
    //let mut pipeline = unsafe { Box::from_raw(pipeline_ptr) };
    //let mut op = unsafe { &(*op_ptr) };
    println!("op addr: {:p}", op_ptr);
    let mut op: Box<Operator> = unsafe { Box::from_raw(op_ptr) };
    //let mut raw_p = unsafe { &mut *(p_pipeline as *mut Pipeline) };
    //let mut pipeline = unsafe { Box::from_raw(raw_p) };
    println!("value: {}", &pipeline.value);
    
    //let op = unsafe { Box::from_raw(p_op) };
    pipeline.add_operator(op);
    //println!("create p");
    //let mut p = Pipeline::new();
    //println!("add map to p");
    //p.add_operator(operators::map::map(Box::new(| i | { i+1 })));
    //println!("run p");
    //pipeline.run();
    //println!("ran p");
}

#[no_mangle]
pub extern "C" fn pipeline_run(p_pipeline: *mut Pipeline, p_source: *mut SourceOp) {
    //let mut pipeline = unsafe { Box::from_raw(p_pipeline) };
    let mut pipeline = unsafe { &mut (*p_pipeline) };
    let mut source: Box<SourceOp> = unsafe { Box::from_raw(p_source) };

    pipeline.run(source);
}

#[no_mangle]
pub extern "C" fn pipeline_subscribe(
    p_pipeline: *mut Pipeline,
    p_source: *mut SourceOp,
    p_state_store: *mut StateStore,
    on_next: fn(i32, i32),
    index: i32,
) {
    //let mut pipeline = unsafe { Box::from_raw(p_pipeline) };
    let mut pipeline = unsafe { &mut (*p_pipeline) };
    let mut source: Box<SourceOp> = unsafe { Box::from_raw(p_source) };

    println!("pipeline_subscribe");
    ////let mut state_store: Rc<dyn state::state::StateStore> = unsafe { Rc::from_raw(p_state_store) };
    let state_store: Rc<StateStore> = unsafe { Rc::from_raw(p_state_store) };
    //let mut state_store = Rc::new(state::state::MemoryStateStore::new());

    //on_next(0, 0);
    println!("done next");
    pipeline.subscribe(source, state_store.store.clone(), on_next, index);
}


/*
#[no_mangle]
pub extern "C" fn compose(p_op1: *mut Operator<i32>, p_op2: *mut Operator<i32>) -> *const Callbag<i32, i32> {
    let op1: Operator<i32> = unsafe { Box::from_raw(*p_op1) };
    let op2: Operator<i32> = unsafe { Box::from_raw(*p_op2) };

    let op = op2(op1);
    Rc::into_raw(op)
}
*/