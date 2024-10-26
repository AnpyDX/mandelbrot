use std::sync::{ Arc, Mutex };
use crossbeam;

struct Queue<T> {
    data: Vec<Option<T>>,
    index: usize
}

impl<T> Queue<T> {
    fn new() -> Queue<T> {
        Queue {
            data: Vec::new(),
            index: 0
        }
    }

    fn submit(&mut self, element: T) {
        self.data.push(Some(element));
    }

    fn takeout(&mut self) -> Option<T> {
        if self.index == self.data.len() {
            return None;
        }

        println!("[info] accept task id: {}/{}.", self.index + 1, self.data.len());

        self.index += 1;
        return self.data[self.index - 1].take();
    }
}


type Task<'env> = Box<dyn FnOnce() + Send + 'env>;

pub struct Manager<'owner> {
    size: usize,
    tasks: Arc<Mutex<Queue<Task<'owner>>>>,
}

impl<'a> Manager<'a> {
    /// create a new Manger
    pub fn new(size: usize) -> Manager<'a> {
        Manager {
            size,
            tasks: Arc::new(Mutex::new(Queue::new()))
        }
    }

    /// submit a new task to manager
    pub fn submit<T>(&mut self, task: T)
    where
          T: FnOnce() + Send + 'a
    {
        self.tasks.lock().unwrap().submit(Box::new(task));
    }

    /// launch to run all tasks, 
    /// which will consume Manager object
    /// 
    /// FIXME exceuate order behave like stack, not a queue.
    pub fn launch(self) {
        println!("[info] {} tasks generated in total.", self.tasks.lock().unwrap().data.len());

        crossbeam::scope(|scope| {
            for _ in 0..self.size {
                let tasks = Arc::clone(&self.tasks);
                scope.spawn(move |_| {
                    loop {
                        let task = tasks.lock().unwrap().takeout();

                        match task {
                            Some(t) => t(),
                            None => break
                        }
                    }
                });
            }

        }).unwrap();

        println!("[info] all tasks have finished.");
    }
}