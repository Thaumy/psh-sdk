mod raw;
use crate::infra::wasm::{copy_to_vm, to_host_ptr};
use crate::op::perf::convert::Wrap;
use crate::profiling::runtime::Data;
use profiling_prelude_perf_types::config::{Cpu, Process};
use profiling_prelude_perf_types::counting::{Config, CounterStat};
use profiling_prelude_perf_types::{raw_parts_de, ser};
use wasmtime::Caller;

pub fn counter_new(
    mut caller: Caller<Data>,
    ret_area_vm_ptr: u32,
    sered_process_vm_ptr: u32,
    sered_process_len: u32,
    sered_cpu_vm_ptr: u32,
    sered_cpu_len: u32,
    sered_cfg_vm_ptr: u32,
    sered_cfg_len: u32,
) {
    let caller = &mut caller;

    let process: Process = unsafe {
        let ptr = to_host_ptr(caller, sered_process_vm_ptr);
        raw_parts_de(ptr as _, sered_process_len as _)
    };

    let cpu: Cpu = unsafe {
        let ptr = to_host_ptr(caller, sered_cpu_vm_ptr);
        raw_parts_de(ptr as _, sered_cpu_len as _)
    };

    let cfg: Config = unsafe {
        let ptr = to_host_ptr(caller, sered_cfg_vm_ptr);
        raw_parts_de(ptr as _, sered_cfg_len as _)
    };

    let counter_rid =
        raw::counter_new(&process, &cpu, &cfg).map(|it| caller.data_mut().add_resource(it));

    let ret_area = unsafe { &mut *(to_host_ptr(caller, ret_area_vm_ptr) as *mut [u32; 3]) };
    match counter_rid {
        Ok(counter_rid) => {
            ret_area[0] = 1;
            ret_area[1] = counter_rid;
        }
        Err(e) => {
            let e = e.to_string();
            let vm_ptr = unsafe { copy_to_vm(caller, e.as_str()) };
            *ret_area = [0, vm_ptr, e.len() as _];
        }
    }
}

pub fn counter_enable(mut caller: Caller<Data>, ret_area_vm_ptr: u32, counter_rid: u32) {
    let caller = &mut caller;

    let result = caller
        .data()
        .get_resource(counter_rid)
        .ok_or("Invalid rid".to_string())
        .and_then(|it| raw::counter_enable(it).map_err(|e| e.to_string()));

    let ret_area = unsafe { &mut *(to_host_ptr(caller, ret_area_vm_ptr) as *mut [u32; 3]) };
    match result {
        Ok(_) => {
            ret_area[0] = 1;
        }
        Err(e) => {
            let vm_ptr = unsafe { copy_to_vm(caller, e.as_str()) };
            *ret_area = [0, vm_ptr, e.len() as _];
        }
    }
}

pub fn counter_disable(mut caller: Caller<Data>, ret_area_vm_ptr: u32, counter_rid: u32) {
    let caller = &mut caller;

    let result = caller
        .data()
        .get_resource(counter_rid)
        .ok_or("Invalid rid".to_string())
        .and_then(|it| raw::counter_disable(it).map_err(|e| e.to_string()));

    let ret_area = unsafe { &mut *(to_host_ptr(caller, ret_area_vm_ptr) as *mut [u32; 3]) };
    match result {
        Ok(_) => {
            ret_area[0] = 1;
        }
        Err(e) => {
            let vm_ptr = unsafe { copy_to_vm(caller, e.as_str()) };
            *ret_area = [0, vm_ptr, e.len() as _];
        }
    }
}

pub fn counter_reset(mut caller: Caller<Data>, ret_area_vm_ptr: u32, counter_rid: u32) {
    let caller = &mut caller;

    let result = caller
        .data()
        .get_resource(counter_rid)
        .ok_or("Invalid rid".to_string())
        .and_then(|it| raw::counter_reset(it).map_err(|e| e.to_string()));

    let ret_area = unsafe { &mut *(to_host_ptr(caller, ret_area_vm_ptr) as *mut [u32; 3]) };
    match result {
        Ok(_) => {
            ret_area[0] = 1;
        }
        Err(e) => {
            let vm_ptr = unsafe { copy_to_vm(caller, e.as_str()) };
            *ret_area = [0, vm_ptr, e.len() as _];
        }
    }
}

pub fn counter_stat(mut caller: Caller<Data>, ret_area_vm_ptr: u32, counter_rid: u32) {
    let caller = &mut caller;

    let stat = caller
        .data_mut()
        .get_resource_mut(counter_rid)
        .ok_or_else(|| "Invalid rid".to_string())
        .and_then(|it| raw::counter_stat(it).map_err(|e| e.to_string()));

    let ret_area = unsafe { &mut *(to_host_ptr(caller, ret_area_vm_ptr) as *mut [u32; 3]) };
    match stat {
        Ok(stat) => {
            let stat = Wrap::<CounterStat>::from(&stat).into_inner();
            let sered_stat = ser(&stat);
            let vm_ptr = unsafe { copy_to_vm(caller, sered_stat.as_ref()) };

            *ret_area = [1, vm_ptr, sered_stat.len() as _];
        }
        Err(e) => {
            let vm_ptr = unsafe { copy_to_vm(caller, e.as_str()) };
            *ret_area = [0, vm_ptr, e.len() as _];
        }
    }
}