#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STAR {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `TXNACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXNACKR {
    #[doc = "Write a Transmit ACK for each received word"]
    TXNACK_0,
    #[doc = "Write a Transmit NACK for each received word"]
    TXNACK_1,
}
impl TXNACKR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            TXNACKR::TXNACK_0 => false,
            TXNACKR::TXNACK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXNACKR {
        match value {
            false => TXNACKR::TXNACK_0,
            true => TXNACKR::TXNACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXNACK_0`"]
    #[inline]
    pub fn is_txnack_0(&self) -> bool {
        *self == TXNACKR::TXNACK_0
    }
    #[doc = "Checks if the value of the field is `TXNACK_1`"]
    #[inline]
    pub fn is_txnack_1(&self) -> bool {
        *self == TXNACKR::TXNACK_1
    }
}
#[doc = "Values that can be written to the field `TXNACK`"]
pub enum TXNACKW {
    #[doc = "Write a Transmit ACK for each received word"]
    TXNACK_0,
    #[doc = "Write a Transmit NACK for each received word"]
    TXNACK_1,
}
impl TXNACKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXNACKW::TXNACK_0 => false,
            TXNACKW::TXNACK_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXNACKW<'a> {
    w: &'a mut W,
}
impl<'a> _TXNACKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXNACKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write a Transmit ACK for each received word"]
    #[inline]
    pub fn txnack_0(self) -> &'a mut W {
        self.variant(TXNACKW::TXNACK_0)
    }
    #[doc = "Write a Transmit NACK for each received word"]
    #[inline]
    pub fn txnack_1(self) -> &'a mut W {
        self.variant(TXNACKW::TXNACK_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Transmit NACK"]
    #[inline]
    pub fn txnack(&self) -> TXNACKR {
        TXNACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Transmit NACK"]
    #[inline]
    pub fn txnack(&mut self) -> _TXNACKW {
        _TXNACKW { w: self }
    }
}
