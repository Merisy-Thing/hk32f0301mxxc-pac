#[doc = "Register `OFFSET` reader"]
pub type R = crate::R<OFFSETrs>;
#[doc = "Register `OFFSET` writer"]
pub type W = crate::W<OFFSETrs>;
#[doc = "Field `CAL_OFFSET` reader - ADC Calibration Offset data"]
pub type CAL_OFFSET_R = crate::FieldReader;
#[doc = "Field `CAL_OFFSET` writer - ADC Calibration Offset data"]
pub type CAL_OFFSET_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `OFFSET_EN` reader - Calibration Offset enable"]
pub type OFFSET_EN_R = crate::BitReader;
#[doc = "Field `OFFSET_EN` writer - Calibration Offset enable"]
pub type OFFSET_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - ADC Calibration Offset data"]
    #[inline(always)]
    pub fn cal_offset(&self) -> CAL_OFFSET_R {
        CAL_OFFSET_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - Calibration Offset enable"]
    #[inline(always)]
    pub fn offset_en(&self) -> OFFSET_EN_R {
        OFFSET_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - ADC Calibration Offset data"]
    #[inline(always)]
    #[must_use]
    pub fn cal_offset(&mut self) -> CAL_OFFSET_W<OFFSETrs> {
        CAL_OFFSET_W::new(self, 0)
    }
    #[doc = "Bit 6 - Calibration Offset enable"]
    #[inline(always)]
    #[must_use]
    pub fn offset_en(&mut self) -> OFFSET_EN_W<OFFSETrs> {
        OFFSET_EN_W::new(self, 6)
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
#[doc = "OFFSET\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`offset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`offset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OFFSETrs;
impl crate::RegisterSpec for OFFSETrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`offset::R`](R) reader structure"]
impl crate::Readable for OFFSETrs {}
#[doc = "`write(|w| ..)` method takes [`offset::W`](W) writer structure"]
impl crate::Writable for OFFSETrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OFFSET to value 0"]
impl crate::Resettable for OFFSETrs {
    const RESET_VALUE: u32 = 0;
}
