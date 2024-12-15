use serde::Serialize;

#[derive(Serialize)]
enum TaskStatus {
    Todo,
    Doing,
    Done,
}

#[derive(Serialize)]
struct Task {
    status: TaskStatus,
}

fn main() -> Result<(), std::io::Error> {
    let t = Task {
        status: TaskStatus::Done
    };
    let json = serde_json::to_string(&t)?;
    println!("{}", json);
    Ok(())
}
