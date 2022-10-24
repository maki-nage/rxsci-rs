use std::rc::Rc;
use std::cell::RefCell;
use core::fmt::Debug;

pub trait State<T: Default> {
    //type Value;

    fn set(&mut self, key: &Vec<usize>, value: &Rc<T>);
    fn get(&self, key: &Vec<usize>) -> Rc<T>;
    fn create_key(&mut self, key: &Vec<usize>);
    fn delete_key(&mut self, key: &Vec<usize>);
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

#[derive(Clone, Debug)]
enum ValueState {
    Cleared,
    NotSet,
    Set,
}

pub struct MemoryState<T> {
    values: Vec<Rc<T>>,
    states: Vec<ValueState>,
    keys: Vec<Vec<usize>>,
}

pub struct MemoryStateStore {
}

impl<T> MemoryState<T> {
    fn new() -> Self {
        MemoryState {
            values: Vec::new(),
            states: Vec::new(),
            keys: Vec::new(),
        } 
    }
}

impl<T: Default> State<T> for MemoryState<T> {
    //Value: T;

    fn set(&mut self, key: &Vec<usize>, value: &Rc<T>) {
        self.values[key[0]] = Rc::clone(value);
    }

    fn get(&self, key: &Vec<usize>) -> Rc<T> {
        Rc::clone(&self.values[key[0]])
    }

    fn create_key(&mut self, key: &Vec<usize>) {
        let append_count = key[0]+1 - self.values.len();
        if append_count > 0 {
            for _ in 0..append_count {
                self.values.push(Rc::new(T::default()));
                self.states.push(ValueState::Cleared);
                self.keys.push(Vec::new());
            }
        }
        self.states[key[0]] = ValueState::NotSet;
        self.keys[key[0]] = key.clone();
    }

    fn delete_key(&mut self, key: &Vec<usize>) {
        
    }
}


impl MemoryStateStore {
    pub fn new() -> Self {
        MemoryStateStore {} 
    }
}

impl StateStore for MemoryStateStore {
    fn create_state_i64(&self, name: &str) -> Rc<RefCell<dyn State<i64>>> {
        Rc::new(RefCell::new(MemoryState::<i64>::new()))
    }

    fn create_state_i32(&self, name: &str) -> Rc<RefCell<dyn State<i32>>> {
        Rc::new(RefCell::new(MemoryState::<i32>::new()))
    } 
}