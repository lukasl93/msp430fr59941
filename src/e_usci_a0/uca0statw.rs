#[doc = "Reader of register UCA0STATW"]
pub type R = crate::R<u16, super::UCA0STATW>;
#[doc = "Writer for register UCA0STATW"]
pub type W = crate::W<u16, super::UCA0STATW>;
#[doc = "Register UCA0STATW `reset()`'s with value 0"]
impl crate::ResetValue for super::UCA0STATW {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "0:0\\]
eUSCI_A busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCBUSY_A {
    #[doc = "0: eUSCI_A inactive"]
    IDLE = 0,
    #[doc = "1: eUSCI_A transmitting or receiving"]
    BUSY = 1,
}
impl From<UCBUSY_A> for bool {
    #[inline(always)]
    fn from(variant: UCBUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UCBUSY`"]
pub type UCBUSY_R = crate::R<bool, UCBUSY_A>;
impl UCBUSY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCBUSY_A {
        match self.bits {
            false => UCBUSY_A::IDLE,
            true => UCBUSY_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == UCBUSY_A::IDLE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == UCBUSY_A::BUSY
    }
}
#[doc = "Write proxy for field `UCBUSY`"]
pub struct UCBUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> UCBUSY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCBUSY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "eUSCI_A inactive"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut W {
        self.variant(UCBUSY_A::IDLE)
    }
    #[doc = "eUSCI_A transmitting or receiving"]
    #[inline(always)]
    pub fn busy(self) -> &'a mut W {
        self.variant(UCBUSY_A::BUSY)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `UCADDR_UCIDLE`"]
pub type UCADDR_UCIDLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCADDR_UCIDLE`"]
pub struct UCADDR_UCIDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCADDR_UCIDLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "2:2\\]
Receive error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCRXERR_A {
    #[doc = "0: No receive errors detected"]
    UCRXERR_0 = 0,
    #[doc = "1: Receive error detected"]
    UCRXERR_1 = 1,
}
impl From<UCRXERR_A> for bool {
    #[inline(always)]
    fn from(variant: UCRXERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UCRXERR`"]
pub type UCRXERR_R = crate::R<bool, UCRXERR_A>;
impl UCRXERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCRXERR_A {
        match self.bits {
            false => UCRXERR_A::UCRXERR_0,
            true => UCRXERR_A::UCRXERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCRXERR_0`"]
    #[inline(always)]
    pub fn is_ucrxerr_0(&self) -> bool {
        *self == UCRXERR_A::UCRXERR_0
    }
    #[doc = "Checks if the value of the field is `UCRXERR_1`"]
    #[inline(always)]
    pub fn is_ucrxerr_1(&self) -> bool {
        *self == UCRXERR_A::UCRXERR_1
    }
}
#[doc = "Write proxy for field `UCRXERR`"]
pub struct UCRXERR_W<'a> {
    w: &'a mut W,
}
impl<'a> UCRXERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCRXERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No receive errors detected"]
    #[inline(always)]
    pub fn ucrxerr_0(self) -> &'a mut W {
        self.variant(UCRXERR_A::UCRXERR_0)
    }
    #[doc = "Receive error detected"]
    #[inline(always)]
    pub fn ucrxerr_1(self) -> &'a mut W {
        self.variant(UCRXERR_A::UCRXERR_1)
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
Break detect flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCBRK_A {
    #[doc = "0: No break condition"]
    UCBRK_0 = 0,
    #[doc = "1: Break condition occurred"]
    UCBRK_1 = 1,
}
impl From<UCBRK_A> for bool {
    #[inline(always)]
    fn from(variant: UCBRK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UCBRK`"]
pub type UCBRK_R = crate::R<bool, UCBRK_A>;
impl UCBRK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCBRK_A {
        match self.bits {
            false => UCBRK_A::UCBRK_0,
            true => UCBRK_A::UCBRK_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCBRK_0`"]
    #[inline(always)]
    pub fn is_ucbrk_0(&self) -> bool {
        *self == UCBRK_A::UCBRK_0
    }
    #[doc = "Checks if the value of the field is `UCBRK_1`"]
    #[inline(always)]
    pub fn is_ucbrk_1(&self) -> bool {
        *self == UCBRK_A::UCBRK_1
    }
}
#[doc = "Write proxy for field `UCBRK`"]
pub struct UCBRK_W<'a> {
    w: &'a mut W,
}
impl<'a> UCBRK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCBRK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No break condition"]
    #[inline(always)]
    pub fn ucbrk_0(self) -> &'a mut W {
        self.variant(UCBRK_A::UCBRK_0)
    }
    #[doc = "Break condition occurred"]
    #[inline(always)]
    pub fn ucbrk_1(self) -> &'a mut W {
        self.variant(UCBRK_A::UCBRK_1)
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
Parity error flag. When UCPEN = 0, UCPE is read as 0. UCPE is cleared when UCAxRXBUF is read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCPE_A {
    #[doc = "0: No error"]
    UCPE_0 = 0,
    #[doc = "1: Character received with parity error"]
    UCPE_1 = 1,
}
impl From<UCPE_A> for bool {
    #[inline(always)]
    fn from(variant: UCPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UCPE`"]
pub type UCPE_R = crate::R<bool, UCPE_A>;
impl UCPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCPE_A {
        match self.bits {
            false => UCPE_A::UCPE_0,
            true => UCPE_A::UCPE_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCPE_0`"]
    #[inline(always)]
    pub fn is_ucpe_0(&self) -> bool {
        *self == UCPE_A::UCPE_0
    }
    #[doc = "Checks if the value of the field is `UCPE_1`"]
    #[inline(always)]
    pub fn is_ucpe_1(&self) -> bool {
        *self == UCPE_A::UCPE_1
    }
}
#[doc = "Write proxy for field `UCPE`"]
pub struct UCPE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCPE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn ucpe_0(self) -> &'a mut W {
        self.variant(UCPE_A::UCPE_0)
    }
    #[doc = "Character received with parity error"]
    #[inline(always)]
    pub fn ucpe_1(self) -> &'a mut W {
        self.variant(UCPE_A::UCPE_1)
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
#[doc = "5:5\\]
Overrun error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCOE_A {
    #[doc = "0: No error"]
    UCOE_0 = 0,
    #[doc = "1: Overrun error occurred"]
    UCOE_1 = 1,
}
impl From<UCOE_A> for bool {
    #[inline(always)]
    fn from(variant: UCOE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UCOE`"]
pub type UCOE_R = crate::R<bool, UCOE_A>;
impl UCOE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCOE_A {
        match self.bits {
            false => UCOE_A::UCOE_0,
            true => UCOE_A::UCOE_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCOE_0`"]
    #[inline(always)]
    pub fn is_ucoe_0(&self) -> bool {
        *self == UCOE_A::UCOE_0
    }
    #[doc = "Checks if the value of the field is `UCOE_1`"]
    #[inline(always)]
    pub fn is_ucoe_1(&self) -> bool {
        *self == UCOE_A::UCOE_1
    }
}
#[doc = "Write proxy for field `UCOE`"]
pub struct UCOE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCOE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCOE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn ucoe_0(self) -> &'a mut W {
        self.variant(UCOE_A::UCOE_0)
    }
    #[doc = "Overrun error occurred"]
    #[inline(always)]
    pub fn ucoe_1(self) -> &'a mut W {
        self.variant(UCOE_A::UCOE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u16) & 0x01) << 5);
        self.w
    }
}
#[doc = "6:6\\]
Framing error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCFE_A {
    #[doc = "0: No error"]
    UCFE_0 = 0,
    #[doc = "1: Character received with low stop bit"]
    UCFE_1 = 1,
}
impl From<UCFE_A> for bool {
    #[inline(always)]
    fn from(variant: UCFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UCFE`"]
pub type UCFE_R = crate::R<bool, UCFE_A>;
impl UCFE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCFE_A {
        match self.bits {
            false => UCFE_A::UCFE_0,
            true => UCFE_A::UCFE_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCFE_0`"]
    #[inline(always)]
    pub fn is_ucfe_0(&self) -> bool {
        *self == UCFE_A::UCFE_0
    }
    #[doc = "Checks if the value of the field is `UCFE_1`"]
    #[inline(always)]
    pub fn is_ucfe_1(&self) -> bool {
        *self == UCFE_A::UCFE_1
    }
}
#[doc = "Write proxy for field `UCFE`"]
pub struct UCFE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCFE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCFE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn ucfe_0(self) -> &'a mut W {
        self.variant(UCFE_A::UCFE_0)
    }
    #[doc = "Character received with low stop bit"]
    #[inline(always)]
    pub fn ucfe_1(self) -> &'a mut W {
        self.variant(UCFE_A::UCFE_1)
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
#[doc = "7:7\\]
Listen enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCLISTEN_A {
    #[doc = "0: Disabled"]
    UCLISTEN_0 = 0,
    #[doc = "1: Enabled. UCAxTXD is internally fed back to the receiver"]
    UCLISTEN_1 = 1,
}
impl From<UCLISTEN_A> for bool {
    #[inline(always)]
    fn from(variant: UCLISTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UCLISTEN`"]
pub type UCLISTEN_R = crate::R<bool, UCLISTEN_A>;
impl UCLISTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCLISTEN_A {
        match self.bits {
            false => UCLISTEN_A::UCLISTEN_0,
            true => UCLISTEN_A::UCLISTEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCLISTEN_0`"]
    #[inline(always)]
    pub fn is_uclisten_0(&self) -> bool {
        *self == UCLISTEN_A::UCLISTEN_0
    }
    #[doc = "Checks if the value of the field is `UCLISTEN_1`"]
    #[inline(always)]
    pub fn is_uclisten_1(&self) -> bool {
        *self == UCLISTEN_A::UCLISTEN_1
    }
}
#[doc = "Write proxy for field `UCLISTEN`"]
pub struct UCLISTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UCLISTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCLISTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn uclisten_0(self) -> &'a mut W {
        self.variant(UCLISTEN_A::UCLISTEN_0)
    }
    #[doc = "Enabled. UCAxTXD is internally fed back to the receiver"]
    #[inline(always)]
    pub fn uclisten_1(self) -> &'a mut W {
        self.variant(UCLISTEN_A::UCLISTEN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 0:0\\]
eUSCI_A busy"]
    #[inline(always)]
    pub fn ucbusy(&self) -> UCBUSY_R {
        UCBUSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Address received / Idle line detected"]
    #[inline(always)]
    pub fn ucaddr_ucidle(&self) -> UCADDR_UCIDLE_R {
        UCADDR_UCIDLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Receive error flag"]
    #[inline(always)]
    pub fn ucrxerr(&self) -> UCRXERR_R {
        UCRXERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Break detect flag"]
    #[inline(always)]
    pub fn ucbrk(&self) -> UCBRK_R {
        UCBRK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Parity error flag. When UCPEN = 0, UCPE is read as 0. UCPE is cleared when UCAxRXBUF is read."]
    #[inline(always)]
    pub fn ucpe(&self) -> UCPE_R {
        UCPE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Overrun error flag"]
    #[inline(always)]
    pub fn ucoe(&self) -> UCOE_R {
        UCOE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Framing error flag"]
    #[inline(always)]
    pub fn ucfe(&self) -> UCFE_R {
        UCFE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Listen enable"]
    #[inline(always)]
    pub fn uclisten(&self) -> UCLISTEN_R {
        UCLISTEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
eUSCI_A busy"]
    #[inline(always)]
    pub fn ucbusy(&mut self) -> UCBUSY_W {
        UCBUSY_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Address received / Idle line detected"]
    #[inline(always)]
    pub fn ucaddr_ucidle(&mut self) -> UCADDR_UCIDLE_W {
        UCADDR_UCIDLE_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Receive error flag"]
    #[inline(always)]
    pub fn ucrxerr(&mut self) -> UCRXERR_W {
        UCRXERR_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Break detect flag"]
    #[inline(always)]
    pub fn ucbrk(&mut self) -> UCBRK_W {
        UCBRK_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Parity error flag. When UCPEN = 0, UCPE is read as 0. UCPE is cleared when UCAxRXBUF is read."]
    #[inline(always)]
    pub fn ucpe(&mut self) -> UCPE_W {
        UCPE_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
Overrun error flag"]
    #[inline(always)]
    pub fn ucoe(&mut self) -> UCOE_W {
        UCOE_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
Framing error flag"]
    #[inline(always)]
    pub fn ucfe(&mut self) -> UCFE_W {
        UCFE_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
Listen enable"]
    #[inline(always)]
    pub fn uclisten(&mut self) -> UCLISTEN_W {
        UCLISTEN_W { w: self }
    }
}
