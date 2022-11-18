use std::rc::Rc;
use std::cell::RefCell;
use core::fmt::Debug;

#[derive(Clone, Debug)]
enum ValueState {
    Cleared,
    NotSet,
    Set,
}

pub trait State<T: Default> {
    //type Value;

    fn set(&mut self, key: usize, value: &Rc<T>);
    fn get(&self, key: usize) -> Option<Rc<T>>;
    fn create_key(&mut self, key: usize);
    fn delete_key(&mut self, key: usize);
}

pub trait StateStore {
    fn create_state_i64(&self, name: &str) -> Rc<RefCell<dyn State<i64>>>;
    fn create_state_i32(&self, name: &str) -> Rc<RefCell<dyn State<i32>>>;
}


impl Debug for dyn StateStore {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "StateStore")
    }
}


pub struct MemoryState<T> {
    values: Vec<Rc<T>>,
    states: Vec<ValueState>,
}

pub struct MemoryStateStore {
}

impl<T> MemoryState<T> {
    fn new() -> Self {
        MemoryState {
            values: Vec::new(),
            states: Vec::new(),
        } 
    }
}

impl<T: Default> State<T> for MemoryState<T> {
    //Value: T;

    fn set(&mut self, key: usize, value: &Rc<T>) {
        self.values[key] = Rc::clone(value);
        self.states[key] = ValueState::Set;
    }

    fn get(&self, key: usize) -> Option<Rc<T>> {
        match self.states[key] {
            ValueState::Cleared => panic!("key {} does not exist", key),
            ValueState::NotSet => None,
            ValueState::Set => Some(Rc::clone(&self.values[key])),
        }
    }

    fn create_key(&mut self, key: usize) {
        let append_count = key+1 - self.values.len();
        if append_count > 0 {
            for _ in 0..append_count {
                self.values.push(Rc::new(T::default()));
                self.states.push(ValueState::Cleared);
            }
        }
        self.states[key] = ValueState::NotSet;
    }

    fn delete_key(&mut self, key: usize) {
        self.values[key] = Rc::new(T::default());
        self.states[key] = ValueState::Cleared;
    }
}


impl MemoryStateStore {
    pub fn new() -> Self {
        MemoryStateStore {} 
    }
}

impl StateStore for MemoryStateStore {
    fn create_state_i64(&self, _name: &str) -> Rc<RefCell<dyn State<i64>>> {
        Rc::new(RefCell::new(MemoryState::<i64>::new()))
    }

    fn create_state_i32(&self, _name: &str) -> Rc<RefCell<dyn State<i32>>> {
        Rc::new(RefCell::new(MemoryState::<i32>::new()))
    } 
}