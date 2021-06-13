#[derive(Debug)]
enum StatusMessage {
    Ok,
}

fn check_status(sat_id: CubeSat) -> StatusMessage {
    StatusMessage::Ok
}

type Message = String;

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

#[derive(Debug)]
struct CubeSat {
    id: u64,
    mailbox: Mailbox,
}

impl CubeSat {
    fn recv(&mut self) -> Option<Message> {
        self.mailbox.messages.pop()
    }
}

#[derive(Debug)]
struct GroundStation {}

impl GroundStation {
    fn send(&self, to: &mut CubeSat, msg: Message) {
        to.mailbox.messages.push(msg);
    }
}

fn main() {
    let base = GroundStation {};
    let mut sat_a = CubeSat {
        id: 0,
        mailbox: Mailbox { messages: vec![] },
    };
    let sat_b = CubeSat {
        id: 0,
        mailbox: Mailbox { messages: vec![] },
    };
    let sat_c = CubeSat {
        id: 0,
        mailbox: Mailbox { messages: vec![] },
    };

    println!("t0: {:?}", sat_a);
    base.send(&mut sat_a, Message::from("PING"));

    println!("t1: {:?}", sat_a);
    let msg = sat_a.recv();

    println!("t2: {:?}", sat_a);
    println!("message: {:?}", msg);
}
