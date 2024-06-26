#![cfg_attr(rustfmt, rustfmt_skip)]
use crate::imports::profiling::perf;

// profiling:perf/config
pub type Event             = perf::config::Event;
pub type HardwareEvent     = perf::config::HardwareEvent;
pub type SoftwareEvent     = perf::config::SoftwareEvent;
pub type RawEvent          = perf::config::RawEvent;
pub type TracepointEvent   = perf::config::TracepointEvent;
pub type BreakpointEvent   = perf::config::BreakpointEvent;
pub type BreakpointType    = perf::config::BreakpointType;
pub type BreakpointLen     = perf::config::BreakpointLen;
pub type DynamicPmuEvent   = perf::config::DynamicPmuEvent;
pub type DpOtherConfig     = perf::config::DpOtherConfig;
pub type DpKprobeConfig    = perf::config::DpKprobeConfig;
pub type DpKprobeConfigVar = perf::config::DpKprobeConfigVar;
pub type DpUprobeConfig    = perf::config::DpUprobeConfig;
pub type CacheOp           = perf::config::CacheOp;
pub type CacheOpResult     = perf::config::CacheOpResult;
pub type EventScope        = perf::config::EventScope;
pub type ExtraConfig       = perf::config::ExtraConfig;
pub type Config            = perf::config::Config;
pub type Process           = perf::config::Process;
pub type Cpu               = perf::config::Cpu;

// profiling:perf/counter
pub type Counter     = perf::counter::Counter;
pub type CounterStat = perf::counter::CounterStat;

// profiling:perf/counter-group
pub type CounterGroup      = perf::counter_group::CounterGroup;
pub type FixedCounterGroup = perf::counter_group::FixedCounterGroup;
pub type CounterGroupStat  = perf::counter_group::CounterGroupStat;
pub type CounterGuard      = perf::counter_group::CounterGuard;
