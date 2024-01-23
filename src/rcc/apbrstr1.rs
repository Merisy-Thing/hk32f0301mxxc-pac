#[doc = "Register `APBRSTR1` reader"]
pub type R = crate::R<APBRSTR1rs>;
#[doc = "Register `APBRSTR1` writer"]
pub type W = crate::W<APBRSTR1rs>;
#[doc = "Field `TIM2RST` reader - Reset TIM2 timer"]
pub type TIM2RST_R = crate::BitReader;
#[doc = "Field `TIM2RST` writer - Reset TIM2 timer"]
pub type TIM2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM6RST` reader - Reset TIM6 timer"]
pub type TIM6RST_R = crate::BitReader;
#[doc = "Field `TIM6RST` writer - Reset TIM6 timer"]
pub type TIM6RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDGRST` reader - Reset Window watchdog"]
pub type WWDGRST_R = crate::BitReader;
#[doc = "Field `WWDGRST` writer - Reset Window watchdog"]
pub type WWDGRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWURST` reader - Auto_wake up reset in stop mode"]
pub type AWURST_R = crate::BitReader;
#[doc = "Field `AWURST` writer - Auto_wake up reset in stop mode"]
pub type AWURST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART2RST` reader - Reset UART2"]
pub type UART2RST_R = crate::BitReader;
#[doc = "Field `UART2RST` writer - Reset UART2"]
pub type UART2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2CRST` reader - Reset I2C"]
pub type I2CRST_R = crate::BitReader;
#[doc = "Field `I2CRST` writer - Reset I2C"]
pub type I2CRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRRST` reader - Reset power interface"]
pub type PWRRST_R = crate::BitReader;
#[doc = "Field `PWRRST` writer - Reset power interface"]
pub type PWRRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOMUXRST` reader - Reset IOMUX"]
pub type IOMUXRST_R = crate::BitReader;
#[doc = "Field `IOMUXRST` writer - Reset IOMUX"]
pub type IOMUXRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reset TIM2 timer"]
    #[inline(always)]
    pub fn tim2rst(&self) -> TIM2RST_R {
        TIM2RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Reset TIM6 timer"]
    #[inline(always)]
    pub fn tim6rst(&self) -> TIM6RST_R {
        TIM6RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 11 - Reset Window watchdog"]
    #[inline(always)]
    pub fn wwdgrst(&self) -> WWDGRST_R {
        WWDGRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Auto_wake up reset in stop mode"]
    #[inline(always)]
    pub fn awurst(&self) -> AWURST_R {
        AWURST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Reset UART2"]
    #[inline(always)]
    pub fn uart2rst(&self) -> UART2RST_R {
        UART2RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 21 - Reset I2C"]
    #[inline(always)]
    pub fn i2crst(&self) -> I2CRST_R {
        I2CRST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 28 - Reset power interface"]
    #[inline(always)]
    pub fn pwrrst(&self) -> PWRRST_R {
        PWRRST_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - Reset IOMUX"]
    #[inline(always)]
    pub fn iomuxrst(&self) -> IOMUXRST_R {
        IOMUXRST_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset TIM2 timer"]
    #[inline(always)]
    #[must_use]
    pub fn tim2rst(&mut self) -> TIM2RST_W<APBRSTR1rs> {
        TIM2RST_W::new(self, 0)
    }
    #[doc = "Bit 4 - Reset TIM6 timer"]
    #[inline(always)]
    #[must_use]
    pub fn tim6rst(&mut self) -> TIM6RST_W<APBRSTR1rs> {
        TIM6RST_W::new(self, 4)
    }
    #[doc = "Bit 11 - Reset Window watchdog"]
    #[inline(always)]
    #[must_use]
    pub fn wwdgrst(&mut self) -> WWDGRST_W<APBRSTR1rs> {
        WWDGRST_W::new(self, 11)
    }
    #[doc = "Bit 16 - Auto_wake up reset in stop mode"]
    #[inline(always)]
    #[must_use]
    pub fn awurst(&mut self) -> AWURST_W<APBRSTR1rs> {
        AWURST_W::new(self, 16)
    }
    #[doc = "Bit 17 - Reset UART2"]
    #[inline(always)]
    #[must_use]
    pub fn uart2rst(&mut self) -> UART2RST_W<APBRSTR1rs> {
        UART2RST_W::new(self, 17)
    }
    #[doc = "Bit 21 - Reset I2C"]
    #[inline(always)]
    #[must_use]
    pub fn i2crst(&mut self) -> I2CRST_W<APBRSTR1rs> {
        I2CRST_W::new(self, 21)
    }
    #[doc = "Bit 28 - Reset power interface"]
    #[inline(always)]
    #[must_use]
    pub fn pwrrst(&mut self) -> PWRRST_W<APBRSTR1rs> {
        PWRRST_W::new(self, 28)
    }
    #[doc = "Bit 30 - Reset IOMUX"]
    #[inline(always)]
    #[must_use]
    pub fn iomuxrst(&mut self) -> IOMUXRST_W<APBRSTR1rs> {
        IOMUXRST_W::new(self, 30)
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
#[doc = "APBRSTR1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbrstr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbrstr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APBRSTR1rs;
impl crate::RegisterSpec for APBRSTR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbrstr1::R`](R) reader structure"]
impl crate::Readable for APBRSTR1rs {}
#[doc = "`write(|w| ..)` method takes [`apbrstr1::W`](W) writer structure"]
impl crate::Writable for APBRSTR1rs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBRSTR1 to value 0"]
impl crate::Resettable for APBRSTR1rs {
    const RESET_VALUE: u32 = 0;
}
