#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 32usize],
    #[doc = "0x20 - Port 3 Input"]
    pub p3in: P3IN,
    _reserved1: [u8; 1usize],
    #[doc = "0x22 - Port 3 Output"]
    pub p3out: P3OUT,
    _reserved2: [u8; 1usize],
    #[doc = "0x24 - Port 3 Direction"]
    pub p3dir: P3DIR,
    _reserved3: [u8; 1usize],
    #[doc = "0x26 - Port 3 Resistor Enable"]
    pub p3ren: P3REN,
    _reserved4: [u8; 3usize],
    #[doc = "0x2a - Port 3 Select 0"]
    pub p3sel0: P3SEL0,
    _reserved5: [u8; 1usize],
    #[doc = "0x2c - Port 3 Select 1"]
    pub p3sel1: P3SEL1,
    _reserved6: [u8; 1usize],
    #[doc = "0x2e - Port 3 Interrupt Vector Register"]
    pub p3iv: P3IV,
    _reserved7: [u8; 6usize],
    #[doc = "0x36 - Port 3 Complement Select"]
    pub p3selc: P3SELC,
    _reserved8: [u8; 1usize],
    #[doc = "0x38 - Port 3 Interrupt Edge Select"]
    pub p3ies: P3IES,
    _reserved9: [u8; 1usize],
    #[doc = "0x3a - Port 3 Interrupt Enable"]
    pub p3ie: P3IE,
    _reserved10: [u8; 1usize],
    #[doc = "0x3c - Port 3 Interrupt Flag"]
    pub p3ifg: P3IFG,
}
#[doc = "Port 3 Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p3iv](p3iv) module"]
pub type P3IV = crate::Reg<u16, _P3IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P3IV;
#[doc = "`read()` method returns [p3iv::R](p3iv::R) reader structure"]
impl crate::Readable for P3IV {}
#[doc = "`write(|w| ..)` method takes [p3iv::W](p3iv::W) writer structure"]
impl crate::Writable for P3IV {}
#[doc = "Port 3 Interrupt Vector Register"]
pub mod p3iv;
#[doc = "Port 3 Input\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p3in](p3in) module"]
pub type P3IN = crate::Reg<u8, _P3IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P3IN;
#[doc = "`read()` method returns [p3in::R](p3in::R) reader structure"]
impl crate::Readable for P3IN {}
#[doc = "`write(|w| ..)` method takes [p3in::W](p3in::W) writer structure"]
impl crate::Writable for P3IN {}
#[doc = "Port 3 Input"]
pub mod p3in;
#[doc = "Port 3 Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p3out](p3out) module"]
pub type P3OUT = crate::Reg<u8, _P3OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P3OUT;
#[doc = "`read()` method returns [p3out::R](p3out::R) reader structure"]
impl crate::Readable for P3OUT {}
#[doc = "`write(|w| ..)` method takes [p3out::W](p3out::W) writer structure"]
impl crate::Writable for P3OUT {}
#[doc = "Port 3 Output"]
pub mod p3out;
#[doc = "Port 3 Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p3dir](p3dir) module"]
pub type P3DIR = crate::Reg<u8, _P3DIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P3DIR;
#[doc = "`read()` method returns [p3dir::R](p3dir::R) reader structure"]
impl crate::Readable for P3DIR {}
#[doc = "`write(|w| ..)` method takes [p3dir::W](p3dir::W) writer structure"]
impl crate::Writable for P3DIR {}
#[doc = "Port 3 Direction"]
pub mod p3dir;
#[doc = "Port 3 Resistor Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p3ren](p3ren) module"]
pub type P3REN = crate::Reg<u8, _P3REN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P3REN;
#[doc = "`read()` method returns [p3ren::R](p3ren::R) reader structure"]
impl crate::Readable for P3REN {}
#[doc = "`write(|w| ..)` method takes [p3ren::W](p3ren::W) writer structure"]
impl crate::Writable for P3REN {}
#[doc = "Port 3 Resistor Enable"]
pub mod p3ren;
#[doc = "Port 3 Select 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p3sel0](p3sel0) module"]
pub type P3SEL0 = crate::Reg<u8, _P3SEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P3SEL0;
#[doc = "`read()` method returns [p3sel0::R](p3sel0::R) reader structure"]
impl crate::Readable for P3SEL0 {}
#[doc = "`write(|w| ..)` method takes [p3sel0::W](p3sel0::W) writer structure"]
impl crate::Writable for P3SEL0 {}
#[doc = "Port 3 Select 0"]
pub mod p3sel0;
#[doc = "Port 3 Select 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p3sel1](p3sel1) module"]
pub type P3SEL1 = crate::Reg<u8, _P3SEL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P3SEL1;
#[doc = "`read()` method returns [p3sel1::R](p3sel1::R) reader structure"]
impl crate::Readable for P3SEL1 {}
#[doc = "`write(|w| ..)` method takes [p3sel1::W](p3sel1::W) writer structure"]
impl crate::Writable for P3SEL1 {}
#[doc = "Port 3 Select 1"]
pub mod p3sel1;
#[doc = "Port 3 Complement Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p3selc](p3selc) module"]
pub type P3SELC = crate::Reg<u8, _P3SELC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P3SELC;
#[doc = "`read()` method returns [p3selc::R](p3selc::R) reader structure"]
impl crate::Readable for P3SELC {}
#[doc = "`write(|w| ..)` method takes [p3selc::W](p3selc::W) writer structure"]
impl crate::Writable for P3SELC {}
#[doc = "Port 3 Complement Select"]
pub mod p3selc;
#[doc = "Port 3 Interrupt Edge Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p3ies](p3ies) module"]
pub type P3IES = crate::Reg<u8, _P3IES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P3IES;
#[doc = "`read()` method returns [p3ies::R](p3ies::R) reader structure"]
impl crate::Readable for P3IES {}
#[doc = "`write(|w| ..)` method takes [p3ies::W](p3ies::W) writer structure"]
impl crate::Writable for P3IES {}
#[doc = "Port 3 Interrupt Edge Select"]
pub mod p3ies;
#[doc = "Port 3 Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p3ie](p3ie) module"]
pub type P3IE = crate::Reg<u8, _P3IE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P3IE;
#[doc = "`read()` method returns [p3ie::R](p3ie::R) reader structure"]
impl crate::Readable for P3IE {}
#[doc = "`write(|w| ..)` method takes [p3ie::W](p3ie::W) writer structure"]
impl crate::Writable for P3IE {}
#[doc = "Port 3 Interrupt Enable"]
pub mod p3ie;
#[doc = "Port 3 Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p3ifg](p3ifg) module"]
pub type P3IFG = crate::Reg<u8, _P3IFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P3IFG;
#[doc = "`read()` method returns [p3ifg::R](p3ifg::R) reader structure"]
impl crate::Readable for P3IFG {}
#[doc = "`write(|w| ..)` method takes [p3ifg::W](p3ifg::W) writer structure"]
impl crate::Writable for P3IFG {}
#[doc = "Port 3 Interrupt Flag"]
pub mod p3ifg;
