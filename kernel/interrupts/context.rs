use paging::KERNEL_CR3;
use machine::to_user_mode;

pub struct RawContext {
    cr3: u32,
    _ds: u32,
    edi: u32,
    esi: u32,
    ebp: u32,
    _int_esp: u32,
    ebx: u32,
    edx: u32,
    ecx: u32,
    eax: u32,
    eip: u32,
    _cs: u32,
    eflags: u32,
    esp: u32,
    ss: u32,
}

pub struct RawErrContext {
    cr3: u32,
    _ds: u32,
    edi: u32,
    esi: u32,
    ebp: u32,
    _int_esp: u32,
    ebx: u32,
    edx: u32,
    ecx: u32,
    eax: u32,
    _error: u32,
    eip: u32,
    _cs: u32,
    eflags: u32,
    esp: u32,
    _ss: u32
}

pub struct Context {
    cr3: u32,
    edi: u32,
    esi: u32,
    ebp: u32,
    ebx: u32,
    edx: u32,
    ecx: u32,
    eax: u32,
    eip: u32,
    eflags: u32,
    esp: u32,
}

impl RawContext {
    pub fn set_return_code(&mut self, code: u32) {
        self.eax = code;
    }

    pub fn was_kernel(&self) -> bool {
        self.ss != 0x23
    }

    pub fn user_paging(&self) {
        unsafe {
            asm!("mov %eax, %cr3" :: "{eax}"(self.cr3));
        }
    }

    pub fn kernel_paging(&self) {
        unsafe {
            asm!("mov %eax, %cr3" ::"{eax}"(KERNEL_CR3));
        }
    }
}

impl Context {
    pub fn new(eip: usize, esp: usize, cr3: usize) -> Context {
        Context {
            cr3: cr3 as u32,
            edi: 0, esi: 0, ebp: 0, ebx: 0,
            edx: 0, ecx: 0, eax: 0,
            eip: eip as u32, eflags: 0x202, // IF and the always-1 bit
            esp: esp as u32,
        }
    }

    pub fn to_user_mode(&self) -> ! {
        to_user_mode(self.eip, self.esp, self.cr3);
    }
}

pub trait Contextable {
    fn save(&self, context: &mut Context);
    fn load(&mut self, context: &Context);
}

macro_rules! transfer_context {
    ($from:ident, $to:ident) => ({
        $to.cr3 = $from.cr3;
        $to.edi = $from.edi;
        $to.esi = $from.esi;
        $to.ebp = $from.ebp;
        $to.ebx = $from.ebx;
        $to.edx = $from.edx;
        $to.ecx = $from.ecx;
        $to.eax = $from.eax;
        $to.eip = $from.eip;
        $to.eflags = $from.eflags;
        $to.esp = $from.esp;
    });
}

impl Contextable for RawContext {
    fn save(&self, context: &mut Context) {
        transfer_context!(self, context)
    }

    fn load(&mut self, context: &Context) {
        transfer_context!(context, self)
    }
}

impl Contextable for RawErrContext {
    fn save(&self, context: &mut Context) {
        transfer_context!(self, context)
    }

    fn load(&mut self, context: &Context) {
        transfer_context!(context, self)
    }
}
