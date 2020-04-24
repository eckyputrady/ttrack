use crate::core::services::*;
use crate::core::types::*;
use rusqlite::{params, Connection};
use rusqlite::NO_PARAMS;
use uuid::Uuid;
use chrono::prelude::*;

impl From<rusqlite::Error> for Error {
  fn from(e: rusqlite::Error) -> Error {
    Error {
      id: "db_error".to_string(),
      msg: format!("{:?}", e),
      kind: ErrorType::ServerError
    }
  }
}

fn row_to_activity(row: &rusqlite::Row) -> rusqlite::Result<Activity> {
  let id_str: String = row.get(0)?;
  let tags_str: String = row.get(4)?;
  Ok(Activity {
    id: Uuid::parse_str(&id_str).unwrap(),
    name: row.get(1)?,
    start: row.get(2)?,
    end: row.get(3)?,
    tags: tags_str.split(',').map(String::from).filter(|x| x.len() > 0).collect()
  })
}

pub struct SQLiteActivityRepo {
  conn: Connection
}

impl SQLiteActivityRepo {

  pub fn new() -> rusqlite::Result<SQLiteActivityRepo> {
    let path = dirs::data_dir().unwrap().join("ttrack");
    std::fs::create_dir_all(&path).unwrap();
    let conn = Connection::open(path.join("ttrack.db"))?;
    conn.execute("
      create table if not exists activity ( 
        id char(36) primary key,
        name varchar(100) not null,
        start timestamp not null,
        end timestamp
      )
    ", NO_PARAMS)?;
    conn.execute("
      create table if not exists tag (
        activity_id char(36) not null,
        tag varchar(100) not null,
        foreign key (activity_id) references activity (id),
        primary key (activity_id, tag)
      )
    ", NO_PARAMS)?;

    Ok(SQLiteActivityRepo { conn })
  }

  fn get_activity_by_short_id(&self, id: &String) -> Result<Option<Activity>, Error> {
    Ok(
      self.conn
        .prepare("
          select id, name, start, end, coalesce(tags, '')
          from activity left join (
            select activity_id, group_concat(tag) as tags
            from tag
            group by activity_id
          ) on (activity.id = activity_id)
          where id like ?1
          limit 1
        ")?
        .query_map(
          params![format!("{}{}", id, '%')],
          row_to_activity
        )?
        .filter_map(Result::ok)
        .next()
    )
  }

  fn get_activity_by_short_id_or_err(&self, id: &String) -> Result<Activity, Error> {
    let activity_option = self.get_activity_by_short_id(id)?;
    activity_option.ok_or(Error {
      id: "activity_id_not_found".to_string(),
      msg: "ID not found!".to_string(),
      kind: ErrorType::ClientError
    })
  }
}

impl ActivityRepo for SQLiteActivityRepo {

  fn add_activity(&mut self, activity: &Activity) -> Result<(), Error> {
    let tx = self.conn.transaction()?;
    tx.execute(
      "insert into activity (id, name, start, end) values (?1, ?2, ?3, ?4)",
      params![activity.id.to_string(), activity.name, activity.start, activity.end])?;
    for tag in activity.tags.iter() {
      tx.execute(
        "insert or replace into tag (activity_id, tag) values (?1, ?2)",
        params![activity.id.to_string(), tag])?;
    }
    tx.commit()?;

    Ok(())
  }

  fn edit_activity(&mut self, request: &EditActivityRequest) -> Result<Activity, Error> {
    let activity = self.get_activity_by_short_id_or_err(&request.activity_id)?;

    let tx = self.conn.transaction()?;
    tx.execute(
      "update activity set
        name = coalesce(?2, name),
        start = coalesce(?3, start),
        end = coalesce(?4, end)
       where id like ?1
      ",
      params![format!("{}{}", activity.id, '%'), request.name, request.start_time, request.end_time])?;
    for tag in request.tags_to_add.iter() {
      tx.execute(
        "insert or replace into tag (activity_id, tag) values (?1, ?2)",
        params![activity.id.to_string(), tag])?;
    }
    for tag in request.tags_to_del.iter() {
      tx.execute(
        "delete from tag where activity_id = ?1 and tag = ?2",
        params![activity.id.to_string(), tag])?;
    }
    tx.commit()?;
    
    self.get_activity_by_short_id_or_err(&request.activity_id)
  }

  fn delete_activity(&mut self, request: &DeleteActivityRequest) -> Result<Activity, Error> {
    let activity = self.get_activity_by_short_id_or_err(&request.activity_id)?;

    let tx = self.conn.transaction()?;
    tx.execute(
      "delete from activity where id = ?1",
      params![activity.id.to_string()]
    )?;
    tx.execute(
      "delete from tag where activity_id = ?1",
      params![activity.id.to_string()]
    )?;
    tx.commit()?;

    Ok(activity)
  }

  fn get_last_active_activity(&self) -> Result<Option<Activity>, Error> {
    let request = GetHistoryRequest{
      page: 0,
      page_size: 1,
      since_datetime: Utc.ymd(1970, 1, 1).and_hms(0, 1, 1),
      until_datetime: Utc::now() 
    };
    match ActivityRepo::get_history(self, &request)?.pop() {
      Some(activity) => Ok(if activity.end.is_none() { Some(activity) } else { None }),
      None => Ok(None)
    }
  }

  fn get_history(&self, request: &GetHistoryRequest) -> Result<Vec<Activity>, Error> {
    let GetHistoryRequest { page, page_size, since_datetime, until_datetime } = request;
    Ok(
      self.conn
        .prepare("
          select id, name, start, end, coalesce(tags, '')
          from activity left join (
            select activity_id, group_concat(tag) as tags
            from tag
            group by activity_id
          ) on (activity.id = activity_id)
          where start between ?3 and ?4
          order by start desc
          limit ?1 offset ?2
        ")?
        .query_map(
          params![page_size, page * page_size, since_datetime, until_datetime],
          row_to_activity
        )?
        .filter_map(Result::ok)
        .collect()
    )
  }

  fn get_report(&self, request: &GetReportRequest) -> Result<Report, Error> {
    Ok(
      self.conn
        .prepare("
            select
              coalesce(tag, 'untagged'),
              sum(
                cast(strftime('%s', coalesce(end, CURRENT_TIMESTAMP)) as integer) - 
                cast(strftime('%s', start) as integer)
              ) as duration
            from
              activity left join tag on (activity.id = tag.activity_id)
            where start between ?1 and ?2
            group by tag.tag
            order by duration desc
          ")?
        .query_map(
          params![&request.since_datetime, &request.until_datetime],
          |row| {
            Ok(ReportItem{
              tag: row.get(0)?,
              duration_secs: row.get(1)?
            })
          }
        )?
        .filter_map(Result::ok)
        .collect()
    )
  }
}