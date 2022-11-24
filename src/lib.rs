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
        blockhash: &Hash,
        _trace_analyzer: &TraceAnalyzer,
        trace: &[TraceItem],
    ) -> Result<()> {
        println!(
            "BPF Tracing for program ID: {}, block hash: {}, {} record(s)",
            program_id,
            blockhash,
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
