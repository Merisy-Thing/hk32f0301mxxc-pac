#[doc = "Register `CHSELR` reader"]
pub type R = crate::R<CHSELRrs>;
#[doc = "Register `CHSELR` writer"]
pub type W = crate::W<CHSELRrs>;
#[doc = "Field `CHSEL0` reader - Channel-x selection"]
pub type CHSEL0_R = crate::BitReader;
#[doc = "Field `CHSEL0` writer - Channel-x selection"]
pub type CHSEL0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHSEL1` reader - Channel-x selection"]
pub type CHSEL1_R = crate::BitReader;
#[doc = "Field `CHSEL1` writer - Channel-x selection"]
pub type CHSEL1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHSEL2` reader - Channel-x selection"]
pub type CHSEL2_R = crate::BitReader;
#[doc = "Field `CHSEL2` writer - Channel-x selection"]
pub type CHSEL2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHSEL3` reader - Channel-x selection"]
pub type CHSEL3_R = crate::BitReader;
#[doc = "Field `CHSEL3` writer - Channel-x selection"]
pub type CHSEL3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHSEL4` reader - Channel-x selection"]
pub type CHSEL4_R = crate::BitReader;
#[doc = "Field `CHSEL4` writer - Channel-x selection"]
pub type CHSEL4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHSEL5` reader - Channel-x selection"]
pub type CHSEL5_R = crate::BitReader;
#[doc = "Field `CHSEL5` writer - Channel-x selection"]
pub type CHSEL5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHSEL6` reader - Channel-x selection"]
pub type CHSEL6_R = crate::BitReader;
#[doc = "Field `CHSEL6` writer - Channel-x selection"]
pub type CHSEL6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHSEL7` reader - Channel-x selection"]
pub type CHSEL7_R = crate::BitReader;
#[doc = "Field `CHSEL7` writer - Channel-x selection"]
pub type CHSEL7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHSEL8` reader - Channel-x selection"]
pub type CHSEL8_R = crate::BitReader;
#[doc = "Field `CHSEL8` writer - Channel-x selection"]
pub type CHSEL8_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel0(&self) -> CHSEL0_R {
        CHSEL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel1(&self) -> CHSEL1_R {
        CHSEL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel2(&self) -> CHSEL2_R {
        CHSEL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel3(&self) -> CHSEL3_R {
        CHSEL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel4(&self) -> CHSEL4_R {
        CHSEL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel5(&self) -> CHSEL5_R {
        CHSEL5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel6(&self) -> CHSEL6_R {
        CHSEL6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel7(&self) -> CHSEL7_R {
        CHSEL7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel8(&self) -> CHSEL8_R {
        CHSEL8_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel-x selection"]
    #[inline(always)]
    #[must_use]
    pub fn chsel0(&mut self) -> CHSEL0_W<CHSELRrs> {
        CHSEL0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel-x selection"]
    #[inline(always)]
    #[must_use]
    pub fn chsel1(&mut self) -> CHSEL1_W<CHSELRrs> {
        CHSEL1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel-x selection"]
    #[inline(always)]
    #[must_use]
    pub fn chsel2(&mut self) -> CHSEL2_W<CHSELRrs> {
        CHSEL2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel-x selection"]
    #[inline(always)]
    #[must_use]
    pub fn chsel3(&mut self) -> CHSEL3_W<CHSELRrs> {
        CHSEL3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel-x selection"]
    #[inline(always)]
    #[must_use]
    pub fn chsel4(&mut self) -> CHSEL4_W<CHSELRrs> {
        CHSEL4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Channel-x selection"]
    #[inline(always)]
    #[must_use]
    pub fn chsel5(&mut self) -> CHSEL5_W<CHSELRrs> {
        CHSEL5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Channel-x selection"]
    #[inline(always)]
    #[must_use]
    pub fn chsel6(&mut self) -> CHSEL6_W<CHSELRrs> {
        CHSEL6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Channel-x selection"]
    #[inline(always)]
    #[must_use]
    pub fn chsel7(&mut self) -> CHSEL7_W<CHSELRrs> {
        CHSEL7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Channel-x selection"]
    #[inline(always)]
    #[must_use]
    pub fn chsel8(&mut self) -> CHSEL8_W<CHSELRrs> {
        CHSEL8_W::new(self, 8)
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
#[doc = "CHSELR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chselr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chselr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHSELRrs;
impl crate::RegisterSpec for CHSELRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chselr::R`](R) reader structure"]
impl crate::Readable for CHSELRrs {}
#[doc = "`write(|w| ..)` method takes [`chselr::W`](W) writer structure"]
impl crate::Writable for CHSELRrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHSELR to value 0"]
impl crate::Resettable for CHSELRrs {
    const RESET_VALUE: u32 = 0;
}
