#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Field `CKSEL` reader - Automatic wakeup timer clock selection"]
pub type CKSEL_R = crate::BitReader;
#[doc = "Field `CKSEL` writer - Automatic wakeup timer clock selection"]
pub type CKSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWUT_RLR` reader - Automatic wakeup timer reload register"]
pub type AWUT_RLR_R = crate::FieldReader<u32>;
#[doc = "Field `AWUT_RLR` writer - Automatic wakeup timer reload register"]
pub type AWUT_RLR_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
#[doc = "Field `RLR_WBUSY` reader - Reload register write busy"]
pub type RLR_WBUSY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Automatic wakeup timer clock selection"]
    #[inline(always)]
    pub fn cksel(&self) -> CKSEL_R {
        CKSEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:22 - Automatic wakeup timer reload register"]
    #[inline(always)]
    pub fn awut_rlr(&self) -> AWUT_RLR_R {
        AWUT_RLR_R::new((self.bits >> 1) & 0x003f_ffff)
    }
    #[doc = "Bit 31 - Reload register write busy"]
    #[inline(always)]
    pub fn rlr_wbusy(&self) -> RLR_WBUSY_R {
        RLR_WBUSY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Automatic wakeup timer clock selection"]
    #[inline(always)]
    #[must_use]
    pub fn cksel(&mut self) -> CKSEL_W<CRrs> {
        CKSEL_W::new(self, 0)
    }
    #[doc = "Bits 1:22 - Automatic wakeup timer reload register"]
    #[inline(always)]
    #[must_use]
    pub fn awut_rlr(&mut self) -> AWUT_RLR_W<CRrs> {
        AWUT_RLR_W::new(self, 1)
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
#[doc = "CR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CRrs {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CRrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}
