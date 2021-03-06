#[doc = "Reader of register P5IV"]
pub type R = crate::R<u16, super::P5IV>;
#[doc = "Writer for register P5IV"]
pub type W = crate::W<u16, super::P5IV>;
#[doc = "Register P5IV `reset()`'s with value 0"]
impl crate::ResetValue for super::P5IV {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "4:0\\]
Port 5 interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P5IV_A {
    #[doc = "0: No interrupt pending"]
    NONE = 0,
    #[doc = "2: Interrupt Source: Port 5.0 interrupt; Interrupt Flag: P5IFG0; Interrupt Priority: Highest"]
    P5IFG0 = 2,
    #[doc = "4: Interrupt Source: Port 5.1 interrupt; Interrupt Flag: P5IFG1"]
    P5IFG1 = 4,
    #[doc = "6: Interrupt Source: Port 5.2 interrupt; Interrupt Flag: P5IFG2"]
    P5IFG2 = 6,
    #[doc = "8: Interrupt Source: Port 5.3 interrupt; Interrupt Flag: P5IFG3"]
    P5IFG3 = 8,
    #[doc = "10: Interrupt Source: Port 5.4 interrupt; Interrupt Flag: P5IFG4"]
    P5IFG4 = 10,
    #[doc = "12: Interrupt Source: Port 5.5 interrupt; Interrupt Flag: P5IFG5"]
    P5IFG5 = 12,
    #[doc = "14: Interrupt Source: Port 5.6 interrupt; Interrupt Flag: P5IFG6"]
    P5IFG6 = 14,
    #[doc = "16: Interrupt Source: Port 5.7 interrupt; Interrupt Flag: P5IFG7; Interrupt Priority: Lowest"]
    P5IFG7 = 16,
}
impl From<P5IV_A> for u8 {
    #[inline(always)]
    fn from(variant: P5IV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P5IV`"]
pub type P5IV_R = crate::R<u8, P5IV_A>;
impl P5IV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, P5IV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(P5IV_A::NONE),
            2 => Val(P5IV_A::P5IFG0),
            4 => Val(P5IV_A::P5IFG1),
            6 => Val(P5IV_A::P5IFG2),
            8 => Val(P5IV_A::P5IFG3),
            10 => Val(P5IV_A::P5IFG4),
            12 => Val(P5IV_A::P5IFG5),
            14 => Val(P5IV_A::P5IFG6),
            16 => Val(P5IV_A::P5IFG7),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == P5IV_A::NONE
    }
    #[doc = "Checks if the value of the field is `P5IFG0`"]
    #[inline(always)]
    pub fn is_p5ifg0(&self) -> bool {
        *self == P5IV_A::P5IFG0
    }
    #[doc = "Checks if the value of the field is `P5IFG1`"]
    #[inline(always)]
    pub fn is_p5ifg1(&self) -> bool {
        *self == P5IV_A::P5IFG1
    }
    #[doc = "Checks if the value of the field is `P5IFG2`"]
    #[inline(always)]
    pub fn is_p5ifg2(&self) -> bool {
        *self == P5IV_A::P5IFG2
    }
    #[doc = "Checks if the value of the field is `P5IFG3`"]
    #[inline(always)]
    pub fn is_p5ifg3(&self) -> bool {
        *self == P5IV_A::P5IFG3
    }
    #[doc = "Checks if the value of the field is `P5IFG4`"]
    #[inline(always)]
    pub fn is_p5ifg4(&self) -> bool {
        *self == P5IV_A::P5IFG4
    }
    #[doc = "Checks if the value of the field is `P5IFG5`"]
    #[inline(always)]
    pub fn is_p5ifg5(&self) -> bool {
        *self == P5IV_A::P5IFG5
    }
    #[doc = "Checks if the value of the field is `P5IFG6`"]
    #[inline(always)]
    pub fn is_p5ifg6(&self) -> bool {
        *self == P5IV_A::P5IFG6
    }
    #[doc = "Checks if the value of the field is `P5IFG7`"]
    #[inline(always)]
    pub fn is_p5ifg7(&self) -> bool {
        *self == P5IV_A::P5IFG7
    }
}
#[doc = "Write proxy for field `P5IV`"]
pub struct P5IV_W<'a> {
    w: &'a mut W,
}
impl<'a> P5IV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P5IV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(P5IV_A::NONE)
    }
    #[doc = "Interrupt Source: Port 5.0 interrupt; Interrupt Flag: P5IFG0; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn p5ifg0(self) -> &'a mut W {
        self.variant(P5IV_A::P5IFG0)
    }
    #[doc = "Interrupt Source: Port 5.1 interrupt; Interrupt Flag: P5IFG1"]
    #[inline(always)]
    pub fn p5ifg1(self) -> &'a mut W {
        self.variant(P5IV_A::P5IFG1)
    }
    #[doc = "Interrupt Source: Port 5.2 interrupt; Interrupt Flag: P5IFG2"]
    #[inline(always)]
    pub fn p5ifg2(self) -> &'a mut W {
        self.variant(P5IV_A::P5IFG2)
    }
    #[doc = "Interrupt Source: Port 5.3 interrupt; Interrupt Flag: P5IFG3"]
    #[inline(always)]
    pub fn p5ifg3(self) -> &'a mut W {
        self.variant(P5IV_A::P5IFG3)
    }
    #[doc = "Interrupt Source: Port 5.4 interrupt; Interrupt Flag: P5IFG4"]
    #[inline(always)]
    pub fn p5ifg4(self) -> &'a mut W {
        self.variant(P5IV_A::P5IFG4)
    }
    #[doc = "Interrupt Source: Port 5.5 interrupt; Interrupt Flag: P5IFG5"]
    #[inline(always)]
    pub fn p5ifg5(self) -> &'a mut W {
        self.variant(P5IV_A::P5IFG5)
    }
    #[doc = "Interrupt Source: Port 5.6 interrupt; Interrupt Flag: P5IFG6"]
    #[inline(always)]
    pub fn p5ifg6(self) -> &'a mut W {
        self.variant(P5IV_A::P5IFG6)
    }
    #[doc = "Interrupt Source: Port 5.7 interrupt; Interrupt Flag: P5IFG7; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn p5ifg7(self) -> &'a mut W {
        self.variant(P5IV_A::P5IFG7)
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
Port 5 interrupt vector value"]
    #[inline(always)]
    pub fn p5iv(&self) -> P5IV_R {
        P5IV_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Port 5 interrupt vector value"]
    #[inline(always)]
    pub fn p5iv(&mut self) -> P5IV_W {
        P5IV_W { w: self }
    }
}
