mod idt;
mod pic;
pub mod apic;
pub mod handlers;
pub mod context;

pub use self::idt::{idt_desc};
pub use self::context::{Context,Contextable};

#[link(name="asmcode", repr="static")]
extern {
    fn double_fault_wrapper();
    fn gpf_wrapper();
    fn page_fault_wrapper();
    fn broadcast_timer_wrapper();
    fn timer_wrapper();
    fn kbd_interrupt_wrapper();
    fn spurious_interrupt_handler();
    fn syscall_handler();
}

pub fn init_idt() {
    idt::init();

    pic::remap_pic();
    pic::mask_pic(0xff, 0xff);

    idt::register_interrupt(0x8, double_fault_wrapper, 0);
    idt::register_trap(0xD, gpf_wrapper, 0);
    idt::register_trap(0xE, page_fault_wrapper, 0);
    idt::register_interrupt(0x22, timer_wrapper, 0);
    idt::register_interrupt(0x21, kbd_interrupt_wrapper, 0);
    idt::register_trap(0x49, broadcast_timer_wrapper, 0);
    idt::register_trap(0x50, syscall_handler, 3);
    idt::register_interrupt(0xFF, spurious_interrupt_handler, 0);
}

// Needs the IO APIC to have an ID
pub fn init() {
    apic::init();
    pic::timer_init(25);
    unsafe { asm!("sti" :::: "volatile") }
}
