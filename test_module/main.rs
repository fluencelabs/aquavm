use fluence::fce;

fn main() {}

#[fce]
pub struct CallServiceResult {
    pub result: i32,
    pub outcome: Vec<u8>,
}

#[fce]
pub fn call_service(service_id: String, fn_name: String, args: Vec<u8>) -> CallServiceResult {
    println!(
        "call service invoked with:\n  service_id: {}\n  fn_name: {}\n  args: {:?}",
        service_id, fn_name, args
    );

    CallServiceResult {
        result: 0,
        outcome: vec![1, 2, 3],
    }
}
