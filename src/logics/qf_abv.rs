use std::fmt;

use theories::{array_ex, bitvec, core};
use backends::backend::{Logic, SMTNode};

define_sorts_for_logic!(QFABVSorts,
                        BV -> bitvec::Sorts,
                        Core -> core::Sorts,
                        ArrayEx -> array_ex::Sorts<QFABVSorts, QFABVSorts>
                        );

define_fns_for_logic!(QFABVFn,
                      BVOps -> bitvec::OpCodes,
                      CoreOps -> core::OpCodes,
                      ArrayOps -> array_ex::OpCodes<QFABVSorts, QFABVSorts, QFABVFn>
                      );

define_logic!(QFABV,
              QFABVFn,
              QFABVSorts,
              map { QFABVSorts::BV(_) => bitvec::OpCodes::FreeVar,
              QFABVSorts::ArrayEx(_) => array_ex::OpCodes::FreeVar
              }
              );


// Helper methods that help in contruction of array and bitvector types.
pub fn array_sort<T: Into<QFABVSorts>, P: Into<QFABVSorts>>(idx: T, data: P) -> QFABVSorts {
    QFABVSorts::ArrayEx(array_ex::Sorts::new(idx.into(), data.into()))
}

pub fn bv_sort(size: usize) -> QFABVSorts {
    QFABVSorts::BV(bitvec::Sorts::BitVector(size))
}

pub fn bv_const(val: u64, size: usize) -> QFABVFn {
    QFABVFn::BVOps(bitvec::OpCodes::Const(val, size))
}

pub fn array_const<T, P, Q>(idx_ty: T, val_ty: P, val: Q) -> QFABVFn
    where T: Into<QFABVSorts>,
          P: Into<QFABVSorts>,
          Q: Into<QFABVFn>
{
    let arr_ty = array_ex::Sorts::Array(Box::new(idx_ty.into()), Box::new(val_ty.into()));
    let arr_const = array_ex::OpCodes::Const(arr_ty, Box::new(val.into()));
    QFABVFn::ArrayOps(arr_const)
}
