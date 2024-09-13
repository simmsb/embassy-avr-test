#[doc = "Register `CMPACLR` reader"]
pub type R = crate::R<CMPACLR_SPEC>;
#[doc = "Register `CMPACLR` writer"]
pub type W = crate::W<CMPACLR_SPEC>;
#[doc = "Field `CMPACLR` reader - Compare A Set"]
pub type CMPACLR_R = crate::FieldReader<u16>;
#[doc = "Field `CMPACLR` writer - Compare A Set"]
pub type CMPACLR_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Compare A Set"]
    #[inline(always)]
    pub fn cmpaclr(&self) -> CMPACLR_R {
        CMPACLR_R::new(self.bits & 0x0fff)
    }
}
impl W {
    #[doc = "Bits 0:11 - Compare A Set"]
    #[inline(always)]
    #[must_use]
    pub fn cmpaclr(&mut self) -> CMPACLR_W<CMPACLR_SPEC> {
        CMPACLR_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Compare A Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpaclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpaclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMPACLR_SPEC;
impl crate::RegisterSpec for CMPACLR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cmpaclr::R`](R) reader structure"]
impl crate::Readable for CMPACLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmpaclr::W`](W) writer structure"]
impl crate::Writable for CMPACLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMPACLR to value 0"]
impl crate::Resettable for CMPACLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}