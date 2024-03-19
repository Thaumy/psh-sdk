mod data;
mod run_profiling;

pub use data::*;

use anyhow::Result;
use wasmtime::{Config, Engine, IntoFunc, Linker};

use crate::profiling::Profiling;

pub struct ProfilingRuntime {
    pub(crate) engine: Box<Engine>,
    pub(crate) linker: Linker<Data>,
}

impl Default for ProfilingRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl ProfilingRuntime {
    pub fn new() -> Self {
        let engine = {
            let mut cfg = Config::new();
            cfg.epoch_interruption(true);
            let engine = Engine::new(&cfg).unwrap();
            Box::new(engine)
        };
        let linker = Linker::new(&engine);
        Self { linker, engine }
    }

    pub fn precompile_profiling(&self, profiling: Profiling) -> Result<Profiling> {
        if profiling.is_aot {
            return Ok(profiling);
        }

        let precompiled_bytes = self.engine.precompile_module(&profiling.bytes)?;

        let profiling = unsafe { Profiling::from_precompiled(precompiled_bytes) };
        Ok(profiling)
    }

    pub fn link_op<P, R>(&mut self, name: &str, f: impl IntoFunc<Data, P, R>) -> Result<&mut Self> {
        self.linker.func_wrap("op", name, f)?;
        Ok(self)
    }

    pub fn run_profiling(&self, data: Data, profiling: &Profiling) -> (Data, Result<()>) {
        run_profiling::run_profiling(self, data, profiling)
    }
}
