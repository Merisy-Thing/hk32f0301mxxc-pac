#[doc = "Register `IMR` reader"]
pub type R = crate::R<IMRrs>;
#[doc = "Register `IMR` writer"]
pub type W = crate::W<IMRrs>;
#[doc = "Field `IM0` reader - Interrupt mask on line x"]
pub type IM0_R = crate::BitReader;
#[doc = "Field `IM0` writer - Interrupt mask on line x"]
pub type IM0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM1` reader - Interrupt mask on line x"]
pub type IM1_R = crate::BitReader;
#[doc = "Field `IM1` writer - Interrupt mask on line x"]
pub type IM1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM2` reader - Interrupt mask on line x"]
pub type IM2_R = crate::BitReader;
#[doc = "Field `IM2` writer - Interrupt mask on line x"]
pub type IM2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM3` reader - Interrupt mask on line x"]
pub type IM3_R = crate::BitReader;
#[doc = "Field `IM3` writer - Interrupt mask on line x"]
pub type IM3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM4` reader - Interrupt mask on line x"]
pub type IM4_R = crate::BitReader;
#[doc = "Field `IM4` writer - Interrupt mask on line x"]
pub type IM4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM5` reader - Interrupt mask on line x"]
pub type IM5_R = crate::BitReader;
#[doc = "Field `IM5` writer - Interrupt mask on line x"]
pub type IM5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM6` reader - Interrupt mask on line x"]
pub type IM6_R = crate::BitReader;
#[doc = "Field `IM6` writer - Interrupt mask on line x"]
pub type IM6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM7` reader - Interrupt mask on line x"]
pub type IM7_R = crate::BitReader;
#[doc = "Field `IM7` writer - Interrupt mask on line x"]
pub type IM7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM8` reader - Interrupt mask on line x"]
pub type IM8_R = crate::BitReader;
#[doc = "Field `IM8` writer - Interrupt mask on line x"]
pub type IM8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM10` reader - Interrupt mask on line x"]
pub type IM10_R = crate::BitReader;
#[doc = "Field `IM10` writer - Interrupt mask on line x"]
pub type IM10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM11` reader - Interrupt mask on line x"]
pub type IM11_R = crate::BitReader;
#[doc = "Field `IM11` writer - Interrupt mask on line x"]
pub type IM11_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Interrupt mask on line x"]
    #[inline(always)]
    pub fn im0(&self) -> IM0_R {
        IM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt mask on line x"]
    #[inline(always)]
    pub fn im1(&self) -> IM1_R {
        IM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt mask on line x"]
    #[inline(always)]
    pub fn im2(&self) -> IM2_R {
        IM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt mask on line x"]
    #[inline(always)]
    pub fn im3(&self) -> IM3_R {
        IM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt mask on line x"]
    #[inline(always)]
    pub fn im4(&self) -> IM4_R {
        IM4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt mask on line x"]
    #[inline(always)]
    pub fn im5(&self) -> IM5_R {
        IM5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt mask on line x"]
    #[inline(always)]
    pub fn im6(&self) -> IM6_R {
        IM6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt mask on line x"]
    #[inline(always)]
    pub fn im7(&self) -> IM7_R {
        IM7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt mask on line x"]
    #[inline(always)]
    pub fn im8(&self) -> IM8_R {
        IM8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt mask on line x"]
    #[inline(always)]
    pub fn im10(&self) -> IM10_R {
        IM10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt mask on line x"]
    #[inline(always)]
    pub fn im11(&self) -> IM11_R {
        IM11_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt mask on line x"]
    #[inline(always)]
    #[must_use]
    pub fn im0(&mut self) -> IM0_W<IMRrs> {
        IM0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Interrupt mask on line x"]
    #[inline(always)]
    #[must_use]
    pub fn im1(&mut self) -> IM1_W<IMRrs> {
        IM1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt mask on line x"]
    #[inline(always)]
    #[must_use]
    pub fn im2(&mut self) -> IM2_W<IMRrs> {
        IM2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt mask on line x"]
    #[inline(always)]
    #[must_use]
    pub fn im3(&mut self) -> IM3_W<IMRrs> {
        IM3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Interrupt mask on line x"]
    #[inline(always)]
    #[must_use]
    pub fn im4(&mut self) -> IM4_W<IMRrs> {
        IM4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt mask on line x"]
    #[inline(always)]
    #[must_use]
    pub fn im5(&mut self) -> IM5_W<IMRrs> {
        IM5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Interrupt mask on line x"]
    #[inline(always)]
    #[must_use]
    pub fn im6(&mut self) -> IM6_W<IMRrs> {
        IM6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Interrupt mask on line x"]
    #[inline(always)]
    #[must_use]
    pub fn im7(&mut self) -> IM7_W<IMRrs> {
        IM7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Interrupt mask on line x"]
    #[inline(always)]
    #[must_use]
    pub fn im8(&mut self) -> IM8_W<IMRrs> {
        IM8_W::new(self, 8)
    }
    #[doc = "Bit 10 - Interrupt mask on line x"]
    #[inline(always)]
    #[must_use]
    pub fn im10(&mut self) -> IM10_W<IMRrs> {
        IM10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Interrupt mask on line x"]
    #[inline(always)]
    #[must_use]
    pub fn im11(&mut self) -> IM11_W<IMRrs> {
        IM11_W::new(self, 11)
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
#[doc = "IMR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IMRrs;
impl crate::RegisterSpec for IMRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for IMRrs {}
#[doc = "`write(|w| ..)` method takes [`imr::W`](W) writer structure"]
impl crate::Writable for IMRrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for IMRrs {
    const RESET_VALUE: u32 = 0;
}
