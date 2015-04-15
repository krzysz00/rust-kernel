#[derive(Copy, Clone)]
#[repr(C,packed)]
pub struct Descriptor {
    pub f0: u32, pub f1: u32, pub f2: u32, pub f3: u32,
}

#[repr(C,packed)]
pub struct TableDescriptor {
    pub limit: u16, pub base: u64
}

#[inline]
fn mask(left_bit: u32, right_bit: u32) -> u32 {
    let left_bit = left_bit & 0x1f;
    let right_bit = right_bit & 0x1f;
    let n = left_bit - right_bit + 1;
    ((1 << n) - 1) << right_bit
}

#[inline]
fn mms(p: &mut u32, v: u32, left_bit: u32, right_bit: u32) {
    let sm = mask(left_bit, right_bit);
    *p = (*p & (!sm)) | ((v << right_bit) & sm)
}

#[inline]
fn mme(v: u32, left_bit: u32, right_bit: u32) -> u32 {
    (v & mask(left_bit, right_bit)) >> right_bit
}

impl Descriptor {
    pub fn clear(&mut self) {
        self.f0 = 0;
        self.f1 = 0;
    }

    pub fn set_p(&mut self, v: u32) {
        mms(&mut self.f1, v, 15, 15)
    }

    pub fn get_p(&self) -> u32{
        mme(self.f1, 15, 15)
    }

    pub fn set_dpl(&mut self, v: u32) {
        mms(&mut self.f1, v, 14, 13)
    }

    pub fn set_s(&mut self, v: u32) {
        mms(&mut self.f1, v, 12, 12)
    }

    pub fn set_type(&mut self, v: u32) {
        mms(&mut self.f1, v, 11, 8)
    }

    pub fn set_selector(&mut self, v: u32) {
        mms(&mut self.f0, v, 31, 16)
    }

    pub fn set_offset(&mut self, v: u64) {
        mms(&mut self.f0, v as u32, 15, 0);
        mms(&mut self.f1, (v >> 16) as u32, 31, 16);
        mms(&mut self.f2, (v >> 32) as u32, 31, 0);
    }

    pub fn set_interrupt_descriptor(&mut self, selector: u32, offset: u64, dpl: u32) {
        self.clear();
        self.set_selector(selector);
        self.set_offset(offset);
        self.set_p(1);
        self.set_dpl(dpl);
        self.set_s(0);
        self.set_type(0xE);
    }
}
