#![feature(llvm_asm)]

use register::cpu::RegisterReadWrite;

pub struct Reg;

impl RegisterReadWrite<u64, ()> for Reg {
    #[inline]
    fn get(&self) -> u64 {
        let reg;
        unsafe {
            llvm_asm!("mv $0, sp" : "=r"(reg) ::: "volatile");
        }
        reg
    }

    #[inline]
    fn set(&self, value: u64) {
        unsafe {
            llvm_asm!(concat!("mv sp, $0") :: "r"(value) :: "volatile")
        }
    }
}

pub static SP: Reg = Reg {};
