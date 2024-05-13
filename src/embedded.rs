use esp_alloc::EspHeap;
use esp_println::println;
use core::{
    mem::MaybeUninit,
    panic::PanicInfo,
};

#[global_allocator]
static ALLOCATOR: EspHeap = EspHeap::empty();

pub fn init_heap() {
    const HEAP_SIZE: usize = 32 * 1024;
    static mut HEAP: MaybeUninit<[u8; HEAP_SIZE]> = MaybeUninit::uninit();

    unsafe {
        ALLOCATOR.init(HEAP.as_mut_ptr() as *mut u8, HEAP_SIZE);
    }
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    println!("{_panic}");
    loop {}
}
