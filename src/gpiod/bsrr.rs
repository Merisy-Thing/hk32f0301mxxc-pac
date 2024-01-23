#[doc = "Register `BSRR` writer"]
pub type W = crate::W<BSRRrs>;
#[doc = "Field `BS0` writer - Port x set bit y"]
pub type BS0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS1` writer - Port x set bit y"]
pub type BS1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS2` writer - Port x set bit y"]
pub type BS2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS3` writer - Port x set bit y"]
pub type BS3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS4` writer - Port x set bit y"]
pub type BS4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS5` writer - Port x set bit y"]
pub type BS5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS6` writer - Port x set bit y"]
pub type BS6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS7` writer - Port x set bit y"]
pub type BS7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR0` writer - Port x reset bit y"]
pub type BR0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR1` writer - Port x reset bit y"]
pub type BR1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR2` writer - Port x reset bit y"]
pub type BR2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR3` writer - Port x reset bit y"]
pub type BR3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR4` writer - Port x reset bit y"]
pub type BR4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR5` writer - Port x reset bit y"]
pub type BR5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR6` writer - Port x reset bit y"]
pub type BR6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR7` writer - Port x reset bit y"]
pub type BR7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Port x set bit y"]
    #[inline(always)]
    #[must_use]
    pub fn bs0(&mut self) -> BS0_W<BSRRrs> {
        BS0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Port x set bit y"]
    #[inline(always)]
    #[must_use]
    pub fn bs1(&mut self) -> BS1_W<BSRRrs> {
        BS1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Port x set bit y"]
    #[inline(always)]
    #[must_use]
    pub fn bs2(&mut self) -> BS2_W<BSRRrs> {
        BS2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Port x set bit y"]
    #[inline(always)]
    #[must_use]
    pub fn bs3(&mut self) -> BS3_W<BSRRrs> {
        BS3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Port x set bit y"]
    #[inline(always)]
    #[must_use]
    pub fn bs4(&mut self) -> BS4_W<BSRRrs> {
        BS4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Port x set bit y"]
    #[inline(always)]
    #[must_use]
    pub fn bs5(&mut self) -> BS5_W<BSRRrs> {
        BS5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Port x set bit y"]
    #[inline(always)]
    #[must_use]
    pub fn bs6(&mut self) -> BS6_W<BSRRrs> {
        BS6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Port x set bit y"]
    #[inline(always)]
    #[must_use]
    pub fn bs7(&mut self) -> BS7_W<BSRRrs> {
        BS7_W::new(self, 7)
    }
    #[doc = "Bit 16 - Port x reset bit y"]
    #[inline(always)]
    #[must_use]
    pub fn br0(&mut self) -> BR0_W<BSRRrs> {
        BR0_W::new(self, 16)
    }
    #[doc = "Bit 17 - Port x reset bit y"]
    #[inline(always)]
    #[must_use]
    pub fn br1(&mut self) -> BR1_W<BSRRrs> {
        BR1_W::new(self, 17)
    }
    #[doc = "Bit 18 - Port x reset bit y"]
    #[inline(always)]
    #[must_use]
    pub fn br2(&mut self) -> BR2_W<BSRRrs> {
        BR2_W::new(self, 18)
    }
    #[doc = "Bit 19 - Port x reset bit y"]
    #[inline(always)]
    #[must_use]
    pub fn br3(&mut self) -> BR3_W<BSRRrs> {
        BR3_W::new(self, 19)
    }
    #[doc = "Bit 20 - Port x reset bit y"]
    #[inline(always)]
    #[must_use]
    pub fn br4(&mut self) -> BR4_W<BSRRrs> {
        BR4_W::new(self, 20)
    }
    #[doc = "Bit 21 - Port x reset bit y"]
    #[inline(always)]
    #[must_use]
    pub fn br5(&mut self) -> BR5_W<BSRRrs> {
        BR5_W::new(self, 21)
    }
    #[doc = "Bit 22 - Port x reset bit y"]
    #[inline(always)]
    #[must_use]
    pub fn br6(&mut self) -> BR6_W<BSRRrs> {
        BR6_W::new(self, 22)
    }
    #[doc = "Bit 23 - Port x reset bit y"]
    #[inline(always)]
    #[must_use]
    pub fn br7(&mut self) -> BR7_W<BSRRrs> {
        BR7_W::new(self, 23)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "BSRR\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bsrr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BSRRrs;
impl crate::RegisterSpec for BSRRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`bsrr::W`](W) writer structure"]
impl crate::Writable for BSRRrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BSRR to value 0"]
impl crate::Resettable for BSRRrs {
    const RESET_VALUE: u32 = 0;
}
