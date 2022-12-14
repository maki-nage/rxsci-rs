use std::rc::Rc;
use std::cell::RefCell;
use core::fmt::Debug;
use crate::flextuple;

#[derive(Clone, Debug)]
enum ValueState {
    Cleared,
    NotSet,
    Set,
}

pub trait State<T: Default> {
    //type Value;

    fn set(&mut self, key: usize, value: T);
    fn get(&self, key: usize) -> Option<T>;
    fn set_rc(&mut self, key: usize, value: &Rc<T>);
    fn get_rc(&self, key: usize) -> Option<Rc<T>>;
    fn create_key(&mut self, key: usize);
    fn delete_key(&mut self, key: usize);
}

pub trait StateStore {
    fn create_state_i64(&self, name: &str) -> Rc<RefCell<dyn State<i64>>>;
    fn create_state_i32(&self, name: &str) -> Rc<RefCell<dyn State<i32>>>;
    fn create_state_ft(&self, name: &str) -> Rc<RefCell<dyn State<flextuple::FlexTuple>>>;
    //fn create_state<T>(&self, name: &str) -> Rc<RefCell<dyn State<T>>>;
}


impl Debug for dyn StateStore {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "StateStore")
    }
}


pub struct MemoryState<T> {
    values: Vec<T>,
    states: Vec<ValueState>,
}

pub struct MemoryStateRc<T> {
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

impl<T> MemoryStateRc<T> {
    fn new() -> Self {
        MemoryStateRc {
            values: Vec::new(),
            states: Vec::new(),
        }
    }
}

impl<T: Default + Copy> State<T> for MemoryState<T> {
    fn set(&mut self, key: usize, value: T) {
        self.values[key] = value;
        self.states[key] = ValueState::Set;
    }

    fn get(&self, key: usize) -> Option<T> {
        match self.states[key] {
            ValueState::Cleared => panic!("key {} does not exist", key),
            ValueState::NotSet => None,
            ValueState::Set => Some(self.values[key]),
        }
    }

    fn set_rc(&mut self, key: usize, value: &Rc<T>) {
        panic!("set_rc is invalid in MemoryState");
    }
    fn get_rc(&self, key: usize) -> Option<Rc<T>> {
        panic!("get_rc is invalid in MemoryState");
    }

    fn create_key(&mut self, key: usize) {
        let append_count = key+1 - self.values.len();
        if append_count > 0 {
            for _ in 0..append_count {
                self.values.push(T::default());
                self.states.push(ValueState::Cleared);
            }
        }
        self.states[key] = ValueState::NotSet;
    }

    fn delete_key(&mut self, key: usize) {
        self.values[key] = T::default();
        self.states[key] = ValueState::Cleared;
    }
}

impl<T: Default> State<T> for MemoryStateRc<T>
{
    fn set(&mut self, key: usize, value: T) {
        panic!("set is invalid in MemoryStateRc");
    }
    fn get(&self, key: usize) -> Option<T> {
        panic!("get is invalid in MemoryStateRc");
    }

    fn set_rc(&mut self, key: usize, value: &Rc<T>) {
        self.values[key] = Rc::clone(value);
        self.states[key] = ValueState::Set;
    }

    fn get_rc(&self, key: usize) -> Option<Rc<T>> {
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

    fn create_state_ft(&self, _name: &str) -> Rc<RefCell<dyn State<flextuple::FlexTuple>>> {
        Rc::new(RefCell::new(MemoryStateRc::<flextuple::FlexTuple>::new()))
    } 
}