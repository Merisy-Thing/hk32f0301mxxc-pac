#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    moder: MODER,
    otyper: OTYPER,
    ospeedr: OSPEEDR,
    pupdr: PUPDR,
    idr: IDR,
    odr: ODR,
    bsrr: BSRR,
    lckr: LCKR,
    afr: AFR,
    _reserved9: [u8; 0x04],
    brr: BRR,
    _reserved10: [u8; 0x04],
    iosr: IOSR,
}
impl RegisterBlock {
    #[doc = "0x00 - MODER"]
    #[inline(always)]
    pub const fn moder(&self) -> &MODER {
        &self.moder
    }
    #[doc = "0x04 - OTYPER"]
    #[inline(always)]
    pub const fn otyper(&self) -> &OTYPER {
        &self.otyper
    }
    #[doc = "0x08 - OSPEEDR"]
    #[inline(always)]
    pub const fn ospeedr(&self) -> &OSPEEDR {
        &self.ospeedr
    }
    #[doc = "0x0c - PUPDR"]
    #[inline(always)]
    pub const fn pupdr(&self) -> &PUPDR {
        &self.pupdr
    }
    #[doc = "0x10 - IDR"]
    #[inline(always)]
    pub const fn idr(&self) -> &IDR {
        &self.idr
    }
    #[doc = "0x14 - ODR"]
    #[inline(always)]
    pub const fn odr(&self) -> &ODR {
        &self.odr
    }
    #[doc = "0x18 - BSRR"]
    #[inline(always)]
    pub const fn bsrr(&self) -> &BSRR {
        &self.bsrr
    }
    #[doc = "0x1c - LCKR"]
    #[inline(always)]
    pub const fn lckr(&self) -> &LCKR {
        &self.lckr
    }
    #[doc = "0x20 - AFR"]
    #[inline(always)]
    pub const fn afr(&self) -> &AFR {
        &self.afr
    }
    #[doc = "0x28 - BRR"]
    #[inline(always)]
    pub const fn brr(&self) -> &BRR {
        &self.brr
    }
    #[doc = "0x30 - IOSR"]
    #[inline(always)]
    pub const fn iosr(&self) -> &IOSR {
        &self.iosr
    }
}
#[doc = "MODER (rw) register accessor: MODER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`moder::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`moder::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@moder`]
module"]
pub type MODER = crate::Reg<moder::MODERrs>;
#[doc = "MODER"]
pub mod moder;
#[doc = "OTYPER (rw) register accessor: OTYPER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otyper::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otyper::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otyper`]
module"]
pub type OTYPER = crate::Reg<otyper::OTYPERrs>;
#[doc = "OTYPER"]
pub mod otyper;
#[doc = "OSPEEDR (rw) register accessor: OSPEEDR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospeedr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospeedr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospeedr`]
module"]
pub type OSPEEDR = crate::Reg<ospeedr::OSPEEDRrs>;
#[doc = "OSPEEDR"]
pub mod ospeedr;
#[doc = "PUPDR (rw) register accessor: PUPDR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pupdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pupdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pupdr`]
module"]
pub type PUPDR = crate::Reg<pupdr::PUPDRrs>;
#[doc = "PUPDR"]
pub mod pupdr;
#[doc = "IDR (r) register accessor: IDR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr`]
module"]
pub type IDR = crate::Reg<idr::IDRrs>;
#[doc = "IDR"]
pub mod idr;
#[doc = "ODR (rw) register accessor: ODR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`odr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`odr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@odr`]
module"]
pub type ODR = crate::Reg<odr::ODRrs>;
#[doc = "ODR"]
pub mod odr;
#[doc = "BSRR (w) register accessor: BSRR\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bsrr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bsrr`]
module"]
pub type BSRR = crate::Reg<bsrr::BSRRrs>;
#[doc = "BSRR"]
pub mod bsrr;
#[doc = "LCKR (rw) register accessor: LCKR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lckr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lckr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lckr`]
module"]
pub type LCKR = crate::Reg<lckr::LCKRrs>;
#[doc = "LCKR"]
pub mod lckr;
#[doc = "AFR (rw) register accessor: AFR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afr`]
module"]
pub type AFR = crate::Reg<afr::AFRrs>;
#[doc = "AFR"]
pub mod afr;
#[doc = "BRR (w) register accessor: BRR\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brr`]
module"]
pub type BRR = crate::Reg<brr::BRRrs>;
#[doc = "BRR"]
pub mod brr;
#[doc = "IOSR (rw) register accessor: IOSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iosr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iosr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iosr`]
module"]
pub type IOSR = crate::Reg<iosr::IOSRrs>;
#[doc = "IOSR"]
pub mod iosr;
