#[doc = "Register `PIN_FUNC_SEL` reader"]
pub type R = crate::R<PIN_FUNC_SELrs>;
#[doc = "Register `PIN_FUNC_SEL` writer"]
pub type W = crate::W<PIN_FUNC_SELrs>;
#[doc = "Field `PC3_TIM1_SEL` reader - Pin functional selection for PC3"]
pub type PC3_TIM1_SEL_R = crate::BitReader;
#[doc = "Field `PC3_TIM1_SEL` writer - Pin functional selection for PC3"]
pub type PC3_TIM1_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC4_TIM1_SEL` reader - Pin functional selection for PC4"]
pub type PC4_TIM1_SEL_R = crate::BitReader;
#[doc = "Field `PC4_TIM1_SEL` writer - Pin functional selection for PC4"]
pub type PC4_TIM1_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PB5_I2C_SEL` reader - Pin functional selection for PB5"]
pub type PB5_I2C_SEL_R = crate::BitReader;
#[doc = "Field `PB5_I2C_SEL` writer - Pin functional selection for PB5"]
pub type PB5_I2C_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD7_BKIN_SEL` reader - Pin functional selection for PD5"]
pub type PD7_BKIN_SEL_R = crate::BitReader;
#[doc = "Field `PD7_BKIN_SEL` writer - Pin functional selection for PD5"]
pub type PD7_BKIN_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Pin functional selection for PC3"]
    #[inline(always)]
    pub fn pc3_tim1_sel(&self) -> PC3_TIM1_SEL_R {
        PC3_TIM1_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pin functional selection for PC4"]
    #[inline(always)]
    pub fn pc4_tim1_sel(&self) -> PC4_TIM1_SEL_R {
        PC4_TIM1_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pin functional selection for PB5"]
    #[inline(always)]
    pub fn pb5_i2c_sel(&self) -> PB5_I2C_SEL_R {
        PB5_I2C_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pin functional selection for PD5"]
    #[inline(always)]
    pub fn pd7_bkin_sel(&self) -> PD7_BKIN_SEL_R {
        PD7_BKIN_SEL_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pin functional selection for PC3"]
    #[inline(always)]
    #[must_use]
    pub fn pc3_tim1_sel(&mut self) -> PC3_TIM1_SEL_W<PIN_FUNC_SELrs> {
        PC3_TIM1_SEL_W::new(self, 0)
    }
    #[doc = "Bit 1 - Pin functional selection for PC4"]
    #[inline(always)]
    #[must_use]
    pub fn pc4_tim1_sel(&mut self) -> PC4_TIM1_SEL_W<PIN_FUNC_SELrs> {
        PC4_TIM1_SEL_W::new(self, 1)
    }
    #[doc = "Bit 2 - Pin functional selection for PB5"]
    #[inline(always)]
    #[must_use]
    pub fn pb5_i2c_sel(&mut self) -> PB5_I2C_SEL_W<PIN_FUNC_SELrs> {
        PB5_I2C_SEL_W::new(self, 2)
    }
    #[doc = "Bit 3 - Pin functional selection for PD5"]
    #[inline(always)]
    #[must_use]
    pub fn pd7_bkin_sel(&mut self) -> PD7_BKIN_SEL_W<PIN_FUNC_SELrs> {
        PD7_BKIN_SEL_W::new(self, 3)
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
#[doc = "PIN_FUNC_SEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin_func_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin_func_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIN_FUNC_SELrs;
impl crate::RegisterSpec for PIN_FUNC_SELrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pin_func_sel::R`](R) reader structure"]
impl crate::Readable for PIN_FUNC_SELrs {}
#[doc = "`write(|w| ..)` method takes [`pin_func_sel::W`](W) writer structure"]
impl crate::Writable for PIN_FUNC_SELrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PIN_FUNC_SEL to value 0"]
impl crate::Resettable for PIN_FUNC_SELrs {
    const RESET_VALUE: u32 = 0;
}
