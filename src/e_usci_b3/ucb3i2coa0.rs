#[doc = "Reader of register UCB3I2COA0"]
pub type R = crate::R<u16, super::UCB3I2COA0>;
#[doc = "Writer for register UCB3I2COA0"]
pub type W = crate::W<u16, super::UCB3I2COA0>;
#[doc = "Register UCB3I2COA0 `reset()`'s with value 0"]
impl crate::ResetValue for super::UCB3I2COA0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2COA0`"]
pub type I2COA0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `I2COA0`"]
pub struct I2COA0_W<'a> {
    w: &'a mut W,
}
impl<'a> I2COA0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u16) & 0x03ff);
        self.w
    }
}
#[doc = "10:10\\]
Own Address enable register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCOAEN_A {
    #[doc = "0: The slave address defined in I2COA0 is disabled"]
    DISABLE = 0,
    #[doc = "1: The slave address defined in I2COA0 is enabled"]
    ENABLE = 1,
}
impl From<UCOAEN_A> for bool {
    #[inline(always)]
    fn from(variant: UCOAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UCOAEN`"]
pub type UCOAEN_R = crate::R<bool, UCOAEN_A>;
impl UCOAEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCOAEN_A {
        match self.bits {
            false => UCOAEN_A::DISABLE,
            true => UCOAEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == UCOAEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == UCOAEN_A::ENABLE
    }
}
#[doc = "Write proxy for field `UCOAEN`"]
pub struct UCOAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UCOAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCOAEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The slave address defined in I2COA0 is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(UCOAEN_A::DISABLE)
    }
    #[doc = "The slave address defined in I2COA0 is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(UCOAEN_A::ENABLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u16) & 0x01) << 10);
        self.w
    }
}
#[doc = "15:15\\]
General call response enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCGCEN_A {
    #[doc = "0: Do not respond to a general call"]
    UCGCEN_0 = 0,
    #[doc = "1: Respond to a general call"]
    UCGCEN_1 = 1,
}
impl From<UCGCEN_A> for bool {
    #[inline(always)]
    fn from(variant: UCGCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UCGCEN`"]
pub type UCGCEN_R = crate::R<bool, UCGCEN_A>;
impl UCGCEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCGCEN_A {
        match self.bits {
            false => UCGCEN_A::UCGCEN_0,
            true => UCGCEN_A::UCGCEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCGCEN_0`"]
    #[inline(always)]
    pub fn is_ucgcen_0(&self) -> bool {
        *self == UCGCEN_A::UCGCEN_0
    }
    #[doc = "Checks if the value of the field is `UCGCEN_1`"]
    #[inline(always)]
    pub fn is_ucgcen_1(&self) -> bool {
        *self == UCGCEN_A::UCGCEN_1
    }
}
#[doc = "Write proxy for field `UCGCEN`"]
pub struct UCGCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UCGCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCGCEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do not respond to a general call"]
    #[inline(always)]
    pub fn ucgcen_0(self) -> &'a mut W {
        self.variant(UCGCEN_A::UCGCEN_0)
    }
    #[doc = "Respond to a general call"]
    #[inline(always)]
    pub fn ucgcen_1(self) -> &'a mut W {
        self.variant(UCGCEN_A::UCGCEN_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u16) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
I2C own address"]
    #[inline(always)]
    pub fn i2coa0(&self) -> I2COA0_R {
        I2COA0_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - 10:10\\]
Own Address enable register"]
    #[inline(always)]
    pub fn ucoaen(&self) -> UCOAEN_R {
        UCOAEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
General call response enable"]
    #[inline(always)]
    pub fn ucgcen(&self) -> UCGCEN_R {
        UCGCEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
I2C own address"]
    #[inline(always)]
    pub fn i2coa0(&mut self) -> I2COA0_W {
        I2COA0_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
Own Address enable register"]
    #[inline(always)]
    pub fn ucoaen(&mut self) -> UCOAEN_W {
        UCOAEN_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\]
General call response enable"]
    #[inline(always)]
    pub fn ucgcen(&mut self) -> UCGCEN_W {
        UCGCEN_W { w: self }
    }
}
