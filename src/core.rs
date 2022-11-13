use never::Never;
use std::{
    //error::Error,
    fmt::{self, Debug},
    ops::Deref,
    rc::Rc,
};
use crate::state::state::{StateStore};

pub type Key = Vec<usize>;

#[derive(Clone, Debug)]
pub struct Item<V> {
    pub key: Key,
    pub value: V,
}

#[derive(Clone, Debug)]
pub enum Event<I, O> {
    Subscribe(Rc<Callbag<O, I>>, Rc<dyn StateStore>),
    KeyCreated(Key),
    KeyCompleted(Key),
    ForwardKeyCreated(Key),
    ForwardKeyCompleted(Key),
    PushItem(Item<I>),
    //PushNonFatalError(dyn Error),
    PollItem,
    Completed,
    //Error(Arc<dyn Error + Send + Sync + 'static>),
    //Terminate,
}

pub struct Callbag<I, O>(CallbagFn<I, O>);
pub type Source<T> = Callbag<Never, T>;
pub type Sink<T> = Callbag<T, Never>;

pub type CallbagFn<I, O> = Box<dyn Fn(Event<I, O>)>; // + Send + Sync>;

impl<I, O> Deref for Callbag<I, O> {
    type Target = CallbagFn<I, O>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<I, O> Debug for Callbag<I, O> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Callbag<{}, {}>",
            std::any::type_name::<I>(),
            std::any::type_name::<O>(),
        )
    }
}

impl<I, O, F: 'static> From<F> for Callbag<I, O>
where
    F: Fn(Event<I, O>), // + Send + Sync,
{
    fn from(handler: F) -> Self {
        Callbag(Box::new(handler))
    }
}
