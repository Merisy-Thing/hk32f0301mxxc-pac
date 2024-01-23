#[doc = "Register `MODER` reader"]
pub type R = crate::R<MODERrs>;
#[doc = "Register `MODER` writer"]
pub type W = crate::W<MODERrs>;
#[doc = "Field `MODER0` reader - Port x pin y mode configuration bits"]
pub type MODER0_R = crate::FieldReader;
#[doc = "Field `MODER0` writer - Port x pin y mode configuration bits"]
pub type MODER0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODER1` reader - Port x pin y mode configuration bits"]
pub type MODER1_R = crate::FieldReader;
#[doc = "Field `MODER1` writer - Port x pin y mode configuration bits"]
pub type MODER1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODER2` reader - Port x pin y mode configuration bits"]
pub type MODER2_R = crate::FieldReader;
#[doc = "Field `MODER2` writer - Port x pin y mode configuration bits"]
pub type MODER2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODER3` reader - Port x pin y mode configuration bits"]
pub type MODER3_R = crate::FieldReader;
#[doc = "Field `MODER3` writer - Port x pin y mode configuration bits"]
pub type MODER3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODER4` reader - Port x pin y mode configuration bits"]
pub type MODER4_R = crate::FieldReader;
#[doc = "Field `MODER4` writer - Port x pin y mode configuration bits"]
pub type MODER4_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODER5` reader - Port x pin y mode configuration bits"]
pub type MODER5_R = crate::FieldReader;
#[doc = "Field `MODER5` writer - Port x pin y mode configuration bits"]
pub type MODER5_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODER6` reader - Port x pin y mode configuration bits"]
pub type MODER6_R = crate::FieldReader;
#[doc = "Field `MODER6` writer - Port x pin y mode configuration bits"]
pub type MODER6_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODER7` reader - Port x pin y mode configuration bits"]
pub type MODER7_R = crate::FieldReader;
#[doc = "Field `MODER7` writer - Port x pin y mode configuration bits"]
pub type MODER7_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Port x pin y mode configuration bits"]
    #[inline(always)]
    pub fn moder0(&self) -> MODER0_R {
        MODER0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port x pin y mode configuration bits"]
    #[inline(always)]
    pub fn moder1(&self) -> MODER1_R {
        MODER1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port x pin y mode configuration bits"]
    #[inline(always)]
    pub fn moder2(&self) -> MODER2_R {
        MODER2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port x pin y mode configuration bits"]
    #[inline(always)]
    pub fn moder3(&self) -> MODER3_R {
        MODER3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port x pin y mode configuration bits"]
    #[inline(always)]
    pub fn moder4(&self) -> MODER4_R {
        MODER4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port x pin y mode configuration bits"]
    #[inline(always)]
    pub fn moder5(&self) -> MODER5_R {
        MODER5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port x pin y mode configuration bits"]
    #[inline(always)]
    pub fn moder6(&self) -> MODER6_R {
        MODER6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port x pin y mode configuration bits"]
    #[inline(always)]
    pub fn moder7(&self) -> MODER7_R {
        MODER7_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port x pin y mode configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn moder0(&mut self) -> MODER0_W<MODERrs> {
        MODER0_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Port x pin y mode configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn moder1(&mut self) -> MODER1_W<MODERrs> {
        MODER1_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Port x pin y mode configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn moder2(&mut self) -> MODER2_W<MODERrs> {
        MODER2_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Port x pin y mode configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn moder3(&mut self) -> MODER3_W<MODERrs> {
        MODER3_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Port x pin y mode configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn moder4(&mut self) -> MODER4_W<MODERrs> {
        MODER4_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Port x pin y mode configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn moder5(&mut self) -> MODER5_W<MODERrs> {
        MODER5_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Port x pin y mode configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn moder6(&mut self) -> MODER6_W<MODERrs> {
        MODER6_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Port x pin y mode configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn moder7(&mut self) -> MODER7_W<MODERrs> {
        MODER7_W::new(self, 14)
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
#[doc = "MODER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`moder::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`moder::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MODERrs;
impl crate::RegisterSpec for MODERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`moder::R`](R) reader structure"]
impl crate::Readable for MODERrs {}
#[doc = "`write(|w| ..)` method takes [`moder::W`](W) writer structure"]
impl crate::Writable for MODERrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MODER to value 0xfbff"]
impl crate::Resettable for MODERrs {
    const RESET_VALUE: u32 = 0xfbff;
}
