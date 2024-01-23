#[doc = "Register `IDR` reader"]
pub type R = crate::R<IDRrs>;
#[doc = "Field `ID0` reader - Port x pin y input data"]
pub type ID0_R = crate::BitReader;
#[doc = "Field `ID1` reader - Port x pin y input data"]
pub type ID1_R = crate::BitReader;
#[doc = "Field `ID2` reader - Port x pin y input data"]
pub type ID2_R = crate::BitReader;
#[doc = "Field `ID3` reader - Port x pin y input data"]
pub type ID3_R = crate::BitReader;
#[doc = "Field `ID4` reader - Port x pin y input data"]
pub type ID4_R = crate::BitReader;
#[doc = "Field `ID5` reader - Port x pin y input data"]
pub type ID5_R = crate::BitReader;
#[doc = "Field `ID6` reader - Port x pin y input data"]
pub type ID6_R = crate::BitReader;
#[doc = "Field `ID7` reader - Port x pin y input data"]
pub type ID7_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Port x pin y input data"]
    #[inline(always)]
    pub fn id0(&self) -> ID0_R {
        ID0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port x pin y input data"]
    #[inline(always)]
    pub fn id1(&self) -> ID1_R {
        ID1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port x pin y input data"]
    #[inline(always)]
    pub fn id2(&self) -> ID2_R {
        ID2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port x pin y input data"]
    #[inline(always)]
    pub fn id3(&self) -> ID3_R {
        ID3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port x pin y input data"]
    #[inline(always)]
    pub fn id4(&self) -> ID4_R {
        ID4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port x pin y input data"]
    #[inline(always)]
    pub fn id5(&self) -> ID5_R {
        ID5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port x pin y input data"]
    #[inline(always)]
    pub fn id6(&self) -> ID6_R {
        ID6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port x pin y input data"]
    #[inline(always)]
    pub fn id7(&self) -> ID7_R {
        ID7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "IDR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDRrs;
impl crate::RegisterSpec for IDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idr::R`](R) reader structure"]
impl crate::Readable for IDRrs {}
#[doc = "`reset()` method sets IDR to value 0"]
impl crate::Resettable for IDRrs {
    const RESET_VALUE: u32 = 0;
}
