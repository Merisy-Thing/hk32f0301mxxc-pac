#[doc = "Register `TXCRCR` reader"]
pub type R = crate::R<TXCRCRrs>;
#[doc = "Field `TXCRC` reader - TX CRC register"]
pub type TXCRC_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - TX CRC register"]
    #[inline(always)]
    pub fn txcrc(&self) -> TXCRC_R {
        TXCRC_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "TXCRCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txcrcr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXCRCRrs;
impl crate::RegisterSpec for TXCRCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txcrcr::R`](R) reader structure"]
impl crate::Readable for TXCRCRrs {}
#[doc = "`reset()` method sets TXCRCR to value 0"]
impl crate::Resettable for TXCRCRrs {
    const RESET_VALUE: u32 = 0;
}
