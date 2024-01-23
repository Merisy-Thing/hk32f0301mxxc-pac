#[doc = "Register `INT_VEC_OFFSET` reader"]
pub type R = crate::R<INT_VEC_OFFSETrs>;
#[doc = "Register `INT_VEC_OFFSET` writer"]
pub type W = crate::W<INT_VEC_OFFSETrs>;
#[doc = "Field `INT_VEC_OFFSET` reader - Interrupt Vector table address Offset"]
pub type INT_VEC_OFFSET_R = crate::FieldReader<u16>;
#[doc = "Field `INT_VEC_OFFSET` writer - Interrupt Vector table address Offset"]
pub type INT_VEC_OFFSET_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - Interrupt Vector table address Offset"]
    #[inline(always)]
    pub fn int_vec_offset(&self) -> INT_VEC_OFFSET_R {
        INT_VEC_OFFSET_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Interrupt Vector table address Offset"]
    #[inline(always)]
    #[must_use]
    pub fn int_vec_offset(&mut self) -> INT_VEC_OFFSET_W<INT_VEC_OFFSETrs> {
        INT_VEC_OFFSET_W::new(self, 0)
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
#[doc = "INT_VEC_OFFSET\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_vec_offset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_vec_offset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_VEC_OFFSETrs;
impl crate::RegisterSpec for INT_VEC_OFFSETrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_vec_offset::R`](R) reader structure"]
impl crate::Readable for INT_VEC_OFFSETrs {}
#[doc = "`write(|w| ..)` method takes [`int_vec_offset::W`](W) writer structure"]
impl crate::Writable for INT_VEC_OFFSETrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_VEC_OFFSET to value 0x04"]
impl crate::Resettable for INT_VEC_OFFSETrs {
    const RESET_VALUE: u32 = 0x04;
}
