use crate::task::task::{Task, hex_to_rgb_piechart};
use piechart::{Chart, Data};
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus
{
  Canceled = -1,
  Running = 0,
  Complete = 1,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskInstance
{
  pub task: Task,
  pub duration: f32,
  pub status: TaskStatus,
  pub start_time: String,
  pub end_time: String,
}

impl TaskInstance
{
  pub fn complete(&mut self) -> i8
  {
    if self.status != TaskStatus::Running
    {
      return -1
    }

    self.status = TaskStatus::Complete;
    0
  }
}

pub fn draw_chart(tasks: &Vec<TaskInstance>) -> i8 {
  if tasks.len() == 0{
    return -1;
  }    
  
  let mut data: Vec<Data> = Vec::new();
  for t in tasks {
      data.push(Data {
          label: t.task.name.clone(),
          value: t.duration,
          color: Some(hex_to_rgb_piechart(&t.task.color).into()),
          fill: '•',
      });
  }

  Chart::new()
      .radius(11)
      .aspect_ratio(4)
      .legend(true)
      .draw(&data);
  0
}
