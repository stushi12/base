use std::time::Duration;

use base_consensus_derive::AttributesBuilder;

use crate::{
    Conductor, Metrics, OriginSelector, SequencerActor, SequencerEngineClient,
    UnsafePayloadGossipClient,
};

/// `SequencerActor` metrics-related method implementations.
impl<
    AttributesBuilder_,
    Conductor_,
    OriginSelector_,
    SequencerEngineClient_,
    UnsafePayloadGossipClient_,
>
    SequencerActor<
        AttributesBuilder_,
        Conductor_,
        OriginSelector_,
        SequencerEngineClient_,
        UnsafePayloadGossipClient_,
    >
where
    AttributesBuilder_: AttributesBuilder,
    Conductor_: Conductor,
    OriginSelector_: OriginSelector,
    SequencerEngineClient_: SequencerEngineClient,
    UnsafePayloadGossipClient_: UnsafePayloadGossipClient,
{
    /// Updates the metrics for the sequencer actor.
    pub(super) fn update_metrics(&self) {
        // no-op if disabled.
        #[cfg(feature = "metrics")]
        {
            let state_flags: [(&str, String); 2] = [
                ("active", self.is_active.to_string()),
                ("recovery", self.recovery_mode.get().to_string()),
            ];

            let gauge = metrics::gauge!("base_node.sequencer_state", &state_flags);
            gauge.set(1);
        }
    }
}

#[inline]
pub(super) fn update_attributes_build_duration_metrics(_duration: Duration) {
    Metrics::sequencer_attributes_build_duration().set(_duration);
}

#[inline]
pub(super) fn update_block_build_duration_metrics(_duration: Duration) {
    Metrics::sequencer_block_building_start_task_duration().set(_duration);
}

#[inline]
pub(super) fn update_seal_duration_metrics(_duration: Duration) {
    Metrics::sequencer_block_building_seal_task_duration().set(_duration);
}

#[inline]
pub(super) fn update_total_transactions_sequenced(_transaction_count: u64) {
    Metrics::sequencer_total_transactions_sequenced().increment(_transaction_count);
}

#[inline]
pub(super) fn inc_seal_step_retry(_step: &'static str) {
    Metrics::sequencer_seal_step_retries_total(_step).increment(1);
}

#[inline]
pub(super) fn update_seal_step_duration(_step: &'static str, _duration: Duration) {
    Metrics::sequencer_seal_step_duration(_step).set(_duration);
}

#[inline]
pub(super) fn inc_seal_error(fatal: bool) {
    let label = if fatal { "true" } else { "false" };
    Metrics::sequencer_seal_errors_total(label).increment(1);
}

#[inline]
pub(super) fn inc_start_rejected(_reason: &'static str) {
    Metrics::sequencer_start_rejected_total(_reason).increment(1);
}

#[inline]
pub(super) fn inc_stop_deferred() {
    Metrics::sequencer_stop_deferred_total().increment(1);
}

#[inline]
pub(super) fn inc_recovery_mode_block() {
    Metrics::sequencer_recovery_mode_blocks_total().increment(1);
}

#[inline]
pub(super) fn inc_drift_empty_block() {
    Metrics::sequencer_drift_empty_blocks_total().increment(1);
}

#[inline]
pub(super) fn inc_stale_build_discarded() {
    Metrics::sequencer_stale_build_discarded_total().increment(1);
}
