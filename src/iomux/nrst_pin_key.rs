#[doc = "Register `NRST_PIN_KEY` reader"]
pub type R = crate::R<NRST_PIN_KEYrs>;
#[doc = "Register `NRST_PIN_KEY` writer"]
pub type W = crate::W<NRST_PIN_KEYrs>;
#[doc = "Field `NRST_PIN_KEY` reader - The key of pin NRST change to PA0"]
pub type NRST_PIN_KEY_R = crate::FieldReader<u16>;
#[doc = "Field `NRST_PIN_KEY` writer - The key of pin NRST change to PA0"]
pub type NRST_PIN_KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - The key of pin NRST change to PA0"]
    #[inline(always)]
    pub fn nrst_pin_key(&self) -> NRST_PIN_KEY_R {
        NRST_PIN_KEY_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The key of pin NRST change to PA0"]
    #[inline(always)]
    #[must_use]
    pub fn nrst_pin_key(&mut self) -> NRST_PIN_KEY_W<NRST_PIN_KEYrs> {
        NRST_PIN_KEY_W::new(self, 0)
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
#[doc = "NRST_PIN_KEY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nrst_pin_key::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nrst_pin_key::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NRST_PIN_KEYrs;
impl crate::RegisterSpec for NRST_PIN_KEYrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nrst_pin_key::R`](R) reader structure"]
impl crate::Readable for NRST_PIN_KEYrs {}
#[doc = "`write(|w| ..)` method takes [`nrst_pin_key::W`](W) writer structure"]
impl crate::Writable for NRST_PIN_KEYrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NRST_PIN_KEY to value 0"]
impl crate::Resettable for NRST_PIN_KEYrs {
    const RESET_VALUE: u32 = 0;
}
