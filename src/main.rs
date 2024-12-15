use serde::Serialize;
use std::{fs::File, io::Write, os::unix::raw::uid_t};

type GenericError = Box<dyn std::error::Error>;
type GenericResult<T> = Result<T, GenericError>;

#[derive(Serialize, Clone)]
enum TaskStatus {
    Todo,
    Doing,
    Done,
}

struct User {
    tasks: [Task],
}

impl User {
    fn update(&self, next_status: TaskStatus) -> GenericResult<&[Task]> {
        unimplemented!();
    }
}


impl TaskStatus {
    fn updated(&self, next: TaskStatus) -> GenericResult<Self> {
        unimplemented!();
        // match (self, next) {
        //     (TaskStatus::Todo, _) => Ok(next),
        //     (TaskStatus::Doing, TaskStatus::Doing) => Err(std::error::Error )
        // }
    }
}

#[derive(Serialize, Clone)]
struct Task {
    status: TaskStatus,
}

impl Task {
    fn updated(&self, _: TaskStatus) -> GenericResult<&Self> {
        Ok(self)
    }
}

trait TaskRepository {
    fn fetch(&self, user_id: i32) -> GenericResult<&User>;

    fn put(&self, t: Task) -> GenericResult<()>;
}

struct TaskService<R: TaskRepository> {
    task_repository: R,
}

impl <R: TaskRepository> TaskService<R> {
    pub fn new(r: R) -> Self {
        Self {task_repository : r}
    }

    pub fn update(&self, user_id: i32, next_status: TaskStatus) -> GenericResult<()> {
        let u = self.task_repository.fetch(user_id)?;
        let ns = u.update(next_status)?;
        
        for n in ns {
            self.task_repository.put(n.clone())?;
        }
        Ok(())
    }
}

struct TaskRepositoryJson {
    path: String,
}

impl TaskRepository for TaskRepositoryJson {
    fn fetch(&self, user_id: i32) -> GenericResult<&User> {
        unimplemented!()
    }

    fn put(&self, t: Task) -> GenericResult<()> {
        let json = serde_json::to_string(&t)?;
        let mut f = File::create(&self.path)?;
        f.write_all(json.as_bytes())?;
        Ok(())
    }
}


fn main()  {
    let s = TaskService::new(
        TaskRepositoryJson {
            path: "./task.json".to_string(),
        }
    );
    match s.update(1, TaskStatus::Doing) {
        Ok(_) => println!("ok"),
        Err(e) => eprintln!("{:?}", e),
    }
}
