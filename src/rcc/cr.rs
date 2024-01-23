#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Field `HSION` reader - HSI clock enable"]
pub type HSION_R = crate::BitReader<HSION>;
#[doc = "HSI clock enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSION {
    #[doc = "0: 关闭 HSI"]
    Off = 0,
    #[doc = "1: 打开 HSI"]
    On = 1,
}
impl From<HSION> for bool {
    #[inline(always)]
    fn from(variant: HSION) -> Self {
        variant as u8 != 0
    }
}
impl HSION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSION {
        match self.bits {
            false => HSION::Off,
            true => HSION::On,
        }
    }
    #[doc = "关闭 HSI"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == HSION::Off
    }
    #[doc = "打开 HSI"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == HSION::On
    }
}
#[doc = "Field `HSION` writer - HSI clock enable"]
pub type HSION_W<'a, REG> = crate::BitWriter<'a, REG, HSION>;
impl<'a, REG> HSION_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "关闭 HSI"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(HSION::Off)
    }
    #[doc = "打开 HSI"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(HSION::On)
    }
}
#[doc = "Field `HSIRDY` reader - HSI clock ready flag"]
pub type HSIRDY_R = crate::BitReader<HSIRDYR>;
#[doc = "HSI clock ready flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIRDYR {
    #[doc = "0: HSI 未稳定"]
    NotReady = 0,
    #[doc = "1: HSI 已稳定"]
    Ready = 1,
}
impl From<HSIRDYR> for bool {
    #[inline(always)]
    fn from(variant: HSIRDYR) -> Self {
        variant as u8 != 0
    }
}
impl HSIRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSIRDYR {
        match self.bits {
            false => HSIRDYR::NotReady,
            true => HSIRDYR::Ready,
        }
    }
    #[doc = "HSI 未稳定"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == HSIRDYR::NotReady
    }
    #[doc = "HSI 已稳定"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == HSIRDYR::Ready
    }
}
#[doc = "Field `HSICAL` reader - HSI clock calibration"]
pub type HSICAL_R = crate::FieldReader;
#[doc = "Field `HSICAL` writer - HSI clock calibration"]
pub type HSICAL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 6>;
#[doc = "Field `HSITRIM` reader - HSI clock trimming"]
pub type HSITRIM_R = crate::FieldReader;
#[doc = "Field `HSITRIM` writer - HSI clock trimming"]
pub type HSITRIM_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 6>;
#[doc = "Field `EXTCLKON` reader - External clock enable"]
pub type EXTCLKON_R = crate::BitReader<EXTCLKON>;
#[doc = "External clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTCLKON {
    #[doc = "0: 关闭 GPIO 输入时钟"]
    Off = 0,
    #[doc = "1: 使能 GPIO 输入时钟"]
    On = 1,
}
impl From<EXTCLKON> for bool {
    #[inline(always)]
    fn from(variant: EXTCLKON) -> Self {
        variant as u8 != 0
    }
}
impl EXTCLKON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXTCLKON {
        match self.bits {
            false => EXTCLKON::Off,
            true => EXTCLKON::On,
        }
    }
    #[doc = "关闭 GPIO 输入时钟"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == EXTCLKON::Off
    }
    #[doc = "使能 GPIO 输入时钟"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == EXTCLKON::On
    }
}
#[doc = "Field `EXTCLKON` writer - External clock enable"]
pub type EXTCLKON_W<'a, REG> = crate::BitWriter<'a, REG, EXTCLKON>;
impl<'a, REG> EXTCLKON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "关闭 GPIO 输入时钟"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(EXTCLKON::Off)
    }
    #[doc = "使能 GPIO 输入时钟"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(EXTCLKON::On)
    }
}
#[doc = "Field `EXTCLKRDY` reader - External clock ready flag"]
pub type EXTCLKRDY_R = crate::BitReader<EXTCLKRDYR>;
#[doc = "External clock ready flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTCLKRDYR {
    #[doc = "0: GPIO 输入时钟未稳定"]
    NotReady = 0,
    #[doc = "1: GPIO 输入时钟稳定"]
    Ready = 1,
}
impl From<EXTCLKRDYR> for bool {
    #[inline(always)]
    fn from(variant: EXTCLKRDYR) -> Self {
        variant as u8 != 0
    }
}
impl EXTCLKRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXTCLKRDYR {
        match self.bits {
            false => EXTCLKRDYR::NotReady,
            true => EXTCLKRDYR::Ready,
        }
    }
    #[doc = "GPIO 输入时钟未稳定"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == EXTCLKRDYR::NotReady
    }
    #[doc = "GPIO 输入时钟稳定"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == EXTCLKRDYR::Ready
    }
}
#[doc = "Field `CSSON` reader - Clock Security System enable"]
pub type CSSON_R = crate::BitReader<CSSON>;
#[doc = "Clock Security System enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSON {
    #[doc = "0: 0：关闭时钟检测"]
    Off = 0,
    #[doc = "1: 开启时钟检测"]
    On = 1,
}
impl From<CSSON> for bool {
    #[inline(always)]
    fn from(variant: CSSON) -> Self {
        variant as u8 != 0
    }
}
impl CSSON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSSON {
        match self.bits {
            false => CSSON::Off,
            true => CSSON::On,
        }
    }
    #[doc = "0：关闭时钟检测"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == CSSON::Off
    }
    #[doc = "开启时钟检测"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == CSSON::On
    }
}
#[doc = "Field `CSSON` writer - Clock Security System enable"]
pub type CSSON_W<'a, REG> = crate::BitWriter<'a, REG, CSSON>;
impl<'a, REG> CSSON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "0：关闭时钟检测"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(CSSON::Off)
    }
    #[doc = "开启时钟检测"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(CSSON::On)
    }
}
impl R {
    #[doc = "Bit 0 - HSI clock enable"]
    #[inline(always)]
    pub fn hsion(&self) -> HSION_R {
        HSION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HSI clock ready flag"]
    #[inline(always)]
    pub fn hsirdy(&self) -> HSIRDY_R {
        HSIRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - HSI clock calibration"]
    #[inline(always)]
    pub fn hsical(&self) -> HSICAL_R {
        HSICAL_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - HSI clock trimming"]
    #[inline(always)]
    pub fn hsitrim(&self) -> HSITRIM_R {
        HSITRIM_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - External clock enable"]
    #[inline(always)]
    pub fn extclkon(&self) -> EXTCLKON_R {
        EXTCLKON_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - External clock ready flag"]
    #[inline(always)]
    pub fn extclkrdy(&self) -> EXTCLKRDY_R {
        EXTCLKRDY_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Clock Security System enable"]
    #[inline(always)]
    pub fn csson(&self) -> CSSON_R {
        CSSON_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HSI clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsion(&mut self) -> HSION_W<CRrs> {
        HSION_W::new(self, 0)
    }
    #[doc = "Bits 2:7 - HSI clock calibration"]
    #[inline(always)]
    #[must_use]
    pub fn hsical(&mut self) -> HSICAL_W<CRrs> {
        HSICAL_W::new(self, 2)
    }
    #[doc = "Bits 8:13 - HSI clock trimming"]
    #[inline(always)]
    #[must_use]
    pub fn hsitrim(&mut self) -> HSITRIM_W<CRrs> {
        HSITRIM_W::new(self, 8)
    }
    #[doc = "Bit 16 - External clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn extclkon(&mut self) -> EXTCLKON_W<CRrs> {
        EXTCLKON_W::new(self, 16)
    }
    #[doc = "Bit 19 - Clock Security System enable"]
    #[inline(always)]
    #[must_use]
    pub fn csson(&mut self) -> CSSON_W<CRrs> {
        CSSON_W::new(self, 19)
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
#[doc = "CR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CRrs {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CRrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0x03"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0x03;
}
