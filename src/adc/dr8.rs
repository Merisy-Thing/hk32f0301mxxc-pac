#[doc = "Register `DR8` reader"]
pub type R = crate::R<DR8rs>;
#[doc = "Field `DRx` reader - Channel x latest converted data"]
pub type DRX_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Channel x latest converted data"]
    #[inline(always)]
    pub fn drx(&self) -> DRX_R {
        DRX_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "DR8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr8::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DR8rs;
impl crate::RegisterSpec for DR8rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr8::R`](R) reader structure"]
impl crate::Readable for DR8rs {}
#[doc = "`reset()` method sets DR8 to value 0"]
impl crate::Resettable for DR8rs {
    const RESET_VALUE: u32 = 0;
}
