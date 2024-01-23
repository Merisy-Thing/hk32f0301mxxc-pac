#[doc = "Register `ENGR_IDCODE` reader"]
pub type R = crate::R<ENGR_IDCODErs>;
#[doc = "Field `ENG_ID` reader - Chip identifier"]
pub type ENG_ID_R = crate::FieldReader<u16>;
#[doc = "Field `DEV_ID2` reader - Device identifier 2"]
pub type DEV_ID2_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Chip identifier"]
    #[inline(always)]
    pub fn eng_id(&self) -> ENG_ID_R {
        ENG_ID_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:31 - Device identifier 2"]
    #[inline(always)]
    pub fn dev_id2(&self) -> DEV_ID2_R {
        DEV_ID2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "ENGR_IDCODE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`engr_idcode::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENGR_IDCODErs;
impl crate::RegisterSpec for ENGR_IDCODErs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`engr_idcode::R`](R) reader structure"]
impl crate::Readable for ENGR_IDCODErs {}
#[doc = "`reset()` method sets ENGR_IDCODE to value 0x0521"]
impl crate::Resettable for ENGR_IDCODErs {
    const RESET_VALUE: u32 = 0x0521;
}
