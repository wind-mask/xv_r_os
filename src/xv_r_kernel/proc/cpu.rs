use alloc::sync::Arc;
use log::debug;
use spin::mutex::SpinMutex;

use crate::{
    task::{
        add_init_proc, context::TaskContext, manager::fetch_task, switch::__switch,
        TaskControlBlock, TaskStatus,
    },
    trap::context::TrapContext,
};
use lazy_static::lazy_static;
lazy_static! {
    pub static ref PROCESSOR: SpinMutex<Processor> = {
        debug!("Processor init");
        SpinMutex::new(Processor::new())
    };
}
pub struct Processor {
    current: Option<Arc<TaskControlBlock>>,
    idle_task_cx: TaskContext,
}
impl Default for Processor {
    fn default() -> Self {
        Self::new()
    }
}
impl Processor {
    pub fn new() -> Self {
        Self {
            current: None,
            idle_task_cx: TaskContext::zero_init(),
        }
    }
}
impl Processor {
    pub fn take_current(&mut self) -> Option<Arc<TaskControlBlock>> {
        self.current.take()
    }
    pub fn current(&self) -> Option<Arc<TaskControlBlock>> {
        self.current.as_ref().map(Arc::clone)
    }
}

pub fn take_current_task() -> Option<Arc<TaskControlBlock>> {
    PROCESSOR.lock().take_current()
}

pub fn current_task() -> Option<Arc<TaskControlBlock>> {
    PROCESSOR.lock().current()
}

pub fn current_user_token() -> usize {
    let task = current_task().unwrap();
    let token = task.inner_exclusive_access().get_user_token();
    token
}

pub fn current_trap_cx() -> &'static mut TrapContext {
    current_task()
        .unwrap()
        .inner_exclusive_access()
        .get_trap_cx()
}
/// 启动任务调度
///
/// # Safety
///
/// 仅在内核初始化后调用一次
pub unsafe fn run_tasks() {
    add_init_proc();
    loop {
        let mut processor = PROCESSOR.lock();

        if let Some(task) = fetch_task() {
            let idle_task_cx_ptr = processor.get_idle_task_cx_ptr();
            // access coming task TCB exclusively
            let mut task_inner = task.inner_exclusive_access();
            let next_task_cx_ptr = &task_inner.task_cx as *const TaskContext;
            task_inner.task_status = TaskStatus::Running;
            // stop exclusively accessing coming task TCB manually
            drop(task_inner);
            processor.current = Some(task);
            // stop exclusively accessing processor manually
            drop(processor);
            __switch(idle_task_cx_ptr, next_task_cx_ptr);
        }
    }
}

impl Processor {
    fn get_idle_task_cx_ptr(&mut self) -> *mut TaskContext {
        &mut self.idle_task_cx as *mut _
    }
}
/// Schedule the next task.
///
/// # Safety
///
/// 只能由切换任务的代码调用
pub unsafe fn schedule(switched_task_cx_ptr: *mut TaskContext) {
    let mut processor = PROCESSOR.lock();
    let idle_task_cx_ptr = processor.get_idle_task_cx_ptr();
    drop(processor);
    __switch(switched_task_cx_ptr, idle_task_cx_ptr);
}
