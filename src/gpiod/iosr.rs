#[doc = "Register `IOSR` reader"]
pub type R = crate::R<IOSRrs>;
#[doc = "Register `IOSR` writer"]
pub type W = crate::W<IOSRrs>;
#[doc = "Field `SEN0` reader - Port x pin y Schmitt switch"]
pub type SEN0_R = crate::BitReader;
#[doc = "Field `SEN0` writer - Port x pin y Schmitt switch"]
pub type SEN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEN1` reader - Port x pin y Schmitt switch"]
pub type SEN1_R = crate::BitReader;
#[doc = "Field `SEN1` writer - Port x pin y Schmitt switch"]
pub type SEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEN2` reader - Port x pin y Schmitt switch"]
pub type SEN2_R = crate::BitReader;
#[doc = "Field `SEN2` writer - Port x pin y Schmitt switch"]
pub type SEN2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEN3` reader - Port x pin y Schmitt switch"]
pub type SEN3_R = crate::BitReader;
#[doc = "Field `SEN3` writer - Port x pin y Schmitt switch"]
pub type SEN3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEN4` reader - Port x pin y Schmitt switch"]
pub type SEN4_R = crate::BitReader;
#[doc = "Field `SEN4` writer - Port x pin y Schmitt switch"]
pub type SEN4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEN5` reader - Port x pin y Schmitt switch"]
pub type SEN5_R = crate::BitReader;
#[doc = "Field `SEN5` writer - Port x pin y Schmitt switch"]
pub type SEN5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEN6` reader - Port x pin y Schmitt switch"]
pub type SEN6_R = crate::BitReader;
#[doc = "Field `SEN6` writer - Port x pin y Schmitt switch"]
pub type SEN6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEN7` reader - Port x pin y Schmitt switch"]
pub type SEN7_R = crate::BitReader;
#[doc = "Field `SEN7` writer - Port x pin y Schmitt switch"]
pub type SEN7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Port x pin y Schmitt switch"]
    #[inline(always)]
    pub fn sen0(&self) -> SEN0_R {
        SEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port x pin y Schmitt switch"]
    #[inline(always)]
    pub fn sen1(&self) -> SEN1_R {
        SEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port x pin y Schmitt switch"]
    #[inline(always)]
    pub fn sen2(&self) -> SEN2_R {
        SEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port x pin y Schmitt switch"]
    #[inline(always)]
    pub fn sen3(&self) -> SEN3_R {
        SEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port x pin y Schmitt switch"]
    #[inline(always)]
    pub fn sen4(&self) -> SEN4_R {
        SEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port x pin y Schmitt switch"]
    #[inline(always)]
    pub fn sen5(&self) -> SEN5_R {
        SEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port x pin y Schmitt switch"]
    #[inline(always)]
    pub fn sen6(&self) -> SEN6_R {
        SEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port x pin y Schmitt switch"]
    #[inline(always)]
    pub fn sen7(&self) -> SEN7_R {
        SEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port x pin y Schmitt switch"]
    #[inline(always)]
    #[must_use]
    pub fn sen0(&mut self) -> SEN0_W<IOSRrs> {
        SEN0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Port x pin y Schmitt switch"]
    #[inline(always)]
    #[must_use]
    pub fn sen1(&mut self) -> SEN1_W<IOSRrs> {
        SEN1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Port x pin y Schmitt switch"]
    #[inline(always)]
    #[must_use]
    pub fn sen2(&mut self) -> SEN2_W<IOSRrs> {
        SEN2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Port x pin y Schmitt switch"]
    #[inline(always)]
    #[must_use]
    pub fn sen3(&mut self) -> SEN3_W<IOSRrs> {
        SEN3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Port x pin y Schmitt switch"]
    #[inline(always)]
    #[must_use]
    pub fn sen4(&mut self) -> SEN4_W<IOSRrs> {
        SEN4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Port x pin y Schmitt switch"]
    #[inline(always)]
    #[must_use]
    pub fn sen5(&mut self) -> SEN5_W<IOSRrs> {
        SEN5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Port x pin y Schmitt switch"]
    #[inline(always)]
    #[must_use]
    pub fn sen6(&mut self) -> SEN6_W<IOSRrs> {
        SEN6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Port x pin y Schmitt switch"]
    #[inline(always)]
    #[must_use]
    pub fn sen7(&mut self) -> SEN7_W<IOSRrs> {
        SEN7_W::new(self, 7)
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
#[doc = "IOSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iosr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iosr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOSRrs;
impl crate::RegisterSpec for IOSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iosr::R`](R) reader structure"]
impl crate::Readable for IOSRrs {}
#[doc = "`write(|w| ..)` method takes [`iosr::W`](W) writer structure"]
impl crate::Writable for IOSRrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IOSR to value 0xffff"]
impl crate::Resettable for IOSRrs {
    const RESET_VALUE: u32 = 0xffff;
}
