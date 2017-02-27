//! Module that describes LIA logic.
//!
//! Note that the functions and structs that are defined.

use std::fmt;

use theories::{integer, core};
use backends::backend::{Logic, SMTNode};

define_sorts_for_logic!(LIASorts,
                  Int -> integer::Sorts,
                  Core -> core::Sorts
                 );

define_fns_for_logic!(LIAFn,
                      IntOps -> integer::OpCodes,
                      CoreOps -> core::OpCodes
                     );

#[derive(Clone, Copy, Debug)]
pub struct LIA;

impl fmt::Display for LIA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "LIA")
    }
}

impl Logic for LIA {
    type Fns = LIAFn;
    type Sorts = LIASorts;

    fn free_var<T: AsRef<str>>(name: T, ty: LIASorts) -> Self::Fns {
        let fv = match ty {
            LIASorts::Int(_) => integer::OpCodes::FreeVar(name.as_ref().to_owned()),
            LIASorts::Core(_) => unreachable!(),
        };
        LIAFn::IntOps(fv)
    }
}
