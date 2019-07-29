#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct PrePrePare{
    view_number: i64,
    sequence: i64,
    replica_id:i64,
    proposal: String,
    proposal_hash: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct PrePare{
    view_number: i64,
    sequence: i64,
    replica_id:i64,
    proposal_hash: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Commit{
    view_number: i64,
    sequence: i64,
    replica_id:i64,
    proposal_hash: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct ViewChange{
    view_number: i64,
    sequence: i64,
    replica_id:i64,
    proposal_hash: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct ViewChangeFull{
    view_change: Box<MessageEnv>, //MessageEnv::ViewChange
    pre_prepare: Box<MessageEnv>, //MessageEnv::PrePrePare
    prepares: Vec<Box<MessageEnv>>,//MessageEnv::PrePare
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct NewView{
    view_number: i64,
    sequence: i64,
    replica_id:i64,
    pre_prepare: Box<MessageEnv>, //MessageEnv::PrePrePare
    view_changes: Vec<Box<MessageEnv>>, //MessageEnv::ViewChange
}

//对Message 和 MessageEnv 都要实现Serialize trait
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Message{
    NullMsg,
    PrePrePare(PrePrePare),
    PrePare(PrePare),
    Commit(Commit),
    ViewChange(ViewChange),
    ViewChangeFull(ViewChangeFull),
    NewView(NewView),
}

impl Message{
    fn serialize(msg: &Message) -> String {
        serde_json::to_string(msg).unwrap()
    }

    fn deserialize(msg_str: &str) -> Message{
        serde_json::from_str(msg_str).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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

impl MessageEnv{
    fn serialize(msg_env: &MessageEnv) -> String {
        serde_json::to_string(msg_env).unwrap()
    }

    fn deserialize(msg_env_str: &str) -> MessageEnv{
        serde_json::from_str(msg_env_str).unwrap()
    }
}

#[test]
fn test_serialize_message(){
    let prepare = Message::PrePare(PrePare{view_number: 10, sequence: 100, replica_id:3, proposal_hash: String::from("sdf")});

    let prepare_serialized = Message::serialize(&prepare);
    println!("prepare serialized: {}", prepare_serialized);

    let prepare_deserialized = Message::deserialize(&prepare_serialized);
    println!("prepare deserialize: {:#?}", prepare_deserialized);

    assert_eq!(prepare, prepare_deserialized);
}

#[test]
fn test_serialize_message_env(){
    let new_view = Message::NewView(NewView{
        view_number: 10,
        sequence: 100,
        replica_id:3,
        pre_prepare: Box::new(MessageEnv{
            round_number: 5,
            message: Message::NullMsg,
            signature: String::from("signature for pre_prepare")
        }),
        view_changes: vec![
            Box::new(MessageEnv{
                round_number: 5,
                message: Message::Commit(Commit{
                    view_number: 10, sequence: 100, replica_id:3, proposal_hash: String::from("commit hash")
                }),
                signature: String::from("signature for new_view1")
            }),
            Box::new(MessageEnv{
                round_number: 6, message: Message::NullMsg, signature: String::from("signature for new_view2")
            })
        ],
    });

    let new_view_serialized= Message::serialize(&new_view);
    println!("new_view serialized: {}", new_view_serialized);

    let new_view_deserialized = Message::deserialize(&new_view_serialized);
    println!("new_view deserialize: {:#?}", new_view_deserialized);

    assert_eq!(new_view, new_view_deserialized);
}
