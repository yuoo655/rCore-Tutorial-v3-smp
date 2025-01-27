use super::TaskControlBlock;
use super::{fetch_task, TaskStatus};
use super::TaskContext;
use super::__switch;
use super::add_task;
use super::kthread_trap_cx_bottom_from_tid;

use alloc::sync::Arc;
use alloc::vec::Vec;
use core::cell::RefCell;
use crate::trap::TrapContext;
use crate::config::CPU_NUM;
use lazy_static::*;
use core::arch::asm;
pub struct Processor {
    inner: RefCell<ProcessorInner>,
}

impl Default for Processor {
    fn default() -> Self {
        Self {
            inner: RefCell::new(ProcessorInner {
                current: None,
                idle_task_cx: TaskContext::zero_init(),
            }),
        }
    }
}


unsafe impl Sync for Processor {}

struct ProcessorInner {
    current: Option<Arc<TaskControlBlock>>,
    idle_task_cx: TaskContext,
}

impl Processor {
    pub fn new() -> Self {
        Self {
            inner: RefCell::new(ProcessorInner {
                current: None,
                idle_task_cx: TaskContext::zero_init(),
            }),
        }
    }

    fn get_idle_task_cx_ptr(&self) -> *mut TaskContext {
        let mut inner = self.inner.borrow_mut();
        &mut inner.idle_task_cx as *mut TaskContext
    }

    pub fn run_next(&self, task: Arc<TaskControlBlock>){
        
        let idle_task_cx_ptr = self.get_idle_task_cx_ptr();
        // acquire
        let mut task_inner = task.inner_exclusive_access();
        let next_task_cx_ptr = task_inner.get_task_cx_ptr();
        task_inner.task_status = TaskStatus::Running(hart_id());
        

        // release
        drop(task_inner);
        self.inner.borrow_mut().current = Some(task);

        // println_hart!("switching idle:{:#x?} to:{:#x?}", hart_id(), idle_task_cx_ptr, next_task_cx_ptr );
        unsafe {
            __switch(idle_task_cx_ptr, next_task_cx_ptr);
        }
    }

    #[no_mangle]
    fn suspend_current(&self) {
        
        // info!("[suspend current]");
        if let Some(task) = take_current_task() {

            // info!("task pid: {} suspend", task.pid.0);

            // ---- hold current PCB lock
            let mut task_inner = task.inner_exclusive_access();
            // Change status to Ready
            task_inner.task_status = TaskStatus::Ready;

            drop(task_inner);
            // ---- release current PCB lock

            // push back to ready queue.
            add_task(task);
        }
    }

    #[no_mangle]
    pub fn run(&self) {
        loop {
            if let Some(task) = fetch_task() {
                self.run_next(task);
                self.suspend_current();
            }
        }
    }
    
    pub fn take_current(&self) -> Option<Arc<TaskControlBlock>> {
        self.inner.borrow_mut().current.take()
    }
    pub fn take_current_mut(&self) -> Option<Arc<TaskControlBlock>> {
        self.inner.borrow_mut().current.take()
    }

    pub fn current(&self) -> Option<Arc<TaskControlBlock>> {
        self.inner.borrow().current.as_ref().map(|task| Arc::clone(task))
    }
}

lazy_static! {
    pub static ref PROCESSORS: [Processor; CPU_NUM] = Default::default();
}


pub fn hart_id() -> usize {
    let hart_id: usize;
    unsafe {
        asm!("mv {}, tp", out(reg) hart_id);
    }
    hart_id
}

pub fn run_tasks() {
    debug!("run_tasks");
    PROCESSORS[hart_id()].run();
}

pub fn take_current_task() -> Option<Arc<TaskControlBlock>> {
    PROCESSORS[hart_id()].take_current()
}

pub fn current_task() -> Option<Arc<TaskControlBlock>> {
    PROCESSORS[hart_id()].current()
}

#[allow(unused)]
pub fn current_tasks() -> Vec<Option<Arc<TaskControlBlock>>> {
    PROCESSORS
        .iter()
        .map(|processor| processor.current())
        .collect()
}

pub fn current_user_token() -> usize {
    let task = current_task().unwrap();
    let token = task.inner_exclusive_access().get_user_token();
    token
}

pub fn current_trap_cx() -> &'static mut TrapContext {
    current_task().unwrap().inner_exclusive_access().get_trap_cx()
}

#[no_mangle]
pub fn schedule(switched_task_cx_ptr: *mut TaskContext) {
    let idle_task_cx_ptr = PROCESSORS[hart_id()].get_idle_task_cx_ptr();
    unsafe {
        __switch(switched_task_cx_ptr, idle_task_cx_ptr);
    }
}


pub fn current_trap_cx_user_va() -> usize {
    current_task().unwrap().trap_cx_user_va()
}


pub fn current_trap_cx_kernel_va() -> usize {
    let task = current_task().unwrap();
    let tgid = task.tgid;
    let va = kthread_trap_cx_bottom_from_tid(tgid);
    va
}