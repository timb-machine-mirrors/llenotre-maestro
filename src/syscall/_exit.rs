//! The _exit syscall allows to terminate the current process with the given status code.

use crate::process::Process;
use crate::process::Regs;

/// The implementation of the `write` syscall.
pub fn _exit(regs: &Regs) -> ! {
	{
		let mut mutex = Process::get_current().unwrap();
		let mut guard = mutex.lock(false);
		let proc = guard.get_mut();

		proc.exit(regs.ebx);
	}

	unsafe {
		// Waiting for the next tick
		asm!("jmp $kernel_loop");
	}

	// This loop is here only to avoid a compilation error
	loop {}
}
