use structopt::StructOpt;
use crate::core::types::*;
use chrono::prelude::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "ttrack")]
enum Opt {
  /// Start tracking an activity
  Start {
    /// Activity tags.
    /// You can supply multiple values for this flag
    #[structopt(short = "t", long = "tag")]
    tags: Vec<String>,

    /// Activity name
    #[structopt(name = "ACTIVITY_NAME")]
    name: String,
  },

  /// Finish tracking current activity
  Stop,

  /// Edit tracked activity
  Edit {
    /// Tags to be added.
    /// You can supply multiple values for this tag
    #[structopt(short="a", long)]
    tags_to_add: Vec<String>,

    /// Tags to be removed.
    /// You can supply multiple values for this tag
    #[structopt(short="d", long)]
    tags_to_del: Vec<String>,

    /// Edit the activity name
    #[structopt(short, long)]
    name: Option<String>,

    /// Edit the activity start time
    #[structopt(short = "s", long)]
    start_time: Option<DateTime<Utc>>,

    /// Edit the activity end time
    #[structopt(short = "e", long)]
    end_time: Option<DateTime<Utc>>,

    /// ID of the activity to be changed. e.g. `6aed4521`. You can get IDs from running `history` command. 
    #[structopt(name = "ACTIVITY_ID")]
    activity_id: String
  },

  /// Delete tracked activity
  Delete {
    /// ID of the activity to be changed. e.g. `6aed4521`. You can get IDs from running `history` command. 
    #[structopt(name = "ACTIVITY_ID")]
    activity_id: String
  },

  /// Show the list of tracked activities, starting from recent ones.
  History {
    #[structopt(short = "p", long, default_value = "0")]
    page: i32,

    #[structopt(short = "n", long, default_value = "20")]
    page_size: i32,

    #[structopt(short = "s", long, default_value = "1970-01-01T00:00:00+00:00")]
    start_time: DateTime<Utc>,

    #[structopt(short = "e", long, default_value = "3000-01-01T00:00:00+00:00")]
    end_time: DateTime<Utc>
  },

  /// Show report of total activities duration broken down by tag
  Report {
    #[structopt(short = "s", long, default_value = "1970-01-01T00:00:00+00:00")]
    start_time: DateTime<Utc>,

    #[structopt(short = "e", long, default_value = "3000-01-01T00:00:00+00:00")]
    end_time: DateTime<Utc>
  }
}

pub fn run<T: Service>(service: &mut T) {
  let opt = Opt::from_args();
  match opt {
    Opt::History{ page, page_size, start_time, end_time } =>
      print_activities(&handle_error(service.get_history(&GetHistoryRequest{ page, page_size, since_datetime: start_time, until_datetime: end_time }))),
    Opt::Start{ tags, name } =>
      print_activity(&handle_error(service.start_activity(&StartActivityRequest { tags, name }))),
    Opt::Stop =>
      print_activity(&handle_error(service.stop_activity())),
    Opt::Edit{ tags_to_add, tags_to_del, name, end_time, start_time, activity_id } =>
      print_activity(&handle_error(service.edit_activity(&EditActivityRequest { tags_to_add, tags_to_del, name, end_time, start_time, activity_id }))),
    Opt::Delete{ activity_id } =>
      print_activity(&handle_error(service.delete_activity(&DeleteActivityRequest{ activity_id }))),
    Opt::Report{ start_time, end_time } =>
      print_report(&handle_error(service.get_report(&GetReportRequest{ since_datetime: start_time, until_datetime: end_time })))
  }
}

fn handle_error<T>(result: Result<T, Error>) -> T {
  match result {
    Err(e) => { exit_with_error(e); panic!("Shouldn't reach here") },
    Ok(a) => a
  }
}

fn exit_with_error(e: Error) {
  println!("error: {}", e.msg);
  ::std::process::exit(
    if e.kind == ErrorType::ClientError { 
      exitcode::USAGE 
    } else { 
      exitcode::SOFTWARE 
    }
  );
}

fn print_activity(activity: &Activity) {
  println!("{}", format_activity(activity))
}

fn print_activities(activities: &Vec<Activity>) {
  for a in activities.iter() {
    print_activity(a);
  }
}

fn print_report(report: &Report) {
  for i in report.iter() {
    println!("{} {}", format_duration_secs(i.duration_secs), i.tag)
  }
}

fn format_activity(activity: &Activity) -> String {
  format!(
    "{} {} - {} {} {:.<50} {}",
    activity.short_id(),
    format_datetime(&activity.start),
    activity.end.map(|x| format_datetime(&x)).unwrap_or(".........................".to_string()),
    format_duration(activity.duration()),
    activity.name,
    activity.tags.iter().map(|x| format!("[{}]", x.to_string())).collect::<Vec<String>>().join(" ")
  )
}

fn format_datetime(dt: &DateTime<Utc>) -> String {
  let converted: DateTime<Local> = DateTime::from(dt.clone());
  converted.to_rfc3339_opts(SecondsFormat::Secs, false)
}

fn format_duration(duration: chrono::Duration) -> String {
  let secs = duration.num_seconds();
  format_duration_secs(secs)
}

fn format_duration_secs(secs: i64) -> String {
  format!("{:02}:{:02}:{:02}", secs / 60 / 60, secs / 60 % 60, secs % 60)
}