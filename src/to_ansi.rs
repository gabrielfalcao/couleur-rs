use crate::{Error, Prefix, Result, AnsiRenderable};
use serde::Serialize;
use std::fmt::Display;

pub trait ToAnsi: Sized + Serialize {
    /// implementors must implement at least
    /// [`ToAnsi::as_ansi_suffix`] for their instances.
    fn as_ansi_suffix(&self) -> String;

    /// [`ToAnsi::ansi_suffix`] is the associated function equivalent
    /// of [`as_ansi_suffix`]. Currently, the only type which can
    /// opt-in a non-none implementation is [`Reset`](crate::Reset)
    /// because it never changes.
    ///
    /// Not sure if this should be here;
    ///
    /// I'm not planning bike-shedding over this any time soon.
    ///
    /// BUT... (and this is a big BUT)
    ///
    /// Here is some food for thought:
    ///
    /// Perhaps enum variants such as [`Layer::BG`] and [`Layer::FG`]
    /// **could** be refactored into distinct unit structs (.e.g.:
    /// `Background` and `Foreground`, respectively) and those types
    /// **could** each implement [`ToAnsi::ansi_suffix`] to return
    /// `Some("[48;")` and `Some("[38;")`, respectively.
    ///
    /// Now, would such approach culminate in **EVERY VARIANT**, of
    /// **EVERY ENUM** in this crate having corresponding unit
    /// structs, implementations of traits such [`From`] and [`Into`]
    /// notwithstanding ?
    ///
    /// I can't answer any of that right now.
    ///
    /// What I can say is that, maybe, just maybe, this
    /// **`ansi_suffix`** excuse for static-method should not exist at
    /// all.
    ///
    /// Sadly, I am currently living too physically far away from
    /// people I used to ask their opinions and stranded from my
    /// passionate coder friends to have such irresistible debates
    /// however long they could drag along.
    ///
    /// Happily, I do not vibe code at all and, frankly, I think that
    /// I would never even waste my time asking any given A.I. model
    /// for anything like this.
    fn ansi_suffix() -> Option<String> {
        None
    }
    fn render_ansi<T: Display>(&self, text: T, prefix: Option<Prefix>) -> String;
}
