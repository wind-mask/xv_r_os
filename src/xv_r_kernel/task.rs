use alloc::{
    sync::{Arc, Weak},
    vec::Vec,
};
use context::TaskContext;
use log::debug;
use manager::add_task;
use spin::mutex::{spin::SpinMutex, SpinMutexGuard};

use crate::{
    config::TRAP_CONTEXT,
    loader::get_app_data_by_name,
    mm::{
        address::{PhysPageNum, VirtAddr},
        memory_set::{MemorySet, KERNEL_SPACE},
    },
    proc::{
        cpu::{schedule, take_current_task},
        pid::{pid_alloc, KernelStack, PidHandle},
    },
    trap::{context::TrapContext, trap_handler},
};

pub mod context;
pub mod manager;
pub mod switch;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref INITPROC: Arc<TaskControlBlock> = {
        debug!("init INITPROC");
        Arc::new(TaskControlBlock::new(get_app_data_by_name("init").unwrap()))
    };
}
pub(crate) fn add_init_proc() {
    add_task(INITPROC.clone());
}

#[allow(unused)]
/// task control block structure
pub struct TaskControlBlock {
    // immutable
    pub pid: PidHandle,
    pub kernel_stack: KernelStack,
    // mutable
    inner: SpinMutex<TaskControlBlockInner>,
}
pub struct TaskControlBlockInner {
    pub trap_cx_ppn: PhysPageNum,
    pub base_size: usize,
    pub task_cx: TaskContext,
    pub task_status: TaskStatus,
    pub memory_set: MemorySet,
    pub parent: Option<Weak<TaskControlBlock>>,
    pub children: Vec<Arc<TaskControlBlock>>,
    pub exit_code: i32,
}
impl TaskControlBlockInner {
    pub fn get_trap_cx(&self) -> &'static mut TrapContext {
        self.trap_cx_ppn.get_mut()
    }
    pub fn get_user_token(&self) -> usize {
        self.memory_set.token()
    }
    fn get_status(&self) -> TaskStatus {
        self.task_status
    }
    pub fn is_zombie(&self) -> bool {
        self.get_status() == TaskStatus::Zombie
    }
}
impl TaskControlBlock {
    pub fn inner_exclusive_access(&self) -> SpinMutexGuard<'_, TaskControlBlockInner> {
        self.inner.lock()
    }
    pub fn getpid(&self) -> usize {
        self.pid.0
    }
    pub fn new(elf_data: &[u8]) -> Self {
        // memory_set with elf program headers/trampoline/trap context/user stack
        let (memory_set, user_sp, entry_point) = MemorySet::from_elf(elf_data);
        let trap_cx_ppn = memory_set
            .translate(VirtAddr::from(TRAP_CONTEXT).into())
            .unwrap()
            .ppn();
        // alloc a pid and a kernel stack in kernel space
        let pid_handle = pid_alloc();
        let kernel_stack = KernelStack::new(&pid_handle);
        let kernel_stack_top = kernel_stack.get_top();
        // push a task context which goes to trap_return to the top of kernel stack
        let task_control_block = Self {
            pid: pid_handle,
            kernel_stack,
            inner: SpinMutex::new(TaskControlBlockInner {
                trap_cx_ppn,
                base_size: user_sp,
                task_cx: TaskContext::goto_trap_return(kernel_stack_top),
                task_status: TaskStatus::Ready,
                memory_set,
                parent: None,
                children: Vec::new(),
                exit_code: 0,
            }),
        };
        // prepare TrapContext in user space
        let trap_cx = task_control_block.inner_exclusive_access().get_trap_cx();
        *trap_cx = TrapContext::app_init_context(
            entry_point,
            user_sp,
            KERNEL_SPACE.lock().token(),
            kernel_stack_top,
            trap_handler as usize,
        );
        task_control_block
    }
    pub fn exec(&self, elf_data: &[u8]) {
        // memory_set with elf program headers/trampoline/trap context/user stack
        let (memory_set, user_sp, entry_point) = MemorySet::from_elf(elf_data);
        let trap_cx_ppn = memory_set
            .translate(VirtAddr::from(TRAP_CONTEXT).into())
            .unwrap()
            .ppn();

        // **** access inner exclusively
        let mut inner = self.inner_exclusive_access();
        // substitute memory_set
        inner.memory_set = memory_set;
        // update trap_cx ppn
        inner.trap_cx_ppn = trap_cx_ppn;
        // initialize trap_cx
        let trap_cx = inner.get_trap_cx();
        *trap_cx = TrapContext::app_init_context(
            entry_point,
            user_sp,
            KERNEL_SPACE.lock().token(),
            self.kernel_stack.get_top(),
            trap_handler as usize,
        );
        // **** stop exclusively accessing inner automatically
    }
    pub fn fork(self: &Arc<TaskControlBlock>) -> Arc<TaskControlBlock> {
        // ---- access parent PCB exclusively
        let mut parent_inner = self.inner_exclusive_access();
        // copy user space(include trap context)
        let memory_set = MemorySet::from_existed_user(&parent_inner.memory_set);
        let trap_cx_ppn = memory_set
            .translate(VirtAddr::from(TRAP_CONTEXT).into())
            .unwrap()
            .ppn();
        // alloc a pid and a kernel stack in kernel space
        let pid_handle = pid_alloc();
        let kernel_stack = KernelStack::new(&pid_handle);
        let kernel_stack_top = kernel_stack.get_top();
        let task_control_block = Arc::new(TaskControlBlock {
            pid: pid_handle,
            kernel_stack,
            inner: SpinMutex::new(TaskControlBlockInner {
                trap_cx_ppn,
                base_size: parent_inner.base_size,
                task_cx: TaskContext::goto_trap_return(kernel_stack_top),
                task_status: TaskStatus::Ready,
                memory_set,
                parent: Some(Arc::downgrade(self)),
                children: Vec::new(),
                exit_code: 0,
            }),
        });
        // add child
        parent_inner.children.push(task_control_block.clone());
        // modify kernel_sp in trap_cx
        // **** access children PCB exclusively
        let trap_cx = task_control_block.inner_exclusive_access().get_trap_cx();
        trap_cx.kernel_sp = kernel_stack_top;
        // return
        task_control_block
        // ---- stop exclusively accessing parent/children PCB automatically
    }
}
// impl TaskControlBlock {
//     pub fn new(elf_data: &[u8], app_id: usize) -> Self {
//         // memory_set with elf program headers/trampoline/trap context/user stack
//         let (memory_set, user_sp, entry_point) = MemorySet::from_elf(elf_data);
//         let trap_cx_ppn = memory_set
//             .translate(VirtAddr::from(TRAP_CONTEXT).into())
//             .unwrap()
//             .ppn();

//         let task_status = TaskStatus::Ready;
//         // map a kernel-stack in kernel space
//         let (kernel_stack_bottom, kernel_stack_top) = kernel_stack_position(app_id);
//         KERNEL_SPACE.exclusive_access().insert_framed_area(
//             kernel_stack_bottom.into(),
//             kernel_stack_top.into(),
//             MapPermission::R | MapPermission::W,
//         );

//         let task_control_block = Self {
//             task_status,
//             task_cx: TaskContext::goto_trap_return(kernel_stack_top),
//             memory_set,
//             trap_cx_ppn,
//             base_size: user_sp,
//         };
//         // prepare TrapContext in user space
//         let trap_cx = task_control_block.get_trap_cx();
//         *trap_cx = TrapContext::app_init_context(
//             entry_point,
//             user_sp,
//             KERNEL_SPACE.exclusive_access().token(),
//             kernel_stack_top,
//             trap_handler as usize,
//         );
//         task_control_block
//     }
//     pub fn get_trap_cx(&self) -> &'static mut TrapContext {
//         self.trap_cx_ppn.get_mut()
//     }
//     pub fn get_user_token(&self) -> usize {
//         self.memory_set.token()
//     }
// }
#[allow(unused)]
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Zombie,
    Exited,
}

/// Suspend the current 'Running' task and run the next task in task list.
pub fn suspend_current_and_run_next() {
    // There must be an application running.
    let task = take_current_task().unwrap();

    // ---- access current TCB exclusively
    let mut task_inner = task.inner_exclusive_access();

    let task_cx_ptr = &mut task_inner.task_cx as *mut TaskContext;
    // Change status to Ready
    task_inner.task_status = TaskStatus::Ready;
    drop(task_inner);
    // ---- stop exclusively accessing current PCB

    // push back to ready queue.
    add_task(task);
    // jump to scheduling cycle
    // UNSAFE: 挂起切换到下一个任务，故调用unsafe函数
    unsafe { schedule(task_cx_ptr) };
}
// /// Change the status of current `Running` task into `Exited`.
// fn mark_current_exited() {
//     TASK_MANAGER.mark_current_exited();
// }
// /// Exit the current 'Running' task and run the next task in task list.
pub fn exit_current_and_run_next(exit_code: i32) {
    // take from Processor
    let task = take_current_task().unwrap();
    // **** access current TCB exclusively
    let mut inner = task.inner_exclusive_access();
    // Change status to Zombie
    inner.task_status = TaskStatus::Zombie;
    // Record exit code
    inner.exit_code = exit_code;
    // do not move to its parent but under initproc

    // ++++++ access initproc TCB exclusively
    {
        let mut initproc_inner = INITPROC.inner_exclusive_access();
        for child in inner.children.iter() {
            child.inner_exclusive_access().parent = Some(Arc::downgrade(&INITPROC));
            initproc_inner.children.push(child.clone());
        }
    }
    // ++++++ stop exclusively accessing parent PCB

    inner.children.clear();
    // deallocate user space
    inner.memory_set.recycle_data_pages();
    drop(inner);
    // **** stop exclusively accessing current PCB
    // drop task manually to maintain rc correctly
    drop(task);
    // we do not have to save task context
    let mut _unused = TaskContext::zero_init();
    // UNSAFE：退出当前任务并调度到下一个任务，故调用unsafe函数
    unsafe { schedule(&mut _unused as *mut _) };
}
