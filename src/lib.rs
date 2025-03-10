/*
 * Copyright (C) 2021 Vaticle
 *
 * Licensed to the Apache Software Foundation (ASF) under one
 * or more contributor license agreements.  See the NOTICE file
 * distributed with this work for additional information
 * regarding copyright ownership.  The ASF licenses this file
 * to you under the Apache License, Version 2.0 (the
 * "License"); you may not use this file except in compliance
 * with the License.  You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing,
 * software distributed under the License is distributed on an
 * "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 * KIND, either express or implied.  See the License for the
 * specific language governing permissions and limitations
 * under the License.
 */

pub mod answer;
pub mod common;
pub mod concept;
pub mod database;
pub mod query;
mod rpc;
pub mod session;
pub mod transaction;

pub use crate::common::Result;
pub use crate::database::DatabaseManager;
pub use crate::database::Database;
pub use crate::session::Session;
pub use crate::transaction::Transaction;

use std::sync::Arc;

use crate::rpc::client::RpcClient;

#[derive(Clone)]
pub struct TypeDBClient {
    pub databases: DatabaseManager,
    pub(crate) rpc_client: Arc<RpcClient>,
}

impl TypeDBClient {
    #[must_use]
    pub async fn new(host: &str, port: u16) -> Result<Self> {
        let rpc_client = Arc::new(RpcClient::new(host, port).await?);
        Ok(TypeDBClient {
            databases: DatabaseManager::new(Arc::clone(&rpc_client)),
            rpc_client,
        })
    }

    pub async fn with_default_address() -> Result<Self> {
        Self::new("0.0.0.0", 1729).await
    }

    #[must_use]
    pub async fn session(&self, db_name: &str, session_type: session::Type) -> Result<Session> {
        Session::new(db_name, session_type, Arc::clone(&self.rpc_client)).await
    }
}
