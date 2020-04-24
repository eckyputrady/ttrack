use super::types::*;
use chrono::prelude::*;
use uuid::Uuid;

pub trait ActivityRepo {
  fn add_activity(&mut self, activity: &Activity) -> Result<(), Error>;
  fn edit_activity(&mut self, activity: &EditActivityRequest) -> Result<Activity, Error>;
  fn get_last_active_activity(&self) -> Result<Option<Activity>, Error>;
  fn get_history(&self, request: &GetHistoryRequest) -> Result<Vec<Activity>, Error>;
  fn delete_activity(&mut self, request: &DeleteActivityRequest) -> Result<Activity, Error>;
  fn get_report(&self, request: &GetReportRequest) -> Result<Report, Error>;
}

impl<T: ActivityRepo> Service for T {
  fn start_activity(&mut self, request: &StartActivityRequest) -> Result<Activity, Error> {
    if self.get_last_active_activity()?.is_some() {
      return Err(Error {
        id: "active_activity_exists".to_string(),
        msg: "You can't start tracking another activity while there is an actively tracked activity!".to_string(),
        kind: ErrorType::ClientError
      });
    }
  
    let new_activity = Activity {
      id: Uuid::new_v4(),
      start: Utc::now(),
      end: None,
      name: request.name.clone(),
      tags: request.tags.clone()
    };
    self.add_activity(&new_activity)?;
  
    return Ok(new_activity);
  }

  fn stop_activity(&mut self) -> Result<Activity, Error> {
    if let Some(activity) = self.get_last_active_activity()? {
      self.edit_activity(&EditActivityRequest {
        activity_id: activity.id.to_string(),
        start_time: None,
        end_time: Some(Utc::now()),
        name: None,
        tags_to_add: vec![],
        tags_to_del: vec![]
      })
    } else {
      Err(Error {
        id: "active_activity_does_not_exist".to_string(),
        msg: "You are currently not tracking any activity".to_string(),
        kind: ErrorType::ClientError
      })
    }
  }

  fn edit_activity(&mut self, request: &EditActivityRequest) -> Result<Activity, Error> {
    ActivityRepo::edit_activity(self, request)
  }
  
  fn delete_activity(&mut self, request: &DeleteActivityRequest) -> Result<Activity, Error> {
    ActivityRepo::delete_activity(self, request)
  }

  fn get_history(&self, request: &GetHistoryRequest) -> Result<Vec<Activity>, Error> {
    ActivityRepo::get_history(self, request)
  }

  fn get_report(&self, request: &GetReportRequest) -> Result<Report, Error> {
    ActivityRepo::get_report(self, request)
  }
}