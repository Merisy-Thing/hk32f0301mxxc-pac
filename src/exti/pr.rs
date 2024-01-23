#[doc = "Register `PR` reader"]
pub type R = crate::R<PRrs>;
#[doc = "Register `PR` writer"]
pub type W = crate::W<PRrs>;
#[doc = "Field `PIF0` reader - Pending interrupt flag on line x"]
pub type PIF0_R = crate::BitReader;
#[doc = "Field `PIF0` writer - Pending interrupt flag on line x"]
pub type PIF0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIF1` reader - Pending interrupt flag on line x"]
pub type PIF1_R = crate::BitReader;
#[doc = "Field `PIF1` writer - Pending interrupt flag on line x"]
pub type PIF1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIF2` reader - Pending interrupt flag on line x"]
pub type PIF2_R = crate::BitReader;
#[doc = "Field `PIF2` writer - Pending interrupt flag on line x"]
pub type PIF2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIF3` reader - Pending interrupt flag on line x"]
pub type PIF3_R = crate::BitReader;
#[doc = "Field `PIF3` writer - Pending interrupt flag on line x"]
pub type PIF3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIF4` reader - Pending interrupt flag on line x"]
pub type PIF4_R = crate::BitReader;
#[doc = "Field `PIF4` writer - Pending interrupt flag on line x"]
pub type PIF4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIF5` reader - Pending interrupt flag on line x"]
pub type PIF5_R = crate::BitReader;
#[doc = "Field `PIF5` writer - Pending interrupt flag on line x"]
pub type PIF5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIF6` reader - Pending interrupt flag on line x"]
pub type PIF6_R = crate::BitReader;
#[doc = "Field `PIF6` writer - Pending interrupt flag on line x"]
pub type PIF6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIF7` reader - Pending interrupt flag on line x"]
pub type PIF7_R = crate::BitReader;
#[doc = "Field `PIF7` writer - Pending interrupt flag on line x"]
pub type PIF7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIF11` reader - Pending interrupt flag on line 11"]
pub type PIF11_R = crate::BitReader;
#[doc = "Field `PIF11` writer - Pending interrupt flag on line 11"]
pub type PIF11_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Pending interrupt flag on line x"]
    #[inline(always)]
    pub fn pif0(&self) -> PIF0_R {
        PIF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pending interrupt flag on line x"]
    #[inline(always)]
    pub fn pif1(&self) -> PIF1_R {
        PIF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pending interrupt flag on line x"]
    #[inline(always)]
    pub fn pif2(&self) -> PIF2_R {
        PIF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pending interrupt flag on line x"]
    #[inline(always)]
    pub fn pif3(&self) -> PIF3_R {
        PIF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pending interrupt flag on line x"]
    #[inline(always)]
    pub fn pif4(&self) -> PIF4_R {
        PIF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pending interrupt flag on line x"]
    #[inline(always)]
    pub fn pif5(&self) -> PIF5_R {
        PIF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pending interrupt flag on line x"]
    #[inline(always)]
    pub fn pif6(&self) -> PIF6_R {
        PIF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pending interrupt flag on line x"]
    #[inline(always)]
    pub fn pif7(&self) -> PIF7_R {
        PIF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 11 - Pending interrupt flag on line 11"]
    #[inline(always)]
    pub fn pif11(&self) -> PIF11_R {
        PIF11_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pending interrupt flag on line x"]
    #[inline(always)]
    #[must_use]
    pub fn pif0(&mut self) -> PIF0_W<PRrs> {
        PIF0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Pending interrupt flag on line x"]
    #[inline(always)]
    #[must_use]
    pub fn pif1(&mut self) -> PIF1_W<PRrs> {
        PIF1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Pending interrupt flag on line x"]
    #[inline(always)]
    #[must_use]
    pub fn pif2(&mut self) -> PIF2_W<PRrs> {
        PIF2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Pending interrupt flag on line x"]
    #[inline(always)]
    #[must_use]
    pub fn pif3(&mut self) -> PIF3_W<PRrs> {
        PIF3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Pending interrupt flag on line x"]
    #[inline(always)]
    #[must_use]
    pub fn pif4(&mut self) -> PIF4_W<PRrs> {
        PIF4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Pending interrupt flag on line x"]
    #[inline(always)]
    #[must_use]
    pub fn pif5(&mut self) -> PIF5_W<PRrs> {
        PIF5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Pending interrupt flag on line x"]
    #[inline(always)]
    #[must_use]
    pub fn pif6(&mut self) -> PIF6_W<PRrs> {
        PIF6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Pending interrupt flag on line x"]
    #[inline(always)]
    #[must_use]
    pub fn pif7(&mut self) -> PIF7_W<PRrs> {
        PIF7_W::new(self, 7)
    }
    #[doc = "Bit 11 - Pending interrupt flag on line 11"]
    #[inline(always)]
    #[must_use]
    pub fn pif11(&mut self) -> PIF11_W<PRrs> {
        PIF11_W::new(self, 11)
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
#[doc = "PR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRrs;
impl crate::RegisterSpec for PRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr::R`](R) reader structure"]
impl crate::Readable for PRrs {}
#[doc = "`write(|w| ..)` method takes [`pr::W`](W) writer structure"]
impl crate::Writable for PRrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR to value 0"]
impl crate::Resettable for PRrs {
    const RESET_VALUE: u32 = 0;
}
