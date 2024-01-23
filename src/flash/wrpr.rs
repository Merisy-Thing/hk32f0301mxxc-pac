#[doc = "Register `WRPR` reader"]
pub type R = crate::R<WRPRrs>;
#[doc = "Register `WRPR` writer"]
pub type W = crate::W<WRPRrs>;
#[doc = "Field `WRP0` reader - Write protect"]
pub type WRP0_R = crate::FieldReader;
#[doc = "Field `WRP0` writer - Write protect"]
pub type WRP0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WRP1` reader - Write protect"]
pub type WRP1_R = crate::FieldReader;
#[doc = "Field `WRP1` writer - Write protect"]
pub type WRP1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WRP2` reader - Write protect"]
pub type WRP2_R = crate::FieldReader;
#[doc = "Field `WRP2` writer - Write protect"]
pub type WRP2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WRP3` reader - Write protect"]
pub type WRP3_R = crate::FieldReader;
#[doc = "Field `WRP3` writer - Write protect"]
pub type WRP3_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Write protect"]
    #[inline(always)]
    pub fn wrp0(&self) -> WRP0_R {
        WRP0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Write protect"]
    #[inline(always)]
    pub fn wrp1(&self) -> WRP1_R {
        WRP1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Write protect"]
    #[inline(always)]
    pub fn wrp2(&self) -> WRP2_R {
        WRP2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Write protect"]
    #[inline(always)]
    pub fn wrp3(&self) -> WRP3_R {
        WRP3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Write protect"]
    #[inline(always)]
    #[must_use]
    pub fn wrp0(&mut self) -> WRP0_W<WRPRrs> {
        WRP0_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Write protect"]
    #[inline(always)]
    #[must_use]
    pub fn wrp1(&mut self) -> WRP1_W<WRPRrs> {
        WRP1_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Write protect"]
    #[inline(always)]
    #[must_use]
    pub fn wrp2(&mut self) -> WRP2_W<WRPRrs> {
        WRP2_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Write protect"]
    #[inline(always)]
    #[must_use]
    pub fn wrp3(&mut self) -> WRP3_W<WRPRrs> {
        WRP3_W::new(self, 24)
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
#[doc = "WRPR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wrpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WRPRrs;
impl crate::RegisterSpec for WRPRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wrpr::R`](R) reader structure"]
impl crate::Readable for WRPRrs {}
#[doc = "`write(|w| ..)` method takes [`wrpr::W`](W) writer structure"]
impl crate::Writable for WRPRrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WRPR to value 0"]
impl crate::Resettable for WRPRrs {
    const RESET_VALUE: u32 = 0;
}
