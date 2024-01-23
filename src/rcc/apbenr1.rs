#[doc = "Register `APBENR1` reader"]
pub type R = crate::R<APBENR1rs>;
#[doc = "Register `APBENR1` writer"]
pub type W = crate::W<APBENR1rs>;
#[doc = "Field `TIM2EN` reader - TIM2 timer clock enable"]
pub type TIM2EN_R = crate::BitReader;
#[doc = "Field `TIM2EN` writer - TIM2 timer clock enable"]
pub type TIM2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM6EN` reader - TIM6 timer clock enable"]
pub type TIM6EN_R = crate::BitReader;
#[doc = "Field `TIM6EN` writer - TIM6 timer clock enable"]
pub type TIM6EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDGEN` reader - Window watchdog clock enable"]
pub type WWDGEN_R = crate::BitReader;
#[doc = "Field `WWDGEN` writer - Window watchdog clock enable"]
pub type WWDGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWUEN` reader - Auto-wake up clock enable in stop mode"]
pub type AWUEN_R = crate::BitReader;
#[doc = "Field `AWUEN` writer - Auto-wake up clock enable in stop mode"]
pub type AWUEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART2EN` reader - UART2 clock enable"]
pub type UART2EN_R = crate::BitReader;
#[doc = "Field `UART2EN` writer - UART2 clock enable"]
pub type UART2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2CEN` reader - I2C clock enable"]
pub type I2CEN_R = crate::BitReader;
#[doc = "Field `I2CEN` writer - I2C clock enable"]
pub type I2CEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWREN` reader - Power interface clock enable"]
pub type PWREN_R = crate::BitReader;
#[doc = "Field `PWREN` writer - Power interface clock enable"]
pub type PWREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOMUXEN` reader - IOMUX clock enable"]
pub type IOMUXEN_R = crate::BitReader;
#[doc = "Field `IOMUXEN` writer - IOMUX clock enable"]
pub type IOMUXEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TIM2 timer clock enable"]
    #[inline(always)]
    pub fn tim2en(&self) -> TIM2EN_R {
        TIM2EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6 timer clock enable"]
    #[inline(always)]
    pub fn tim6en(&self) -> TIM6EN_R {
        TIM6EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog clock enable"]
    #[inline(always)]
    pub fn wwdgen(&self) -> WWDGEN_R {
        WWDGEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Auto-wake up clock enable in stop mode"]
    #[inline(always)]
    pub fn awuen(&self) -> AWUEN_R {
        AWUEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - UART2 clock enable"]
    #[inline(always)]
    pub fn uart2en(&self) -> UART2EN_R {
        UART2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C clock enable"]
    #[inline(always)]
    pub fn i2cen(&self) -> I2CEN_R {
        I2CEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface clock enable"]
    #[inline(always)]
    pub fn pwren(&self) -> PWREN_R {
        PWREN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - IOMUX clock enable"]
    #[inline(always)]
    pub fn iomuxen(&self) -> IOMUXEN_R {
        IOMUXEN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim2en(&mut self) -> TIM2EN_W<APBENR1rs> {
        TIM2EN_W::new(self, 0)
    }
    #[doc = "Bit 4 - TIM6 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim6en(&mut self) -> TIM6EN_W<APBENR1rs> {
        TIM6EN_W::new(self, 4)
    }
    #[doc = "Bit 11 - Window watchdog clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn wwdgen(&mut self) -> WWDGEN_W<APBENR1rs> {
        WWDGEN_W::new(self, 11)
    }
    #[doc = "Bit 16 - Auto-wake up clock enable in stop mode"]
    #[inline(always)]
    #[must_use]
    pub fn awuen(&mut self) -> AWUEN_W<APBENR1rs> {
        AWUEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - UART2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn uart2en(&mut self) -> UART2EN_W<APBENR1rs> {
        UART2EN_W::new(self, 17)
    }
    #[doc = "Bit 21 - I2C clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2cen(&mut self) -> I2CEN_W<APBENR1rs> {
        I2CEN_W::new(self, 21)
    }
    #[doc = "Bit 28 - Power interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwren(&mut self) -> PWREN_W<APBENR1rs> {
        PWREN_W::new(self, 28)
    }
    #[doc = "Bit 30 - IOMUX clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn iomuxen(&mut self) -> IOMUXEN_W<APBENR1rs> {
        IOMUXEN_W::new(self, 30)
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
#[doc = "APBENR1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbenr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbenr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APBENR1rs;
impl crate::RegisterSpec for APBENR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbenr1::R`](R) reader structure"]
impl crate::Readable for APBENR1rs {}
#[doc = "`write(|w| ..)` method takes [`apbenr1::W`](W) writer structure"]
impl crate::Writable for APBENR1rs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBENR1 to value 0"]
impl crate::Resettable for APBENR1rs {
    const RESET_VALUE: u32 = 0;
}
