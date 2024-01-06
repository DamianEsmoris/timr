use crate::models::task::TaskModel;
use crate::task::task::Task;
use crate::task::task_instance::TaskInstance;

pub struct TaskController {}
pub struct TaskError {
    pub code: i8,
    desc: String,
}

impl TaskController {
    pub fn create_task(t: Task) -> Result<Task, TaskError> {
        if TaskModel::task_exits(&t.name) {
            return Err(TaskError{code: -1, desc: "The task already exists".to_string()});
        }

        let _ = TaskModel::create_task(&t);
        return Ok(t)
    }

    pub fn get_tasks() -> Vec<Task> {
        TaskModel::get_tasks().unwrap()
    }

    pub fn start_task(task_name: &str ) -> Result<(), TaskError>{
        if !TaskModel::task_exits(&task_name) {
            return Err(TaskError{code: -1, desc: "The task doesn't exists".to_string()});
        }
        
        if TaskModel::is_task_runing(&task_name) {
            return Err(TaskError{code: -2, desc: "The task is already running".to_string()});
        }
        
        let _ = TaskModel::start_task(&task_name);
        Ok(())
    }

    pub fn stop_task(task_name: &str) -> Result<(), TaskError> {
        if !TaskModel::task_exits(&task_name) {
            return Err(TaskError{code: -1, desc: "The task doesn't exists".to_string()});
        }
        
        if !TaskModel::is_task_runing(&task_name) {
            return Err(TaskError{code: -2, desc: "The task is not running".to_string()});
        }

        let _ = TaskModel::stop_task(&task_name);
        Ok(())
    }

    pub fn update_task_instance(task: &TaskInstance, edited_task: &TaskInstance) -> Result<(), TaskError>{
        if !TaskModel::task_exits(&edited_task.task.name) {
            return Err(TaskError{code: -1, desc: "The task doesn't exists".to_string()});
        }

        let _ = TaskModel::update_task_instance(&task.task.name, &task.start_time, &task.end_time, edited_task); 
        Ok(())
    }

    pub fn add_task_description(task_name: &str, desc: &str, running: bool) -> Result<(), TaskError> {
        let _ = TaskModel::add_task_description(&task_name, desc, running);
        Ok(())
    }

    pub fn get_date_tasks(date: String) -> Vec<TaskInstance> {
        TaskModel::get_date_tasks(&date).unwrap()
    }

    pub fn get_running_tasks() -> Vec<TaskInstance> {
        TaskModel::get_running_tasks().unwrap()
    }

}
