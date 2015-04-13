use mmu::Descriptor;

extern {
    static mut gdt: [Descriptor; 10];
}

pub fn gdt_get(i: usize) -> Descriptor {
    unsafe { gdt[i] }
}
