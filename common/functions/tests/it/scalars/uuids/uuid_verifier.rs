// Copyright 2021 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use common_datavalues::prelude::*;
use common_exception::Result;
use common_functions::scalars::*;

use crate::scalars::scalar_function2_test::test_scalar_functions;
use crate::scalars::scalar_function2_test::ScalarFunctionTest;

#[test]
fn test_uuid_is_empty_functions() -> Result<()> {
    let tests = vec![ScalarFunctionTest {
        name: "is-empty-uuid-passed",
        columns: vec![Series::from_data(vec![
            "00000000-0000-0000-0000-000000000000",
            "5",
        ])],
        expect: Series::from_data(vec![true, true]),
        error: "",
    }];

    test_scalar_functions(UUIDIsEmptyFunction::try_create("")?, &tests, false)
}

#[test]
fn test_uuid_is_not_empty_functions() -> Result<()> {
    let tests = vec![ScalarFunctionTest {
        name: "is-not-empty-uuid-passed",
        columns: vec![Series::from_data(vec![
            "59b69da3-81d0-4db2-96e8-3e20b505a7b2",
            "5",
        ])],
        expect: Series::from_data(vec![true, false]),
        error: "",
    }];

    test_scalar_functions(UUIDIsNotEmptyFunction::try_create("")?, &tests, false)
}
