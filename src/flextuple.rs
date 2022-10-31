use std::rc::Rc;
use std::ffi::c_void;
use std::ptr::null;
//use flexbuffers
//use flexbuffers::{BitWidth, Builder, Reader, ReaderError};

/*
flextuple:
- is backed by a flexbuffer
- contains a schema
*/


pub enum FlexField {
    Int64(i64),
    Float64(f64),
    String(String),
    Bool(bool),
    FlexTuple(FlexTuple),
}

pub enum SchemaFieldType {
    Int64,
    Float64,
    String,
    Bool,
    FlexTuple,
}

pub struct SchemaField {
    name: String,
    field_type: SchemaFieldType,
}

pub struct FlexSchema {    
    name: String,
    handle: *const c_void,
    fields: Vec<SchemaField>,
}
pub struct FlexTuple {
    schema: Rc<FlexSchema>,
    data: Vec<Rc<FlexField>>
}

impl SchemaField {
    fn new(name: String, field_type: SchemaFieldType) -> Self {
        SchemaField {
            name: name,
            field_type: field_type,
        }
    }
}

impl FlexSchema {
    pub fn new(name: String) -> Self {
        FlexSchema {            
            name: name,
            handle: null(),
            fields: Vec::new(),
        }
    }

    pub fn set_handle(&mut self, handle: *const c_void) {
        self.handle = handle;
    }

    pub fn get_handle(&self) -> *const c_void {
        self.handle
    }

    pub fn add_int64(&mut self, name: String) {
        self.fields.push(SchemaField::new(
            name,
            SchemaFieldType::Int64,
        ));
    }
}

impl FlexTuple {
    /*Creates a FlexTuple object.
    */
    pub fn new(schema: Rc<FlexSchema>) -> Self {
        FlexTuple {
            schema: schema.clone(),
            data: Vec::new(),
        }
    }

    pub fn get_handle(&self) -> *const c_void {
        //println!("get handle schema: {}", self.schema.name);
        //println!("get_handle: {:?}", self.schema.handle);
        self.schema.get_handle()
    }

    pub fn add_int64(&mut self, value: i64) {
        self.data.push(Rc::new(FlexField::Int64(value)))
    }

    pub fn add_float64(&mut self, value: f64) {
        self.data.push(Rc::new(FlexField::Float64(value)))
    }

    pub fn get_int64_at(&self, index: usize) -> i64 {
        match *self.data[index] {
            FlexField::Int64(v) => v,
            _ => panic!("Internal error: index {} on FlexTuple {} is not typed Int64", index, self.schema.name),
        }
    }

    pub fn get_float64_at(&self, index: usize) -> f64 {
        match *self.data[index] {
            FlexField::Float64(v) => v,
            _ => panic!("Internal error: index {} on FlexTuple {} is not typed Int64", index, self.schema.name),
        }
    }

}