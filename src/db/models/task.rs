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
            Ok(_) => true,
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

    pub fn update_task_instance(task_name: &str, start_time: &str, end_time: &str, edited_task: &TaskInstance) -> Result<()> {
        
        let db = Db::new().unwrap();
        db.conn
            .execute(
                "UPDATE history SET
                    name=?, start_date=datetime(?),
                    desc=(CASE WHEN ? = 'Null' THEN '' ELSE ? END),
                    end_date=(CASE WHEN ? = 'Null' THEN NULL ELSE datetime(?) END)

                WHERE name=? AND start_date=? AND
                    (CASE WHEN ? = 'Null' THEN end_date IS NULL ELSE end_date = ? END)",
                params_from_iter(&[
                    &edited_task.task.name.to_string(),
                    &edited_task.start_time,
                    &edited_task.desc,
                    &edited_task.desc,
                    &edited_task.end_time,
                    &edited_task.end_time,
                    task_name,
                    &start_time,
                    &end_time,
                    &edited_task.end_time,
                ]
            ),
        ).expect(&format!("Error, cannot update the task; {} -> {}", &task_name, &edited_task.task.name.to_string()));

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
            Ok(_) => true,
            Err(_) => false,
        }
   }

    pub fn add_task_description(task_name: &str, desc: &str, running: bool) -> Result<()> {
        let db = Db::new().unwrap();
            db.conn
                .execute(
                    "
                    WITH task AS (
                        SELECT name, start_date FROM history
                        WHERE name = ?1 AND
                        (CASE WHEN ?3 IS 'true' THEN end_date is NULL ELSE end_date is not NULL END)
                    )
                    UPDATE history SET desc = ?2
                    WHERE (name, start_date) IN task",
                    params_from_iter(&[task_name, desc, running.to_string().as_str()]),
                )
                .expect(&format!("Error, cannot edit the task; {}", &task_name));
                Ok(())
            }

            pub fn get_date_tasks(date: &str) -> Result<Vec<TaskInstance>> {
        let db = Db::new().unwrap();

        let mut statement = db
            .conn
            .prepare(
                "SELECT s.name, color, start_date AS started,
                CASE WHEN end_date IS NULL THEN 'Null'
                ELSE end_date
                END AS ended,
                
                CASE WHEN end_date IS NULL THEN
                CAST((JULIANDAY((SELECT datetime(CURRENT_TIMESTAMP, 'localtime'))) - JULIANDAY(start_date)) * 1440 AS INT)
                ELSE CAST((JULIANDAY(end_date) - JULIANDAY(start_date)) * 1440 AS INT)
                END AS time, desc
                
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
                    desc: row.get(5)?,
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
                CAST((JULIANDAY((SELECT datetime(CURRENT_TIMESTAMP, 'localtime'))) - JULIANDAY(start_date)) * 1440 AS INT) AS time, desc
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
                    desc: row.get(4)?,
                    status: TaskStatus::Running,
                })
            })
            .and_then(|mapped_rows| mapped_rows.collect());
        tasks

    }
  
}
