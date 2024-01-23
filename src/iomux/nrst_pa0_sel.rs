#[doc = "Register `NRST_PA0_SEL` reader"]
pub type R = crate::R<NRST_PA0_SELrs>;
#[doc = "Register `NRST_PA0_SEL` writer"]
pub type W = crate::W<NRST_PA0_SELrs>;
#[doc = "Field `NRST_PA0_SEL` reader - Pin Functional Selection for NRST/PA0"]
pub type NRST_PA0_SEL_R = crate::BitReader;
#[doc = "Field `NRST_PA0_SEL` writer - Pin Functional Selection for NRST/PA0"]
pub type NRST_PA0_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Pin Functional Selection for NRST/PA0"]
    #[inline(always)]
    pub fn nrst_pa0_sel(&self) -> NRST_PA0_SEL_R {
        NRST_PA0_SEL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pin Functional Selection for NRST/PA0"]
    #[inline(always)]
    #[must_use]
    pub fn nrst_pa0_sel(&mut self) -> NRST_PA0_SEL_W<NRST_PA0_SELrs> {
        NRST_PA0_SEL_W::new(self, 0)
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
#[doc = "NRST_PA0_SEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nrst_pa0_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nrst_pa0_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NRST_PA0_SELrs;
impl crate::RegisterSpec for NRST_PA0_SELrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nrst_pa0_sel::R`](R) reader structure"]
impl crate::Readable for NRST_PA0_SELrs {}
#[doc = "`write(|w| ..)` method takes [`nrst_pa0_sel::W`](W) writer structure"]
impl crate::Writable for NRST_PA0_SELrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NRST_PA0_SEL to value 0"]
impl crate::Resettable for NRST_PA0_SELrs {
    const RESET_VALUE: u32 = 0;
}
