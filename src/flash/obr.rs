#[doc = "Register `OBR` reader"]
pub type R = crate::R<OBRrs>;
#[doc = "Field `OPTERR` reader - Option byte error"]
pub type OPTERR_R = crate::BitReader;
#[doc = "Field `RDPRT` reader - Read protection level status"]
pub type RDPRT_R = crate::FieldReader;
#[doc = "Field `WDG_SW` reader - Select a software or hardware watchdog"]
pub type WDG_SW_R = crate::BitReader;
#[doc = "Field `nRST_STOP` reader - Enter the stop mode for a reset"]
pub type N_RST_STOP_R = crate::BitReader;
#[doc = "Field `DATA0` reader - User data"]
pub type DATA0_R = crate::FieldReader;
#[doc = "Field `DATA1` reader - User data"]
pub type DATA1_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Option byte error"]
    #[inline(always)]
    pub fn opterr(&self) -> OPTERR_R {
        OPTERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Read protection level status"]
    #[inline(always)]
    pub fn rdprt(&self) -> RDPRT_R {
        RDPRT_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 8 - Select a software or hardware watchdog"]
    #[inline(always)]
    pub fn wdg_sw(&self) -> WDG_SW_R {
        WDG_SW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enter the stop mode for a reset"]
    #[inline(always)]
    pub fn n_rst_stop(&self) -> N_RST_STOP_R {
        N_RST_STOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:23 - User data"]
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - User data"]
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "OBR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`obr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OBRrs;
impl crate::RegisterSpec for OBRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`obr::R`](R) reader structure"]
impl crate::Readable for OBRrs {}
#[doc = "`reset()` method sets OBR to value 0"]
impl crate::Resettable for OBRrs {
    const RESET_VALUE: u32 = 0;
}
