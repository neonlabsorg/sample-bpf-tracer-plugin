use solana_bpf_tracer_plugin_interface::bpf_tracer_plugin_interface::{BpfTracerPlugin, Result};
use solana_rbpf::vm::{TraceAnalyzer, TraceItem};
use solana_sdk::{hash::Hash, pubkey::Pubkey};

#[derive(Debug)]
struct SampleBpfTracerPlugin;

impl BpfTracerPlugin for SampleBpfTracerPlugin {
    fn name(&self) -> &'static str {
        "Sample BPF tracing plugin"
    }

    fn trace_bpf<'a>(
        &mut self,
        program_id: &Pubkey,
        block_hash: &Hash,
        transaction_id: &[u8],
        _trace_analyzer: &TraceAnalyzer,
        trace: &[TraceItem],
    ) -> Result<()> {
        let transaction_id = solana_sdk::bs58::encode(transaction_id).into_string();
        println!(
            "BPF Tracing for program ID: {}, transaction ID: {}, block hash: {}, {} record(s)",
            program_id,
            transaction_id,
            block_hash,
            trace.len()
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
