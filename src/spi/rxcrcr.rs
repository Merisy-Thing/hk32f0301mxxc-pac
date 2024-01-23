#[doc = "Register `RXCRCR` reader"]
pub type R = crate::R<RXCRCRrs>;
#[doc = "Field `RXCRC` reader - RX CRC register"]
pub type RXCRC_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - RX CRC register"]
    #[inline(always)]
    pub fn rxcrc(&self) -> RXCRC_R {
        RXCRC_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "RXCRCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxcrcr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXCRCRrs;
impl crate::RegisterSpec for RXCRCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxcrcr::R`](R) reader structure"]
impl crate::Readable for RXCRCRrs {}
#[doc = "`reset()` method sets RXCRCR to value 0"]
impl crate::Resettable for RXCRCRrs {
    const RESET_VALUE: u32 = 0;
}
