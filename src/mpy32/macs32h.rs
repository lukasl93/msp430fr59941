#[doc = "Reader of register MACS32H"]
pub type R = crate::R<u16, super::MACS32H>;
#[doc = "Writer for register MACS32H"]
pub type W = crate::W<u16, super::MACS32H>;
#[doc = "Register MACS32H `reset()`'s with value 0"]
impl crate::ResetValue for super::MACS32H {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MACS32H`"]
pub type MACS32H_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MACS32H`"]
pub struct MACS32H_W<'a> {
    w: &'a mut W,
}
impl<'a> MACS32H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15 - 15:8\\]
32-bit operand 1 signed multiply accumulate high word"]
    #[inline(always)]
    pub fn macs32h(&self) -> MACS32H_R {
        MACS32H_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - 15:8\\]
32-bit operand 1 signed multiply accumulate high word"]
    #[inline(always)]
    pub fn macs32h(&mut self) -> MACS32H_W {
        MACS32H_W { w: self }
    }
}
