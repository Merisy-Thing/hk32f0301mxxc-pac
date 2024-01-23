#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    idcode: IDCODE,
    cr: CR,
    apb_fz: APB_FZ,
    _reserved3: [u8; 0x24],
    engr_idcode: ENGR_IDCODE,
}
impl RegisterBlock {
    #[doc = "0x00 - IDCODE"]
    #[inline(always)]
    pub const fn idcode(&self) -> &IDCODE {
        &self.idcode
    }
    #[doc = "0x04 - CR"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x08 - APB_FZ"]
    #[inline(always)]
    pub const fn apb_fz(&self) -> &APB_FZ {
        &self.apb_fz
    }
    #[doc = "0x30 - ENGR_IDCODE"]
    #[inline(always)]
    pub const fn engr_idcode(&self) -> &ENGR_IDCODE {
        &self.engr_idcode
    }
}
#[doc = "IDCODE (r) register accessor: IDCODE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idcode::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idcode`]
module"]
pub type IDCODE = crate::Reg<idcode::IDCODErs>;
#[doc = "IDCODE"]
pub mod idcode;
#[doc = "CR (rw) register accessor: CR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "CR"]
pub mod cr;
#[doc = "APB_FZ (rw) register accessor: APB_FZ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb_fz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb_fz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb_fz`]
module"]
pub type APB_FZ = crate::Reg<apb_fz::APB_FZrs>;
#[doc = "APB_FZ"]
pub mod apb_fz;
#[doc = "ENGR_IDCODE (r) register accessor: ENGR_IDCODE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`engr_idcode::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@engr_idcode`]
module"]
pub type ENGR_IDCODE = crate::Reg<engr_idcode::ENGR_IDCODErs>;
#[doc = "ENGR_IDCODE"]
pub mod engr_idcode;
