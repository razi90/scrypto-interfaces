// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

//! Defines the interface of the adapters used to communicate with pools.
use common::prelude::*;
use scrypto::prelude::*;
use scrypto_interface::*;

define_interface! {
    PoolAdapter impl [
        #[cfg(feature = "trait")]
        Trait,
        #[cfg(feature = "scrypto-stubs")]
        ScryptoStub,
        #[cfg(feature = "scrypto-test-stubs")]
        ScryptoTestStub,
    ] {
        /// Swap one asset into another
        fn swap(&mut self, pool_address: ComponentAddress, input_bucket: Bucket) -> (Bucket, Bucket);

        /// Returns the price of the pair of assets in the pool.
        fn price(&mut self, pool_address: ComponentAddress) -> Price;

        /// The addresses of the pool's resources.
        fn resource_addresses(
            &mut self,
            pool_address: ComponentAddress
        ) -> (ResourceAddress, ResourceAddress);
    }
}
