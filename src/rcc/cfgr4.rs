#[doc = "Register `CFGR4` reader"]
pub type R = crate::R<CFGR4rs>;
#[doc = "Register `CFGR4` writer"]
pub type W = crate::W<CFGR4rs>;
#[doc = "Field `UARTHSIPRE` reader - UART_HSI prescaler"]
pub type UARTHSIPRE_R = crate::FieldReader;
#[doc = "Field `UARTHSIPRE` writer - UART_HSI prescaler"]
pub type UARTHSIPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `FLITFCLK_SEL` reader - FLITFCLK clock selection"]
pub type FLITFCLK_SEL_R = crate::FieldReader;
#[doc = "Field `FLITFCLK_SEL` writer - FLITFCLK clock selection"]
pub type FLITFCLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FLITFCLK_PRE` reader - FLITFCLK prescaler"]
pub type FLITFCLK_PRE_R = crate::FieldReader;
#[doc = "Field `FLITFCLK_PRE` writer - FLITFCLK prescaler"]
pub type FLITFCLK_PRE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `I2CCLK_SEL` reader - I2C clock selection"]
pub type I2CCLK_SEL_R = crate::BitReader;
#[doc = "Field `I2CCLK_SEL` writer - I2C clock selection"]
pub type I2CCLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2CHSIPRE` reader - I2C_HSI prescaler"]
pub type I2CHSIPRE_R = crate::FieldReader;
#[doc = "Field `I2CHSIPRE` writer - I2C_HSI prescaler"]
pub type I2CHSIPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `EXTCLK_SEL` reader - External clock io selection"]
pub type EXTCLK_SEL_R = crate::FieldReader;
#[doc = "Field `EXTCLK_SEL` writer - External clock io selection"]
pub type EXTCLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADCHSIPRE` reader - ADC_HSI prescaler"]
pub type ADCHSIPRE_R = crate::FieldReader;
#[doc = "Field `ADCHSIPRE` writer - ADC_HSI prescaler"]
pub type ADCHSIPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - UART_HSI prescaler"]
    #[inline(always)]
    pub fn uarthsipre(&self) -> UARTHSIPRE_R {
        UARTHSIPRE_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 9:10 - FLITFCLK clock selection"]
    #[inline(always)]
    pub fn flitfclk_sel(&self) -> FLITFCLK_SEL_R {
        FLITFCLK_SEL_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:14 - FLITFCLK prescaler"]
    #[inline(always)]
    pub fn flitfclk_pre(&self) -> FLITFCLK_PRE_R {
        FLITFCLK_PRE_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - I2C clock selection"]
    #[inline(always)]
    pub fn i2cclk_sel(&self) -> I2CCLK_SEL_R {
        I2CCLK_SEL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - I2C_HSI prescaler"]
    #[inline(always)]
    pub fn i2chsipre(&self) -> I2CHSIPRE_R {
        I2CHSIPRE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:25 - External clock io selection"]
    #[inline(always)]
    pub fn extclk_sel(&self) -> EXTCLK_SEL_R {
        EXTCLK_SEL_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:30 - ADC_HSI prescaler"]
    #[inline(always)]
    pub fn adchsipre(&self) -> ADCHSIPRE_R {
        ADCHSIPRE_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - UART_HSI prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn uarthsipre(&mut self) -> UARTHSIPRE_W<CFGR4rs> {
        UARTHSIPRE_W::new(self, 0)
    }
    #[doc = "Bits 9:10 - FLITFCLK clock selection"]
    #[inline(always)]
    #[must_use]
    pub fn flitfclk_sel(&mut self) -> FLITFCLK_SEL_W<CFGR4rs> {
        FLITFCLK_SEL_W::new(self, 9)
    }
    #[doc = "Bits 11:14 - FLITFCLK prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn flitfclk_pre(&mut self) -> FLITFCLK_PRE_W<CFGR4rs> {
        FLITFCLK_PRE_W::new(self, 11)
    }
    #[doc = "Bit 15 - I2C clock selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2cclk_sel(&mut self) -> I2CCLK_SEL_W<CFGR4rs> {
        I2CCLK_SEL_W::new(self, 15)
    }
    #[doc = "Bits 16:20 - I2C_HSI prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn i2chsipre(&mut self) -> I2CHSIPRE_W<CFGR4rs> {
        I2CHSIPRE_W::new(self, 16)
    }
    #[doc = "Bits 24:25 - External clock io selection"]
    #[inline(always)]
    #[must_use]
    pub fn extclk_sel(&mut self) -> EXTCLK_SEL_W<CFGR4rs> {
        EXTCLK_SEL_W::new(self, 24)
    }
    #[doc = "Bits 26:30 - ADC_HSI prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn adchsipre(&mut self) -> ADCHSIPRE_W<CFGR4rs> {
        ADCHSIPRE_W::new(self, 26)
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
#[doc = "CFGR4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGR4rs;
impl crate::RegisterSpec for CFGR4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr4::R`](R) reader structure"]
impl crate::Readable for CFGR4rs {}
#[doc = "`write(|w| ..)` method takes [`cfgr4::W`](W) writer structure"]
impl crate::Writable for CFGR4rs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR4 to value 0x1806_1806"]
impl crate::Resettable for CFGR4rs {
    const RESET_VALUE: u32 = 0x1806_1806;
}
