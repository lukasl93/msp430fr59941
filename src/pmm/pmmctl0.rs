#[doc = "Reader of register PMMCTL0"]
pub type R = crate::R<u16, super::PMMCTL0>;
#[doc = "Writer for register PMMCTL0"]
pub type W = crate::W<u16, super::PMMCTL0>;
#[doc = "Register PMMCTL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::PMMCTL0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "2:2\\]
Software brownout reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMMSWBOR_A {
    #[doc = "0: Normal operation"]
    PMMSWBOR_0 = 0,
    #[doc = "1: Set to 1 to trigger a BOR"]
    PMMSWBOR_1 = 1,
}
impl From<PMMSWBOR_A> for bool {
    #[inline(always)]
    fn from(variant: PMMSWBOR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PMMSWBOR`"]
pub type PMMSWBOR_R = crate::R<bool, PMMSWBOR_A>;
impl PMMSWBOR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMMSWBOR_A {
        match self.bits {
            false => PMMSWBOR_A::PMMSWBOR_0,
            true => PMMSWBOR_A::PMMSWBOR_1,
        }
    }
    #[doc = "Checks if the value of the field is `PMMSWBOR_0`"]
    #[inline(always)]
    pub fn is_pmmswbor_0(&self) -> bool {
        *self == PMMSWBOR_A::PMMSWBOR_0
    }
    #[doc = "Checks if the value of the field is `PMMSWBOR_1`"]
    #[inline(always)]
    pub fn is_pmmswbor_1(&self) -> bool {
        *self == PMMSWBOR_A::PMMSWBOR_1
    }
}
#[doc = "Write proxy for field `PMMSWBOR`"]
pub struct PMMSWBOR_W<'a> {
    w: &'a mut W,
}
impl<'a> PMMSWBOR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PMMSWBOR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn pmmswbor_0(self) -> &'a mut W {
        self.variant(PMMSWBOR_A::PMMSWBOR_0)
    }
    #[doc = "Set to 1 to trigger a BOR"]
    #[inline(always)]
    pub fn pmmswbor_1(self) -> &'a mut W {
        self.variant(PMMSWBOR_A::PMMSWBOR_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "3:3\\]
Software POR.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMMSWPOR_A {
    #[doc = "0: Normal operation"]
    PMMSWPOR_0 = 0,
    #[doc = "1: Set to 1 to trigger a POR"]
    PMMSWPOR_1 = 1,
}
impl From<PMMSWPOR_A> for bool {
    #[inline(always)]
    fn from(variant: PMMSWPOR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PMMSWPOR`"]
pub type PMMSWPOR_R = crate::R<bool, PMMSWPOR_A>;
impl PMMSWPOR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMMSWPOR_A {
        match self.bits {
            false => PMMSWPOR_A::PMMSWPOR_0,
            true => PMMSWPOR_A::PMMSWPOR_1,
        }
    }
    #[doc = "Checks if the value of the field is `PMMSWPOR_0`"]
    #[inline(always)]
    pub fn is_pmmswpor_0(&self) -> bool {
        *self == PMMSWPOR_A::PMMSWPOR_0
    }
    #[doc = "Checks if the value of the field is `PMMSWPOR_1`"]
    #[inline(always)]
    pub fn is_pmmswpor_1(&self) -> bool {
        *self == PMMSWPOR_A::PMMSWPOR_1
    }
}
#[doc = "Write proxy for field `PMMSWPOR`"]
pub struct PMMSWPOR_W<'a> {
    w: &'a mut W,
}
impl<'a> PMMSWPOR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PMMSWPOR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn pmmswpor_0(self) -> &'a mut W {
        self.variant(PMMSWPOR_A::PMMSWPOR_0)
    }
    #[doc = "Set to 1 to trigger a POR"]
    #[inline(always)]
    pub fn pmmswpor_1(self) -> &'a mut W {
        self.variant(PMMSWPOR_A::PMMSWPOR_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "4:4\\]
Regulator off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMMREGOFF_A {
    #[doc = "0: Regulator remains on when going into LPM3 or LPM4"]
    PMMREGOFF_0 = 0,
    #[doc = "1: Regulator is turned off when going to LPM3 or LPM4. System enters LPM3.5 or LPM4.5, respectively."]
    PMMREGOFF_1 = 1,
}
impl From<PMMREGOFF_A> for bool {
    #[inline(always)]
    fn from(variant: PMMREGOFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PMMREGOFF`"]
pub type PMMREGOFF_R = crate::R<bool, PMMREGOFF_A>;
impl PMMREGOFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMMREGOFF_A {
        match self.bits {
            false => PMMREGOFF_A::PMMREGOFF_0,
            true => PMMREGOFF_A::PMMREGOFF_1,
        }
    }
    #[doc = "Checks if the value of the field is `PMMREGOFF_0`"]
    #[inline(always)]
    pub fn is_pmmregoff_0(&self) -> bool {
        *self == PMMREGOFF_A::PMMREGOFF_0
    }
    #[doc = "Checks if the value of the field is `PMMREGOFF_1`"]
    #[inline(always)]
    pub fn is_pmmregoff_1(&self) -> bool {
        *self == PMMREGOFF_A::PMMREGOFF_1
    }
}
#[doc = "Write proxy for field `PMMREGOFF`"]
pub struct PMMREGOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> PMMREGOFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PMMREGOFF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Regulator remains on when going into LPM3 or LPM4"]
    #[inline(always)]
    pub fn pmmregoff_0(self) -> &'a mut W {
        self.variant(PMMREGOFF_A::PMMREGOFF_0)
    }
    #[doc = "Regulator is turned off when going to LPM3 or LPM4. System enters LPM3.5 or LPM4.5, respectively."]
    #[inline(always)]
    pub fn pmmregoff_1(self) -> &'a mut W {
        self.variant(PMMREGOFF_A::PMMREGOFF_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "6:6\\]
High-side SVS enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVSHE_A {
    #[doc = "0: High-side SVS (SVSH) is disabled in LPM2, LPM3, LPM4, LPM3.5, and LPM4.5. SVSH is always enabled in active mode, LPM0, and LPM1."]
    SVSHE_0 = 0,
    #[doc = "1: SVSH is always enabled."]
    SVSHE_1 = 1,
}
impl From<SVSHE_A> for bool {
    #[inline(always)]
    fn from(variant: SVSHE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SVSHE`"]
pub type SVSHE_R = crate::R<bool, SVSHE_A>;
impl SVSHE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SVSHE_A {
        match self.bits {
            false => SVSHE_A::SVSHE_0,
            true => SVSHE_A::SVSHE_1,
        }
    }
    #[doc = "Checks if the value of the field is `SVSHE_0`"]
    #[inline(always)]
    pub fn is_svshe_0(&self) -> bool {
        *self == SVSHE_A::SVSHE_0
    }
    #[doc = "Checks if the value of the field is `SVSHE_1`"]
    #[inline(always)]
    pub fn is_svshe_1(&self) -> bool {
        *self == SVSHE_A::SVSHE_1
    }
}
#[doc = "Write proxy for field `SVSHE`"]
pub struct SVSHE_W<'a> {
    w: &'a mut W,
}
impl<'a> SVSHE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SVSHE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "High-side SVS (SVSH) is disabled in LPM2, LPM3, LPM4, LPM3.5, and LPM4.5. SVSH is always enabled in active mode, LPM0, and LPM1."]
    #[inline(always)]
    pub fn svshe_0(self) -> &'a mut W {
        self.variant(SVSHE_A::SVSHE_0)
    }
    #[doc = "SVSH is always enabled."]
    #[inline(always)]
    pub fn svshe_1(self) -> &'a mut W {
        self.variant(SVSHE_A::SVSHE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u16) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `PMMPW`"]
pub type PMMPW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PMMPW`"]
pub struct PMMPW_W<'a> {
    w: &'a mut W,
}
impl<'a> PMMPW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - 2:2\\]
Software brownout reset."]
    #[inline(always)]
    pub fn pmmswbor(&self) -> PMMSWBOR_R {
        PMMSWBOR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Software POR."]
    #[inline(always)]
    pub fn pmmswpor(&self) -> PMMSWPOR_R {
        PMMSWPOR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Regulator off"]
    #[inline(always)]
    pub fn pmmregoff(&self) -> PMMREGOFF_R {
        PMMREGOFF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
High-side SVS enable."]
    #[inline(always)]
    pub fn svshe(&self) -> SVSHE_R {
        SVSHE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
PMM password."]
    #[inline(always)]
    pub fn pmmpw(&self) -> PMMPW_R {
        PMMPW_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - 2:2\\]
Software brownout reset."]
    #[inline(always)]
    pub fn pmmswbor(&mut self) -> PMMSWBOR_W {
        PMMSWBOR_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Software POR."]
    #[inline(always)]
    pub fn pmmswpor(&mut self) -> PMMSWPOR_W {
        PMMSWPOR_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Regulator off"]
    #[inline(always)]
    pub fn pmmregoff(&mut self) -> PMMREGOFF_W {
        PMMREGOFF_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
High-side SVS enable."]
    #[inline(always)]
    pub fn svshe(&mut self) -> SVSHE_W {
        SVSHE_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
PMM password."]
    #[inline(always)]
    pub fn pmmpw(&mut self) -> PMMPW_W {
        PMMPW_W { w: self }
    }
}
