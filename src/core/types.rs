use chrono::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Activity {
  pub id: Uuid,
  pub start: DateTime<Utc>,
  pub end: Option<DateTime<Utc>>,
  pub name: String,
  pub tags: Vec<String>
}

impl Activity {
  pub fn duration(&self) -> chrono::Duration {
    self.end.unwrap_or(Utc::now()) - self.start
  }

  pub fn short_id(&self) -> String {
    format!("{}", &self.id.to_string()[..8])
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrorType {
  ClientError,
  ServerError
}

#[derive(Clone, Debug)]
pub struct Error {
  pub id: String,
  pub msg: String,
  pub kind: ErrorType
}

#[derive(Clone, Debug)]
pub struct StartActivityRequest {
  pub name: String,
  pub tags: Vec<String>
}

#[derive(Clone, Debug)]
pub struct EditActivityRequest {
  pub activity_id: String,
  pub start_time: Option<DateTime<Utc>>,
  pub end_time: Option<DateTime<Utc>>,
  pub name: Option<String>,
  pub tags_to_add: Vec<String>,
  pub tags_to_del: Vec<String>,
}

pub struct GetHistoryRequest {
  pub page: i32,
  pub page_size: i32,
  pub since_datetime: DateTime<Utc>,
  pub until_datetime: DateTime<Utc>
}

pub struct DeleteActivityRequest {
  pub activity_id: String,
}

pub struct ReportItem {
  pub tag: String,
  pub duration_secs: i64
}

pub type Report = Vec<ReportItem>;

pub struct GetReportRequest {
  pub since_datetime: DateTime<Utc>,
  pub until_datetime: DateTime<Utc>
}

pub trait Service {
  fn start_activity(&mut self, request: &StartActivityRequest) -> Result<Activity, Error>;
  fn stop_activity(&mut self) -> Result<Activity, Error>;
  fn edit_activity(&mut self, request: &EditActivityRequest) -> Result<Activity, Error>;
  fn delete_activity(&mut self, request: &DeleteActivityRequest) -> Result<Activity, Error>;
  fn get_history(&self, request: &GetHistoryRequest) -> Result<Vec<Activity>, Error>;
  fn get_report(&self, request: &GetReportRequest) -> Result<Report, Error>;
}