#[derive(Debug)]
struct Message {
    to: u64,
    content: String,
}

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

impl Mailbox {
    fn deliver(&mut self, recipient: &CubeSat) -> Option<Message> {
        for i in 0..self.messages.len() {
            if self.messages[i].to == recipient.id {
                return Some(self.messages.remove(i));
            }
        }
        None
    }

    fn post(&mut self, message: Message) {
        self.messages.push(message);
    }
}

#[derive(Debug)]
struct CubeSat {
    id: u64,
    mailbox: Mailbox,
}

impl CubeSat {
    fn recv(&self, mailbox: &mut Mailbox) -> Option<Message> {
        mailbox.deliver(&self)
    }
}

fn fetch_sat_ids() -> Vec<u64> {
    vec![1, 2, 3]
}

#[derive(Debug)]
struct GroundStation {}

impl GroundStation {
    fn send(&self, mailbox: &mut Mailbox, msg: Message) {
        mailbox.post(msg);
    }

    fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat {
            id: sat_id,
            mailbox: Mailbox { messages: vec![] },
        }
    }
}

fn main() {
    let base = GroundStation {};
    let mut mailbox = Mailbox { messages: vec![] };
    let sat_ids = fetch_sat_ids();

    for sat_id in sat_ids {
        let msg = Message {
            to: sat_id,
            content: String::from("hello"),
        };
        base.send(&mut mailbox, msg)
    }

    let sat_ids = fetch_sat_ids();

    for sat_id in sat_ids {
        let sat = base.connect(sat_id);
        let msg = sat.recv(&mut mailbox);
        match msg {
            Some(data) => println!("sat_id {} 👂 '{}'", sat.id, data.content),
            None => println!("sat_id {}: 👂 '{}'", sat.id, "NA"),
        }
    }
}
