#[doc = "Register `PKG_PIN_SEL` reader"]
pub type R = crate::R<PKG_PIN_SELrs>;
#[doc = "Register `PKG_PIN_SEL` writer"]
pub type W = crate::W<PKG_PIN_SELrs>;
#[doc = "Field `NRSTPA0_PIN_SEL` reader - Pin MUX for NRST"]
pub type NRSTPA0_PIN_SEL_R = crate::BitReader;
#[doc = "Field `NRSTPA0_PIN_SEL` writer - Pin MUX for NRST"]
pub type NRSTPA0_PIN_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PB5_PIN_SEL` reader - Pin MUX for PB5"]
pub type PB5_PIN_SEL_R = crate::FieldReader;
#[doc = "Field `PB5_PIN_SEL` writer - Pin MUX for PB5"]
pub type PB5_PIN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PC4_PIN_SEL` reader - Pin MUX for PC4"]
pub type PC4_PIN_SEL_R = crate::FieldReader;
#[doc = "Field `PC4_PIN_SEL` writer - Pin MUX for PC4"]
pub type PC4_PIN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PD5_PIN_SEL` reader - Pin MUX for PD5"]
pub type PD5_PIN_SEL_R = crate::FieldReader;
#[doc = "Field `PD5_PIN_SEL` writer - Pin MUX for PD5"]
pub type PD5_PIN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PD6_PIN_SEL` reader - Pin MUX for PD6"]
pub type PD6_PIN_SEL_R = crate::FieldReader;
#[doc = "Field `PD6_PIN_SEL` writer - Pin MUX for PD6"]
pub type PD6_PIN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Pin MUX for NRST"]
    #[inline(always)]
    pub fn nrstpa0_pin_sel(&self) -> NRSTPA0_PIN_SEL_R {
        NRSTPA0_PIN_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Pin MUX for PB5"]
    #[inline(always)]
    pub fn pb5_pin_sel(&self) -> PB5_PIN_SEL_R {
        PB5_PIN_SEL_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - Pin MUX for PC4"]
    #[inline(always)]
    pub fn pc4_pin_sel(&self) -> PC4_PIN_SEL_R {
        PC4_PIN_SEL_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - Pin MUX for PD5"]
    #[inline(always)]
    pub fn pd5_pin_sel(&self) -> PD5_PIN_SEL_R {
        PD5_PIN_SEL_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 7:8 - Pin MUX for PD6"]
    #[inline(always)]
    pub fn pd6_pin_sel(&self) -> PD6_PIN_SEL_R {
        PD6_PIN_SEL_R::new(((self.bits >> 7) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Pin MUX for NRST"]
    #[inline(always)]
    #[must_use]
    pub fn nrstpa0_pin_sel(&mut self) -> NRSTPA0_PIN_SEL_W<PKG_PIN_SELrs> {
        NRSTPA0_PIN_SEL_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - Pin MUX for PB5"]
    #[inline(always)]
    #[must_use]
    pub fn pb5_pin_sel(&mut self) -> PB5_PIN_SEL_W<PKG_PIN_SELrs> {
        PB5_PIN_SEL_W::new(self, 1)
    }
    #[doc = "Bits 3:4 - Pin MUX for PC4"]
    #[inline(always)]
    #[must_use]
    pub fn pc4_pin_sel(&mut self) -> PC4_PIN_SEL_W<PKG_PIN_SELrs> {
        PC4_PIN_SEL_W::new(self, 3)
    }
    #[doc = "Bits 5:6 - Pin MUX for PD5"]
    #[inline(always)]
    #[must_use]
    pub fn pd5_pin_sel(&mut self) -> PD5_PIN_SEL_W<PKG_PIN_SELrs> {
        PD5_PIN_SEL_W::new(self, 5)
    }
    #[doc = "Bits 7:8 - Pin MUX for PD6"]
    #[inline(always)]
    #[must_use]
    pub fn pd6_pin_sel(&mut self) -> PD6_PIN_SEL_W<PKG_PIN_SELrs> {
        PD6_PIN_SEL_W::new(self, 7)
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
#[doc = "PKG_PIN_SEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pkg_pin_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pkg_pin_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PKG_PIN_SELrs;
impl crate::RegisterSpec for PKG_PIN_SELrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pkg_pin_sel::R`](R) reader structure"]
impl crate::Readable for PKG_PIN_SELrs {}
#[doc = "`write(|w| ..)` method takes [`pkg_pin_sel::W`](W) writer structure"]
impl crate::Writable for PKG_PIN_SELrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKG_PIN_SEL to value 0"]
impl crate::Resettable for PKG_PIN_SELrs {
    const RESET_VALUE: u32 = 0;
}
