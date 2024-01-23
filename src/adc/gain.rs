#[doc = "Register `GAIN` reader"]
pub type R = crate::R<GAINrs>;
#[doc = "Register `GAIN` writer"]
pub type W = crate::W<GAINrs>;
#[doc = "Field `GAIN_SEL1` reader - Gain calibration factor 1 selection"]
pub type GAIN_SEL1_R = crate::FieldReader;
#[doc = "Field `GAIN_SEL1` writer - Gain calibration factor 1 selection"]
pub type GAIN_SEL1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GAIN_SEL2` reader - Gain calibration factor 2 selection"]
pub type GAIN_SEL2_R = crate::FieldReader;
#[doc = "Field `GAIN_SEL2` writer - Gain calibration factor 2 selection"]
pub type GAIN_SEL2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Gain calibration factor 1 selection"]
    #[inline(always)]
    pub fn gain_sel1(&self) -> GAIN_SEL1_R {
        GAIN_SEL1_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Gain calibration factor 2 selection"]
    #[inline(always)]
    pub fn gain_sel2(&self) -> GAIN_SEL2_R {
        GAIN_SEL2_R::new(((self.bits >> 3) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Gain calibration factor 1 selection"]
    #[inline(always)]
    #[must_use]
    pub fn gain_sel1(&mut self) -> GAIN_SEL1_W<GAINrs> {
        GAIN_SEL1_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - Gain calibration factor 2 selection"]
    #[inline(always)]
    #[must_use]
    pub fn gain_sel2(&mut self) -> GAIN_SEL2_W<GAINrs> {
        GAIN_SEL2_W::new(self, 3)
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
#[doc = "GAIN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gain::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gain::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GAINrs;
impl crate::RegisterSpec for GAINrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gain::R`](R) reader structure"]
impl crate::Readable for GAINrs {}
#[doc = "`write(|w| ..)` method takes [`gain::W`](W) writer structure"]
impl crate::Writable for GAINrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GAIN to value 0"]
impl crate::Resettable for GAINrs {
    const RESET_VALUE: u32 = 0;
}
