#[doc = "Register `EMR` reader"]
pub type R = crate::R<EMRrs>;
#[doc = "Register `EMR` writer"]
pub type W = crate::W<EMRrs>;
#[doc = "Field `EM0` reader - Event mask on line x"]
pub type EM0_R = crate::BitReader;
#[doc = "Field `EM0` writer - Event mask on line x"]
pub type EM0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM1` reader - Event mask on line x"]
pub type EM1_R = crate::BitReader;
#[doc = "Field `EM1` writer - Event mask on line x"]
pub type EM1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM2` reader - Event mask on line x"]
pub type EM2_R = crate::BitReader;
#[doc = "Field `EM2` writer - Event mask on line x"]
pub type EM2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM3` reader - Event mask on line x"]
pub type EM3_R = crate::BitReader;
#[doc = "Field `EM3` writer - Event mask on line x"]
pub type EM3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM4` reader - Event mask on line x"]
pub type EM4_R = crate::BitReader;
#[doc = "Field `EM4` writer - Event mask on line x"]
pub type EM4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM5` reader - Event mask on line x"]
pub type EM5_R = crate::BitReader;
#[doc = "Field `EM5` writer - Event mask on line x"]
pub type EM5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM6` reader - Event mask on line x"]
pub type EM6_R = crate::BitReader;
#[doc = "Field `EM6` writer - Event mask on line x"]
pub type EM6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM7` reader - Event mask on line x"]
pub type EM7_R = crate::BitReader;
#[doc = "Field `EM7` writer - Event mask on line x"]
pub type EM7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM8` reader - Event mask on line x"]
pub type EM8_R = crate::BitReader;
#[doc = "Field `EM8` writer - Event mask on line x"]
pub type EM8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM10` reader - Event mask on line x"]
pub type EM10_R = crate::BitReader;
#[doc = "Field `EM10` writer - Event mask on line x"]
pub type EM10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM11` reader - Event mask on line x"]
pub type EM11_R = crate::BitReader;
#[doc = "Field `EM11` writer - Event mask on line x"]
pub type EM11_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Event mask on line x"]
    #[inline(always)]
    pub fn em0(&self) -> EM0_R {
        EM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Event mask on line x"]
    #[inline(always)]
    pub fn em1(&self) -> EM1_R {
        EM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Event mask on line x"]
    #[inline(always)]
    pub fn em2(&self) -> EM2_R {
        EM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Event mask on line x"]
    #[inline(always)]
    pub fn em3(&self) -> EM3_R {
        EM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Event mask on line x"]
    #[inline(always)]
    pub fn em4(&self) -> EM4_R {
        EM4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Event mask on line x"]
    #[inline(always)]
    pub fn em5(&self) -> EM5_R {
        EM5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Event mask on line x"]
    #[inline(always)]
    pub fn em6(&self) -> EM6_R {
        EM6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Event mask on line x"]
    #[inline(always)]
    pub fn em7(&self) -> EM7_R {
        EM7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Event mask on line x"]
    #[inline(always)]
    pub fn em8(&self) -> EM8_R {
        EM8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Event mask on line x"]
    #[inline(always)]
    pub fn em10(&self) -> EM10_R {
        EM10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Event mask on line x"]
    #[inline(always)]
    pub fn em11(&self) -> EM11_R {
        EM11_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Event mask on line x"]
    #[inline(always)]
    #[must_use]
    pub fn em0(&mut self) -> EM0_W<EMRrs> {
        EM0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Event mask on line x"]
    #[inline(always)]
    #[must_use]
    pub fn em1(&mut self) -> EM1_W<EMRrs> {
        EM1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Event mask on line x"]
    #[inline(always)]
    #[must_use]
    pub fn em2(&mut self) -> EM2_W<EMRrs> {
        EM2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Event mask on line x"]
    #[inline(always)]
    #[must_use]
    pub fn em3(&mut self) -> EM3_W<EMRrs> {
        EM3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Event mask on line x"]
    #[inline(always)]
    #[must_use]
    pub fn em4(&mut self) -> EM4_W<EMRrs> {
        EM4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Event mask on line x"]
    #[inline(always)]
    #[must_use]
    pub fn em5(&mut self) -> EM5_W<EMRrs> {
        EM5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Event mask on line x"]
    #[inline(always)]
    #[must_use]
    pub fn em6(&mut self) -> EM6_W<EMRrs> {
        EM6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Event mask on line x"]
    #[inline(always)]
    #[must_use]
    pub fn em7(&mut self) -> EM7_W<EMRrs> {
        EM7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Event mask on line x"]
    #[inline(always)]
    #[must_use]
    pub fn em8(&mut self) -> EM8_W<EMRrs> {
        EM8_W::new(self, 8)
    }
    #[doc = "Bit 10 - Event mask on line x"]
    #[inline(always)]
    #[must_use]
    pub fn em10(&mut self) -> EM10_W<EMRrs> {
        EM10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Event mask on line x"]
    #[inline(always)]
    #[must_use]
    pub fn em11(&mut self) -> EM11_W<EMRrs> {
        EM11_W::new(self, 11)
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
#[doc = "EMR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMRrs;
impl crate::RegisterSpec for EMRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emr::R`](R) reader structure"]
impl crate::Readable for EMRrs {}
#[doc = "`write(|w| ..)` method takes [`emr::W`](W) writer structure"]
impl crate::Writable for EMRrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EMR to value 0"]
impl crate::Resettable for EMRrs {
    const RESET_VALUE: u32 = 0;
}
