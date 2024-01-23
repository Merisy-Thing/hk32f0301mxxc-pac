#[doc = "Register `DR7` reader"]
pub type R = crate::R<DR7rs>;
#[doc = "Field `DRx` reader - Channel x latest converted data"]
pub type DRX_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Channel x latest converted data"]
    #[inline(always)]
    pub fn drx(&self) -> DRX_R {
        DRX_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "DR7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr7::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DR7rs;
impl crate::RegisterSpec for DR7rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr7::R`](R) reader structure"]
impl crate::Readable for DR7rs {}
#[doc = "`reset()` method sets DR7 to value 0"]
impl crate::Resettable for DR7rs {
    const RESET_VALUE: u32 = 0;
}
