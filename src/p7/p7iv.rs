#[doc = "Reader of register P7IV"]
pub type R = crate::R<u16, super::P7IV>;
#[doc = "Writer for register P7IV"]
pub type W = crate::W<u16, super::P7IV>;
#[doc = "Register P7IV `reset()`'s with value 0"]
impl crate::ResetValue for super::P7IV {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "4:0\\]
Port 7 interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P7IV_A {
    #[doc = "0: No interrupt pending"]
    NONE = 0,
    #[doc = "2: Interrupt Source: Port 7.0 interrupt; Interrupt Flag: P7IFG0; Interrupt Priority: Highest"]
    P7IFG0 = 2,
    #[doc = "4: Interrupt Source: Port 7.1 interrupt; Interrupt Flag: P7IFG1"]
    P7IFG1 = 4,
    #[doc = "6: Interrupt Source: Port 7.2 interrupt; Interrupt Flag: P7IFG2"]
    P7IFG2 = 6,
    #[doc = "8: Interrupt Source: Port 7.3 interrupt; Interrupt Flag: P7IFG3"]
    P7IFG3 = 8,
    #[doc = "10: Interrupt Source: Port 7.4 interrupt; Interrupt Flag: P7IFG4"]
    P7IFG4 = 10,
    #[doc = "12: Interrupt Source: Port 7.5 interrupt; Interrupt Flag: P7IFG5"]
    P7IFG5 = 12,
    #[doc = "14: Interrupt Source: Port 7.6 interrupt; Interrupt Flag: P7IFG6"]
    P7IFG6 = 14,
    #[doc = "16: Interrupt Source: Port 7.7 interrupt; Interrupt Flag: P7IFG7; Interrupt Priority: Lowest"]
    P7IFG7 = 16,
}
impl From<P7IV_A> for u8 {
    #[inline(always)]
    fn from(variant: P7IV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P7IV`"]
pub type P7IV_R = crate::R<u8, P7IV_A>;
impl P7IV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, P7IV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(P7IV_A::NONE),
            2 => Val(P7IV_A::P7IFG0),
            4 => Val(P7IV_A::P7IFG1),
            6 => Val(P7IV_A::P7IFG2),
            8 => Val(P7IV_A::P7IFG3),
            10 => Val(P7IV_A::P7IFG4),
            12 => Val(P7IV_A::P7IFG5),
            14 => Val(P7IV_A::P7IFG6),
            16 => Val(P7IV_A::P7IFG7),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == P7IV_A::NONE
    }
    #[doc = "Checks if the value of the field is `P7IFG0`"]
    #[inline(always)]
    pub fn is_p7ifg0(&self) -> bool {
        *self == P7IV_A::P7IFG0
    }
    #[doc = "Checks if the value of the field is `P7IFG1`"]
    #[inline(always)]
    pub fn is_p7ifg1(&self) -> bool {
        *self == P7IV_A::P7IFG1
    }
    #[doc = "Checks if the value of the field is `P7IFG2`"]
    #[inline(always)]
    pub fn is_p7ifg2(&self) -> bool {
        *self == P7IV_A::P7IFG2
    }
    #[doc = "Checks if the value of the field is `P7IFG3`"]
    #[inline(always)]
    pub fn is_p7ifg3(&self) -> bool {
        *self == P7IV_A::P7IFG3
    }
    #[doc = "Checks if the value of the field is `P7IFG4`"]
    #[inline(always)]
    pub fn is_p7ifg4(&self) -> bool {
        *self == P7IV_A::P7IFG4
    }
    #[doc = "Checks if the value of the field is `P7IFG5`"]
    #[inline(always)]
    pub fn is_p7ifg5(&self) -> bool {
        *self == P7IV_A::P7IFG5
    }
    #[doc = "Checks if the value of the field is `P7IFG6`"]
    #[inline(always)]
    pub fn is_p7ifg6(&self) -> bool {
        *self == P7IV_A::P7IFG6
    }
    #[doc = "Checks if the value of the field is `P7IFG7`"]
    #[inline(always)]
    pub fn is_p7ifg7(&self) -> bool {
        *self == P7IV_A::P7IFG7
    }
}
#[doc = "Write proxy for field `P7IV`"]
pub struct P7IV_W<'a> {
    w: &'a mut W,
}
impl<'a> P7IV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P7IV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(P7IV_A::NONE)
    }
    #[doc = "Interrupt Source: Port 7.0 interrupt; Interrupt Flag: P7IFG0; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn p7ifg0(self) -> &'a mut W {
        self.variant(P7IV_A::P7IFG0)
    }
    #[doc = "Interrupt Source: Port 7.1 interrupt; Interrupt Flag: P7IFG1"]
    #[inline(always)]
    pub fn p7ifg1(self) -> &'a mut W {
        self.variant(P7IV_A::P7IFG1)
    }
    #[doc = "Interrupt Source: Port 7.2 interrupt; Interrupt Flag: P7IFG2"]
    #[inline(always)]
    pub fn p7ifg2(self) -> &'a mut W {
        self.variant(P7IV_A::P7IFG2)
    }
    #[doc = "Interrupt Source: Port 7.3 interrupt; Interrupt Flag: P7IFG3"]
    #[inline(always)]
    pub fn p7ifg3(self) -> &'a mut W {
        self.variant(P7IV_A::P7IFG3)
    }
    #[doc = "Interrupt Source: Port 7.4 interrupt; Interrupt Flag: P7IFG4"]
    #[inline(always)]
    pub fn p7ifg4(self) -> &'a mut W {
        self.variant(P7IV_A::P7IFG4)
    }
    #[doc = "Interrupt Source: Port 7.5 interrupt; Interrupt Flag: P7IFG5"]
    #[inline(always)]
    pub fn p7ifg5(self) -> &'a mut W {
        self.variant(P7IV_A::P7IFG5)
    }
    #[doc = "Interrupt Source: Port 7.6 interrupt; Interrupt Flag: P7IFG6"]
    #[inline(always)]
    pub fn p7ifg6(self) -> &'a mut W {
        self.variant(P7IV_A::P7IFG6)
    }
    #[doc = "Interrupt Source: Port 7.7 interrupt; Interrupt Flag: P7IFG7; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn p7ifg7(self) -> &'a mut W {
        self.variant(P7IV_A::P7IFG7)
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
Port 7 interrupt vector value"]
    #[inline(always)]
    pub fn p7iv(&self) -> P7IV_R {
        P7IV_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Port 7 interrupt vector value"]
    #[inline(always)]
    pub fn p7iv(&mut self) -> P7IV_W {
        P7IV_W { w: self }
    }
}
