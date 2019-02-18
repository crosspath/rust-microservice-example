#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

extern crate dotenv;

use dotenv::dotenv;
use std::env;

use diesel::prelude::*;
use diesel::pg::PgConnection;

use rocket::response::content;

use serde_json::json;

use chrono::NaiveDateTime;

use std::collections::HashMap;

use schema; // local file

#[derive(Queryable)]
#[belongs_to(User)]
pub struct BonusAccount {
  pub id:         i32,
  pub user_id:    i32,
  pub bonuses:    f32,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime
  // has many bonus_logs
}

#[derive(Queryable)]
#[belongs_to(BonusAccount)]
#[belongs_to(UserOrder)]
pub struct BonusLog {
  pub id:               i32,
  pub bonus_account_id: i32,
  pub user_order_id:    i32,
  pub bonuses:          f32,
  pub created_at:       NaiveDateTime,
  pub updated_at:       NaiveDateTime
}

#[derive(Queryable)]
#[belongs_to(User)]
pub struct UserOrder {
  pub id:         i32,
  pub user_id:    i32,
  pub product:    f32
  pub price:      f32,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime
  // has one bonus_log
}

#[derive(Queryable)]
pub struct User {
  pub id:         i32,
  pub email:      string,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime
  // has one bonus_account
  // has many user_orders
}

struct ApiKey {
  // todo: implement FromRequest
  // https://api.rocket.rs/v0.4/rocket/request/trait.FromRequest.html
  // if params[:api_key] != env::var("API_KEY")
}

#[derive(FromForm)]
struct ReferralForm {
  // input params are referrer & order,
  // while we can use them as referrer_id & user_order_id
  
  #[form(field = "referrer")]
  referrer_id: i32,
  #[form(field = "order")]
  user_order_id: i32
}

#[post("/api/v1/referrals", data = "<form>")]
fn referrals(
  form: Option<LenientForm<ReferralForm>>,
  api_key: ApiKey
) -> content::Json<&'static str> {
  // todo: read post params
  // todo: search & update data in database
  
  // todo: send json
  let mut json_string: serde_json::Value;
  json_string = json!({
    "status": status,
    "bonuses": bonuses
  })
  json_string = json!({
    "status": status,
    "error": CODES.get(status).expect("")
  })
  if show_exceptions() {
    json_string["message"] = message
    json_string["trace"] = trace
  }
  content::Json(json_string.to_string())
}

pub fn response_success(options: serde_json::Value) -> serde_json::Value {
  match options {
    Object(Map<String, Value>) => options["status"] = 200; options
    _ => response_error(500, None, options)
  }
}

pub fn response_error(
  code: i8,
  exception: ,
  options: serde_json::Value
) -> serde_json::Value {
  options = match options {
    Object(Map<String, Value>) => options
    _ => json!({
      "data": options
    })
  }
  
  options["status"] = code;
  options["error"] = CODES.get(status).expect("")
  
  if show_exceptions() {
    options["message"] = message
    options["trace"] = trace
  }
  
  // todo: send status :unprocessable_entity
  options
}

pub fn show_exceptions() -> Bool {
  env::var("SHOW_EXCEPTIONS") > "0"
}

pub fn connect_db() -> PgConnection {
  let database_url = env::var("DATABASE_URL")
      .expect("DATABASE_URL must be set");
  PgConnection::establish(&database_url)
      .expect(&format!("Error connecting to {}", database_url))
}

// Not equal to HTTP Response Statuses
pub const CODES: HashMap<i8, &str> = [
  (100, "Not Authorised"),       // Custom status
  (110, "User Not Found"),
  (111, "UserOrder Not Found"),
  (112, "User Cannot Invite Himself"),
  (113, "This UserOrder Is Already Referenced For Bonuses"),
  (200, "OK"),                   // Typical HTTP status for success
  (500, "Internal Server Error") // Typical HTTP status for server error
]

fn main() {
  dotenv().ok();
  rocket::ignite().mount("/", routes![referrals]).launch();
}
