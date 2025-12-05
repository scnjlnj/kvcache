use super::*;
use crate::engine::Engine;
use std::sync::{Arc, RwLock};

pub struct RwLockScheduler<E: Engine + Send + 'static> {
    engine: Arc<RwLock<E>>,
}
impl<E: Engine + Send + 'static> RwLockScheduler<E> {
    pub fn new(engine: E) -> Self {
        Self {
            engine: Arc::new(RwLock::new(engine)),
        }
    }
}
impl<E: Engine + Send + 'static> Scheduler for RwLockScheduler<E> {
    type Output = E::Output;

    fn submit(&self, op: Op) -> Task<Self::Output> {
        let mut engine = self.engine.write().unwrap();
        engine.submit(op)
    }
}
