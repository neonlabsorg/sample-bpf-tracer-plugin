use std::sync::Arc;

use solana_bpf_tracer_plugin_interface::bpf_tracer_plugin_interface::{
    BpfTracerPlugin, ExecutorAdditional, Result,
};
use solana_rbpf::static_analysis::TraceLogEntry;
use solana_sdk::{hash::Hash, pubkey::Pubkey};

#[derive(Debug)]
struct SampleBpfTracerPlugin;

impl BpfTracerPlugin for SampleBpfTracerPlugin {
    fn name(&self) -> &'static str {
        "Sample BPF tracing plugin"
    }

    fn trace_bpf(
        &mut self,
        program_id: &Pubkey,
        block_hash: &Hash,
        transaction_id: &[u8],
        trace: &[TraceLogEntry],
        consumed_bpf_units: &[(usize, u64)],
        _executor: Arc<dyn ExecutorAdditional>,
    ) -> Result<()> {
        let transaction_id = solana_sdk::bs58::encode(transaction_id).into_string();
        println!(
            "BPF Tracing for program ID: {}, transaction ID: {}, block hash: {}, {}/{} record(s)",
            program_id,
            transaction_id,
            block_hash,
            trace.len(),
            consumed_bpf_units.len(),
        );

        Ok(())
    }
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
/// # Safety
///
/// This function returns the SampleBpfTracerPlugin pointer as trait BpfTracerPlugin.
pub unsafe extern "C" fn _create_bpf_tracer_plugin() -> *mut dyn BpfTracerPlugin {
    Box::into_raw(Box::new(SampleBpfTracerPlugin))
}
