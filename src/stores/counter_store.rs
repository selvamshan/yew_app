use yewdux::prelude::*;

#[derive(PartialEq, Default, Clone, Store)]
pub struct CounterStore {
    pub count: u32,
}


