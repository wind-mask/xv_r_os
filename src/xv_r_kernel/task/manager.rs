use alloc::{collections::vec_deque::VecDeque, sync::Arc};
use log::debug;
use spin::mutex::SpinMutex;

use super::TaskControlBlock;

pub struct TaskManager {
    ready_queue: VecDeque<Arc<TaskControlBlock>>,
}
impl Default for TaskManager {
    fn default() -> Self {
        Self::new()
    }
}
/// A simple FIFO scheduler.
impl TaskManager {
    pub fn new() -> Self {
        Self {
            ready_queue: VecDeque::new(),
        }
    }
    pub fn add(&mut self, task: Arc<TaskControlBlock>) {
        self.ready_queue.push_back(task);
    }
    pub fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        self.ready_queue.pop_front()
    }
}
use lazy_static::lazy_static;
lazy_static! {
    pub static ref TASK_MANAGER: SpinMutex<TaskManager> = {
        debug!("TaskManager init");
        SpinMutex::new(TaskManager::new())
    };
}

pub fn add_task(task: Arc<TaskControlBlock>) {
    TASK_MANAGER.lock().add(task);
}

pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    TASK_MANAGER.lock().fetch()
}
