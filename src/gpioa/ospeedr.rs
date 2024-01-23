#[doc = "Register `OSPEEDR` reader"]
pub type R = crate::R<OSPEEDRrs>;
#[doc = "Register `OSPEEDR` writer"]
pub type W = crate::W<OSPEEDRrs>;
#[doc = "Field `OSPEEDR0` reader - Port x pin y output speed configuration bits"]
pub type OSPEEDR0_R = crate::FieldReader;
#[doc = "Field `OSPEEDR0` writer - Port x pin y output speed configuration bits"]
pub type OSPEEDR0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSPEEDR1` reader - Port x pin y output speed configuration bits"]
pub type OSPEEDR1_R = crate::FieldReader;
#[doc = "Field `OSPEEDR1` writer - Port x pin y output speed configuration bits"]
pub type OSPEEDR1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSPEEDR2` reader - Port x pin y output speed configuration bits"]
pub type OSPEEDR2_R = crate::FieldReader;
#[doc = "Field `OSPEEDR2` writer - Port x pin y output speed configuration bits"]
pub type OSPEEDR2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSPEEDR3` reader - Port x pin y output speed configuration bits"]
pub type OSPEEDR3_R = crate::FieldReader;
#[doc = "Field `OSPEEDR3` writer - Port x pin y output speed configuration bits"]
pub type OSPEEDR3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSPEEDR4` reader - Port x pin y output speed configuration bits"]
pub type OSPEEDR4_R = crate::FieldReader;
#[doc = "Field `OSPEEDR4` writer - Port x pin y output speed configuration bits"]
pub type OSPEEDR4_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSPEEDR5` reader - Port x pin y output speed configuration bits"]
pub type OSPEEDR5_R = crate::FieldReader;
#[doc = "Field `OSPEEDR5` writer - Port x pin y output speed configuration bits"]
pub type OSPEEDR5_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSPEEDR6` reader - Port x pin y output speed configuration bits"]
pub type OSPEEDR6_R = crate::FieldReader;
#[doc = "Field `OSPEEDR6` writer - Port x pin y output speed configuration bits"]
pub type OSPEEDR6_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSPEEDR7` reader - Port x pin y output speed configuration bits"]
pub type OSPEEDR7_R = crate::FieldReader;
#[doc = "Field `OSPEEDR7` writer - Port x pin y output speed configuration bits"]
pub type OSPEEDR7_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Port x pin y output speed configuration bits"]
    #[inline(always)]
    pub fn ospeedr0(&self) -> OSPEEDR0_R {
        OSPEEDR0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port x pin y output speed configuration bits"]
    #[inline(always)]
    pub fn ospeedr1(&self) -> OSPEEDR1_R {
        OSPEEDR1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port x pin y output speed configuration bits"]
    #[inline(always)]
    pub fn ospeedr2(&self) -> OSPEEDR2_R {
        OSPEEDR2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port x pin y output speed configuration bits"]
    #[inline(always)]
    pub fn ospeedr3(&self) -> OSPEEDR3_R {
        OSPEEDR3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port x pin y output speed configuration bits"]
    #[inline(always)]
    pub fn ospeedr4(&self) -> OSPEEDR4_R {
        OSPEEDR4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port x pin y output speed configuration bits"]
    #[inline(always)]
    pub fn ospeedr5(&self) -> OSPEEDR5_R {
        OSPEEDR5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port x pin y output speed configuration bits"]
    #[inline(always)]
    pub fn ospeedr6(&self) -> OSPEEDR6_R {
        OSPEEDR6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port x pin y output speed configuration bits"]
    #[inline(always)]
    pub fn ospeedr7(&self) -> OSPEEDR7_R {
        OSPEEDR7_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port x pin y output speed configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn ospeedr0(&mut self) -> OSPEEDR0_W<OSPEEDRrs> {
        OSPEEDR0_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Port x pin y output speed configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn ospeedr1(&mut self) -> OSPEEDR1_W<OSPEEDRrs> {
        OSPEEDR1_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Port x pin y output speed configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn ospeedr2(&mut self) -> OSPEEDR2_W<OSPEEDRrs> {
        OSPEEDR2_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Port x pin y output speed configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn ospeedr3(&mut self) -> OSPEEDR3_W<OSPEEDRrs> {
        OSPEEDR3_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Port x pin y output speed configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn ospeedr4(&mut self) -> OSPEEDR4_W<OSPEEDRrs> {
        OSPEEDR4_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Port x pin y output speed configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn ospeedr5(&mut self) -> OSPEEDR5_W<OSPEEDRrs> {
        OSPEEDR5_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Port x pin y output speed configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn ospeedr6(&mut self) -> OSPEEDR6_W<OSPEEDRrs> {
        OSPEEDR6_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Port x pin y output speed configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn ospeedr7(&mut self) -> OSPEEDR7_W<OSPEEDRrs> {
        OSPEEDR7_W::new(self, 14)
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
#[doc = "OSPEEDR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospeedr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospeedr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSPEEDRrs;
impl crate::RegisterSpec for OSPEEDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ospeedr::R`](R) reader structure"]
impl crate::Readable for OSPEEDRrs {}
#[doc = "`write(|w| ..)` method takes [`ospeedr::W`](W) writer structure"]
impl crate::Writable for OSPEEDRrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSPEEDR to value 0"]
impl crate::Resettable for OSPEEDRrs {
    const RESET_VALUE: u32 = 0;
}
