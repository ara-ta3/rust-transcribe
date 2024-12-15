use serde::Serialize;
use std::{fs::File, io::Write};

type GenericError = Box<dyn std::error::Error>;
type GenericResult<T> = Result<T, GenericError>;

#[derive(Serialize, Clone)]
enum TaskStatus {
    Todo,
    Doing,
    Done,
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
    fn put(&self, t: Task) -> GenericResult<()>;
}

struct TaskService<R: TaskRepository> {
    task_repository: R,
}

impl <R: TaskRepository> TaskService<R> {
    pub fn new(r: R) -> Self {
        Self {task_repository : r}
    }

    pub fn update(&self, t: Task, next_status: TaskStatus) -> GenericResult<Task> {
        let n = t.updated(next_status)?;
        self.task_repository.put(n.clone())?;
        Ok(n.clone())
    }
}

struct TaskRepositoryJson {
    path: String,
}

impl TaskRepository for TaskRepositoryJson {
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
    let t = Task {
        status: TaskStatus::Done
    };

    match s.update(t, TaskStatus::Doing) {
        Ok(_) => println!("ok"),
        Err(e) => eprintln!("{:?}", e),
    }
}
