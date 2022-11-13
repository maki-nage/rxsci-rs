//pub use crate::map::map;
//use crate::sources::from_iterable::from_iterable;
pub use crate::core::*;
use never::Never;
use std::rc::Rc;
use std::ffi::{CStr, c_void};
use std::os::raw::c_char;

mod core;
//mod map;
mod operators;
mod sources;
//mod sinks;
mod state;
mod flextuple;

//type Operator<T> = dyn Fn(core::Source<T>) -> core::Source<T>;
type OperatorConnectFn<T> = Box<dyn Fn(core::Source<T>) -> core::Source<T>>;

pub struct Operator {
    op: OperatorConnectFn<Rc<flextuple::FlexTuple>>
}


pub struct StateStore {
    store: Rc<dyn state::state::StateStore>
}

pub struct SourceOp {
    src: core::Source<Rc<flextuple::FlexTuple>>
}

pub struct SourceSubscription {
    sink: Rc<core::Callbag<Rc<flextuple::FlexTuple>, Rc<flextuple::FlexTuple>>>,
    user_index: i64,
}

impl Operator {
    pub fn new(op: OperatorConnectFn<Rc<flextuple::FlexTuple>>) -> Self {
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
    pub fn new(src: core::Source<Rc<flextuple::FlexTuple>>) -> Self {
        SourceOp {
            src: src,
        }
    }
}

impl SourceSubscription {
    pub fn new(sink: Rc<core::Callbag<Rc<flextuple::FlexTuple>, Rc<flextuple::FlexTuple>>>, user_index: i64) -> Self {
        sink(Event::KeyCreated(vec![0]));
        SourceSubscription {
            sink: sink,
            user_index: user_index,
        }
    }

    pub fn on_next(&self, i: Rc<flextuple::FlexTuple>) {
        (self.sink)(core::Event::PushItem(core::Item {
            key: vec![0],
            value: i.clone(),
        }));
    }

    pub fn on_completed(&self) {
        (self.sink)(Event::KeyCompleted(vec![0]));
        (self.sink)(core::Event::Completed);
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

    fn compose(
        &self,
        source: Source<Rc<flextuple::FlexTuple>>
    ) -> Callbag<Never, Rc<flextuple::FlexTuple>> {
        //println!("{:p}", &source);
        let mut callbag = source;
        

        for op in &self.ops {
            //println!("op: {:p}", op.op);
            callbag = (op.op)(callbag);
        }
        callbag
    }

    pub fn run(&self, source: Box<SourceOp>) {
        let source = source.src;
        let _callbag = self.compose(source);
    }

    pub fn subscribe(
        &self,
        source: Box<SourceOp>, 
        state: Rc<dyn state::state::StateStore>,
        on_next: extern fn(i32, *const flextuple::FlexTuple),
        index: i32
    ) {
        //println!("subscribe");
        let source = source.src;
        let callbag = self.compose(source);
        let sink = operators::forward::forward();
        let push_fn = sink(callbag);
        //println!("created subscribe sink");
        push_fn(
            core::Event::Subscribe(Rc::new(
                {
                    move |event: Event<Rc<flextuple::FlexTuple>, never::Never>| {
                        match event {
                            Event::PushItem(item) => {
                                //println!("PushItem in subscribe: {}, {}", index, item.value);
                                on_next(index, Rc::into_raw(item.value.clone()));
                                //println!("done");
                            },
                            Event::KeyCreated(key) => {},
                            Event::KeyCompleted(key) => {},
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
pub extern "C" fn map(
    f: extern fn(i32, *const flextuple::FlexTuple) -> *const flextuple::FlexTuple,
    index: i32
) -> *const Operator {
    // c_void
    let op = operators::map::map(Box::new(move | i | {
        let r = f(index, Rc::into_raw(i));
        //let mut ft_wrap = unsafe { &(*r) };
        let ft = unsafe { Rc::from_raw(r) };
        ft.clone()
    }));
    Box::into_raw(Box::new(Operator::new(op)))
}


#[no_mangle]
pub extern "C" fn count(
    schema: *const Rc<flextuple::FlexSchema>,
    reduce: bool,    
) -> *const Operator {
    let schema = unsafe { &(*schema) };
    let op = operators::count::count(schema.clone(), reduce);
    Box::into_raw(Box::new(Operator::new(op)))
}

#[no_mangle]
pub extern "C" fn pop_key() -> *const Operator {
    let op = operators::pop_key::pop_key();
    Box::into_raw(Box::new(Operator::new(op)))
}

#[no_mangle]
pub extern "C" fn push_key_split(
    f: extern fn(i32, *const flextuple::FlexTuple) -> i64,
    index: i32
) -> *const Operator {
    let op = operators::push_key_split::push_key_split(
        Box::new(move | i: &Rc<flextuple::FlexTuple> | {
            let r = f(index, Rc::into_raw(i.clone()));
            //let ft = unsafe { Rc::from_raw(r) };
            //ft.clone()
            r
        }
    ));
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
pub extern "C" fn from_external_source(
    f: extern fn(i32, i64, *const SourceSubscription),
    index: i64
) -> *const SourceOp {
    // c_void
    let src = sources::from_external_source::from_external_source(Box::new(move | e : Event<Rc<flextuple::FlexTuple>, Rc<flextuple::FlexTuple>>| {
        //if let Event::Subscribe(sink) = event {
        //}
        match e {
            Event::Subscribe(sink, _state_store) => {
                let s = Box::new(SourceSubscription::new(sink, index));
                let raw_s = Box::into_raw(s);
                //println!("receive subscribe from_external_source");                
                f(0, index, raw_s);
            }
            _ => {

            }
        };
    }));
    Box::into_raw(Box::new(SourceOp::new(src)))
}

#[no_mangle]
pub extern "C" fn external_source_on_next(
    p_source: *mut SourceSubscription,
    i: *const flextuple::FlexTuple
) -> *const flextuple::FlexTuple {
    let source = unsafe { &mut (*p_source) };
    //let i = unsafe { &(*i) };
    let ft = unsafe { Rc::from_raw(i) };
    source.on_next(ft.clone());
    Rc::into_raw(ft)
}

#[no_mangle]
pub extern "C" fn external_source_on_completed(
    p_source: *mut SourceSubscription
) {
    let source = unsafe { &mut (*p_source) };
    source.on_completed();
}

#[no_mangle]
pub extern "C" fn to_external_sink(_f: extern fn(i: i32), _index: i32) -> *const Operator {
    // c_void
    let op = operators::map::map(Box::new(move | i | { i }));
    Box::into_raw(Box::new(Operator::new(op)))
}


#[no_mangle]
pub extern "C" fn create_memory_state_store() -> *const StateStore {
//*const state::state::StateStore {
    let p = Rc::new(state::state::MemoryStateStore::new());
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
    //println!("enter");
    if pipeline_ptr.is_null() {
        log::error!("provided pipeline is NULL");
        //return ptr::null();
        return;
    }
    //println!("pipeline addr: {:p}", pipeline_ptr);
    let pipeline = unsafe { &mut (*pipeline_ptr) };
    let op: Box<Operator> = unsafe { Box::from_raw(op_ptr) };    
    pipeline.add_operator(op);
}

#[no_mangle]
pub extern "C" fn pipeline_run(p_pipeline: *mut Pipeline, p_source: *mut SourceOp) {
    let pipeline = unsafe { &mut (*p_pipeline) };
    let source: Box<SourceOp> = unsafe { Box::from_raw(p_source) };

    pipeline.run(source);
}

#[no_mangle]
pub extern "C" fn pipeline_subscribe(
    p_pipeline: *mut Pipeline,
    p_source: *mut SourceOp,
    p_state_store: *mut StateStore,
    on_next: extern fn(i32, *const flextuple::FlexTuple),
    index: i32,
) {
    //let mut pipeline = unsafe { Box::from_raw(p_pipeline) };
    let pipeline = unsafe { &mut (*p_pipeline) };
    let source: Box<SourceOp> = unsafe { Box::from_raw(p_source) };

    //println!("pipeline_subscribe");
    ////let mut state_store: Rc<dyn state::state::StateStore> = unsafe { Rc::from_raw(p_state_store) };
    let state_store: Rc<StateStore> = unsafe { Rc::from_raw(p_state_store) };
    //let mut state_store = Rc::new(state::state::MemoryStateStore::new());

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


///////////////////////////////////////////////
// FlexTupleSchema
///////////////////////////////////////////////
pub struct FlexSchemaWrap {
    schema: Rc<flextuple::FlexSchema>
}

impl FlexSchemaWrap {
    fn new(schema: Rc<flextuple::FlexSchema>) -> Self {
        FlexSchemaWrap {
            schema:schema
        }
    }
}

#[no_mangle]
pub extern "C" fn flextuple_schema_builder(
    name: *const c_char,
) -> *mut flextuple::FlexSchema {
    let cstr = unsafe { CStr::from_ptr(name) };
    let rname = String::from_utf8_lossy(cstr.to_bytes()).to_string();
    let schema = Box::new(flextuple::FlexSchema::new(rname));
    Box::into_raw(schema)
}

#[no_mangle]
pub extern "C" fn flextuple_schema_build(
    p_self: *mut flextuple::FlexSchema,
) -> *const FlexSchemaWrap {
    let schema = unsafe { Box::from_raw(p_self) };
    let schema_wrap = Box::new(FlexSchemaWrap::new(
        Rc::from(schema)
    ));
    Box::into_raw(schema_wrap)
}

#[no_mangle]
pub extern "C" fn flextuple_schema_set_handle(
    p_self: *mut flextuple::FlexSchema,
    handle: *const c_void,
) -> *mut flextuple::FlexSchema {
    if p_self.is_null() {
        panic!("flextuple_schema_get_handle error: FlexSchema is NULL");
    }

    let mut schema = unsafe { Box::from_raw(p_self) };
    schema.set_handle(handle);
    Box::into_raw(schema)
}


#[no_mangle]
pub extern "C" fn flextuple_schema_drop(
    p_self: *mut FlexSchemaWrap,
) {
    println!("deref schema");
    unsafe { Box::from_raw(p_self) };
}

#[no_mangle]
pub extern "C" fn flextuple_schema_add_int64(
    p_self: *mut flextuple::FlexSchema,
    name: *const c_char
) -> *mut flextuple::FlexSchema {
    if p_self.is_null() {
        panic!("flextuple_schema_add_int64 error: FlexSchema is NULL");
    }

    if name.is_null() {
        panic!("flextuple_schema_add_int64 error: name is NULL");
    }

    let cstr = unsafe { CStr::from_ptr(name) };
    let name = String::from_utf8_lossy(cstr.to_bytes()).to_string();

    let mut schema = unsafe { Box::from_raw(p_self) };
    schema.add_int64(name);
    Box::into_raw(schema)
}

#[no_mangle]
pub extern "C" fn flextuple_schema_get_handle(
    p_self: *const FlexSchemaWrap,
) -> *const c_void {
    if p_self.is_null() {
        panic!("flextuple_schema_get_handle error: FlexSchema is NULL");
    }

    let schema_wrap = unsafe { &(*p_self) };
    schema_wrap.schema.get_handle()
}

///////////////////////////////////////////////
// FlexTuple
///////////////////////////////////////////////
pub struct FlexTupleWrap {
    ft: Rc<flextuple::FlexTuple>
}

impl FlexTupleWrap {
    fn new(ft: Rc<flextuple::FlexTuple>) -> Self {
        FlexTupleWrap {
            ft:ft
        }
    }
}

#[no_mangle]
pub extern "C" fn flextuple_builder(
    p_schema: *const FlexSchemaWrap,
) -> *const flextuple::FlexTuple {
    //let schema = unsafe { Rc::from_raw(p_schema) };
    let schema = unsafe { &(*p_schema) };

    Box::into_raw(Box::new(flextuple::FlexTuple::new(
        schema.schema.clone()
    )))
}

#[no_mangle]
pub extern "C" fn flextuple_build(
    p_self: *mut flextuple::FlexTuple,
) -> *const flextuple::FlexTuple {
    let ft = unsafe { Box::from_raw(p_self) };
    let r = Rc::into_raw(Rc::from(ft));
    //println!("build {:p}", r);
    r
}

#[no_mangle]
pub extern "C" fn flextuple_add_int64(
    p_ft: *mut flextuple::FlexTuple,
    value: i64
) -> *mut flextuple::FlexTuple {
    let mut ft = unsafe { Box::from_raw(p_ft) };
    ft.add_int64(value);
    Box::into_raw(ft)
}

#[no_mangle]
pub extern "C" fn flextuple_add_float64(
    p_ft: *mut flextuple::FlexTuple,
    value: f64
) -> *mut flextuple::FlexTuple {
    let mut ft = unsafe { Box::from_raw(p_ft) };
    ft.add_float64(value);
    Box::into_raw(ft)
}

#[no_mangle]
pub extern "C" fn flextuple_drop(
    p_self: *const flextuple::FlexTuple
) {
    unsafe { Rc::from_raw(p_self) };
}

#[no_mangle]
pub extern "C" fn flextuple_get_handle(
    p_self: *const flextuple::FlexTuple
) -> *const c_void {
    if p_self.is_null() {
        panic!("flextuple_get_handle error: FlexTuple is NULL");
    }

    let ft = unsafe { &(*p_self) };
    ft.get_handle()
}

#[no_mangle]
pub extern "C" fn flextuple_get_int64_at(
    p_self: *const flextuple::FlexTuple,
    index: usize
) -> i64
{
    let ft = unsafe { &(*p_self) };
    ft.get_int64_at(index)
}

#[no_mangle]
pub extern "C" fn flextuple_get_float64_at(
    p_self: *const flextuple::FlexTuple,
    index: usize
) -> f64
{
    let ft = unsafe { &(*p_self) };
    ft.get_float64_at(index)
}