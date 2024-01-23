#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    pin_func_sel: PIN_FUNC_SEL,
    pkg_pin_sel: PKG_PIN_SEL,
    nrst_pin_key: NRST_PIN_KEY,
    nrst_pa0_sel: NRST_PA0_SEL,
}
impl RegisterBlock {
    #[doc = "0x00 - PIN_FUNC_SEL"]
    #[inline(always)]
    pub const fn pin_func_sel(&self) -> &PIN_FUNC_SEL {
        &self.pin_func_sel
    }
    #[doc = "0x04 - PKG_PIN_SEL"]
    #[inline(always)]
    pub const fn pkg_pin_sel(&self) -> &PKG_PIN_SEL {
        &self.pkg_pin_sel
    }
    #[doc = "0x08 - NRST_PIN_KEY"]
    #[inline(always)]
    pub const fn nrst_pin_key(&self) -> &NRST_PIN_KEY {
        &self.nrst_pin_key
    }
    #[doc = "0x0c - NRST_PA0_SEL"]
    #[inline(always)]
    pub const fn nrst_pa0_sel(&self) -> &NRST_PA0_SEL {
        &self.nrst_pa0_sel
    }
}
#[doc = "PIN_FUNC_SEL (rw) register accessor: PIN_FUNC_SEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin_func_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin_func_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pin_func_sel`]
module"]
pub type PIN_FUNC_SEL = crate::Reg<pin_func_sel::PIN_FUNC_SELrs>;
#[doc = "PIN_FUNC_SEL"]
pub mod pin_func_sel;
#[doc = "PKG_PIN_SEL (rw) register accessor: PKG_PIN_SEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pkg_pin_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pkg_pin_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkg_pin_sel`]
module"]
pub type PKG_PIN_SEL = crate::Reg<pkg_pin_sel::PKG_PIN_SELrs>;
#[doc = "PKG_PIN_SEL"]
pub mod pkg_pin_sel;
#[doc = "NRST_PIN_KEY (rw) register accessor: NRST_PIN_KEY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nrst_pin_key::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nrst_pin_key::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nrst_pin_key`]
module"]
pub type NRST_PIN_KEY = crate::Reg<nrst_pin_key::NRST_PIN_KEYrs>;
#[doc = "NRST_PIN_KEY"]
pub mod nrst_pin_key;
#[doc = "NRST_PA0_SEL (rw) register accessor: NRST_PA0_SEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nrst_pa0_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nrst_pa0_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nrst_pa0_sel`]
module"]
pub type NRST_PA0_SEL = crate::Reg<nrst_pa0_sel::NRST_PA0_SELrs>;
#[doc = "NRST_PA0_SEL"]
pub mod nrst_pa0_sel;
