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

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StateNonFungibleDataRequest {
    #[serde(
        rename = "at_ledger_state",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub at_ledger_state:
        Option<Option<Box<crate::models::LedgerStateSelector>>>,

    #[serde(rename = "resource_address")]
    pub resource_address: String,

    #[serde(rename = "non_fungible_ids")]
    pub non_fungible_ids: Vec<String>,
}

impl StateNonFungibleDataRequest {
    pub fn new(
        resource_address: String,
        non_fungible_ids: Vec<String>,
    ) -> StateNonFungibleDataRequest {
        StateNonFungibleDataRequest {
            at_ledger_state: None,
            resource_address,
            non_fungible_ids,
        }
    }
}
