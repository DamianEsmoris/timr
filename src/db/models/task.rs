use crate::connector::Db;
use crate::task::task::{Task};
use crate::task::task_instance::{TaskInstance, TaskStatus};
use rusqlite::{params_from_iter, params, Result};

pub struct TaskModel {}

impl TaskModel {
    pub fn create_task(t: &Task) -> Result<()> {
        let db = Db::new().unwrap();
        db.conn
            .execute(
                "INSERT INTO task (name, color) VALUES (?1, ?2)",
                (&t.name, &t.color),
            )
            .expect("Error in task recording!");

        Ok(())
    }

    pub fn task_exits(task_name: &str) -> bool {
        let db = Db::new().unwrap();

        let result: Result<String> = db.conn.query_row(
            "SELECT name FROM task
            WHERE name = ?1",
            &[task_name],
            |row| row.get(0),
        );

        match result {
            Ok(name) => true,
            Err(_) => false,
        }
   }


    pub fn get_tasks() -> Result<Vec<Task>> {
        let db = Db::new().unwrap();
        let mut statement = db
            .conn
            .prepare("SELECT name,color FROM task")
            .expect("Cannot create the statement");

        let tasks = statement
            .query_map([], |row| {
                Ok(Task {
                    name: row.get(0)?,
                    color: row.get(1)?,
                })
            })
            .and_then(|mapped_rows| mapped_rows.collect());
        tasks
   }

    pub fn start_task(task_name: &str) -> Result<()> {
        let db = Db::new().unwrap();
        db.conn
            .execute(
                "INSERT INTO history (name, start_date)
                VALUES(?1, (SELECT datetime(CURRENT_TIMESTAMP, 'localtime')))",
                params_from_iter(&[task_name]),
            )
            .expect(&format!("Error, cannot start the task; {}", &task_name));

        Ok(())
    }

    pub fn stop_task(task_name: &str) -> Result<()> {
        let db = Db::new().unwrap();
        db.conn
            .execute(
                "UPDATE history SET end_date = (SELECT datetime(CURRENT_TIMESTAMP, 'localtime'))
                WHERE end_date is NULL AND name = ?1",
                params_from_iter(&[task_name]),
            )
            .expect(&format!("Error, cannot stop the task; {}", &task_name));
        Ok(())
    }

    pub fn is_task_runing(task_name: &str) -> bool {
        let db = Db::new().unwrap();

        let result: Result<String> = db.conn.query_row(
            "SELECT name FROM history
            WHERE name = ?1 AND end_date IS NULL LIMIT 1",
            &[task_name],
            |row| row.get(0),
        );

        match result {
            Ok(name) => true,
            Err(_) => false,
        }
   }

    pub fn get_date_tasks(date: &str) -> Result<Vec<TaskInstance>> {
        let db = Db::new().unwrap();

        let mut statement = db
            .conn
            .prepare(
                "SELECT s.name, color, strftime('%H:%M', start_date) AS started,
                CASE WHEN end_date IS NULL THEN 'Null'
                ELSE strftime('%H:%M', end_date)
                END AS ended,
                
                CASE WHEN end_date IS NULL THEN
                CAST((JULIANDAY((SELECT datetime(CURRENT_TIMESTAMP, 'localtime'))) - JULIANDAY(start_date)) * 1440 AS INT)
                ELSE CAST((JULIANDAY(end_date) - JULIANDAY(start_date)) * 1440 AS INT)
                END AS time
                
                FROM history s, task t
                WHERE s.name = t.name AND DATE(start_date) = ?1
                ORDER BY started ASC;
                "
            )
            .expect("Cannot create the statement");

        //statement.execute(&[date])?;
        let tasks = statement
            .query_map(params![date], |row| {
                Ok(
                TaskInstance {
                    task: Task {
                        name: row.get(0)?,
                        color: row.get(1)?,
                    },

                    start_time: row.get(2)?,
                    end_time: row.get(3)?,
                    duration: row.get(4)?,
                    status:  match row.get::<usize, String>(3)?.as_str() {
                        "Null" => TaskStatus::Running,
                        _ => TaskStatus::Complete,
                    },
                })
            })
            .and_then(|mapped_rows| mapped_rows.collect());
        tasks

    }

    pub fn get_running_tasks() -> Result<Vec<TaskInstance>> {
        let db = Db::new().unwrap();

        let mut statement = db
            .conn
            .prepare(
                "SELECT s.name, color, strftime('%H:%M', start_date) AS started,
                CAST((JULIANDAY((SELECT datetime(CURRENT_TIMESTAMP, 'localtime'))) - JULIANDAY(start_date)) * 1440 AS INT) AS time
                FROM history s, task t WHERE s.name = t.name AND end_date IS NULL ORDER BY started ASC"
            )
            .expect("Cannot create the statement");

        //statement.execute(&[date])?;
        let tasks = statement
            .query_map(params![], |row| {
            Ok(
                TaskInstance {
                    task: Task {
                        name: row.get(0)?,
                        color: row.get(1)?,
                    },

                    start_time: row.get(2)?,
                    end_time: String::from("Null"),
                    duration: row.get(3)?,
                    status: TaskStatus::Running,
                })
            })
            .and_then(|mapped_rows| mapped_rows.collect());
        tasks

    }
  
}
