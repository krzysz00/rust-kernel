#[derive(Copy, Clone)]
#[repr(C,packed)]
pub struct Descriptor {
    pub f0: u32, pub f1: u32,
}

#[repr(C,packed)]
pub struct TableDescriptor {
    pub limit: u16, pub base: u32
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
    fn clear(&mut self) {
        self.f0 = 0;
        self.f1 = 0;
    }

    fn set_p(&mut self, v: u32) {
        mms(&mut self.f1, v, 15, 15)
    }

    fn get_p(&self) -> u32 {
        mme(self.f1, 15, 15)
    }

    fn set_dpl(&mut self, v: u32) {
        mms(&mut self.f1, v, 14, 13)
    }

    fn set_s(&mut self, v: u32) {
        mms(&mut self.f1, v, 12, 12)
    }

    fn set_type(&mut self, v: u32) {
        mms(&mut self.f1, v, 11, 8)
    }

    fn set_selector(&mut self, v: u32) {
        mms(&mut self.f0, v, 31, 16)
    }

    fn set_offset(&mut self, v: u32) {
        mms(&mut self.f0, v, 15, 0);
        mms(&mut self.f1, v >> 16, 19, 16)
    }

    fn set_base(&mut self, v: u32) {
        mms(&mut self.f0, v >> 0, 31, 16);
        mms(&mut self.f1, v >> 16, 7, 0);
        mms(&mut self.f1, v >> 24, 31, 24);
    }

    fn set_limit(&mut self, v: u32) {
        mms(&mut self.f0, v, 15,  0);
        mms(&mut self.f1, v >> 16, 19, 16);
    }

    pub fn set_descriptor(&mut self, selector: u32, offset: u32, dpl: u32, typ: u32) {
        self.clear();
        self.set_selector(selector);
        self.set_offset(offset);
        self.set_p(1);
        self.set_dpl(dpl);
        self.set_s(0);
        self.set_type(typ);
    }

    pub fn set_tss_descriptor(&mut self, base: u32, limit: u32, dpl: u32) {
        self.clear();
        self.set_base(base);
        self.set_limit(limit);
        self.set_p(1);
        self.set_dpl(dpl);
        self.set_s(0);
        self.set_type(0x9);
    }
}
