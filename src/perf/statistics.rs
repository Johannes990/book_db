use std::{collections::VecDeque, time::Duration};

pub struct StatisticsData {
    pub avg_system_cpu_usage: f32,
    pub avg_system_memory_usage: f32,
    pub avg_process_cpu_usage: f32,
    pub avg_process_memory_usage: f32,
    pub avg_render_call_duration: Duration,
}

pub struct StatisticsProfiling {
    system_cpu_readings: VecDeque<f32>,
    process_cpu_readings: VecDeque<f32>,
    system_mem_readings: VecDeque<u64>,
    process_mem_readings: VecDeque<u64>,
    render_call_durations: VecDeque<Duration>,
    buffer_size: usize,
}

impl StatisticsProfiling {
    pub fn new(buffer_size: usize) -> Self {
        Self {
            system_cpu_readings: VecDeque::new(),
            process_cpu_readings: VecDeque::new(),
            system_mem_readings: VecDeque::new(),
            process_mem_readings: VecDeque::new(),
            render_call_durations: VecDeque::new(),
            buffer_size,
        }
    }

    pub fn push_cpu_and_memory_values(
        &mut self,
        sys_cpu: f32,
        sys_mem: u64,
        proc_cpu: f32,
        proc_mem: u64,
    ) {
        self.system_cpu_readings.push_back(sys_cpu);

        if self.system_cpu_readings.len() > self.buffer_size {
            self.system_cpu_readings.pop_front();
        }

        self.system_mem_readings.push_back(sys_mem);

        if self.system_mem_readings.len() > self.buffer_size {
            self.system_mem_readings.pop_front();
        }

        self.process_cpu_readings.push_back(proc_cpu);

        if self.process_cpu_readings.len() > self.buffer_size {
            self.process_cpu_readings.pop_front();
        }

        self.process_mem_readings.push_back(proc_mem);

        if self.process_mem_readings.len() > self.buffer_size {
            self.process_mem_readings.pop_front();
        }
    }

    pub fn push_render_time(&mut self, render_time: Duration) {
        self.render_call_durations.push_back(render_time);

        if self.render_call_durations.len() > self.buffer_size {
            self.render_call_durations.pop_front();
        }
    }

    pub fn get_statistics_data(&self) -> StatisticsData {
        let sys_cpu_div = if self.system_cpu_readings.len() > 0 {
            self.system_cpu_readings.len() as f32
        } else {
            1.0
        };
        let sys_mem_div = if self.system_mem_readings.len() > 0 {
            self.system_mem_readings.len() as f32
        } else {
            1.0
        };
        let proc_cpu_div = if self.process_cpu_readings.len() > 0 {
            self.process_cpu_readings.len() as f32
        } else {
            1.0
        };
        let proc_mem_div = if self.process_mem_readings.len() > 0 {
            self.process_mem_readings.len() as f32
        } else {
            1.0
        };
        let render_call_div = if self.render_call_durations.len() > 0 {
            self.render_call_durations.len()
        } else {
            1
        };
        let avg_system_cpu_usage = self.system_cpu_readings.iter().sum::<f32>() / sys_cpu_div;
        let avg_system_memory_usage =
            self.system_mem_readings.iter().sum::<u64>() as f32 / sys_mem_div;
        let avg_process_cpu_usage = self.process_cpu_readings.iter().sum::<f32>() / proc_cpu_div;
        let avg_process_memory_usage =
            self.process_mem_readings.iter().sum::<u64>() as f32 / proc_mem_div;
        let avg_render_call_duration = self.render_call_durations.iter().sum::<Duration>()
            / render_call_div.try_into().unwrap();

        StatisticsData {
            avg_system_cpu_usage,
            avg_system_memory_usage,
            avg_process_cpu_usage,
            avg_process_memory_usage,
            avg_render_call_duration,
        }
    }
}
