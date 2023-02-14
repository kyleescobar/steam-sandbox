#![feature(unboxed_closures)]

use std::arch::asm;
use std::{mem, sync::RwLock};
use std::io::Error;
use std::marker::Tuple;

use detour::{Function, RawDetour};
use dynasmrt::{DynasmApi, DynasmError};
use dynasmrt::DynasmLabelApi;
use dynasmrt::ExecutableBuffer;
use dynasmrt::{dynasm, x64::Assembler};
use once_cell::sync::OnceCell;

pub use self::function_ref::{FunctionAddress, FunctionRef};
use crate::sdk::Global;

mod function_ref;
mod hook;

pub struct Hook {
    buffer: ExecutableBuffer,
    detour: RawDetour,
}

#[derive(Default)]
pub struct Hooks {
    installed_hooks: RwLock<Vec<Hook>>,
}

impl Global for Hooks {
    fn cell() -> &'static OnceCell<Self> {
        static INSTANCE: OnceCell<Hooks> = OnceCell::new();
        &INSTANCE
    }

    fn create() -> Self {
        Hooks {
            installed_hooks: RwLock::new(vec![]),
        }
    }
}

impl Hooks {
    pub fn install<T, D>(&self, function: &T, detour: D) -> Result<(), Box<dyn std::error::Error>>
        where
            T: FunctionRef,
            T::Target: Function,
            D: 'static
            + Fn<
                <<T as FunctionRef>::Target as Function>::Arguments,
                Output = <<T as FunctionRef>::Target as Function>::Output,
            >, <<T as FunctionRef>::Target as Function>::Arguments: Tuple
    {
        let closure_ptr = Box::into_raw(Box::new(Box::new(detour)
            as Box<
            dyn Fn<
                <<T as FunctionRef>::Target as Function>::Arguments,
                Output = <<T as FunctionRef>::Target as Function>::Output,
            >,
        >));

        let callback = Self::callback::<<<T as FunctionRef>::Target as Function>::Arguments, D>;
        let mut ops = Assembler::new().expect("unable to create assembler");

        let trampoline_offset = ops.offset();
        dynasm!(ops
            ; -> prelude:
            ; mov rax, QWORD closure_ptr as *const () as _
            ; mov r11, QWORD callback as *const () as _
            ; jmp r11
            ; int3
        );

        ops.commit()?;

        let buffer = ops.finalize().expect("unable to assemble hook trampoline");

        let trampoline = unsafe { mem::transmute(buffer.ptr(trampoline_offset)) };
        let detour = unsafe { RawDetour::new(function.get_ptr(), trampoline)? };

        function.set_target(detour.trampoline());
        unsafe { detour.enable()? };

        self.installed_hooks
            .write()
            .expect("unable to get write lock")
            .push(Hook { buffer, detour });

        Ok(())
    }

    #[naked]
    unsafe extern "C" fn get_trampoline_closure() -> *const () {
        asm!("ret", options(noreturn)) // trampoline already put the closure in RAX
    }

    unsafe extern "rust-call" fn callback<A, F: Fn<A> + 'static>(args: A) -> F::Output
    where A: Tuple
    {
        let closure = Self::get_trampoline_closure() as *const Box<F>;
        std::ops::Fn::call(&**closure, args)
    }
}