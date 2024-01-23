#[doc = "Register `FTSR` reader"]
pub type R = crate::R<FTSRrs>;
#[doc = "Register `FTSR` writer"]
pub type W = crate::W<FTSRrs>;
#[doc = "Field `FT0` reader - Falling trigger event configuration bit of line x"]
pub type FT0_R = crate::BitReader;
#[doc = "Field `FT0` writer - Falling trigger event configuration bit of line x"]
pub type FT0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT1` reader - Falling trigger event configuration bit of line x"]
pub type FT1_R = crate::BitReader;
#[doc = "Field `FT1` writer - Falling trigger event configuration bit of line x"]
pub type FT1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT2` reader - Falling trigger event configuration bit of line x"]
pub type FT2_R = crate::BitReader;
#[doc = "Field `FT2` writer - Falling trigger event configuration bit of line x"]
pub type FT2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT3` reader - Falling trigger event configuration bit of line x"]
pub type FT3_R = crate::BitReader;
#[doc = "Field `FT3` writer - Falling trigger event configuration bit of line x"]
pub type FT3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT4` reader - Falling trigger event configuration bit of line x"]
pub type FT4_R = crate::BitReader;
#[doc = "Field `FT4` writer - Falling trigger event configuration bit of line x"]
pub type FT4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT5` reader - Falling trigger event configuration bit of line x"]
pub type FT5_R = crate::BitReader;
#[doc = "Field `FT5` writer - Falling trigger event configuration bit of line x"]
pub type FT5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT6` reader - Falling trigger event configuration bit of line x"]
pub type FT6_R = crate::BitReader;
#[doc = "Field `FT6` writer - Falling trigger event configuration bit of line x"]
pub type FT6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT7` reader - Falling trigger event configuration bit of line x"]
pub type FT7_R = crate::BitReader;
#[doc = "Field `FT7` writer - Falling trigger event configuration bit of line x"]
pub type FT7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT11` reader - Falling trigger event configuration bit of line 11"]
pub type FT11_R = crate::BitReader;
#[doc = "Field `FT11` writer - Falling trigger event configuration bit of line 11"]
pub type FT11_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Falling trigger event configuration bit of line x"]
    #[inline(always)]
    pub fn ft0(&self) -> FT0_R {
        FT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Falling trigger event configuration bit of line x"]
    #[inline(always)]
    pub fn ft1(&self) -> FT1_R {
        FT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Falling trigger event configuration bit of line x"]
    #[inline(always)]
    pub fn ft2(&self) -> FT2_R {
        FT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Falling trigger event configuration bit of line x"]
    #[inline(always)]
    pub fn ft3(&self) -> FT3_R {
        FT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Falling trigger event configuration bit of line x"]
    #[inline(always)]
    pub fn ft4(&self) -> FT4_R {
        FT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Falling trigger event configuration bit of line x"]
    #[inline(always)]
    pub fn ft5(&self) -> FT5_R {
        FT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Falling trigger event configuration bit of line x"]
    #[inline(always)]
    pub fn ft6(&self) -> FT6_R {
        FT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Falling trigger event configuration bit of line x"]
    #[inline(always)]
    pub fn ft7(&self) -> FT7_R {
        FT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 11 - Falling trigger event configuration bit of line 11"]
    #[inline(always)]
    pub fn ft11(&self) -> FT11_R {
        FT11_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Falling trigger event configuration bit of line x"]
    #[inline(always)]
    #[must_use]
    pub fn ft0(&mut self) -> FT0_W<FTSRrs> {
        FT0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Falling trigger event configuration bit of line x"]
    #[inline(always)]
    #[must_use]
    pub fn ft1(&mut self) -> FT1_W<FTSRrs> {
        FT1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Falling trigger event configuration bit of line x"]
    #[inline(always)]
    #[must_use]
    pub fn ft2(&mut self) -> FT2_W<FTSRrs> {
        FT2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Falling trigger event configuration bit of line x"]
    #[inline(always)]
    #[must_use]
    pub fn ft3(&mut self) -> FT3_W<FTSRrs> {
        FT3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Falling trigger event configuration bit of line x"]
    #[inline(always)]
    #[must_use]
    pub fn ft4(&mut self) -> FT4_W<FTSRrs> {
        FT4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Falling trigger event configuration bit of line x"]
    #[inline(always)]
    #[must_use]
    pub fn ft5(&mut self) -> FT5_W<FTSRrs> {
        FT5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Falling trigger event configuration bit of line x"]
    #[inline(always)]
    #[must_use]
    pub fn ft6(&mut self) -> FT6_W<FTSRrs> {
        FT6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Falling trigger event configuration bit of line x"]
    #[inline(always)]
    #[must_use]
    pub fn ft7(&mut self) -> FT7_W<FTSRrs> {
        FT7_W::new(self, 7)
    }
    #[doc = "Bit 11 - Falling trigger event configuration bit of line 11"]
    #[inline(always)]
    #[must_use]
    pub fn ft11(&mut self) -> FT11_W<FTSRrs> {
        FT11_W::new(self, 11)
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
#[doc = "FTSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ftsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ftsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FTSRrs;
impl crate::RegisterSpec for FTSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ftsr::R`](R) reader structure"]
impl crate::Readable for FTSRrs {}
#[doc = "`write(|w| ..)` method takes [`ftsr::W`](W) writer structure"]
impl crate::Writable for FTSRrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FTSR to value 0"]
impl crate::Resettable for FTSRrs {
    const RESET_VALUE: u32 = 0;
}
