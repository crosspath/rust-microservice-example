#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

struct ApiKey {
  // todo: implement FromRequest
  // https://api.rocket.rs/v0.4/rocket/request/trait.FromRequest.html
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
fn referrals(form: Option<LenientForm<ReferralForm>>, api_key: ApiKey) -> &'static str {
  // todo: read post params
  // todo: search & update data in database
  // todo: send json
}

fn main() {
  rocket::ignite().mount("/", routes![referrals]).launch();
}
