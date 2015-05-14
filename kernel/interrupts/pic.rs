use machine::{inb, outb};

const PIC1_CMD: u16 = 0x20;
const PIC1_DATA: u16 = 0x21;
const PIC2_CMD: u16 = 0xA0;
const PIC2_DATA: u16 = 0xA1;

const PIC_TIMER_DATA: u16 = 0x40;
const PIC_TIMER_CMD: u16 = 0x43;

pub fn remap_pic() {
    let mask1 = inb(PIC1_DATA);
    let mask2 = inb(PIC2_DATA);

    outb(PIC1_CMD, 0x11);
    outb(PIC2_CMD, 0x11);

    outb(PIC1_DATA, 0x20);
    outb(PIC2_DATA, 0x28);

    outb(PIC1_DATA, 0x04);
    outb(PIC2_DATA, 0x02);

    outb(PIC1_DATA, 0x01);
    outb(PIC2_DATA, 0x01);

    // Everything is remapped
    outb(PIC1_DATA, mask1);
    outb(PIC2_DATA, mask2);
}

pub fn mask_pic(master_mask: u8, slave_mask: u8) {
    outb(PIC1_DATA, master_mask);
    outb(PIC2_DATA, slave_mask);
}

// Sets the PIC timer to fire every ms milliseconds, approximately
// 30 ms is too many
pub fn timer_init(ms: u16) {
    const TICKS_PER_MS: u16 = 1193;
    let reload = ms * TICKS_PER_MS * 2; // Mode 3 counts by 2
    outb(PIC_TIMER_CMD, 0b00110110);
    // The ticks should be even
    outb(PIC_TIMER_DATA, (reload & 0xfe) as u8);
    outb(PIC_TIMER_DATA, (reload >> 8) as u8);
}
