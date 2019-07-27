trait Serialize{
    fn serialize(&self) -> String;
    fn deserialize(msg_str: &str) -> Self;
}

//对Message 和 MessageEnv 都要实现Serialize trait
#[derive(Debug)]
pub enum Message{
    NullMsg,
    PrePrePare{
        view_number: i64,
        sequence: i64,
        replica_id:i64,
        proposal: String,
        proposal_hash: String,
    },
    PrePare{
        view_number: i64,
        sequence: i64,
        replica_id:i64,
        proposal_hash: String,
    },
    Commit{
        view_number: i64,
        sequence: i64,
        replica_id:i64,
        proposal_hash: String,
    },
    ViewChange{
        view_number: i64,
        sequence: i64,
        replica_id:i64,
        proposal_hash: String,
    },
    ViewChangeFull{
        view_change: Box<MessageEnv>, //MessageEnv::ViewChange
        pre_prepare: Box<MessageEnv>, //MessageEnv::PrePrePare
        prepares: Vec<Box<MessageEnv>>,//MessageEnv::PrePare
    },
    NewView{
        view_number: i64,
        sequence: i64,
        replica_id:i64,
        pre_prepare: Box<MessageEnv>, //MessageEnv::PrePrePare
        view_changes: Vec<Box<MessageEnv>>, //MessageEnv::ViewChange
    },
}

impl Serialize for Message{
    fn serialize(&self) -> String {
        String::from("Message")
    }

    fn deserialize(msg_str: &str) -> Message{
        Message::NullMsg
    }
}

#[derive(Debug)]
pub struct MessageEnv{
    round_number: i64,
    message: Message,
    signature: String,
}

impl MessageEnv{
    pub fn new() -> MessageEnv { MessageEnv{round_number: -1, message: Message::NullMsg, signature: String::from("")} }

    fn set_round_number(&mut self, num: i64){ self.round_number = num;}
    fn set_message(&mut self, msg: Message){ self.message = msg;}
    fn set_signature(&mut self, signature: String){ self.signature = signature;}

    fn round_number(&self) -> &i64 { &self.round_number }
    fn message(&self) -> &Message { &self.message }
    fn signature(&self) -> &str{ &self.signature }
}

