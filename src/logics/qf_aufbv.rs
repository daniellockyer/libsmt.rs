use std::fmt;

use theories::{array_ex, bitvec, core};
use backends::backend::{Logic, SMTNode};

define_sorts_for_logic!(QFAUFBVSorts,
                        BV -> bitvec::Sorts,
                        Core -> core::Sorts,
                        ArrayEx -> array_ex::Sorts<QFAUFBVSorts, QFAUFBVSorts>
                        );

define_fns_for_logic!(QFAUFBVFn,
                      BVOps -> bitvec::OpCodes,
                      CoreOps -> core::OpCodes,
                      ArrayOps -> array_ex::OpCodes<QFAUFBVSorts, QFAUFBVSorts, QFAUFBVFn>
                      );

define_logic!(QFAUFBV,
              QFAUFBVFn,
              QFAUFBVSorts,
              map { QFAUFBVSorts::BV(_) => bitvec::OpCodes::FreeVar,
                  QFAUFBVSorts::ArrayEx(_) => array_ex::OpCodes::FreeVar
              }
              );
