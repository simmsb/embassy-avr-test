#[doc = "Register `DIR` reader"]
pub type R = crate::R<DIR_SPEC>;
#[doc = "Register `DIR` writer"]
pub type W = crate::W<DIR_SPEC>;
#[doc = "Field `PB0` reader - Pin B0"]
pub type PB0_R = crate::BitReader<PB0_A>;
#[doc = "Pin B0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PB0_A {
    #[doc = "0: Input"]
    INPUT = 0,
    #[doc = "1: Output"]
    OUTPUT = 1,
}
impl From<PB0_A> for bool {
    #[inline(always)]
    fn from(variant: PB0_A) -> Self {
        variant as u8 != 0
    }
}
impl PB0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PB0_A {
        match self.bits {
            false => PB0_A::INPUT,
            true => PB0_A::OUTPUT,
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PB0_A::INPUT
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PB0_A::OUTPUT
    }
}
#[doc = "Field `PB0` writer - Pin B0"]
pub type PB0_W<'a, REG> = crate::BitWriter<'a, REG, PB0_A>;
impl<'a, REG> PB0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(PB0_A::INPUT)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(PB0_A::OUTPUT)
    }
}
#[doc = "Field `PB1` reader - Pin B1"]
pub use PB0_R as PB1_R;
#[doc = "Field `PB2` reader - Pin B2"]
pub use PB0_R as PB2_R;
#[doc = "Field `PB3` reader - Pin B3"]
pub use PB0_R as PB3_R;
#[doc = "Field `PB4` reader - Pin B4"]
pub use PB0_R as PB4_R;
#[doc = "Field `PB5` reader - Pin B5"]
pub use PB0_R as PB5_R;
#[doc = "Field `PB1` writer - Pin B1"]
pub use PB0_W as PB1_W;
#[doc = "Field `PB2` writer - Pin B2"]
pub use PB0_W as PB2_W;
#[doc = "Field `PB3` writer - Pin B3"]
pub use PB0_W as PB3_W;
#[doc = "Field `PB4` writer - Pin B4"]
pub use PB0_W as PB4_W;
#[doc = "Field `PB5` writer - Pin B5"]
pub use PB0_W as PB5_W;
impl R {
    #[doc = "Bit 0 - Pin B0"]
    #[inline(always)]
    pub fn pb0(&self) -> PB0_R {
        PB0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pin B1"]
    #[inline(always)]
    pub fn pb1(&self) -> PB1_R {
        PB1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pin B2"]
    #[inline(always)]
    pub fn pb2(&self) -> PB2_R {
        PB2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pin B3"]
    #[inline(always)]
    pub fn pb3(&self) -> PB3_R {
        PB3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pin B4"]
    #[inline(always)]
    pub fn pb4(&self) -> PB4_R {
        PB4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pin B5"]
    #[inline(always)]
    pub fn pb5(&self) -> PB5_R {
        PB5_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pin B0"]
    #[inline(always)]
    #[must_use]
    pub fn pb0(&mut self) -> PB0_W<DIR_SPEC> {
        PB0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Pin B1"]
    #[inline(always)]
    #[must_use]
    pub fn pb1(&mut self) -> PB1_W<DIR_SPEC> {
        PB1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Pin B2"]
    #[inline(always)]
    #[must_use]
    pub fn pb2(&mut self) -> PB2_W<DIR_SPEC> {
        PB2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Pin B3"]
    #[inline(always)]
    #[must_use]
    pub fn pb3(&mut self) -> PB3_W<DIR_SPEC> {
        PB3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Pin B4"]
    #[inline(always)]
    #[must_use]
    pub fn pb4(&mut self) -> PB4_W<DIR_SPEC> {
        PB4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Pin B5"]
    #[inline(always)]
    #[must_use]
    pub fn pb5(&mut self) -> PB5_W<DIR_SPEC> {
        PB5_W::new(self, 5)
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
#[doc = "Data Direction\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIR_SPEC;
impl crate::RegisterSpec for DIR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dir::R`](R) reader structure"]
impl crate::Readable for DIR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dir::W`](W) writer structure"]
impl crate::Writable for DIR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIR to value 0"]
impl crate::Resettable for DIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
