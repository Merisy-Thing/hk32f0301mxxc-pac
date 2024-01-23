#[doc = "Register `CR2` reader"]
pub type R = crate::R<CR2rs>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<CR2rs>;
#[doc = "Field `GCMP` reader - ADC internal delay control"]
pub type GCMP_R = crate::BitReader;
#[doc = "Field `GCMP` writer - ADC internal delay control"]
pub type GCMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIF` reader - Differential input enable"]
pub type SDIF_R = crate::BitReader;
#[doc = "Field `SDIF` writer - Differential input enable"]
pub type SDIF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADC internal delay control"]
    #[inline(always)]
    pub fn gcmp(&self) -> GCMP_R {
        GCMP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Differential input enable"]
    #[inline(always)]
    pub fn sdif(&self) -> SDIF_R {
        SDIF_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC internal delay control"]
    #[inline(always)]
    #[must_use]
    pub fn gcmp(&mut self) -> GCMP_W<CR2rs> {
        GCMP_W::new(self, 0)
    }
    #[doc = "Bit 1 - Differential input enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdif(&mut self) -> SDIF_W<CR2rs> {
        SDIF_W::new(self, 1)
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
#[doc = "CR2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for CR2rs {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for CR2rs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2rs {
    const RESET_VALUE: u32 = 0;
}
