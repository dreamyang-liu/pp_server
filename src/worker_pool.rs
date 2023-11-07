pub mod worker_pool {
    use rand::{random, Error};
    pub trait WorkerPoolAbstract {
        fn add_worker(&mut self, num: usize) -> Result<usize, Error>;
        fn acquire_worker(&mut self) -> Result<u16, Error>;
        fn print_status(&self);
        fn get_status(&self) -> String;
    }
    pub struct Worker {
        workload: usize,
        port: u16,
        process: std::process::Child,
    }
    pub struct WorkerPool {
        pub workload_limit: usize,
        pub worker_pool_capacity: usize,
        pub worker_path: String,
        pub workers_list: std::collections::LinkedList<Worker>,
    }

    impl WorkerPoolAbstract for WorkerPool {
        fn add_worker(&mut self, num: usize) -> Result<usize, Error> {
            let previouse_worker_number = self.workers_list.len();
            if previouse_worker_number >= self.worker_pool_capacity {
                return Result::Err(Error::new("worker pool is full!"));
            }
            println!("Current worker pool size: {}", previouse_worker_number);
            let mut num_of_workers_to_add = self.worker_pool_capacity - previouse_worker_number;
            if num < num_of_workers_to_add {
                num_of_workers_to_add = num;
            }
            if previouse_worker_number + num > self.worker_pool_capacity {
                print!("worker pool is full, only {} workers can be added!", num_of_workers_to_add);
            }
            for _ in 1..(num_of_workers_to_add + 1) {
                let min_port = 25000;
                let max_port = 26000;
                let port = random::<u16>() % (max_port - min_port + 1) + min_port;
                let process = std::process::Command::new(&self.worker_path)
                    .arg(port.to_string())
                    .spawn().expect("Failed to spawn the process!");
                self.workers_list.push_back(Worker { workload: 0, port: port, process: process });
            }
            return Result::Ok(self.workers_list.len());
        }
        fn acquire_worker(&mut self) -> Result<u16, Error> {
            let worker_list_empty = self.workers_list.len() == 0;
            if worker_list_empty {
                self.add_worker(1).expect("Worker pool is empty and failed to add worker!");
            }
            for worker in self.workers_list.iter_mut() {
                if worker.workload < self.workload_limit {
                    worker.workload += 1;
                    return Result::Ok(worker.port);
                }
            }
            if self.workers_list.len() >= self.worker_pool_capacity {
                return Result::Err(Error::new("worker pool is full! And all workers are busy!"));
            }
            self.add_worker(1).expect("Failed to add worker to the worker pool!");
            let last_worker = self.workers_list.back_mut().unwrap();
            last_worker.workload += 1;
            return Result::Ok(last_worker.port);
        }

        fn print_status(&self) {
            println!("worker pool capacity: {}", self.worker_pool_capacity);
            println!("worker pool size: {}", self.workers_list.len());
            println!("worker pool workload limit: {}", self.workload_limit);
            for worker in self.workers_list.iter() {
                println!("worker port: {}, workload: {}", worker.port, worker.workload);
            }
        }

        fn get_status(&self) -> String {
            let mut status = String::new();
            status.push_str(format!("<p>worker pool capacity: {}\n<\
            p>", self.worker_pool_capacity).as_str());
            status.push_str(format!("<p>worker pool size: {}\n<\
            p>", self.workers_list.len()).as_str());
            status.push_str(format!("<p>worker pool workload limit: {}\n<\
            p>", self.workload_limit).as_str());
            for worker in self.workers_list.iter() {
                status.push_str(format!("<p>worker port: {}, workload: {}\n<\
                p>", worker.port, worker.workload).as_str());
            }
            return status;
        }
    }

}