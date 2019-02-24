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

struct ReferralFormResult {
  bonuses:  mut i32,
  referrer: mut Option<User>,
  order:    mut Option<UserOrder>
}

struct FormRunner<T, Y> {
  inputs:     T,
  outputs:    mut Y,
  error_code: mut i8
}

impl FormRunner<ReferralForm, ReferralFormResult> {
  fn set_referrer(&self) {
    if &self.outputs.referrer == None {
      &self.outputs.referrer = schema::users::table.find(&self.inputs.referrer_id)
    }
  }

  fn set_order(&self) {
    if &self.outputs.order == None {
      &self.outputs.order = schema::user_orders::table.find(&self.inputs.user_order_id)
    }
  }

  fn valid?(&self) -> bool {
    set_referrer(&self);
    set_order(&self);
    
    let referrer = &self.outputs.referrer;
    let order = &self.outputs.order;
    
    if referrer == None {
      110
    } else if order == None {
      111
    } else if order.user_id == referrer.id {
      112
    } else if 
  }
}

#[post("/api/v1/referrals", data = "<form>")]
fn referrals(
  form: Option<LenientForm<ReferralForm>>,
  api_key: ApiKey
) -> content::Json<&'static str> {
  // todo: read post params
  let response = match form {
    Some(form_values) => response_success
    None => println!("form is empty"); response_error(110)
  }
  content::Json(response.to_string())
  
  // todo: search & update data in database
  let conn = connect_db();
  let form_runner = FormRunner {
    inputs:  form_values,
    outputs: ReferralFormResult {}
  };
  
  // todo: send json
  let mut json_string: serde_json::Value;
  json_string = json!({
    "status": status,
    "bonuses": bonuses
  })

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
  exception: Option<???> = None,
  options: serde_json::Value = Null
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
