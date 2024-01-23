#[doc = "Register `CSS` reader"]
pub type R = crate::R<CSSrs>;
#[doc = "Register `CSS` writer"]
pub type W = crate::W<CSSrs>;
#[doc = "Field `CSS_THRESHOLD` reader - Clock security system threshold value"]
pub type CSS_THRESHOLD_R = crate::FieldReader;
#[doc = "Field `CSS_THRESHOLD` writer - Clock security system threshold value"]
pub type CSS_THRESHOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 25:31 - Clock security system threshold value"]
    #[inline(always)]
    pub fn css_threshold(&self) -> CSS_THRESHOLD_R {
        CSS_THRESHOLD_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 25:31 - Clock security system threshold value"]
    #[inline(always)]
    #[must_use]
    pub fn css_threshold(&mut self) -> CSS_THRESHOLD_W<CSSrs> {
        CSS_THRESHOLD_W::new(self, 25)
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
#[doc = "CSS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`css::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`css::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSSrs;
impl crate::RegisterSpec for CSSrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`css::R`](R) reader structure"]
impl crate::Readable for CSSrs {}
#[doc = "`write(|w| ..)` method takes [`css::W`](W) writer structure"]
impl crate::Writable for CSSrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSS to value 0x1e00_0000"]
impl crate::Resettable for CSSrs {
    const RESET_VALUE: u32 = 0x1e00_0000;
}
