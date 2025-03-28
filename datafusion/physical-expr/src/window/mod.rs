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

mod aggregate;
mod sliding_aggregate;
mod standard;
mod standard_window_function_expr;
mod window_expr;

#[deprecated(since = "44.0.0", note = "use StandardWindowExpr")]
pub type BuiltInWindowExpr = StandardWindowExpr;

#[deprecated(since = "44.0.0", note = "use StandardWindowFunctionExpr")]
pub type BuiltInWindowFunctionExpr = dyn StandardWindowFunctionExpr;

pub use aggregate::PlainAggregateWindowExpr;
pub use sliding_aggregate::SlidingAggregateWindowExpr;
pub use standard::StandardWindowExpr;
pub use standard_window_function_expr::StandardWindowFunctionExpr;
pub use window_expr::PartitionBatches;
pub use window_expr::PartitionKey;
pub use window_expr::PartitionWindowAggStates;
pub use window_expr::WindowExpr;
pub use window_expr::WindowState;
