#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    isr: ISR,
    ier: IER,
    cr: CR,
    cfgr1: CFGR1,
    cfgr2: CFGR2,
    smpr: SMPR,
    _reserved6: [u8; 0x08],
    tr: TR,
    _reserved7: [u8; 0x04],
    chselr: CHSELR,
    _reserved8: [u8; 0x14],
    dr: DR,
    _reserved9: [u8; 0x02c4],
    ccr: CCR,
    _reserved10: [u8; 0xb4],
    dr0: DR0,
    dr1: DR1,
    dr2: DR2,
    dr3: DR3,
    dr4: DR4,
    dr5: DR5,
    dr6: DR6,
    dr7: DR7,
    dr8: DR8,
    _reserved19: [u8; 0x0c],
    cr2: CR2,
    offset: OFFSET,
    gain: GAIN,
}
impl RegisterBlock {
    #[doc = "0x00 - ISR"]
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    #[doc = "0x04 - IER"]
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    #[doc = "0x08 - CR"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x0c - CFGR1"]
    #[inline(always)]
    pub const fn cfgr1(&self) -> &CFGR1 {
        &self.cfgr1
    }
    #[doc = "0x10 - CFGR2"]
    #[inline(always)]
    pub const fn cfgr2(&self) -> &CFGR2 {
        &self.cfgr2
    }
    #[doc = "0x14 - SMPR"]
    #[inline(always)]
    pub const fn smpr(&self) -> &SMPR {
        &self.smpr
    }
    #[doc = "0x20 - TR"]
    #[inline(always)]
    pub const fn tr(&self) -> &TR {
        &self.tr
    }
    #[doc = "0x28 - CHSELR"]
    #[inline(always)]
    pub const fn chselr(&self) -> &CHSELR {
        &self.chselr
    }
    #[doc = "0x40 - DR"]
    #[inline(always)]
    pub const fn dr(&self) -> &DR {
        &self.dr
    }
    #[doc = "0x308 - CCR"]
    #[inline(always)]
    pub const fn ccr(&self) -> &CCR {
        &self.ccr
    }
    #[doc = "0x3c0 - DR0"]
    #[inline(always)]
    pub const fn dr0(&self) -> &DR0 {
        &self.dr0
    }
    #[doc = "0x3c4 - DR1"]
    #[inline(always)]
    pub const fn dr1(&self) -> &DR1 {
        &self.dr1
    }
    #[doc = "0x3c8 - DR2"]
    #[inline(always)]
    pub const fn dr2(&self) -> &DR2 {
        &self.dr2
    }
    #[doc = "0x3cc - DR3"]
    #[inline(always)]
    pub const fn dr3(&self) -> &DR3 {
        &self.dr3
    }
    #[doc = "0x3d0 - DR4"]
    #[inline(always)]
    pub const fn dr4(&self) -> &DR4 {
        &self.dr4
    }
    #[doc = "0x3d4 - DR5"]
    #[inline(always)]
    pub const fn dr5(&self) -> &DR5 {
        &self.dr5
    }
    #[doc = "0x3d8 - DR6"]
    #[inline(always)]
    pub const fn dr6(&self) -> &DR6 {
        &self.dr6
    }
    #[doc = "0x3dc - DR7"]
    #[inline(always)]
    pub const fn dr7(&self) -> &DR7 {
        &self.dr7
    }
    #[doc = "0x3e0 - DR8"]
    #[inline(always)]
    pub const fn dr8(&self) -> &DR8 {
        &self.dr8
    }
    #[doc = "0x3f0 - CR2"]
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    #[doc = "0x3f4 - OFFSET"]
    #[inline(always)]
    pub const fn offset(&self) -> &OFFSET {
        &self.offset
    }
    #[doc = "0x3f8 - GAIN"]
    #[inline(always)]
    pub const fn gain(&self) -> &GAIN {
        &self.gain
    }
}
#[doc = "ISR (rw) register accessor: ISR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
pub type ISR = crate::Reg<isr::ISRrs>;
#[doc = "ISR"]
pub mod isr;
#[doc = "IER (rw) register accessor: IER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
pub type IER = crate::Reg<ier::IERrs>;
#[doc = "IER"]
pub mod ier;
#[doc = "CR (rw) register accessor: CR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "CR"]
pub mod cr;
#[doc = "CFGR1 (rw) register accessor: CFGR1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr1`]
module"]
pub type CFGR1 = crate::Reg<cfgr1::CFGR1rs>;
#[doc = "CFGR1"]
pub mod cfgr1;
#[doc = "CFGR2 (rw) register accessor: CFGR2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr2`]
module"]
pub type CFGR2 = crate::Reg<cfgr2::CFGR2rs>;
#[doc = "CFGR2"]
pub mod cfgr2;
#[doc = "SMPR (rw) register accessor: SMPR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smpr`]
module"]
pub type SMPR = crate::Reg<smpr::SMPRrs>;
#[doc = "SMPR"]
pub mod smpr;
#[doc = "TR (rw) register accessor: TR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr`]
module"]
pub type TR = crate::Reg<tr::TRrs>;
#[doc = "TR"]
pub mod tr;
#[doc = "CHSELR (rw) register accessor: CHSELR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chselr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chselr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chselr`]
module"]
pub type CHSELR = crate::Reg<chselr::CHSELRrs>;
#[doc = "CHSELR"]
pub mod chselr;
#[doc = "DR (r) register accessor: DR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`]
module"]
pub type DR = crate::Reg<dr::DRrs>;
#[doc = "DR"]
pub mod dr;
#[doc = "DR0 (r) register accessor: DR0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr0`]
module"]
pub type DR0 = crate::Reg<dr0::DR0rs>;
#[doc = "DR0"]
pub mod dr0;
#[doc = "DR1 (r) register accessor: DR1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr1`]
module"]
pub type DR1 = crate::Reg<dr1::DR1rs>;
#[doc = "DR1"]
pub mod dr1;
#[doc = "DR2 (r) register accessor: DR2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr2`]
module"]
pub type DR2 = crate::Reg<dr2::DR2rs>;
#[doc = "DR2"]
pub mod dr2;
#[doc = "DR3 (r) register accessor: DR3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr3`]
module"]
pub type DR3 = crate::Reg<dr3::DR3rs>;
#[doc = "DR3"]
pub mod dr3;
#[doc = "DR4 (r) register accessor: DR4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr4`]
module"]
pub type DR4 = crate::Reg<dr4::DR4rs>;
#[doc = "DR4"]
pub mod dr4;
#[doc = "DR5 (r) register accessor: DR5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr5`]
module"]
pub type DR5 = crate::Reg<dr5::DR5rs>;
#[doc = "DR5"]
pub mod dr5;
#[doc = "DR6 (r) register accessor: DR6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr6`]
module"]
pub type DR6 = crate::Reg<dr6::DR6rs>;
#[doc = "DR6"]
pub mod dr6;
#[doc = "DR7 (r) register accessor: DR7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr7`]
module"]
pub type DR7 = crate::Reg<dr7::DR7rs>;
#[doc = "DR7"]
pub mod dr7;
#[doc = "DR8 (r) register accessor: DR8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr8::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr8`]
module"]
pub type DR8 = crate::Reg<dr8::DR8rs>;
#[doc = "DR8"]
pub mod dr8;
#[doc = "CCR (rw) register accessor: CCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`]
module"]
pub type CCR = crate::Reg<ccr::CCRrs>;
#[doc = "CCR"]
pub mod ccr;
#[doc = "CR2 (rw) register accessor: CR2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`]
module"]
pub type CR2 = crate::Reg<cr2::CR2rs>;
#[doc = "CR2"]
pub mod cr2;
#[doc = "OFFSET (rw) register accessor: OFFSET\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`offset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`offset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@offset`]
module"]
pub type OFFSET = crate::Reg<offset::OFFSETrs>;
#[doc = "OFFSET"]
pub mod offset;
#[doc = "GAIN (rw) register accessor: GAIN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gain::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gain::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gain`]
module"]
pub type GAIN = crate::Reg<gain::GAINrs>;
#[doc = "GAIN"]
pub mod gain;
