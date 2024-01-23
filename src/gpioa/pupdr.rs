#[doc = "Register `PUPDR` reader"]
pub type R = crate::R<PUPDRrs>;
#[doc = "Register `PUPDR` writer"]
pub type W = crate::W<PUPDRrs>;
#[doc = "Field `PUPDR0` reader - Port x pin y pull-up/down configuration bits"]
pub type PUPDR0_R = crate::FieldReader;
#[doc = "Field `PUPDR0` writer - Port x pin y pull-up/down configuration bits"]
pub type PUPDR0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPDR1` reader - Port x pin y pull-up/down configuration bits"]
pub type PUPDR1_R = crate::FieldReader;
#[doc = "Field `PUPDR1` writer - Port x pin y pull-up/down configuration bits"]
pub type PUPDR1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPDR2` reader - Port x pin y pull-up/down configuration bits"]
pub type PUPDR2_R = crate::FieldReader;
#[doc = "Field `PUPDR2` writer - Port x pin y pull-up/down configuration bits"]
pub type PUPDR2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPDR3` reader - Port x pin y pull-up/down configuration bits"]
pub type PUPDR3_R = crate::FieldReader;
#[doc = "Field `PUPDR3` writer - Port x pin y pull-up/down configuration bits"]
pub type PUPDR3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPDR4` reader - Port x pin y pull-up/down configuration bits"]
pub type PUPDR4_R = crate::FieldReader;
#[doc = "Field `PUPDR4` writer - Port x pin y pull-up/down configuration bits"]
pub type PUPDR4_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPDR5` reader - Port x pin y pull-up/down configuration bits"]
pub type PUPDR5_R = crate::FieldReader;
#[doc = "Field `PUPDR5` writer - Port x pin y pull-up/down configuration bits"]
pub type PUPDR5_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPDR6` reader - Port x pin y pull-up/down configuration bits"]
pub type PUPDR6_R = crate::FieldReader;
#[doc = "Field `PUPDR6` writer - Port x pin y pull-up/down configuration bits"]
pub type PUPDR6_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPDR7` reader - Port x pin y pull-up/down configuration bits"]
pub type PUPDR7_R = crate::FieldReader;
#[doc = "Field `PUPDR7` writer - Port x pin y pull-up/down configuration bits"]
pub type PUPDR7_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Port x pin y pull-up/down configuration bits"]
    #[inline(always)]
    pub fn pupdr0(&self) -> PUPDR0_R {
        PUPDR0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port x pin y pull-up/down configuration bits"]
    #[inline(always)]
    pub fn pupdr1(&self) -> PUPDR1_R {
        PUPDR1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port x pin y pull-up/down configuration bits"]
    #[inline(always)]
    pub fn pupdr2(&self) -> PUPDR2_R {
        PUPDR2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port x pin y pull-up/down configuration bits"]
    #[inline(always)]
    pub fn pupdr3(&self) -> PUPDR3_R {
        PUPDR3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port x pin y pull-up/down configuration bits"]
    #[inline(always)]
    pub fn pupdr4(&self) -> PUPDR4_R {
        PUPDR4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port x pin y pull-up/down configuration bits"]
    #[inline(always)]
    pub fn pupdr5(&self) -> PUPDR5_R {
        PUPDR5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port x pin y pull-up/down configuration bits"]
    #[inline(always)]
    pub fn pupdr6(&self) -> PUPDR6_R {
        PUPDR6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port x pin y pull-up/down configuration bits"]
    #[inline(always)]
    pub fn pupdr7(&self) -> PUPDR7_R {
        PUPDR7_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port x pin y pull-up/down configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr0(&mut self) -> PUPDR0_W<PUPDRrs> {
        PUPDR0_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Port x pin y pull-up/down configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr1(&mut self) -> PUPDR1_W<PUPDRrs> {
        PUPDR1_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Port x pin y pull-up/down configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr2(&mut self) -> PUPDR2_W<PUPDRrs> {
        PUPDR2_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Port x pin y pull-up/down configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr3(&mut self) -> PUPDR3_W<PUPDRrs> {
        PUPDR3_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Port x pin y pull-up/down configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr4(&mut self) -> PUPDR4_W<PUPDRrs> {
        PUPDR4_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Port x pin y pull-up/down configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr5(&mut self) -> PUPDR5_W<PUPDRrs> {
        PUPDR5_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Port x pin y pull-up/down configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr6(&mut self) -> PUPDR6_W<PUPDRrs> {
        PUPDR6_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Port x pin y pull-up/down configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr7(&mut self) -> PUPDR7_W<PUPDRrs> {
        PUPDR7_W::new(self, 14)
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
#[doc = "PUPDR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pupdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pupdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PUPDRrs;
impl crate::RegisterSpec for PUPDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pupdr::R`](R) reader structure"]
impl crate::Readable for PUPDRrs {}
#[doc = "`write(|w| ..)` method takes [`pupdr::W`](W) writer structure"]
impl crate::Writable for PUPDRrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PUPDR to value 0"]
impl crate::Resettable for PUPDRrs {
    const RESET_VALUE: u32 = 0;
}
