#[doc = "Register `APBENR2` reader"]
pub type R = crate::R<APBENR2rs>;
#[doc = "Register `APBENR2` writer"]
pub type W = crate::W<APBENR2rs>;
#[doc = "Field `SYSCFGEN` reader - SYSCFG clock enable"]
pub type SYSCFGEN_R = crate::BitReader;
#[doc = "Field `SYSCFGEN` writer - SYSCFG clock enable"]
pub type SYSCFGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCEN` reader - ADC interface clock enable"]
pub type ADCEN_R = crate::BitReader;
#[doc = "Field `ADCEN` writer - ADC interface clock enable"]
pub type ADCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM1EN` reader - TIM1 timer clock enable"]
pub type TIM1EN_R = crate::BitReader;
#[doc = "Field `TIM1EN` writer - TIM1 timer clock enable"]
pub type TIM1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPIEN` reader - SPI clock enable"]
pub type SPIEN_R = crate::BitReader;
#[doc = "Field `SPIEN` writer - SPI clock enable"]
pub type SPIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART1EN` reader - UART1 clock enable"]
pub type UART1EN_R = crate::BitReader;
#[doc = "Field `UART1EN` writer - UART1 clock enable"]
pub type UART1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBGMCUEN` reader - MCU debug module clock enable"]
pub type DBGMCUEN_R = crate::BitReader;
#[doc = "Field `DBGMCUEN` writer - MCU debug module clock enable"]
pub type DBGMCUEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SYSCFG clock enable"]
    #[inline(always)]
    pub fn syscfgen(&self) -> SYSCFGEN_R {
        SYSCFGEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 9 - ADC interface clock enable"]
    #[inline(always)]
    pub fn adcen(&self) -> ADCEN_R {
        ADCEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - TIM1 timer clock enable"]
    #[inline(always)]
    pub fn tim1en(&self) -> TIM1EN_R {
        TIM1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI clock enable"]
    #[inline(always)]
    pub fn spien(&self) -> SPIEN_R {
        SPIEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - UART1 clock enable"]
    #[inline(always)]
    pub fn uart1en(&self) -> UART1EN_R {
        UART1EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 22 - MCU debug module clock enable"]
    #[inline(always)]
    pub fn dbgmcuen(&self) -> DBGMCUEN_R {
        DBGMCUEN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYSCFG clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn syscfgen(&mut self) -> SYSCFGEN_W<APBENR2rs> {
        SYSCFGEN_W::new(self, 0)
    }
    #[doc = "Bit 9 - ADC interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn adcen(&mut self) -> ADCEN_W<APBENR2rs> {
        ADCEN_W::new(self, 9)
    }
    #[doc = "Bit 11 - TIM1 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim1en(&mut self) -> TIM1EN_W<APBENR2rs> {
        TIM1EN_W::new(self, 11)
    }
    #[doc = "Bit 12 - SPI clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spien(&mut self) -> SPIEN_W<APBENR2rs> {
        SPIEN_W::new(self, 12)
    }
    #[doc = "Bit 14 - UART1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn uart1en(&mut self) -> UART1EN_W<APBENR2rs> {
        UART1EN_W::new(self, 14)
    }
    #[doc = "Bit 22 - MCU debug module clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dbgmcuen(&mut self) -> DBGMCUEN_W<APBENR2rs> {
        DBGMCUEN_W::new(self, 22)
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
#[doc = "APBENR2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbenr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbenr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APBENR2rs;
impl crate::RegisterSpec for APBENR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbenr2::R`](R) reader structure"]
impl crate::Readable for APBENR2rs {}
#[doc = "`write(|w| ..)` method takes [`apbenr2::W`](W) writer structure"]
impl crate::Writable for APBENR2rs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBENR2 to value 0"]
impl crate::Resettable for APBENR2rs {
    const RESET_VALUE: u32 = 0;
}
