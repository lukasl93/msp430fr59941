#[doc = "Reader of register TA1EX0"]
pub type R = crate::R<u16, super::TA1EX0>;
#[doc = "Writer for register TA1EX0"]
pub type W = crate::W<u16, super::TA1EX0>;
#[doc = "Register TA1EX0 `reset()`'s with value 0"]
impl crate::ResetValue for super::TA1EX0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "2:0\\]
Input divider expansion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TAIDEX_A {
    #[doc = "0: Divide by 1"]
    _1 = 0,
    #[doc = "1: Divide by 2"]
    _2 = 1,
    #[doc = "2: Divide by 3"]
    _3 = 2,
    #[doc = "3: Divide by 4"]
    _4 = 3,
    #[doc = "4: Divide by 5"]
    _5 = 4,
    #[doc = "5: Divide by 6"]
    _6 = 5,
    #[doc = "6: Divide by 7"]
    _7 = 6,
    #[doc = "7: Divide by 8"]
    _8 = 7,
}
impl From<TAIDEX_A> for u8 {
    #[inline(always)]
    fn from(variant: TAIDEX_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TAIDEX`"]
pub type TAIDEX_R = crate::R<u8, TAIDEX_A>;
impl TAIDEX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAIDEX_A {
        match self.bits {
            0 => TAIDEX_A::_1,
            1 => TAIDEX_A::_2,
            2 => TAIDEX_A::_3,
            3 => TAIDEX_A::_4,
            4 => TAIDEX_A::_5,
            5 => TAIDEX_A::_6,
            6 => TAIDEX_A::_7,
            7 => TAIDEX_A::_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TAIDEX_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == TAIDEX_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        *self == TAIDEX_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == TAIDEX_A::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        *self == TAIDEX_A::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        *self == TAIDEX_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        *self == TAIDEX_A::_7
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == TAIDEX_A::_8
    }
}
#[doc = "Write proxy for field `TAIDEX`"]
pub struct TAIDEX_W<'a> {
    w: &'a mut W,
}
impl<'a> TAIDEX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAIDEX_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TAIDEX_A::_1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(TAIDEX_A::_2)
    }
    #[doc = "Divide by 3"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(TAIDEX_A::_3)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(TAIDEX_A::_4)
    }
    #[doc = "Divide by 5"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(TAIDEX_A::_5)
    }
    #[doc = "Divide by 6"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(TAIDEX_A::_6)
    }
    #[doc = "Divide by 7"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(TAIDEX_A::_7)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(TAIDEX_A::_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u16) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Input divider expansion"]
    #[inline(always)]
    pub fn taidex(&self) -> TAIDEX_R {
        TAIDEX_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Input divider expansion"]
    #[inline(always)]
    pub fn taidex(&mut self) -> TAIDEX_W {
        TAIDEX_W { w: self }
    }
}
