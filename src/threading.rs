use std::{process, sync::mpsc, thread};
use sysinfo::{Pid, ProcessesToUpdate, System};

use crate::perf::resources::Resources;

pub fn spawn_profiler_thread() -> mpsc::Receiver<Resources> {
    let (tx, rx) = mpsc::channel();
    let pid = process::id();

    thread::spawn(move || {
        let mut sys = System::new_all();
        let thread_count = sys.cpus().len();

        loop {
            thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);

            sys.refresh_cpu_usage();
            sys.refresh_processes(ProcessesToUpdate::All, true);

            if let Some(proc) = sys.process(Pid::from_u32(pid)) {
                let overall_cpu = sys.cpus().iter().map(|c| c.cpu_usage()).sum::<f32>();
                let overall_mem = sys.used_memory();
                let proc_cpu = proc.cpu_usage();
                let proc_mem = proc.memory();

                let measured_resources =
                    Resources::new(overall_cpu, overall_mem, proc_cpu, proc_mem, thread_count);

                if tx.send(measured_resources).is_err() {
                    break;
                }
            }
        }
    });

    rx
}
