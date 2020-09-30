use fluence::fce;

fn main() {}

#[fce]
pub struct CallServiceResult {
    pub ret_code: i32,
    pub result: String,
}

#[fce]
pub fn call_service(service_id: String, fn_name: String, args: String) -> CallServiceResult {
    println!(
        "call service invoked with:\n  service_id: {}\n  fn_name: {}\n  args: {:?}",
        service_id, fn_name, args
    );

    CallServiceResult {
        ret_code: 0,
        result: String::from("[\"result string\"]"),
    }
}
