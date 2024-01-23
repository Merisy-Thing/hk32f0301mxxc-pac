#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    kr: KR,
    pr: PR,
    rlr: RLR,
    sr: SR,
    winr: WINR,
}
impl RegisterBlock {
    #[doc = "0x00 - KR"]
    #[inline(always)]
    pub const fn kr(&self) -> &KR {
        &self.kr
    }
    #[doc = "0x04 - PR"]
    #[inline(always)]
    pub const fn pr(&self) -> &PR {
        &self.pr
    }
    #[doc = "0x08 - RLR"]
    #[inline(always)]
    pub const fn rlr(&self) -> &RLR {
        &self.rlr
    }
    #[doc = "0x0c - SR"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x10 - WINR"]
    #[inline(always)]
    pub const fn winr(&self) -> &WINR {
        &self.winr
    }
}
#[doc = "KR (w) register accessor: KR\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`kr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@kr`]
module"]
pub type KR = crate::Reg<kr::KRrs>;
#[doc = "KR"]
pub mod kr;
#[doc = "PR (rw) register accessor: PR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr`]
module"]
pub type PR = crate::Reg<pr::PRrs>;
#[doc = "PR"]
pub mod pr;
#[doc = "RLR (rw) register accessor: RLR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rlr`]
module"]
pub type RLR = crate::Reg<rlr::RLRrs>;
#[doc = "RLR"]
pub mod rlr;
#[doc = "SR (r) register accessor: SR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SRrs>;
#[doc = "SR"]
pub mod sr;
#[doc = "WINR (rw) register accessor: WINR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`winr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`winr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@winr`]
module"]
pub type WINR = crate::Reg<winr::WINRrs>;
#[doc = "WINR"]
pub mod winr;
