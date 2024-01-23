#[doc = "Register `ODR` reader"]
pub type R = crate::R<ODRrs>;
#[doc = "Register `ODR` writer"]
pub type W = crate::W<ODRrs>;
#[doc = "Field `OD0` reader - Port x pin y output data"]
pub type OD0_R = crate::BitReader;
#[doc = "Field `OD0` writer - Port x pin y output data"]
pub type OD0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD1` reader - Port x pin y output data"]
pub type OD1_R = crate::BitReader;
#[doc = "Field `OD1` writer - Port x pin y output data"]
pub type OD1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD2` reader - Port x pin y output data"]
pub type OD2_R = crate::BitReader;
#[doc = "Field `OD2` writer - Port x pin y output data"]
pub type OD2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD3` reader - Port x pin y output data"]
pub type OD3_R = crate::BitReader;
#[doc = "Field `OD3` writer - Port x pin y output data"]
pub type OD3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD4` reader - Port x pin y output data"]
pub type OD4_R = crate::BitReader;
#[doc = "Field `OD4` writer - Port x pin y output data"]
pub type OD4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD5` reader - Port x pin y output data"]
pub type OD5_R = crate::BitReader;
#[doc = "Field `OD5` writer - Port x pin y output data"]
pub type OD5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD6` reader - Port x pin y output data"]
pub type OD6_R = crate::BitReader;
#[doc = "Field `OD6` writer - Port x pin y output data"]
pub type OD6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD7` reader - Port x pin y output data"]
pub type OD7_R = crate::BitReader;
#[doc = "Field `OD7` writer - Port x pin y output data"]
pub type OD7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Port x pin y output data"]
    #[inline(always)]
    pub fn od0(&self) -> OD0_R {
        OD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port x pin y output data"]
    #[inline(always)]
    pub fn od1(&self) -> OD1_R {
        OD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port x pin y output data"]
    #[inline(always)]
    pub fn od2(&self) -> OD2_R {
        OD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port x pin y output data"]
    #[inline(always)]
    pub fn od3(&self) -> OD3_R {
        OD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port x pin y output data"]
    #[inline(always)]
    pub fn od4(&self) -> OD4_R {
        OD4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port x pin y output data"]
    #[inline(always)]
    pub fn od5(&self) -> OD5_R {
        OD5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port x pin y output data"]
    #[inline(always)]
    pub fn od6(&self) -> OD6_R {
        OD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port x pin y output data"]
    #[inline(always)]
    pub fn od7(&self) -> OD7_R {
        OD7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port x pin y output data"]
    #[inline(always)]
    #[must_use]
    pub fn od0(&mut self) -> OD0_W<ODRrs> {
        OD0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Port x pin y output data"]
    #[inline(always)]
    #[must_use]
    pub fn od1(&mut self) -> OD1_W<ODRrs> {
        OD1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Port x pin y output data"]
    #[inline(always)]
    #[must_use]
    pub fn od2(&mut self) -> OD2_W<ODRrs> {
        OD2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Port x pin y output data"]
    #[inline(always)]
    #[must_use]
    pub fn od3(&mut self) -> OD3_W<ODRrs> {
        OD3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Port x pin y output data"]
    #[inline(always)]
    #[must_use]
    pub fn od4(&mut self) -> OD4_W<ODRrs> {
        OD4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Port x pin y output data"]
    #[inline(always)]
    #[must_use]
    pub fn od5(&mut self) -> OD5_W<ODRrs> {
        OD5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Port x pin y output data"]
    #[inline(always)]
    #[must_use]
    pub fn od6(&mut self) -> OD6_W<ODRrs> {
        OD6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Port x pin y output data"]
    #[inline(always)]
    #[must_use]
    pub fn od7(&mut self) -> OD7_W<ODRrs> {
        OD7_W::new(self, 7)
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
#[doc = "ODR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`odr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`odr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ODRrs;
impl crate::RegisterSpec for ODRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`odr::R`](R) reader structure"]
impl crate::Readable for ODRrs {}
#[doc = "`write(|w| ..)` method takes [`odr::W`](W) writer structure"]
impl crate::Writable for ODRrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ODR to value 0"]
impl crate::Resettable for ODRrs {
    const RESET_VALUE: u32 = 0;
}
