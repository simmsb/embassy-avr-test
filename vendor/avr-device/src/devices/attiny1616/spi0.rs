#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ctrla: CTRLA,
    ctrlb: CTRLB,
    intctrl: INTCTRL,
    intflags: INTFLAGS,
    data: DATA,
}
impl RegisterBlock {
    #[doc = "0x00 - Control A"]
    #[inline(always)]
    pub const fn ctrla(&self) -> &CTRLA {
        &self.ctrla
    }
    #[doc = "0x01 - Control B"]
    #[inline(always)]
    pub const fn ctrlb(&self) -> &CTRLB {
        &self.ctrlb
    }
    #[doc = "0x02 - Interrupt Control"]
    #[inline(always)]
    pub const fn intctrl(&self) -> &INTCTRL {
        &self.intctrl
    }
    #[doc = "0x03 - Interrupt Flags"]
    #[inline(always)]
    pub const fn intflags(&self) -> &INTFLAGS {
        &self.intflags
    }
    #[doc = "0x04 - Data"]
    #[inline(always)]
    pub const fn data(&self) -> &DATA {
        &self.data
    }
}
#[doc = "CTRLA (rw) register accessor: Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrla::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`]
module"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "CTRLB (rw) register accessor: Control B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlb`]
module"]
pub type CTRLB = crate::Reg<ctrlb::CTRLB_SPEC>;
#[doc = "Control B"]
pub mod ctrlb;
#[doc = "DATA (rw) register accessor: Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`]
module"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "Data"]
pub mod data;
#[doc = "INTCTRL (rw) register accessor: Interrupt Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intctrl`]
module"]
pub type INTCTRL = crate::Reg<intctrl::INTCTRL_SPEC>;
#[doc = "Interrupt Control"]
pub mod intctrl;
#[doc = "INTFLAGS (rw) register accessor: Interrupt Flags\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intflags::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intflags::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intflags`]
module"]
pub type INTFLAGS = crate::Reg<intflags::INTFLAGS_SPEC>;
#[doc = "Interrupt Flags"]
pub mod intflags;
