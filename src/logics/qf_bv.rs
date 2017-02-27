//! Module that describes QF_BV (closed quatifier-free formulas built over
//! FixedSizeBitVector) logic.
//!
//! Note that the functions and structs that are defined.

use std::fmt;

use theories::{bitvec, core};
use backends::backend::{Logic, SMTNode};

define_sorts_for_logic!(QFBVSorts,
                  BV -> bitvec::Sorts,
                  Core -> core::Sorts
                 );

define_fns_for_logic!(QFBVFn,
                      BVOps -> bitvec::OpCodes,
                      CoreOps -> core::OpCodes
                     );

define_logic!(QFBV,
              QFBVFn,
              QFBVSorts,
              map {
                  QFBVSorts::BV(_) => bitvec::OpCodes::FreeVar
              }
             );
