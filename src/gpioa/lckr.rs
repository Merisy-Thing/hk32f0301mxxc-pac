#[doc = "Register `LCKR` reader"]
pub type R = crate::R<LCKRrs>;
#[doc = "Register `LCKR` writer"]
pub type W = crate::W<LCKRrs>;
#[doc = "Field `LCK0` reader - Port x pin y lock bit y"]
pub type LCK0_R = crate::BitReader;
#[doc = "Field `LCK0` writer - Port x pin y lock bit y"]
pub type LCK0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCK1` reader - Port x pin y lock bit y"]
pub type LCK1_R = crate::BitReader;
#[doc = "Field `LCK1` writer - Port x pin y lock bit y"]
pub type LCK1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCK2` reader - Port x pin y lock bit y"]
pub type LCK2_R = crate::BitReader;
#[doc = "Field `LCK2` writer - Port x pin y lock bit y"]
pub type LCK2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCK3` reader - Port x pin y lock bit y"]
pub type LCK3_R = crate::BitReader;
#[doc = "Field `LCK3` writer - Port x pin y lock bit y"]
pub type LCK3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCK4` reader - Port x pin y lock bit y"]
pub type LCK4_R = crate::BitReader;
#[doc = "Field `LCK4` writer - Port x pin y lock bit y"]
pub type LCK4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCK5` reader - Port x pin y lock bit y"]
pub type LCK5_R = crate::BitReader;
#[doc = "Field `LCK5` writer - Port x pin y lock bit y"]
pub type LCK5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCK6` reader - Port x pin y lock bit y"]
pub type LCK6_R = crate::BitReader;
#[doc = "Field `LCK6` writer - Port x pin y lock bit y"]
pub type LCK6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCK7` reader - Port x pin y lock bit y"]
pub type LCK7_R = crate::BitReader;
#[doc = "Field `LCK7` writer - Port x pin y lock bit y"]
pub type LCK7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCKK` reader - Lock key"]
pub type LCKK_R = crate::BitReader;
#[doc = "Field `LCKK` writer - Lock key"]
pub type LCKK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Port x pin y lock bit y"]
    #[inline(always)]
    pub fn lck0(&self) -> LCK0_R {
        LCK0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port x pin y lock bit y"]
    #[inline(always)]
    pub fn lck1(&self) -> LCK1_R {
        LCK1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port x pin y lock bit y"]
    #[inline(always)]
    pub fn lck2(&self) -> LCK2_R {
        LCK2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port x pin y lock bit y"]
    #[inline(always)]
    pub fn lck3(&self) -> LCK3_R {
        LCK3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port x pin y lock bit y"]
    #[inline(always)]
    pub fn lck4(&self) -> LCK4_R {
        LCK4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port x pin y lock bit y"]
    #[inline(always)]
    pub fn lck5(&self) -> LCK5_R {
        LCK5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port x pin y lock bit y"]
    #[inline(always)]
    pub fn lck6(&self) -> LCK6_R {
        LCK6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port x pin y lock bit y"]
    #[inline(always)]
    pub fn lck7(&self) -> LCK7_R {
        LCK7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Lock key"]
    #[inline(always)]
    pub fn lckk(&self) -> LCKK_R {
        LCKK_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port x pin y lock bit y"]
    #[inline(always)]
    #[must_use]
    pub fn lck0(&mut self) -> LCK0_W<LCKRrs> {
        LCK0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Port x pin y lock bit y"]
    #[inline(always)]
    #[must_use]
    pub fn lck1(&mut self) -> LCK1_W<LCKRrs> {
        LCK1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Port x pin y lock bit y"]
    #[inline(always)]
    #[must_use]
    pub fn lck2(&mut self) -> LCK2_W<LCKRrs> {
        LCK2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Port x pin y lock bit y"]
    #[inline(always)]
    #[must_use]
    pub fn lck3(&mut self) -> LCK3_W<LCKRrs> {
        LCK3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Port x pin y lock bit y"]
    #[inline(always)]
    #[must_use]
    pub fn lck4(&mut self) -> LCK4_W<LCKRrs> {
        LCK4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Port x pin y lock bit y"]
    #[inline(always)]
    #[must_use]
    pub fn lck5(&mut self) -> LCK5_W<LCKRrs> {
        LCK5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Port x pin y lock bit y"]
    #[inline(always)]
    #[must_use]
    pub fn lck6(&mut self) -> LCK6_W<LCKRrs> {
        LCK6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Port x pin y lock bit y"]
    #[inline(always)]
    #[must_use]
    pub fn lck7(&mut self) -> LCK7_W<LCKRrs> {
        LCK7_W::new(self, 7)
    }
    #[doc = "Bit 16 - Lock key"]
    #[inline(always)]
    #[must_use]
    pub fn lckk(&mut self) -> LCKK_W<LCKRrs> {
        LCKK_W::new(self, 16)
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
#[doc = "LCKR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lckr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lckr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCKRrs;
impl crate::RegisterSpec for LCKRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lckr::R`](R) reader structure"]
impl crate::Readable for LCKRrs {}
#[doc = "`write(|w| ..)` method takes [`lckr::W`](W) writer structure"]
impl crate::Writable for LCKRrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LCKR to value 0"]
impl crate::Resettable for LCKRrs {
    const RESET_VALUE: u32 = 0;
}
