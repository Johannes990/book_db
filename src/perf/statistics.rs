use std::{collections::VecDeque, time::Duration};

pub struct StatisticsData {
    pub avg_system_cpu_usage: f32,
    pub avg_system_memory_usage: f32,
    pub avg_process_cpu_usage: f32,
    pub avg_process_memory_usage: f32,
    pub avg_render_call_duration: Duration,
}

struct SizedDeque<T> {
    deque: VecDeque<T>,
    max_size: usize,
}

impl<T> SizedDeque<T> {
    fn new(max_size: usize) -> Self {
        Self {
            deque: VecDeque::new(),
            max_size,
        }
    }

    fn push(&mut self, val: T) {
        if self.deque.len() == self.max_size {
            self.deque.pop_front();
        }
        self.deque.push_back(val);
    }

    fn iter(&self) -> impl Iterator<Item = &T> {
        self.deque.iter()
    }

    fn len(&self) -> usize {
        self.deque.len()
    }
}

trait Mean<T> {
    fn mean(&self) -> Option<f32>;
}

impl Mean<f32> for SizedDeque<f32> {
    fn mean(&self) -> Option<f32> {
        if self.len() == 0 {
            None
        } else {
            Some(self.iter().sum::<f32>() / self.len() as f32)
        }
    }
}

impl Mean<u64> for SizedDeque<u64> {
    fn mean(&self) -> Option<f32> {
        if self.len() == 0 {
            None
        } else {
            Some(self.iter().sum::<u64>() as f32 / self.len() as f32)
        }
    }
}

pub struct StatisticsProfiling {
    system_cpu_readings: SizedDeque<f32>,
    process_cpu_readings: SizedDeque<f32>,
    system_mem_readings: SizedDeque<u64>,
    process_mem_readings: SizedDeque<u64>,
    render_call_durations: SizedDeque<Duration>,
    calculated_data: Option<StatisticsData>,
}

impl StatisticsProfiling {
    pub fn new(buffer_size: usize) -> Self {
        Self {
            system_cpu_readings: SizedDeque::new(buffer_size),
            process_cpu_readings: SizedDeque::new(buffer_size),
            system_mem_readings: SizedDeque::new(buffer_size),
            process_mem_readings: SizedDeque::new(buffer_size),
            render_call_durations: SizedDeque::new(buffer_size),
            calculated_data: None,
        }
    }

    pub fn push_cpu_and_memory_values(
        &mut self,
        sys_cpu: f32,
        sys_mem: u64,
        proc_cpu: f32,
        proc_mem: u64,
    ) {
        self.system_cpu_readings.push(sys_cpu);
        self.system_mem_readings.push(sys_mem);
        self.process_cpu_readings.push(proc_cpu);
        self.process_mem_readings.push(proc_mem);
    }

    pub fn push_render_time(&mut self, render_time: Duration) {
        self.render_call_durations.push(render_time);
    }

    pub fn get_statistics_data(&self) -> &Option<StatisticsData> {
        &self.calculated_data
    }

    pub fn calculate_statistics(&mut self) {
        let avg_system_cpu_usage = self.system_cpu_readings.mean().unwrap_or(0.0);
        let avg_system_memory_usage = self.system_mem_readings.mean().unwrap_or(0.0);
        let avg_process_cpu_usage = self.process_cpu_readings.mean().unwrap_or(0.0);
        let avg_process_memory_usage = self.process_mem_readings.mean().unwrap_or(0.0);
        let avg_render_call_duration = if self.render_call_durations.len() > 0 {
            self.render_call_durations.iter().sum::<Duration>()
                / self.render_call_durations.len() as u32
        } else {
            Duration::ZERO
        };

        self.calculated_data = Some(StatisticsData {
            avg_system_cpu_usage,
            avg_system_memory_usage,
            avg_process_cpu_usage,
            avg_process_memory_usage,
            avg_render_call_duration,
        });
    }
}
