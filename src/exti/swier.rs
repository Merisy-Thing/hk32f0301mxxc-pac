#[doc = "Register `SWIER` reader"]
pub type R = crate::R<SWIERrs>;
#[doc = "Register `SWIER` writer"]
pub type W = crate::W<SWIERrs>;
#[doc = "Field `SWI0` reader - Software interrupt on line x"]
pub type SWI0_R = crate::BitReader;
#[doc = "Field `SWI0` writer - Software interrupt on line x"]
pub type SWI0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI1` reader - Software interrupt on line x"]
pub type SWI1_R = crate::BitReader;
#[doc = "Field `SWI1` writer - Software interrupt on line x"]
pub type SWI1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI2` reader - Software interrupt on line x"]
pub type SWI2_R = crate::BitReader;
#[doc = "Field `SWI2` writer - Software interrupt on line x"]
pub type SWI2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI3` reader - Software interrupt on line x"]
pub type SWI3_R = crate::BitReader;
#[doc = "Field `SWI3` writer - Software interrupt on line x"]
pub type SWI3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI4` reader - Software interrupt on line x"]
pub type SWI4_R = crate::BitReader;
#[doc = "Field `SWI4` writer - Software interrupt on line x"]
pub type SWI4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI5` reader - Software interrupt on line x"]
pub type SWI5_R = crate::BitReader;
#[doc = "Field `SWI5` writer - Software interrupt on line x"]
pub type SWI5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI6` reader - Software interrupt on line x"]
pub type SWI6_R = crate::BitReader;
#[doc = "Field `SWI6` writer - Software interrupt on line x"]
pub type SWI6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI7` reader - Software interrupt on line x"]
pub type SWI7_R = crate::BitReader;
#[doc = "Field `SWI7` writer - Software interrupt on line x"]
pub type SWI7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI11` reader - Software interrupt on line 11"]
pub type SWI11_R = crate::BitReader;
#[doc = "Field `SWI11` writer - Software interrupt on line 11"]
pub type SWI11_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Software interrupt on line x"]
    #[inline(always)]
    pub fn swi0(&self) -> SWI0_R {
        SWI0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software interrupt on line x"]
    #[inline(always)]
    pub fn swi1(&self) -> SWI1_R {
        SWI1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Software interrupt on line x"]
    #[inline(always)]
    pub fn swi2(&self) -> SWI2_R {
        SWI2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Software interrupt on line x"]
    #[inline(always)]
    pub fn swi3(&self) -> SWI3_R {
        SWI3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Software interrupt on line x"]
    #[inline(always)]
    pub fn swi4(&self) -> SWI4_R {
        SWI4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Software interrupt on line x"]
    #[inline(always)]
    pub fn swi5(&self) -> SWI5_R {
        SWI5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Software interrupt on line x"]
    #[inline(always)]
    pub fn swi6(&self) -> SWI6_R {
        SWI6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Software interrupt on line x"]
    #[inline(always)]
    pub fn swi7(&self) -> SWI7_R {
        SWI7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 11 - Software interrupt on line 11"]
    #[inline(always)]
    pub fn swi11(&self) -> SWI11_R {
        SWI11_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software interrupt on line x"]
    #[inline(always)]
    #[must_use]
    pub fn swi0(&mut self) -> SWI0_W<SWIERrs> {
        SWI0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Software interrupt on line x"]
    #[inline(always)]
    #[must_use]
    pub fn swi1(&mut self) -> SWI1_W<SWIERrs> {
        SWI1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Software interrupt on line x"]
    #[inline(always)]
    #[must_use]
    pub fn swi2(&mut self) -> SWI2_W<SWIERrs> {
        SWI2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Software interrupt on line x"]
    #[inline(always)]
    #[must_use]
    pub fn swi3(&mut self) -> SWI3_W<SWIERrs> {
        SWI3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Software interrupt on line x"]
    #[inline(always)]
    #[must_use]
    pub fn swi4(&mut self) -> SWI4_W<SWIERrs> {
        SWI4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Software interrupt on line x"]
    #[inline(always)]
    #[must_use]
    pub fn swi5(&mut self) -> SWI5_W<SWIERrs> {
        SWI5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Software interrupt on line x"]
    #[inline(always)]
    #[must_use]
    pub fn swi6(&mut self) -> SWI6_W<SWIERrs> {
        SWI6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Software interrupt on line x"]
    #[inline(always)]
    #[must_use]
    pub fn swi7(&mut self) -> SWI7_W<SWIERrs> {
        SWI7_W::new(self, 7)
    }
    #[doc = "Bit 11 - Software interrupt on line 11"]
    #[inline(always)]
    #[must_use]
    pub fn swi11(&mut self) -> SWI11_W<SWIERrs> {
        SWI11_W::new(self, 11)
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
#[doc = "SWIER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWIERrs;
impl crate::RegisterSpec for SWIERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swier::R`](R) reader structure"]
impl crate::Readable for SWIERrs {}
#[doc = "`write(|w| ..)` method takes [`swier::W`](W) writer structure"]
impl crate::Writable for SWIERrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWIER to value 0"]
impl crate::Resettable for SWIERrs {
    const RESET_VALUE: u32 = 0;
}
