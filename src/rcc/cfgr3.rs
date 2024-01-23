#[doc = "Register `CFGR3` reader"]
pub type R = crate::R<CFGR3rs>;
#[doc = "Register `CFGR3` writer"]
pub type W = crate::W<CFGR3rs>;
#[doc = "Field `UART1SW` reader - UART1 clock source selection"]
pub type UART1SW_R = crate::FieldReader;
#[doc = "Field `UART1SW` writer - UART1 clock source selection"]
pub type UART1SW_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2CSW` reader - I2C clock source selection"]
pub type I2CSW_R = crate::BitReader;
#[doc = "Field `I2CSW` writer - I2C clock source selection"]
pub type I2CSW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART2SW` reader - UART2 clock source selection"]
pub type UART2SW_R = crate::FieldReader;
#[doc = "Field `UART2SW` writer - UART2 clock source selection"]
pub type UART2SW_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - UART1 clock source selection"]
    #[inline(always)]
    pub fn uart1sw(&self) -> UART1SW_R {
        UART1SW_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - I2C clock source selection"]
    #[inline(always)]
    pub fn i2csw(&self) -> I2CSW_R {
        I2CSW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 16:17 - UART2 clock source selection"]
    #[inline(always)]
    pub fn uart2sw(&self) -> UART2SW_R {
        UART2SW_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - UART1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn uart1sw(&mut self) -> UART1SW_W<CFGR3rs> {
        UART1SW_W::new(self, 0)
    }
    #[doc = "Bit 4 - I2C clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2csw(&mut self) -> I2CSW_W<CFGR3rs> {
        I2CSW_W::new(self, 4)
    }
    #[doc = "Bits 16:17 - UART2 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn uart2sw(&mut self) -> UART2SW_W<CFGR3rs> {
        UART2SW_W::new(self, 16)
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
#[doc = "CFGR3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGR3rs;
impl crate::RegisterSpec for CFGR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr3::R`](R) reader structure"]
impl crate::Readable for CFGR3rs {}
#[doc = "`write(|w| ..)` method takes [`cfgr3::W`](W) writer structure"]
impl crate::Writable for CFGR3rs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR3 to value 0"]
impl crate::Resettable for CFGR3rs {
    const RESET_VALUE: u32 = 0;
}
