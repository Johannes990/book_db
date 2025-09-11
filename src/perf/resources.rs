use core::fmt;

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum MemoryDivisor {
    B = 0x00000001,
    KB = 0x00000400,
    MB = 0x00100000,
    GB = 0x40000000,
}

impl MemoryDivisor {
    fn as_f32(self) -> f32 {
        self as u32 as f32
    }
}

impl fmt::Display for MemoryDivisor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MemoryDivisor::B => write!(f, "B"),
            MemoryDivisor::KB => write!(f, "KB"),
            MemoryDivisor::MB => write!(f, "MB"),
            MemoryDivisor::GB => write!(f, "GB"),
        }
    }
}

#[allow(dead_code)]
pub enum CpuUsage {
    PerThread,
    PerAll,
}

pub struct Resources {
    pub global_used_cpu: f32,
    pub global_used_memory: u64,
    pub process_used_cpu: f32,
    pub process_used_memory: u64,
    pub thread_count: usize,
    display_memory_as: MemoryDivisor,
    cpu_relative_to: CpuUsage,
    display_global: bool,
    display_process: bool,
    cpu_precision: usize,
    memory_precision: usize,
}

impl fmt::Display for Resources {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cpu_divisor = match self.cpu_relative_to {
            CpuUsage::PerThread => 1.0,
            CpuUsage::PerAll => self.thread_count as f32,
        };

        let format_cpu = |val: f32, divisor: f32| -> String {
            match self.cpu_relative_to {
                CpuUsage::PerAll => format!(
                    "{:.prec_cpu$} of {} threads",
                    val / divisor,
                    self.thread_count,
                    prec_cpu = self.cpu_precision
                ),
                CpuUsage::PerThread => format!(
                    "{:.prec_cpu$} of 1 thread",
                    val / divisor,
                    prec_cpu = self.cpu_precision
                ),
            }
        };

        let memory_divisor = self.display_memory_as.as_f32();
        let memory_unit = self.display_memory_as.to_string();

        let format_memory = |val: u64, divisor: f32| -> String {
            match self.display_memory_as {
                MemoryDivisor::B => format!("{} {}", val, memory_unit),
                _ => format!(
                    "{:.prec_mem$} {}",
                    val as f32 / divisor,
                    memory_unit,
                    prec_mem = self.memory_precision
                ),
            }
        };

        if self.display_global {
            writeln!(
                f,
                "System usage: CPU {}, Memory {}",
                format_cpu(self.global_used_cpu, cpu_divisor),
                format_memory(self.global_used_memory, memory_divisor),
            )?;
        }

        if self.display_process {
            writeln!(
                f,
                "Process usage: CPU {}, Memory: {}",
                format_cpu(self.process_used_cpu, cpu_divisor),
                format_memory(self.process_used_memory, memory_divisor)
            )?;
        }

        Ok(())
    }
}

impl Resources {
    pub fn new(
        global_used_cpu: f32,
        global_used_memory: u64,
        process_used_cpu: f32,
        process_used_memory: u64,
        thread_count: usize,
    ) -> Self {
        Self {
            global_used_cpu,
            global_used_memory,
            process_used_cpu,
            process_used_memory,
            thread_count,
            display_memory_as: MemoryDivisor::MB,
            cpu_relative_to: CpuUsage::PerAll,
            display_global: true,
            display_process: true,
            cpu_precision: 2,
            memory_precision: 2,
        }
    }

    #[allow(dead_code)]
    pub fn set_memory_divisor(&mut self, divisor: MemoryDivisor) {
        self.display_memory_as = divisor;
    }

    #[allow(dead_code)]
    pub fn set_cpu_relative_to(&mut self, cpu_relative: CpuUsage) {
        self.cpu_relative_to = cpu_relative;
    }

    #[allow(dead_code)]
    pub fn set_cpu_precision(&mut self, precision: usize) {
        self.cpu_precision = precision;
    }

    #[allow(dead_code)]
    pub fn set_memory_precision(&mut self, precision: usize) {
        self.memory_precision = precision;
    }
}
