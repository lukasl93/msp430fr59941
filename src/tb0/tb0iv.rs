#[doc = "Reader of register TB0IV"]
pub type R = crate::R<u16, super::TB0IV>;
#[doc = "Writer for register TB0IV"]
pub type W = crate::W<u16, super::TB0IV>;
#[doc = "Register TB0IV `reset()`'s with value 0"]
impl crate::ResetValue for super::TB0IV {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "15:0\\]
Timer_B interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum TBIV_A {
    #[doc = "0: No interrupt pending"]
    NONE = 0,
    #[doc = "2: Interrupt Source: Capture/compare 1; Interrupt Flag: TBxCCR1 CCIFG; Interrupt Priority: Highest"]
    TBCCR1 = 2,
    #[doc = "4: Interrupt Source: Capture/compare 2; Interrupt Flag: TBxCCR2 CCIFG"]
    TBCCR2 = 4,
    #[doc = "6: Interrupt Source: Capture/compare 3; Interrupt Flag: TBxCCR3 CCIFG"]
    TBCCR3 = 6,
    #[doc = "8: Interrupt Source: Capture/compare 4; Interrupt Flag: TBxCCR4 CCIFG"]
    TBCCR4 = 8,
    #[doc = "10: Interrupt Source: Capture/compare 5; Interrupt Flag: TBxCCR5 CCIFG"]
    TBCCR5 = 10,
    #[doc = "12: Interrupt Source: Capture/compare 6; Interrupt Flag: TBxCCR6 CCIFG"]
    TBCCR6 = 12,
    #[doc = "14: Interrupt Source: Timer overflow; Interrupt Flag: TBxCTL TBIFG; Interrupt Priority: Lowest"]
    TBIFG = 14,
}
impl From<TBIV_A> for u16 {
    #[inline(always)]
    fn from(variant: TBIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TBIV`"]
pub type TBIV_R = crate::R<u16, TBIV_A>;
impl TBIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, TBIV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TBIV_A::NONE),
            2 => Val(TBIV_A::TBCCR1),
            4 => Val(TBIV_A::TBCCR2),
            6 => Val(TBIV_A::TBCCR3),
            8 => Val(TBIV_A::TBCCR4),
            10 => Val(TBIV_A::TBCCR5),
            12 => Val(TBIV_A::TBCCR6),
            14 => Val(TBIV_A::TBIFG),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == TBIV_A::NONE
    }
    #[doc = "Checks if the value of the field is `TBCCR1`"]
    #[inline(always)]
    pub fn is_tbccr1(&self) -> bool {
        *self == TBIV_A::TBCCR1
    }
    #[doc = "Checks if the value of the field is `TBCCR2`"]
    #[inline(always)]
    pub fn is_tbccr2(&self) -> bool {
        *self == TBIV_A::TBCCR2
    }
    #[doc = "Checks if the value of the field is `TBCCR3`"]
    #[inline(always)]
    pub fn is_tbccr3(&self) -> bool {
        *self == TBIV_A::TBCCR3
    }
    #[doc = "Checks if the value of the field is `TBCCR4`"]
    #[inline(always)]
    pub fn is_tbccr4(&self) -> bool {
        *self == TBIV_A::TBCCR4
    }
    #[doc = "Checks if the value of the field is `TBCCR5`"]
    #[inline(always)]
    pub fn is_tbccr5(&self) -> bool {
        *self == TBIV_A::TBCCR5
    }
    #[doc = "Checks if the value of the field is `TBCCR6`"]
    #[inline(always)]
    pub fn is_tbccr6(&self) -> bool {
        *self == TBIV_A::TBCCR6
    }
    #[doc = "Checks if the value of the field is `TBIFG`"]
    #[inline(always)]
    pub fn is_tbifg(&self) -> bool {
        *self == TBIV_A::TBIFG
    }
}
#[doc = "Write proxy for field `TBIV`"]
pub struct TBIV_W<'a> {
    w: &'a mut W,
}
impl<'a> TBIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TBIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(TBIV_A::NONE)
    }
    #[doc = "Interrupt Source: Capture/compare 1; Interrupt Flag: TBxCCR1 CCIFG; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn tbccr1(self) -> &'a mut W {
        self.variant(TBIV_A::TBCCR1)
    }
    #[doc = "Interrupt Source: Capture/compare 2; Interrupt Flag: TBxCCR2 CCIFG"]
    #[inline(always)]
    pub fn tbccr2(self) -> &'a mut W {
        self.variant(TBIV_A::TBCCR2)
    }
    #[doc = "Interrupt Source: Capture/compare 3; Interrupt Flag: TBxCCR3 CCIFG"]
    #[inline(always)]
    pub fn tbccr3(self) -> &'a mut W {
        self.variant(TBIV_A::TBCCR3)
    }
    #[doc = "Interrupt Source: Capture/compare 4; Interrupt Flag: TBxCCR4 CCIFG"]
    #[inline(always)]
    pub fn tbccr4(self) -> &'a mut W {
        self.variant(TBIV_A::TBCCR4)
    }
    #[doc = "Interrupt Source: Capture/compare 5; Interrupt Flag: TBxCCR5 CCIFG"]
    #[inline(always)]
    pub fn tbccr5(self) -> &'a mut W {
        self.variant(TBIV_A::TBCCR5)
    }
    #[doc = "Interrupt Source: Capture/compare 6; Interrupt Flag: TBxCCR6 CCIFG"]
    #[inline(always)]
    pub fn tbccr6(self) -> &'a mut W {
        self.variant(TBIV_A::TBCCR6)
    }
    #[doc = "Interrupt Source: Timer overflow; Interrupt Flag: TBxCTL TBIFG; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn tbifg(self) -> &'a mut W {
        self.variant(TBIV_A::TBIFG)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Timer_B interrupt vector value"]
    #[inline(always)]
    pub fn tbiv(&self) -> TBIV_R {
        TBIV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Timer_B interrupt vector value"]
    #[inline(always)]
    pub fn tbiv(&mut self) -> TBIV_W {
        TBIV_W { w: self }
    }
}
