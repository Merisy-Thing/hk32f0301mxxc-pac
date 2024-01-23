#[doc = "Register `RTSR` reader"]
pub type R = crate::R<RTSRrs>;
#[doc = "Register `RTSR` writer"]
pub type W = crate::W<RTSRrs>;
#[doc = "Field `RT0` reader - Rising trigger event configuration bit of line x"]
pub type RT0_R = crate::BitReader;
#[doc = "Field `RT0` writer - Rising trigger event configuration bit of line x"]
pub type RT0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT1` reader - Rising trigger event configuration bit of line x"]
pub type RT1_R = crate::BitReader;
#[doc = "Field `RT1` writer - Rising trigger event configuration bit of line x"]
pub type RT1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT2` reader - Rising trigger event configuration bit of line x"]
pub type RT2_R = crate::BitReader;
#[doc = "Field `RT2` writer - Rising trigger event configuration bit of line x"]
pub type RT2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT3` reader - Rising trigger event configuration bit of line x"]
pub type RT3_R = crate::BitReader;
#[doc = "Field `RT3` writer - Rising trigger event configuration bit of line x"]
pub type RT3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT4` reader - Rising trigger event configuration bit of line x"]
pub type RT4_R = crate::BitReader;
#[doc = "Field `RT4` writer - Rising trigger event configuration bit of line x"]
pub type RT4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT5` reader - Rising trigger event configuration bit of line x"]
pub type RT5_R = crate::BitReader;
#[doc = "Field `RT5` writer - Rising trigger event configuration bit of line x"]
pub type RT5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT6` reader - Rising trigger event configuration bit of line x"]
pub type RT6_R = crate::BitReader;
#[doc = "Field `RT6` writer - Rising trigger event configuration bit of line x"]
pub type RT6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT7` reader - Rising trigger event configuration bit of line x"]
pub type RT7_R = crate::BitReader;
#[doc = "Field `RT7` writer - Rising trigger event configuration bit of line x"]
pub type RT7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT11` reader - Rising trigger event configuration bit of line 11"]
pub type RT11_R = crate::BitReader;
#[doc = "Field `RT11` writer - Rising trigger event configuration bit of line 11"]
pub type RT11_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Rising trigger event configuration bit of line x"]
    #[inline(always)]
    pub fn rt0(&self) -> RT0_R {
        RT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rising trigger event configuration bit of line x"]
    #[inline(always)]
    pub fn rt1(&self) -> RT1_R {
        RT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rising trigger event configuration bit of line x"]
    #[inline(always)]
    pub fn rt2(&self) -> RT2_R {
        RT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rising trigger event configuration bit of line x"]
    #[inline(always)]
    pub fn rt3(&self) -> RT3_R {
        RT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rising trigger event configuration bit of line x"]
    #[inline(always)]
    pub fn rt4(&self) -> RT4_R {
        RT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rising trigger event configuration bit of line x"]
    #[inline(always)]
    pub fn rt5(&self) -> RT5_R {
        RT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rising trigger event configuration bit of line x"]
    #[inline(always)]
    pub fn rt6(&self) -> RT6_R {
        RT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rising trigger event configuration bit of line x"]
    #[inline(always)]
    pub fn rt7(&self) -> RT7_R {
        RT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 11 - Rising trigger event configuration bit of line 11"]
    #[inline(always)]
    pub fn rt11(&self) -> RT11_R {
        RT11_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rising trigger event configuration bit of line x"]
    #[inline(always)]
    #[must_use]
    pub fn rt0(&mut self) -> RT0_W<RTSRrs> {
        RT0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Rising trigger event configuration bit of line x"]
    #[inline(always)]
    #[must_use]
    pub fn rt1(&mut self) -> RT1_W<RTSRrs> {
        RT1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Rising trigger event configuration bit of line x"]
    #[inline(always)]
    #[must_use]
    pub fn rt2(&mut self) -> RT2_W<RTSRrs> {
        RT2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Rising trigger event configuration bit of line x"]
    #[inline(always)]
    #[must_use]
    pub fn rt3(&mut self) -> RT3_W<RTSRrs> {
        RT3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Rising trigger event configuration bit of line x"]
    #[inline(always)]
    #[must_use]
    pub fn rt4(&mut self) -> RT4_W<RTSRrs> {
        RT4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Rising trigger event configuration bit of line x"]
    #[inline(always)]
    #[must_use]
    pub fn rt5(&mut self) -> RT5_W<RTSRrs> {
        RT5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Rising trigger event configuration bit of line x"]
    #[inline(always)]
    #[must_use]
    pub fn rt6(&mut self) -> RT6_W<RTSRrs> {
        RT6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Rising trigger event configuration bit of line x"]
    #[inline(always)]
    #[must_use]
    pub fn rt7(&mut self) -> RT7_W<RTSRrs> {
        RT7_W::new(self, 7)
    }
    #[doc = "Bit 11 - Rising trigger event configuration bit of line 11"]
    #[inline(always)]
    #[must_use]
    pub fn rt11(&mut self) -> RT11_W<RTSRrs> {
        RT11_W::new(self, 11)
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
#[doc = "RTSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTSRrs;
impl crate::RegisterSpec for RTSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtsr::R`](R) reader structure"]
impl crate::Readable for RTSRrs {}
#[doc = "`write(|w| ..)` method takes [`rtsr::W`](W) writer structure"]
impl crate::Writable for RTSRrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTSR to value 0"]
impl crate::Resettable for RTSRrs {
    const RESET_VALUE: u32 = 0;
}
