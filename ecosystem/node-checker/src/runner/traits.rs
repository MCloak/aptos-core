// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use anyhow::{Error, Result};
use async_trait::async_trait;
use thiserror::Error as ThisError;

use crate::{
    evaluator::EvaluationSummary,
    evaluators::{
        metrics::MetricsEvaluatorError, system_information::SystemInformationEvaluatorError,
    },
    metric_collector::{MetricCollector, MetricCollectorError},
};

#[derive(Debug, ThisError)]
pub enum RunnerError {
    /// We failed to collect metrics for some reason.
    #[error("Failed to collect metrics: {0}")]
    MetricCollectorError(MetricCollectorError),

    /// We couldn't parse the metrics.
    #[error("Failed to parse metrics")]
    ParseMetricsError(Error),

    /// One of the metrics evaluators failed. This is not the same as a poor score from
    /// an evaluator, this is an actual failure in the evaluation process.
    #[error("Failed to evaluate metrics")]
    MetricEvaluatorError(MetricsEvaluatorError),

    /// One of the system information evaluators failed. This is not the same
    /// as a poor score from an evaluator, this is an actual failure in the
    /// evaluation process.
    #[error("Failed to evaluate system information")]
    SystemInformationEvaluatorError(SystemInformationEvaluatorError),
}

/// This trait describes a Runner, something that can take in instances of other
/// necessary traits, such as a metric collector for the baseline node, and then,
/// upon a `run` call, drive a node evaluation end to end. This is the top level
/// entrypoint to the core functionality of NHC, it should be hooked up fairly
/// directly to the API endpoint handlers.
///
/// Note on trait bounds:
///  - Sync + Send is required because this will be a member of the Api which
///    needs to be used across thread boundaries.
///  - The 'static lifetime is required because this will be stored on the Api
///    which needs to be 'static.
#[async_trait]
pub trait Runner: Sync + Send + 'static {
    async fn run<M: MetricCollector>(
        &self,
        target_collector: &M,
    ) -> Result<EvaluationSummary, RunnerError>;
}