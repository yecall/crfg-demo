#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct BftSignature {
    pub public_key: String,
    pub sign_data: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PrePrePare{
    pub view_number: i64,
    pub sequence: i64,
    pub replica_id:i8,
    pub proposal: String,
    pub proposal_hash: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PrePare{
    pub view_number: i64,
    pub sequence: i64,
    pub replica_id:i8,
    pub proposal_hash: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Commit{
    pub view_number: i64,
    pub sequence: i64,
    pub replica_id:i8,
    pub proposal_hash: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ViewChangeBasic {
    pub view_number: i64,
    pub sequence: i64,
    pub replica_id:i8,
    pub proposal_hash: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PreparedSet {
    pub pre_prepare: Box<BftMsgPkg>, //BftMsgPkg::PrePrePare
    pub prepares: Vec<Box<BftMsgPkg>>,//BftMsgPkg::PrePare
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ViewChange {
    pub view_change: Box<BftMsgPkg>, //BftMsgPkg::ViewChange
    pub prepared_set: PreparedSet,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct NewView{
    pub view_number: i64,
    pub sequence: i64,
    pub replica_id:i8,
    pub pre_prepare: Box<BftMsgPkg>, //BftMsgPkg::PrePrePare
    pub view_changes: Vec<Box<BftMsgPkg>>, //BftMsgPkg::ViewChangeBasic
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum BftMsg{
    NullMsg,
    PrePrePare(PrePrePare),
    PrePare(PrePare),
    Commit(Commit),
    ViewChangeBasic(ViewChangeBasic),
    ViewChange(ViewChange),
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
    signature: BftSignature,
}

impl BftMsgPkg{
    pub fn new() -> BftMsgPkg {
        BftMsgPkg{
            round_number: -1,
            message: BftMsg::NullMsg,
            signature: BftSignature{public_key: String::from(""),sign_data: Default::default()}}
    }

    pub fn set_round_number(&mut self, num: i64){ self.round_number = num;}
    pub fn set_message(&mut self, msg: BftMsg){ self.message = msg;}
    pub fn set_signature(&mut self, signature: BftSignature){ self.signature = signature;}

    pub fn round_number(&self) -> &i64 { &self.round_number }
    pub fn message(&self) -> &BftMsg { &self.message }
    pub fn signature(&self) -> &BftSignature { &self.signature }

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
            signature: BftSignature{
                public_key: String::from("public key."),
                sign_data: String::from("signature.")
            }
        }),
        view_changes: vec![
            Box::new(BftMsgPkg{
                round_number: 5,
                message: BftMsg::Commit(Commit{
                    view_number: 10, sequence: 100, replica_id:3, proposal_hash: String::from("commit hash")
                }),
                signature: BftSignature{
                    public_key: String::from("public key."),
                    sign_data: String::from("signature.")
                }
            }),
            Box::new(BftMsgPkg{
                round_number: 6,
                message: BftMsg::NullMsg,
                signature: BftSignature{
                    public_key: String::from("public key."),
                    sign_data: String::from("signature.")
                }
            })
        ],
    });

    let new_view_serialized= BftMsg::serialize(&new_view);
    println!("new_view serialized: {}", new_view_serialized);

    let new_view_deserialized = BftMsg::deserialize(&new_view_serialized);
    println!("new_view deserialize: {:#?}", new_view_deserialized);

    assert_eq!(new_view, new_view_deserialized);
}
