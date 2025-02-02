// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::anchor_idl;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: anchor_idl = serde_json::from_str(&json).unwrap();
// }

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AnchorIdl {
    pub version: String,

    pub name: String,

    docs: Vec<String>,

    instructions: Vec<Instruction>,

    pub accounts: Vec<AnchorIdlAccount>,

    types: Vec<TypeElement>,

    events: Vec<Event>,

    errors: Vec<Error>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnchorIdlAccount {
    pub name: String,

    #[serde(rename = "type")]
    account_type: AccountType,

    docs: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountType {
    kind: Kind,

    fields: Vec<PurpleField>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurpleField {
    name: String,

    #[serde(rename = "type")]
    field_type: IndecentType,

    docs: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IndecentType {
    Enum(TypeEnum),

    PurpleType(PurpleType),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurpleType {
    defined: Option<String>,

    array: Option<Vec<Array>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Array {
    Enum(TypeEnum),

    Integer(i64),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TypeEnum {
    Bool,

    F64,

    #[serde(rename = "publicKey")]
    PublicKey,

    U128,

    U32,

    U64,

    U8,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Kind {
    Enum,

    Struct,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    code: i64,

    name: String,

    msg: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    name: String,

    fields: Vec<EventField>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventField {
    name: String,

    #[serde(rename = "type")]
    field_type: HilariousType,

    index: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HilariousType {
    Enum(TypeEnum),

    FluffyType(FluffyType),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FluffyType {
    defined: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Instruction {
    name: String,

    docs: Vec<String>,

    pub accounts: Vec<InstructionAccount>,

    pub args: Vec<Arg>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstructionAccount {
    name: String,

    is_mut: bool,

    is_signer: bool,

    docs: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Arg {
    name: String,

    #[serde(rename = "type")]
    arg_type: ArgType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ArgType {
    Enum(TypeEnum),

    TentacledType(TentacledType),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TentacledType {
    defined: Option<String>,

    #[serde(rename = "option")]
    type_option: Option<TypeEnum>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TypeElement {
    name: String,

    docs: Option<Vec<String>>,

    #[serde(rename = "type")]
    type_type: StickyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StickyType {
    kind: Kind,

    fields: Option<Vec<FluffyField>>,

    variants: Option<Vec<Variant>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FluffyField {
    name: String,

    docs: Option<Vec<String>>,

    #[serde(rename = "type")]
    field_type: AmbitiousType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AmbitiousType {
    Enum(TypeEnum),

    IndigoType(IndigoType),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndigoType {
    defined: Option<String>,

    #[serde(rename = "option")]
    type_option: Option<TypeEnum>,

    array: Option<Vec<Array>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Variant {
    name: String,

    fields: Option<Vec<VariantField>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VariantField {
    name: String,

    docs: Option<Vec<String>>,

    #[serde(rename = "type")]
    field_type: HilariousType,
}
