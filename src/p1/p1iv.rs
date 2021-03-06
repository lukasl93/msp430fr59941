#[doc = "Reader of register P1IV"]
pub type R = crate::R<u16, super::P1IV>;
#[doc = "Writer for register P1IV"]
pub type W = crate::W<u16, super::P1IV>;
#[doc = "Register P1IV `reset()`'s with value 0"]
impl crate::ResetValue for super::P1IV {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "4:0\\]
Port 1 interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P1IV_A {
    #[doc = "0: No interrupt pending"]
    NONE = 0,
    #[doc = "2: Interrupt Source: Port 1.0 interrupt; Interrupt Flag: P1IFG0; Interrupt Priority: Highest"]
    P1IFG0 = 2,
    #[doc = "4: Interrupt Source: Port 1.1 interrupt; Interrupt Flag: P1IFG1"]
    P1IFG1 = 4,
    #[doc = "6: Interrupt Source: Port 1.2 interrupt; Interrupt Flag: P1IFG2"]
    P1IFG2 = 6,
    #[doc = "8: Interrupt Source: Port 1.3 interrupt; Interrupt Flag: P1IFG3"]
    P1IFG3 = 8,
    #[doc = "10: Interrupt Source: Port 1.4 interrupt; Interrupt Flag: P1IFG4"]
    P1IFG4 = 10,
    #[doc = "12: Interrupt Source: Port 1.5 interrupt; Interrupt Flag: P1IFG5"]
    P1IFG5 = 12,
    #[doc = "14: Interrupt Source: Port 1.6 interrupt; Interrupt Flag: P1IFG6"]
    P1IFG6 = 14,
    #[doc = "16: Interrupt Source: Port 1.7 interrupt; Interrupt Flag: P1IFG7; Interrupt Priority: Lowest"]
    P1IFG7 = 16,
}
impl From<P1IV_A> for u8 {
    #[inline(always)]
    fn from(variant: P1IV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P1IV`"]
pub type P1IV_R = crate::R<u8, P1IV_A>;
impl P1IV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, P1IV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(P1IV_A::NONE),
            2 => Val(P1IV_A::P1IFG0),
            4 => Val(P1IV_A::P1IFG1),
            6 => Val(P1IV_A::P1IFG2),
            8 => Val(P1IV_A::P1IFG3),
            10 => Val(P1IV_A::P1IFG4),
            12 => Val(P1IV_A::P1IFG5),
            14 => Val(P1IV_A::P1IFG6),
            16 => Val(P1IV_A::P1IFG7),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == P1IV_A::NONE
    }
    #[doc = "Checks if the value of the field is `P1IFG0`"]
    #[inline(always)]
    pub fn is_p1ifg0(&self) -> bool {
        *self == P1IV_A::P1IFG0
    }
    #[doc = "Checks if the value of the field is `P1IFG1`"]
    #[inline(always)]
    pub fn is_p1ifg1(&self) -> bool {
        *self == P1IV_A::P1IFG1
    }
    #[doc = "Checks if the value of the field is `P1IFG2`"]
    #[inline(always)]
    pub fn is_p1ifg2(&self) -> bool {
        *self == P1IV_A::P1IFG2
    }
    #[doc = "Checks if the value of the field is `P1IFG3`"]
    #[inline(always)]
    pub fn is_p1ifg3(&self) -> bool {
        *self == P1IV_A::P1IFG3
    }
    #[doc = "Checks if the value of the field is `P1IFG4`"]
    #[inline(always)]
    pub fn is_p1ifg4(&self) -> bool {
        *self == P1IV_A::P1IFG4
    }
    #[doc = "Checks if the value of the field is `P1IFG5`"]
    #[inline(always)]
    pub fn is_p1ifg5(&self) -> bool {
        *self == P1IV_A::P1IFG5
    }
    #[doc = "Checks if the value of the field is `P1IFG6`"]
    #[inline(always)]
    pub fn is_p1ifg6(&self) -> bool {
        *self == P1IV_A::P1IFG6
    }
    #[doc = "Checks if the value of the field is `P1IFG7`"]
    #[inline(always)]
    pub fn is_p1ifg7(&self) -> bool {
        *self == P1IV_A::P1IFG7
    }
}
#[doc = "Write proxy for field `P1IV`"]
pub struct P1IV_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1IV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(P1IV_A::NONE)
    }
    #[doc = "Interrupt Source: Port 1.0 interrupt; Interrupt Flag: P1IFG0; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn p1ifg0(self) -> &'a mut W {
        self.variant(P1IV_A::P1IFG0)
    }
    #[doc = "Interrupt Source: Port 1.1 interrupt; Interrupt Flag: P1IFG1"]
    #[inline(always)]
    pub fn p1ifg1(self) -> &'a mut W {
        self.variant(P1IV_A::P1IFG1)
    }
    #[doc = "Interrupt Source: Port 1.2 interrupt; Interrupt Flag: P1IFG2"]
    #[inline(always)]
    pub fn p1ifg2(self) -> &'a mut W {
        self.variant(P1IV_A::P1IFG2)
    }
    #[doc = "Interrupt Source: Port 1.3 interrupt; Interrupt Flag: P1IFG3"]
    #[inline(always)]
    pub fn p1ifg3(self) -> &'a mut W {
        self.variant(P1IV_A::P1IFG3)
    }
    #[doc = "Interrupt Source: Port 1.4 interrupt; Interrupt Flag: P1IFG4"]
    #[inline(always)]
    pub fn p1ifg4(self) -> &'a mut W {
        self.variant(P1IV_A::P1IFG4)
    }
    #[doc = "Interrupt Source: Port 1.5 interrupt; Interrupt Flag: P1IFG5"]
    #[inline(always)]
    pub fn p1ifg5(self) -> &'a mut W {
        self.variant(P1IV_A::P1IFG5)
    }
    #[doc = "Interrupt Source: Port 1.6 interrupt; Interrupt Flag: P1IFG6"]
    #[inline(always)]
    pub fn p1ifg6(self) -> &'a mut W {
        self.variant(P1IV_A::P1IFG6)
    }
    #[doc = "Interrupt Source: Port 1.7 interrupt; Interrupt Flag: P1IFG7; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn p1ifg7(self) -> &'a mut W {
        self.variant(P1IV_A::P1IFG7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u16) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Port 1 interrupt vector value"]
    #[inline(always)]
    pub fn p1iv(&self) -> P1IV_R {
        P1IV_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Port 1 interrupt vector value"]
    #[inline(always)]
    pub fn p1iv(&mut self) -> P1IV_W {
        P1IV_W { w: self }
    }
}
