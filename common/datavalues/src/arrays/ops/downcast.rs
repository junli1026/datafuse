// Copyright 2020-2021 The Datafuse Authors.
//
// SPDX-License-Identifier: Apache-2.0.

use common_arrow::arrow::array::Array;
use common_arrow::arrow::array::BooleanArray;
use common_arrow::arrow::array::LargeListArray;
use common_arrow::arrow::array::LargeStringArray;
use common_arrow::arrow::array::PrimitiveArray;

use crate::arrays::DataArray;
use crate::series::IntoSeries;
use crate::series::Series;
use crate::DFBooleanArray;
use crate::DFListArray;
use crate::DFPrimitiveType;
use crate::DFUtf8Array;

impl<T> DataArray<T>
where T: DFPrimitiveType
{
    pub fn downcast_ref(&self) -> &PrimitiveArray<T> {
        let arr = &*self.array;
        unsafe { &*(arr as *const dyn Array as *const PrimitiveArray<T>) }
    }

    pub fn downcast_iter(&self) -> impl Iterator<Item = Option<T::Native>> + DoubleEndedIterator {
        let arr = &*self.array;
        let arr = unsafe { &*(arr as *const dyn Array as *const PrimitiveArray<T>) };
        arr.iter()
    }
}

impl DFBooleanArray {
    pub fn downcast_ref(&self) -> &BooleanArray {
        let arr = &*self.array;
        unsafe { &*(arr as *const dyn Array as *const BooleanArray) }
    }

    pub fn downcast_iter(&self) -> impl Iterator<Item = Option<bool>> + DoubleEndedIterator {
        let arr = &*self.array;
        let arr = unsafe { &*(arr as *const dyn Array as *const BooleanArray) };
        arr.iter()
    }
}

impl DFUtf8Array {
    pub fn downcast_ref(&self) -> &LargeStringArray {
        let arr = &*self.array;
        unsafe { &*(arr as *const dyn Array as *const LargeStringArray) }
    }

    pub fn downcast_iter<'a>(&self) -> impl Iterator<Item = Option<&'a str>> + DoubleEndedIterator {
        let arr = &*self.array;
        let arr = unsafe { &*(arr as *const dyn Array as *const LargeStringArray) };
        arr.iter()
    }
}

impl DFListArray {
    pub fn downcast_ref(&self) -> &LargeListArray {
        let arr = &*self.array;
        unsafe { &*(arr as *const dyn Array as *const LargeListArray) }
    }

    pub fn downcast_iter<'a>(&self) -> impl Iterator<Item = Option<Series>> + DoubleEndedIterator {
        let arr = &*self.array;
        let arr = unsafe { &*(arr as *const dyn Array as *const LargeListArray) };

        arr.iter().map(|a| match a {
            Some(a) => Some(a.into_series()),
            None => None,
        })
    }
}
