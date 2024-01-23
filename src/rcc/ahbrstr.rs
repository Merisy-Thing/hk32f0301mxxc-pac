#[doc = "Register `AHBRSTR` reader"]
pub type R = crate::R<AHBRSTRrs>;
#[doc = "Register `AHBRSTR` writer"]
pub type W = crate::W<AHBRSTRrs>;
#[doc = "Field `CRCRST` reader - Reset CRC"]
pub type CRCRST_R = crate::BitReader;
#[doc = "Field `CRCRST` writer - Reset CRC"]
pub type CRCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOPARST` reader - Reset I/O port A"]
pub type IOPARST_R = crate::BitReader;
#[doc = "Field `IOPARST` writer - Reset I/O port A"]
pub type IOPARST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOPBRST` reader - Reset I/O port B"]
pub type IOPBRST_R = crate::BitReader;
#[doc = "Field `IOPBRST` writer - Reset I/O port B"]
pub type IOPBRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOPCRST` reader - Reset I/O port C"]
pub type IOPCRST_R = crate::BitReader;
#[doc = "Field `IOPCRST` writer - Reset I/O port C"]
pub type IOPCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOPDRST` reader - Reset I/O port D"]
pub type IOPDRST_R = crate::BitReader;
#[doc = "Field `IOPDRST` writer - Reset I/O port D"]
pub type IOPDRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 6 - Reset CRC"]
    #[inline(always)]
    pub fn crcrst(&self) -> CRCRST_R {
        CRCRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 17 - Reset I/O port A"]
    #[inline(always)]
    pub fn ioparst(&self) -> IOPARST_R {
        IOPARST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Reset I/O port B"]
    #[inline(always)]
    pub fn iopbrst(&self) -> IOPBRST_R {
        IOPBRST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Reset I/O port C"]
    #[inline(always)]
    pub fn iopcrst(&self) -> IOPCRST_R {
        IOPCRST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Reset I/O port D"]
    #[inline(always)]
    pub fn iopdrst(&self) -> IOPDRST_R {
        IOPDRST_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Reset CRC"]
    #[inline(always)]
    #[must_use]
    pub fn crcrst(&mut self) -> CRCRST_W<AHBRSTRrs> {
        CRCRST_W::new(self, 6)
    }
    #[doc = "Bit 17 - Reset I/O port A"]
    #[inline(always)]
    #[must_use]
    pub fn ioparst(&mut self) -> IOPARST_W<AHBRSTRrs> {
        IOPARST_W::new(self, 17)
    }
    #[doc = "Bit 18 - Reset I/O port B"]
    #[inline(always)]
    #[must_use]
    pub fn iopbrst(&mut self) -> IOPBRST_W<AHBRSTRrs> {
        IOPBRST_W::new(self, 18)
    }
    #[doc = "Bit 19 - Reset I/O port C"]
    #[inline(always)]
    #[must_use]
    pub fn iopcrst(&mut self) -> IOPCRST_W<AHBRSTRrs> {
        IOPCRST_W::new(self, 19)
    }
    #[doc = "Bit 20 - Reset I/O port D"]
    #[inline(always)]
    #[must_use]
    pub fn iopdrst(&mut self) -> IOPDRST_W<AHBRSTRrs> {
        IOPDRST_W::new(self, 20)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AHBRSTR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbrstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbrstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBRSTRrs;
impl crate::RegisterSpec for AHBRSTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbrstr::R`](R) reader structure"]
impl crate::Readable for AHBRSTRrs {}
#[doc = "`write(|w| ..)` method takes [`ahbrstr::W`](W) writer structure"]
impl crate::Writable for AHBRSTRrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHBRSTR to value 0"]
impl crate::Resettable for AHBRSTRrs {
    const RESET_VALUE: u32 = 0;
}
