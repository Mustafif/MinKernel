#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(min_kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]
extern crate alloc;
use alloc::boxed::Box;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use min_kernel::{print, println};
use min_kernel::task::executor::Executor;
use min_kernel::task::keyboad::print_keypresses;
use min_kernel::task::Task;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use min_kernel::allocator;
    use min_kernel::memory;
    use min_kernel::memory::BootInfoFrameAllocator;
    use x86_64::{structures::paging::Page, VirtAddr};
    min_kernel::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    #[cfg(test)]
    test_main();
    // put here
    let mut executor = Executor::new();
    executor.spawn(Task::new(front_page()));
    executor.spawn(Task::new(print_keypresses()));
    executor.run();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    min_kernel::hlt_loop();
}

async fn front_page(){
    println!("-------------------------------------------");
    println!("Welcome to the Mustafif Minimum Kernel...");
    println!("--------------------------------------------");
}

