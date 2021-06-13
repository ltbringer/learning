#[derive(Debug)]
enum StatusMessage {
    Ok,
}

fn check_status(sat_id: CubeSat) -> StatusMessage {
    StatusMessage::Ok
}

#[derive(Debug)]
struct CubeSat {
    id: u64,
}

fn main() {
    let sat_a = CubeSat { id: 0 };
    let sat_b = CubeSat { id: 1 };
    let sat_c = CubeSat { id: 2 };

    let status_a = check_status(sat_a);
    let status_b = check_status(sat_b);
    let status_c = check_status(sat_c);

    println!("a={:?} b={:?} c={:?}", status_a, status_b, status_c);

    let status_a = check_status(sat_a);
    let status_b = check_status(sat_b);
    let status_c = check_status(sat_c);

    println!("a={:?} b={:?} c={:?}", status_a, status_b, status_c);
}
