#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 65usize],
    #[doc = "0x41 - Port 6 Input"]
    pub p6in: P6IN,
    _reserved1: [u8; 1usize],
    #[doc = "0x43 - Port 6 Output"]
    pub p6out: P6OUT,
    _reserved2: [u8; 1usize],
    #[doc = "0x45 - Port 6 Direction"]
    pub p6dir: P6DIR,
    _reserved3: [u8; 1usize],
    #[doc = "0x47 - Port 6 Resistor Enable"]
    pub p6ren: P6REN,
    _reserved4: [u8; 3usize],
    #[doc = "0x4b - Port 6 Select 0"]
    pub p6sel0: P6SEL0,
    _reserved5: [u8; 1usize],
    #[doc = "0x4d - Port 6 Select 1"]
    pub p6sel1: P6SEL1,
    _reserved6: [u8; 9usize],
    #[doc = "0x57 - Port 6 Complement Select"]
    pub p6selc: P6SELC,
    _reserved7: [u8; 1usize],
    #[doc = "0x59 - Port 6 Interrupt Edge Select"]
    pub p6ies: P6IES,
    _reserved8: [u8; 1usize],
    #[doc = "0x5b - Port 6 Interrupt Enable"]
    pub p6ie: P6IE,
    _reserved9: [u8; 1usize],
    #[doc = "0x5d - Port 6 Interrupt Flag"]
    pub p6ifg: P6IFG,
    #[doc = "0x5e - Port 6 Interrupt Vector Register"]
    pub p6iv: P6IV,
}
#[doc = "Port 6 Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p6iv](p6iv) module"]
pub type P6IV = crate::Reg<u16, _P6IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P6IV;
#[doc = "`read()` method returns [p6iv::R](p6iv::R) reader structure"]
impl crate::Readable for P6IV {}
#[doc = "`write(|w| ..)` method takes [p6iv::W](p6iv::W) writer structure"]
impl crate::Writable for P6IV {}
#[doc = "Port 6 Interrupt Vector Register"]
pub mod p6iv;
#[doc = "Port 6 Input\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p6in](p6in) module"]
pub type P6IN = crate::Reg<u8, _P6IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P6IN;
#[doc = "`read()` method returns [p6in::R](p6in::R) reader structure"]
impl crate::Readable for P6IN {}
#[doc = "`write(|w| ..)` method takes [p6in::W](p6in::W) writer structure"]
impl crate::Writable for P6IN {}
#[doc = "Port 6 Input"]
pub mod p6in;
#[doc = "Port 6 Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p6out](p6out) module"]
pub type P6OUT = crate::Reg<u8, _P6OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P6OUT;
#[doc = "`read()` method returns [p6out::R](p6out::R) reader structure"]
impl crate::Readable for P6OUT {}
#[doc = "`write(|w| ..)` method takes [p6out::W](p6out::W) writer structure"]
impl crate::Writable for P6OUT {}
#[doc = "Port 6 Output"]
pub mod p6out;
#[doc = "Port 6 Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p6dir](p6dir) module"]
pub type P6DIR = crate::Reg<u8, _P6DIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P6DIR;
#[doc = "`read()` method returns [p6dir::R](p6dir::R) reader structure"]
impl crate::Readable for P6DIR {}
#[doc = "`write(|w| ..)` method takes [p6dir::W](p6dir::W) writer structure"]
impl crate::Writable for P6DIR {}
#[doc = "Port 6 Direction"]
pub mod p6dir;
#[doc = "Port 6 Resistor Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p6ren](p6ren) module"]
pub type P6REN = crate::Reg<u8, _P6REN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P6REN;
#[doc = "`read()` method returns [p6ren::R](p6ren::R) reader structure"]
impl crate::Readable for P6REN {}
#[doc = "`write(|w| ..)` method takes [p6ren::W](p6ren::W) writer structure"]
impl crate::Writable for P6REN {}
#[doc = "Port 6 Resistor Enable"]
pub mod p6ren;
#[doc = "Port 6 Select 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p6sel0](p6sel0) module"]
pub type P6SEL0 = crate::Reg<u8, _P6SEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P6SEL0;
#[doc = "`read()` method returns [p6sel0::R](p6sel0::R) reader structure"]
impl crate::Readable for P6SEL0 {}
#[doc = "`write(|w| ..)` method takes [p6sel0::W](p6sel0::W) writer structure"]
impl crate::Writable for P6SEL0 {}
#[doc = "Port 6 Select 0"]
pub mod p6sel0;
#[doc = "Port 6 Select 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p6sel1](p6sel1) module"]
pub type P6SEL1 = crate::Reg<u8, _P6SEL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P6SEL1;
#[doc = "`read()` method returns [p6sel1::R](p6sel1::R) reader structure"]
impl crate::Readable for P6SEL1 {}
#[doc = "`write(|w| ..)` method takes [p6sel1::W](p6sel1::W) writer structure"]
impl crate::Writable for P6SEL1 {}
#[doc = "Port 6 Select 1"]
pub mod p6sel1;
#[doc = "Port 6 Complement Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p6selc](p6selc) module"]
pub type P6SELC = crate::Reg<u8, _P6SELC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P6SELC;
#[doc = "`read()` method returns [p6selc::R](p6selc::R) reader structure"]
impl crate::Readable for P6SELC {}
#[doc = "`write(|w| ..)` method takes [p6selc::W](p6selc::W) writer structure"]
impl crate::Writable for P6SELC {}
#[doc = "Port 6 Complement Select"]
pub mod p6selc;
#[doc = "Port 6 Interrupt Edge Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p6ies](p6ies) module"]
pub type P6IES = crate::Reg<u8, _P6IES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P6IES;
#[doc = "`read()` method returns [p6ies::R](p6ies::R) reader structure"]
impl crate::Readable for P6IES {}
#[doc = "`write(|w| ..)` method takes [p6ies::W](p6ies::W) writer structure"]
impl crate::Writable for P6IES {}
#[doc = "Port 6 Interrupt Edge Select"]
pub mod p6ies;
#[doc = "Port 6 Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p6ie](p6ie) module"]
pub type P6IE = crate::Reg<u8, _P6IE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P6IE;
#[doc = "`read()` method returns [p6ie::R](p6ie::R) reader structure"]
impl crate::Readable for P6IE {}
#[doc = "`write(|w| ..)` method takes [p6ie::W](p6ie::W) writer structure"]
impl crate::Writable for P6IE {}
#[doc = "Port 6 Interrupt Enable"]
pub mod p6ie;
#[doc = "Port 6 Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p6ifg](p6ifg) module"]
pub type P6IFG = crate::Reg<u8, _P6IFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P6IFG;
#[doc = "`read()` method returns [p6ifg::R](p6ifg::R) reader structure"]
impl crate::Readable for P6IFG {}
#[doc = "`write(|w| ..)` method takes [p6ifg::W](p6ifg::W) writer structure"]
impl crate::Writable for P6IFG {}
#[doc = "Port 6 Interrupt Flag"]
pub mod p6ifg;
