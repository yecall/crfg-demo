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
    view_change: Box<BftMsgPkg>, //BftMsgPkg::ViewChange
    pre_prepare: Box<BftMsgPkg>, //BftMsgPkg::PrePrePare
    prepares: Vec<Box<BftMsgPkg>>,//BftMsgPkg::PrePare
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct NewView{
    view_number: i64,
    sequence: i64,
    replica_id:i64,
    pre_prepare: Box<BftMsgPkg>, //BftMsgPkg::PrePrePare
    view_changes: Vec<Box<BftMsgPkg>>, //BftMsgPkg::ViewChange
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum BftMsg{
    NullMsg,
    PrePrePare(PrePrePare),
    PrePare(PrePare),
    Commit(Commit),
    ViewChange(ViewChange),
    ViewChangeFull(ViewChangeFull),
    NewView(NewView),
}

impl BftMsg{
    pub fn serialize(msg: &BftMsg) -> String {
        serde_json::to_string(msg).unwrap()
    }

    pub fn deserialize(msg_str: &str) -> BftMsg{
        serde_json::from_str(msg_str).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct BftMsgPkg{
    round_number: i64,
    message: BftMsg,
    signature: String,
}

impl BftMsgPkg{
    pub fn new() -> BftMsgPkg { BftMsgPkg{round_number: -1, message: BftMsg::NullMsg, signature: String::from("")} }

    pub fn set_round_number(&mut self, num: i64){ self.round_number = num;}
    pub fn set_message(&mut self, msg: BftMsg){ self.message = msg;}
    pub fn set_signature(&mut self, signature: String){ self.signature = signature;}

    pub fn round_number(&self) -> &i64 { &self.round_number }
    pub fn message(&self) -> &BftMsg { &self.message }
    pub fn signature(&self) -> &str{ &self.signature }

    pub fn serialize(msg_env: &BftMsgPkg) -> String {
        serde_json::to_string(msg_env).unwrap()
    }

    pub fn deserialize(msg_env_str: &str) -> BftMsgPkg{
        serde_json::from_str(msg_env_str).unwrap()
    }
}

#[test]
fn test_serialize_message(){
    let prepare = BftMsg::PrePare(PrePare{view_number: 10, sequence: 100, replica_id:3, proposal_hash: String::from("sdf")});

    let prepare_serialized = BftMsg::serialize(&prepare);
    println!("prepare serialized: {}", prepare_serialized);

    let prepare_deserialized = BftMsg::deserialize(&prepare_serialized);
    println!("prepare deserialize: {:#?}", prepare_deserialized);

    assert_eq!(prepare, prepare_deserialized);
}

#[test]
fn test_serialize_message_env(){
    let new_view = BftMsg::NewView(NewView{
        view_number: 10,
        sequence: 100,
        replica_id:3,
        pre_prepare: Box::new(BftMsgPkg{
            round_number: 5,
            message: BftMsg::NullMsg,
            signature: String::from("signature for pre_prepare")
        }),
        view_changes: vec![
            Box::new(BftMsgPkg{
                round_number: 5,
                message: BftMsg::Commit(Commit{
                    view_number: 10, sequence: 100, replica_id:3, proposal_hash: String::from("commit hash")
                }),
                signature: String::from("signature for new_view1")
            }),
            Box::new(BftMsgPkg{
                round_number: 6, message: BftMsg::NullMsg, signature: String::from("signature for new_view2")
            })
        ],
    });

    let new_view_serialized= BftMsg::serialize(&new_view);
    println!("new_view serialized: {}", new_view_serialized);

    let new_view_deserialized = BftMsg::deserialize(&new_view_serialized);
    println!("new_view deserialize: {:#?}", new_view_deserialized);

    assert_eq!(new_view, new_view_deserialized);
}
