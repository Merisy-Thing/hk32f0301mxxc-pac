#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    acr: ACR,
    keyr: KEYR,
    optkeyr: OPTKEYR,
    sr: SR,
    cr: CR,
    ar: AR,
    _reserved6: [u8; 0x04],
    obr: OBR,
    wrpr: WRPR,
    _reserved8: [u8; 0x50],
    int_vec_offset: INT_VEC_OFFSET,
}
impl RegisterBlock {
    #[doc = "0x00 - ACR"]
    #[inline(always)]
    pub const fn acr(&self) -> &ACR {
        &self.acr
    }
    #[doc = "0x04 - KEYR"]
    #[inline(always)]
    pub const fn keyr(&self) -> &KEYR {
        &self.keyr
    }
    #[doc = "0x08 - OPTKEYR"]
    #[inline(always)]
    pub const fn optkeyr(&self) -> &OPTKEYR {
        &self.optkeyr
    }
    #[doc = "0x0c - SR"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x10 - CR"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x14 - AR"]
    #[inline(always)]
    pub const fn ar(&self) -> &AR {
        &self.ar
    }
    #[doc = "0x1c - OBR"]
    #[inline(always)]
    pub const fn obr(&self) -> &OBR {
        &self.obr
    }
    #[doc = "0x20 - WRPR"]
    #[inline(always)]
    pub const fn wrpr(&self) -> &WRPR {
        &self.wrpr
    }
    #[doc = "0x74 - INT_VEC_OFFSET"]
    #[inline(always)]
    pub const fn int_vec_offset(&self) -> &INT_VEC_OFFSET {
        &self.int_vec_offset
    }
}
#[doc = "ACR (rw) register accessor: ACR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acr`]
module"]
pub type ACR = crate::Reg<acr::ACRrs>;
#[doc = "ACR"]
pub mod acr;
#[doc = "KEYR (w) register accessor: KEYR\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyr`]
module"]
pub type KEYR = crate::Reg<keyr::KEYRrs>;
#[doc = "KEYR"]
pub mod keyr;
#[doc = "OPTKEYR (w) register accessor: OPTKEYR\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`optkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@optkeyr`]
module"]
pub type OPTKEYR = crate::Reg<optkeyr::OPTKEYRrs>;
#[doc = "OPTKEYR"]
pub mod optkeyr;
#[doc = "SR (rw) register accessor: SR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SRrs>;
#[doc = "SR"]
pub mod sr;
#[doc = "CR (rw) register accessor: CR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "CR"]
pub mod cr;
#[doc = "AR (w) register accessor: AR\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ar::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ar`]
module"]
pub type AR = crate::Reg<ar::ARrs>;
#[doc = "AR"]
pub mod ar;
#[doc = "OBR (r) register accessor: OBR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`obr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obr`]
module"]
pub type OBR = crate::Reg<obr::OBRrs>;
#[doc = "OBR"]
pub mod obr;
#[doc = "WRPR (rw) register accessor: WRPR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wrpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrpr`]
module"]
pub type WRPR = crate::Reg<wrpr::WRPRrs>;
#[doc = "WRPR"]
pub mod wrpr;
#[doc = "INT_VEC_OFFSET (rw) register accessor: INT_VEC_OFFSET\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_vec_offset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_vec_offset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_vec_offset`]
module"]
pub type INT_VEC_OFFSET = crate::Reg<int_vec_offset::INT_VEC_OFFSETrs>;
#[doc = "INT_VEC_OFFSET"]
pub mod int_vec_offset;
