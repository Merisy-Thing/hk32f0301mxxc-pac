#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    cr: CR,
    cfgr: CFGR,
    cir: CIR,
    apbrstr2: APBRSTR2,
    apbrstr1: APBRSTR1,
    ahbenr: AHBENR,
    apbenr2: APBENR2,
    apbenr1: APBENR1,
    _reserved8: [u8; 0x04],
    csr: CSR,
    ahbrstr: AHBRSTR,
    _reserved10: [u8; 0x04],
    cfgr3: CFGR3,
    _reserved11: [u8; 0xac],
    css: CSS,
    _reserved12: [u8; 0x04],
    cfgr4: CFGR4,
}
impl RegisterBlock {
    #[doc = "0x00 - CR"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x04 - CFGR"]
    #[inline(always)]
    pub const fn cfgr(&self) -> &CFGR {
        &self.cfgr
    }
    #[doc = "0x08 - CIR"]
    #[inline(always)]
    pub const fn cir(&self) -> &CIR {
        &self.cir
    }
    #[doc = "0x0c - APBRSTR2"]
    #[inline(always)]
    pub const fn apbrstr2(&self) -> &APBRSTR2 {
        &self.apbrstr2
    }
    #[doc = "0x10 - APBRSTR1"]
    #[inline(always)]
    pub const fn apbrstr1(&self) -> &APBRSTR1 {
        &self.apbrstr1
    }
    #[doc = "0x14 - AHBENR"]
    #[inline(always)]
    pub const fn ahbenr(&self) -> &AHBENR {
        &self.ahbenr
    }
    #[doc = "0x18 - APBENR2"]
    #[inline(always)]
    pub const fn apbenr2(&self) -> &APBENR2 {
        &self.apbenr2
    }
    #[doc = "0x1c - APBENR1"]
    #[inline(always)]
    pub const fn apbenr1(&self) -> &APBENR1 {
        &self.apbenr1
    }
    #[doc = "0x24 - CSR"]
    #[inline(always)]
    pub const fn csr(&self) -> &CSR {
        &self.csr
    }
    #[doc = "0x28 - AHBRSTR"]
    #[inline(always)]
    pub const fn ahbrstr(&self) -> &AHBRSTR {
        &self.ahbrstr
    }
    #[doc = "0x30 - CFGR3"]
    #[inline(always)]
    pub const fn cfgr3(&self) -> &CFGR3 {
        &self.cfgr3
    }
    #[doc = "0xe0 - CSS"]
    #[inline(always)]
    pub const fn css(&self) -> &CSS {
        &self.css
    }
    #[doc = "0xe8 - CFGR4"]
    #[inline(always)]
    pub const fn cfgr4(&self) -> &CFGR4 {
        &self.cfgr4
    }
}
#[doc = "CR (rw) register accessor: CR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "CR"]
pub mod cr;
#[doc = "CFGR (rw) register accessor: CFGR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr`]
module"]
pub type CFGR = crate::Reg<cfgr::CFGRrs>;
#[doc = "CFGR"]
pub mod cfgr;
#[doc = "CIR (rw) register accessor: CIR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cir`]
module"]
pub type CIR = crate::Reg<cir::CIRrs>;
#[doc = "CIR"]
pub mod cir;
#[doc = "APBRSTR2 (rw) register accessor: APBRSTR2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbrstr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbrstr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbrstr2`]
module"]
pub type APBRSTR2 = crate::Reg<apbrstr2::APBRSTR2rs>;
#[doc = "APBRSTR2"]
pub mod apbrstr2;
#[doc = "APBRSTR1 (rw) register accessor: APBRSTR1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbrstr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbrstr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbrstr1`]
module"]
pub type APBRSTR1 = crate::Reg<apbrstr1::APBRSTR1rs>;
#[doc = "APBRSTR1"]
pub mod apbrstr1;
#[doc = "AHBENR (rw) register accessor: AHBENR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbenr`]
module"]
pub type AHBENR = crate::Reg<ahbenr::AHBENRrs>;
#[doc = "AHBENR"]
pub mod ahbenr;
#[doc = "APBENR2 (rw) register accessor: APBENR2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbenr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbenr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbenr2`]
module"]
pub type APBENR2 = crate::Reg<apbenr2::APBENR2rs>;
#[doc = "APBENR2"]
pub mod apbenr2;
#[doc = "APBENR1 (rw) register accessor: APBENR1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbenr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbenr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbenr1`]
module"]
pub type APBENR1 = crate::Reg<apbenr1::APBENR1rs>;
#[doc = "APBENR1"]
pub mod apbenr1;
#[doc = "CSR (rw) register accessor: CSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`]
module"]
pub type CSR = crate::Reg<csr::CSRrs>;
#[doc = "CSR"]
pub mod csr;
#[doc = "AHBRSTR (rw) register accessor: AHBRSTR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbrstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbrstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbrstr`]
module"]
pub type AHBRSTR = crate::Reg<ahbrstr::AHBRSTRrs>;
#[doc = "AHBRSTR"]
pub mod ahbrstr;
#[doc = "CFGR3 (rw) register accessor: CFGR3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr3`]
module"]
pub type CFGR3 = crate::Reg<cfgr3::CFGR3rs>;
#[doc = "CFGR3"]
pub mod cfgr3;
#[doc = "CSS (rw) register accessor: CSS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`css::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`css::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@css`]
module"]
pub type CSS = crate::Reg<css::CSSrs>;
#[doc = "CSS"]
pub mod css;
#[doc = "CFGR4 (rw) register accessor: CFGR4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr4`]
module"]
pub type CFGR4 = crate::Reg<cfgr4::CFGR4rs>;
#[doc = "CFGR4"]
pub mod cfgr4;
