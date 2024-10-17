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

#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum TransactionStatus {
    #[serde(rename = "Unknown")]
    Unknown,
    #[serde(rename = "CommittedSuccess")]
    CommittedSuccess,
    #[serde(rename = "CommittedFailure")]
    CommittedFailure,
    #[serde(rename = "Pending")]
    Pending,
    #[serde(rename = "Rejected")]
    Rejected,
}

impl ToString for TransactionStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Unknown => String::from("Unknown"),
            Self::CommittedSuccess => String::from("CommittedSuccess"),
            Self::CommittedFailure => String::from("CommittedFailure"),
            Self::Pending => String::from("Pending"),
            Self::Rejected => String::from("Rejected"),
        }
    }
}

impl Default for TransactionStatus {
    fn default() -> TransactionStatus {
        Self::Unknown
    }
}
