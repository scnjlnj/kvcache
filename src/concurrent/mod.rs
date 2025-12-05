use std::process::Output;

use crate::engine::{DelRequest, Engine, GetRequest, PutRequest};

pub enum Op {
    Put(PutRequest),
    Get(GetRequest),
    Del(DelRequest),
}
pub struct Task<T> {
    id: u64,
    result: Option<T>,
}
impl<T> Task<T> {
    pub fn new(id: u64) -> Self {
        Self { id, result: None }
    }
    pub fn set_result(&mut self, result: T) {
        self.result = Some(result);
    }
    pub fn take_result(&mut self) -> Option<T> {
        self.result.take()
    }
}
pub trait Scheduler {
    type Output;
    fn submit(&self, op: Op) -> Task<Self::Output>;
    fn start(&self) -> Option<Self::Output>;
    fn stop(&self) -> Option<Self::Output>;
    fn cancel(&self, task_id: u64) -> bool;
    fn exec(&self, engine: &mut dyn Engine<Output = Self::Output>, op: Op) -> Option<Self::Output> {
        match op {
            Op::Put(req) => {
                engine.put(req);
            }
            Op::Get(req) => engine.get(req),
            Op::Del(req) => engine.del(req),
        }
    }
}

pub mod rwlock;
pub mod single;
