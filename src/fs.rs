use std::path::PathBuf;
use std::io::{Error, ErrorKind, Seek, SeekFrom};
use std::fs::{File, OpenOptions};
use crate::Task;

pub fn add_task(journal_path: PathBuf, task: Task) -> Result<(), Error> {
    
    // Open the file.
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;
    
    // Consume the file's contents as a vector of tasks.
    let mut tasks: Vec<Task> = match serde_yaml::from_reader(&file)
    {
        Ok(tasks) => tasks,
        Err(_e) => Vec::new()
    };
    
    // Rewind the file after reading from it.
    file.seek(SeekFrom::Start(0))?;

    // Write the modified task list back into the file.
    tasks.push(task);
    serde_yaml::to_writer(file, &tasks).unwrap();

    Ok(())
}
/*
pub fn complete_task(journal_path: PathBuf, task_position: usize) -> Result<()> {
    // Open the file.
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(journal_path)?;

    // Consume the file's contents as a vector of tasks.
    let mut tasks: Vec<Task> = serde_yaml::from_reader(&file).unwrap();

    // Remove the task.
    if task_position == 0 || task_position > tasks.len() {
        //return Err(Error::new(ErrorKind::InvalidInput, "Invalid Task ID"));
    }
    tasks.remove(task_position - 1);
    
    // Rewind and truncate the file.
    file.seek(SeekFrom::Start(0))?;
    file.set_len(0)?;

    // Write the modified task list back into the file.
    serde_yaml::to_writer(file, &tasks).unwrap();

    Ok(())
}

pub fn list_tasks(journal_path: PathBuf) -> Result<()> {
    // Open the file.
    let file = OpenOptions::new().read(true).open(journal_path)?;
    // Parse the file and collect the tasks.
    let tasks = collect_tasks(&file)?;

    // Enumerate and display tasks, if any.
    if tasks.is_empty() {
        println!("Task list is empty!");
    } else {
        let mut order: u32 = 1;
        for task in tasks {
            println!("{}: {:?}", order, task);
            order += 1;
        }
    }
    
    Ok(())
}

fn collect_tasks(mut file: &File) -> Result<Vec<Task>, dyn Error> {
    file.seek(SeekFrom::Start(0))?; // Rewind the file before.
    let tasks = serde_yaml::from_reader(file).unwrap();
    file.seek(SeekFrom::Start(0))?; // Rewind the file after.

    Ok(tasks)
}
*/