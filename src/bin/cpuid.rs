extern crate raw_cpuid;

use raw_cpuid::CpuId;

fn main() {
    let cpuid = CpuId::new();
    // Implement Display for each of those structs
    cpuid.get_vendor_info().map(|info| {
        println!("Vendor");
        println!("{}", info);
    });
    cpuid.get_feature_info().map(|info| {
        println!("Feature");
        println!("{:?}", info);
    });
    cpuid.get_cache_info().map(|info| {
        println!("Cache");
        println!("{:?}", info);
    });
    cpuid.get_processor_serial().map(|info| {
        println!("Processor Serial");
        println!("{:?}", info);
    });
    cpuid.get_cache_parameters().map(|info| {
        println!("Cache Parameters");
        println!("{:?}", info);
    });
    cpuid.get_monitor_mwait_info().map(|info| {
        println!("Monitor/MWait");
        println!("{:?}", info);
    });
    cpuid.get_thermal_power_info().map(|info| {
        println!("Thermal Power");
        println!("{:?}", info);
    });
    cpuid.get_extended_feature_info().map(|info| {
        println!("Extended Features");
        println!("{:?}", info);
    });
    cpuid.get_direct_cache_access_info().map(|info| {
        println!("Direct Cache Access");
        println!("{:?}", info);
    });
    cpuid.get_performance_monitoring_info().map(|info| {
        println!("Performance Monitoring");
        println!("{:?}", info);
    });
    cpuid.get_extended_topology_info().map(|info| {
        println!("Extended Topology");
        println!("{:?}", info);
    });
    cpuid.get_extended_state_info().map(|info| {
        println!("Extended State");
        println!("{:?}", info);
    });
    cpuid.get_qos_info().map(|info| {
        println!("Quality of Service");
        println!("{:?}", info);
    });
    cpuid.get_qos_enforcement_info().map(|info| {
        println!("Quality of Service Enforcement");
        println!("{:?}", info);
    });
    cpuid.get_processor_trace_info().map(|info| {
        println!("Processor Trace");
        println!("{:?}", info);
    });
    cpuid.get_tsc_info().map(|info| {
        println!("TSC");
        println!("{:?}", info);
    });
    cpuid.get_processor_frequency_info().map(|info| {
        println!("Processor Frequency");
        println!("{:?}", info);
    });
    cpuid.get_soc_vendor_info().map(|info| {
        println!("SoC Vendor Info");
        println!("{:?}", info);
    });
    cpuid.get_extended_function_info().map(|info| {
        println!("Extended Function Info");
        println!("{:?}", info);
    });
}
