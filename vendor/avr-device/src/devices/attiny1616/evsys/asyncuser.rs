#[doc = "Register `ASYNCUSER%s` reader"]
pub type R = crate::R<ASYNCUSER_SPEC>;
#[doc = "Register `ASYNCUSER%s` writer"]
pub type W = crate::W<ASYNCUSER_SPEC>;
#[doc = "Field `ASYNCUSER0` reader - Asynchronous User Ch 0 Input Selection - TCB0"]
pub type ASYNCUSER0_R = crate::FieldReader<ASYNCUSER0_A>;
#[doc = "Asynchronous User Ch 0 Input Selection - TCB0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ASYNCUSER0_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: Synchronous Event Channel 0"]
    SYNCCH0 = 1,
    #[doc = "2: Synchronous Event Channel 1"]
    SYNCCH1 = 2,
    #[doc = "3: Asynchronous Event Channel 0"]
    ASYNCCH0 = 3,
    #[doc = "4: Asynchronous Event Channel 1"]
    ASYNCCH1 = 4,
    #[doc = "5: Asynchronous Event Channel 2"]
    ASYNCCH2 = 5,
    #[doc = "6: Asynchronous Event Channel 3"]
    ASYNCCH3 = 6,
}
impl From<ASYNCUSER0_A> for u8 {
    #[inline(always)]
    fn from(variant: ASYNCUSER0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ASYNCUSER0_A {
    type Ux = u8;
}
impl ASYNCUSER0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ASYNCUSER0_A> {
        match self.bits {
            0 => Some(ASYNCUSER0_A::OFF),
            1 => Some(ASYNCUSER0_A::SYNCCH0),
            2 => Some(ASYNCUSER0_A::SYNCCH1),
            3 => Some(ASYNCUSER0_A::ASYNCCH0),
            4 => Some(ASYNCUSER0_A::ASYNCCH1),
            5 => Some(ASYNCUSER0_A::ASYNCCH2),
            6 => Some(ASYNCUSER0_A::ASYNCCH3),
            _ => None,
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == ASYNCUSER0_A::OFF
    }
    #[doc = "Synchronous Event Channel 0"]
    #[inline(always)]
    pub fn is_syncch0(&self) -> bool {
        *self == ASYNCUSER0_A::SYNCCH0
    }
    #[doc = "Synchronous Event Channel 1"]
    #[inline(always)]
    pub fn is_syncch1(&self) -> bool {
        *self == ASYNCUSER0_A::SYNCCH1
    }
    #[doc = "Asynchronous Event Channel 0"]
    #[inline(always)]
    pub fn is_asyncch0(&self) -> bool {
        *self == ASYNCUSER0_A::ASYNCCH0
    }
    #[doc = "Asynchronous Event Channel 1"]
    #[inline(always)]
    pub fn is_asyncch1(&self) -> bool {
        *self == ASYNCUSER0_A::ASYNCCH1
    }
    #[doc = "Asynchronous Event Channel 2"]
    #[inline(always)]
    pub fn is_asyncch2(&self) -> bool {
        *self == ASYNCUSER0_A::ASYNCCH2
    }
    #[doc = "Asynchronous Event Channel 3"]
    #[inline(always)]
    pub fn is_asyncch3(&self) -> bool {
        *self == ASYNCUSER0_A::ASYNCCH3
    }
}
#[doc = "Field `ASYNCUSER0` writer - Asynchronous User Ch 0 Input Selection - TCB0"]
pub type ASYNCUSER0_W<'a, REG> = crate::FieldWriter<'a, REG, 8, ASYNCUSER0_A>;
impl<'a, REG> ASYNCUSER0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(ASYNCUSER0_A::OFF)
    }
    #[doc = "Synchronous Event Channel 0"]
    #[inline(always)]
    pub fn syncch0(self) -> &'a mut crate::W<REG> {
        self.variant(ASYNCUSER0_A::SYNCCH0)
    }
    #[doc = "Synchronous Event Channel 1"]
    #[inline(always)]
    pub fn syncch1(self) -> &'a mut crate::W<REG> {
        self.variant(ASYNCUSER0_A::SYNCCH1)
    }
    #[doc = "Asynchronous Event Channel 0"]
    #[inline(always)]
    pub fn asyncch0(self) -> &'a mut crate::W<REG> {
        self.variant(ASYNCUSER0_A::ASYNCCH0)
    }
    #[doc = "Asynchronous Event Channel 1"]
    #[inline(always)]
    pub fn asyncch1(self) -> &'a mut crate::W<REG> {
        self.variant(ASYNCUSER0_A::ASYNCCH1)
    }
    #[doc = "Asynchronous Event Channel 2"]
    #[inline(always)]
    pub fn asyncch2(self) -> &'a mut crate::W<REG> {
        self.variant(ASYNCUSER0_A::ASYNCCH2)
    }
    #[doc = "Asynchronous Event Channel 3"]
    #[inline(always)]
    pub fn asyncch3(self) -> &'a mut crate::W<REG> {
        self.variant(ASYNCUSER0_A::ASYNCCH3)
    }
}
impl R {
    #[doc = "Bits 0:7 - Asynchronous User Ch 0 Input Selection - TCB0"]
    #[inline(always)]
    pub fn asyncuser0(&self) -> ASYNCUSER0_R {
        ASYNCUSER0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Asynchronous User Ch 0 Input Selection - TCB0"]
    #[inline(always)]
    #[must_use]
    pub fn asyncuser0(&mut self) -> ASYNCUSER0_W<ASYNCUSER_SPEC> {
        ASYNCUSER0_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Asynchronous User Ch %s Input Selection - TCB%s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`asyncuser::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`asyncuser::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ASYNCUSER_SPEC;
impl crate::RegisterSpec for ASYNCUSER_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`asyncuser::R`](R) reader structure"]
impl crate::Readable for ASYNCUSER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`asyncuser::W`](W) writer structure"]
impl crate::Writable for ASYNCUSER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ASYNCUSER%s to value 0"]
impl crate::Resettable for ASYNCUSER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
