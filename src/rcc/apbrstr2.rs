#[doc = "Register `APBRSTR2` reader"]
pub type R = crate::R<APBRSTR2rs>;
#[doc = "Register `APBRSTR2` writer"]
pub type W = crate::W<APBRSTR2rs>;
#[doc = "Field `SYSCFGRST` reader - Reset SYSCFG"]
pub type SYSCFGRST_R = crate::BitReader;
#[doc = "Field `SYSCFGRST` writer - Reset SYSCFG"]
pub type SYSCFGRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCRST` reader - Reset ADC interface"]
pub type ADCRST_R = crate::BitReader;
#[doc = "Field `ADCRST` writer - Reset ADC interface"]
pub type ADCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM1RST` reader - Reset TIM1 timer"]
pub type TIM1RST_R = crate::BitReader;
#[doc = "Field `TIM1RST` writer - Reset TIM1 timer"]
pub type TIM1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPIRST` reader - Reset SPI"]
pub type SPIRST_R = crate::BitReader;
#[doc = "Field `SPIRST` writer - Reset SPI"]
pub type SPIRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART1RST` reader - Reset UART1"]
pub type UART1RST_R = crate::BitReader;
#[doc = "Field `UART1RST` writer - Reset UART1"]
pub type UART1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBGMCURST` reader - Debug MCU reset"]
pub type DBGMCURST_R = crate::BitReader;
#[doc = "Field `DBGMCURST` writer - Debug MCU reset"]
pub type DBGMCURST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reset SYSCFG"]
    #[inline(always)]
    pub fn syscfgrst(&self) -> SYSCFGRST_R {
        SYSCFGRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 9 - Reset ADC interface"]
    #[inline(always)]
    pub fn adcrst(&self) -> ADCRST_R {
        ADCRST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Reset TIM1 timer"]
    #[inline(always)]
    pub fn tim1rst(&self) -> TIM1RST_R {
        TIM1RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Reset SPI"]
    #[inline(always)]
    pub fn spirst(&self) -> SPIRST_R {
        SPIRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Reset UART1"]
    #[inline(always)]
    pub fn uart1rst(&self) -> UART1RST_R {
        UART1RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 22 - Debug MCU reset"]
    #[inline(always)]
    pub fn dbgmcurst(&self) -> DBGMCURST_R {
        DBGMCURST_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset SYSCFG"]
    #[inline(always)]
    #[must_use]
    pub fn syscfgrst(&mut self) -> SYSCFGRST_W<APBRSTR2rs> {
        SYSCFGRST_W::new(self, 0)
    }
    #[doc = "Bit 9 - Reset ADC interface"]
    #[inline(always)]
    #[must_use]
    pub fn adcrst(&mut self) -> ADCRST_W<APBRSTR2rs> {
        ADCRST_W::new(self, 9)
    }
    #[doc = "Bit 11 - Reset TIM1 timer"]
    #[inline(always)]
    #[must_use]
    pub fn tim1rst(&mut self) -> TIM1RST_W<APBRSTR2rs> {
        TIM1RST_W::new(self, 11)
    }
    #[doc = "Bit 12 - Reset SPI"]
    #[inline(always)]
    #[must_use]
    pub fn spirst(&mut self) -> SPIRST_W<APBRSTR2rs> {
        SPIRST_W::new(self, 12)
    }
    #[doc = "Bit 14 - Reset UART1"]
    #[inline(always)]
    #[must_use]
    pub fn uart1rst(&mut self) -> UART1RST_W<APBRSTR2rs> {
        UART1RST_W::new(self, 14)
    }
    #[doc = "Bit 22 - Debug MCU reset"]
    #[inline(always)]
    #[must_use]
    pub fn dbgmcurst(&mut self) -> DBGMCURST_W<APBRSTR2rs> {
        DBGMCURST_W::new(self, 22)
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
#[doc = "APBRSTR2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbrstr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbrstr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APBRSTR2rs;
impl crate::RegisterSpec for APBRSTR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbrstr2::R`](R) reader structure"]
impl crate::Readable for APBRSTR2rs {}
#[doc = "`write(|w| ..)` method takes [`apbrstr2::W`](W) writer structure"]
impl crate::Writable for APBRSTR2rs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBRSTR2 to value 0"]
impl crate::Resettable for APBRSTR2rs {
    const RESET_VALUE: u32 = 0;
}
