mod splitter;
#[cfg(feature = "codeclient")]
pub mod template_sender;

pub use splitter::split_templates;

use std::{
    collections::HashMap,
    io::{Read, Write},
};

use base64::{Engine, prelude::BASE64_STANDARD};
use flate2::{read::GzDecoder, write::GzEncoder};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Template {
    pub blocks: Vec<Block>,
}

/// The main structure of a block.
/// This is not actually the type that is serialized/deserialized.
/// Instead, it converts from/to [`JSONBlock`], which is friendlier with the JSON.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(from = "JSONBlock", into = "JSONBlock")]
pub enum Block {
    PlayerEvent {
        action: String,
        ls_cancel: bool,
    },
    EntityEvent {
        action: String,
        ls_cancel: bool,
    },
    Function {
        args: Args,
        name: String,
    },
    Process {
        args: Args,
        name: String,
    },
    PlayerAction {
        args: Args,
        action: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        target: Option<Target>,
    },
    IfPlayer {
        args: Args,
        action: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        target: Option<Target>,
        is_negated: bool,
    },
    StartProcess {
        args: Args,
        proc: String,
    },
    CallFunction {
        args: Args,
        func: String,
    },
    Control {
        args: Args,
        action: String,
    },
    SetVariable {
        args: Args,
        action: String,
    },
    IfEntity {
        args: Args,
        action: String,
        target: Option<Target>,
        is_negated: bool,
    },
    EntityAction {
        args: Args,
        action: String,
        target: Option<Target>,
    },
    IfVariable {
        args: Args,
        action: String,
        is_negated: bool,
    },
    SelectObject {
        args: Args,
        action: String,
        sub_action: Option<String>,
        is_negated: bool,
    },
    GameAction {
        args: Args,
        action: String,
    },
    Repeat {
        args: Args,
        action: String,
        sub_action: Option<String>,
        is_negated: bool,
    },
    IfGame {
        args: Args,
        action: String,
        is_negated: bool,
    },
    Else,
    Bracket {
        direction: BracketDirection,
        kind: BracketKind,
    },
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(from = "JSONArgs", into = "JSONArgs")]
pub struct Args(pub Vec<(usize, Item)>);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "id", content = "data")]
pub enum Item {
    #[serde(rename = "txt")]
    String { name: String },
    #[serde(rename = "comp")]
    Text { name: String },
    #[serde(rename = "num")]
    Number { name: String },
    #[serde(rename = "item")]
    Item { item: String },
    #[serde(rename = "loc")]
    Location {
        #[serde(rename = "isBlock")]
        is_block: bool,
        loc: Location,
    },
    #[serde(rename = "vec")]
    Vector { x: f64, y: f64, z: f64 },
    #[serde(rename = "var")]
    Variable { name: String, scope: VariableScope },
    #[serde(rename = "g_val")]
    GameValue {
        #[serde(rename = "type")]
        kind: String,
        target: Target,
    },
    #[serde(rename = "pn_el")]
    Param {
        name: String,
        #[serde(rename = "type")]
        kind: ParamKind,
        #[serde(skip_serializing_if = "Option::is_none")]
        default_value: Option<Box<Item>>,
        plural: bool,
        optional: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        note: Option<String>,
    },
    #[serde(rename = "bl_tag")]
    Tag {
        option: String,
        tag: String,
        action: String,
        block: CodeBlock,
    },
    #[serde(rename = "snd")]
    Sound {
        pitch: f64,
        vol: f64,
        sound: String,
        #[serde(flatten)]
        extra: HashMap<String, Value>,
    },
    #[serde(rename = "part")]
    Particle {
        particle: String,
        #[serde(flatten)]
        extra: HashMap<String, Value>,
    },
    #[serde(rename = "pot")]
    Potion {
        #[serde(rename = "pot")]
        potion: String,
        #[serde(rename = "dur")]
        duration: u32,
        #[serde(rename = "amp")]
        amplifier: u32,
    },
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Location {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub pitch: f64,
    pub yaw: f64,
}

/// The internal representation of the arguments in the JSON file.
/// This is done separately because I wanted [`Args`] to remain as a tuple.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct JSONArgs {
    items: Vec<SlottedItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct SlottedItem {
    item: Item,
    slot: usize,
}

/// The internal representation of a block in the JSON file.
/// This is done separately beacuse I wanted [`Block`] to remain very user-friendly,
/// however the format of the JSON file is very wonky. So instead, `serde` translates
/// to this version of the block, which is easier to convert to JSON.
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "id")]
enum JSONBlock {
    #[serde(rename = "block")]
    CodeBlock {
        block: CodeBlock,
        #[serde(skip_serializing_if = "Option::is_none")]
        args: Option<Args>,
        #[serde(skip_serializing_if = "Option::is_none")]
        action: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        data: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        attribute: Option<String>,
        #[serde(rename = "subAction")]
        #[serde(skip_serializing_if = "Option::is_none")]
        sub_action: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        target: Option<Target>,
    },
    #[serde(rename = "bracket")]
    Bracket {
        #[serde(rename = "direct")]
        direction: BracketDirection,
        #[serde(rename = "type")]
        kind: BracketKind,
    },
}

//
// -- helper types
//

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CodeBlock {
    #[serde(rename = "event")]
    PlayerEvent,
    #[serde(rename = "entity_event")]
    EntityEvent,
    #[serde(rename = "func")]
    Function,
    #[serde(rename = "process")]
    Process,
    #[serde(rename = "player_action")]
    PlayerAction,
    #[serde(rename = "if_player")]
    IfPlayer,
    #[serde(rename = "start_process")]
    StartProcess,
    #[serde(rename = "call_func")]
    CallFunction,
    #[serde(rename = "control")]
    Control,
    #[serde(rename = "set_var")]
    SetVariable,
    #[serde(rename = "if_entity")]
    IfEntity,
    #[serde(rename = "entity_action")]
    EntityAction,
    #[serde(rename = "if_var")]
    IfVariable,
    #[serde(rename = "select_obj")]
    SelectObject,
    #[serde(rename = "game_action")]
    GameAction,
    #[serde(rename = "repeat")]
    Repeat,
    #[serde(rename = "if_game")]
    IfGame,
    #[serde(rename = "else")]
    Else,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Target {
    Selection,
    Default,
    Killer,
    Damager,
    Victim,
    AllPlayers,
    Shooter,
    Projectile,
    AllEntities,
    AllMobs,
    LastEntity,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
enum JSONBlockId {
    Block,
    Bracket,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum VariableScope {
    #[serde(rename = "unsaved")]
    Game,
    Saved,
    Local,
    Line,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ParamKind {
    #[serde(rename = "any")]
    Any,
    #[serde(rename = "dict")]
    Dictionary,
    #[serde(rename = "item")]
    Item,
    #[serde(rename = "vec")]
    Vector,
    #[serde(rename = "num")]
    Number,
    #[serde(rename = "part")]
    Particle,
    #[serde(rename = "pot")]
    Potion,
    #[serde(rename = "txt")]
    String,
    #[serde(rename = "loc")]
    Location,
    #[serde(rename = "comp")]
    Text,
    #[serde(rename = "list")]
    List,
    #[serde(rename = "snd")]
    Sound,
    #[serde(rename = "var")]
    Variable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BracketDirection {
    Open,
    Close,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BracketKind {
    #[serde(rename = "norm")]
    Normal,
    #[serde(rename = "repeat")]
    Repeat,
}

//
// -- impls
//

impl Args {
    pub fn with(values: Vec<Item>) -> Self {
        Args(values.into_iter().enumerate().collect())
    }

    pub fn with_tags(args: Vec<Item>, tags: Vec<Item>) -> Self {
        let tags_begin = 27 - tags.len();

        Args(
            (0..=27)
                .zip(args)
                .chain((tags_begin..=27).zip(tags))
                .collect(),
        )
    }
}

impl From<JSONBlock> for Block {
    fn from(json: JSONBlock) -> Self {
        match json {
            JSONBlock::CodeBlock {
                block,
                args,
                action,
                data,
                attribute,
                sub_action,
                target,
            } => match block {
                CodeBlock::PlayerEvent => Block::PlayerEvent {
                    action: action.expect("action should be defined"),
                    ls_cancel: attribute.is_some(),
                },
                CodeBlock::EntityEvent => Block::EntityEvent {
                    action: action.expect("action should be defined"),
                    ls_cancel: attribute.is_some(),
                },
                CodeBlock::Function => Block::Function {
                    args: args.expect("args should be defined"),
                    name: data.expect("data should be defined"),
                },
                CodeBlock::Process => Block::Process {
                    args: args.expect("args should be defined"),
                    name: data.expect("data should be defined"),
                },
                CodeBlock::PlayerAction => Block::PlayerAction {
                    args: args.expect("args should be defined"),
                    action: action.expect("action should be defined"),
                    target,
                },
                CodeBlock::IfPlayer => Block::IfPlayer {
                    args: args.expect("args should be defined"),
                    action: action.expect("action should be defined"),
                    target,
                    is_negated: attribute.is_some(),
                },
                CodeBlock::StartProcess => Block::StartProcess {
                    args: args.expect("args should be defined"),
                    proc: data.expect("data should be defined"),
                },
                CodeBlock::CallFunction => Block::CallFunction {
                    args: args.expect("args should be defined"),
                    func: data.expect("data should be defined"),
                },
                CodeBlock::Control => Block::Control {
                    args: args.expect("args should be defined"),
                    action: action.expect("action should be defined"),
                },
                CodeBlock::SetVariable => Block::SetVariable {
                    args: args.expect("args should be defined"),
                    action: action.expect("action should be defined"),
                },
                CodeBlock::IfEntity => Block::IfEntity {
                    args: args.expect("args should be defined"),
                    action: action.expect("action should be defined"),
                    target,
                    is_negated: attribute.is_some(),
                },
                CodeBlock::EntityAction => Block::EntityAction {
                    args: args.expect("args should be defined"),
                    action: action.expect("action should be defined"),
                    target,
                },
                CodeBlock::IfVariable => Block::IfVariable {
                    args: args.expect("args should be defined"),
                    action: action.expect("action should be defined"),
                    is_negated: attribute.is_some(),
                },
                CodeBlock::SelectObject => Block::SelectObject {
                    args: args.expect("args should be defined"),
                    action: action.expect("action should be defined"),
                    sub_action,
                    is_negated: attribute.is_some(),
                },
                CodeBlock::GameAction => Block::GameAction {
                    args: args.expect("args should be defined"),
                    action: action.expect("action should be defined"),
                },
                CodeBlock::Repeat => Block::Repeat {
                    args: args.expect("args should be defined"),
                    action: action.expect("action should be defined"),
                    sub_action,
                    is_negated: attribute.is_some(),
                },
                CodeBlock::IfGame => Block::IfGame {
                    args: args.expect("args should be defined"),
                    action: action.expect("action should be defined"),
                    is_negated: attribute.is_some(),
                },
                CodeBlock::Else => Block::Else,
            },
            JSONBlock::Bracket { direction, kind } => Block::Bracket { direction, kind },
        }
    }
}

impl From<Block> for JSONBlock {
    fn from(block: Block) -> Self {
        match block {
            Block::PlayerEvent { action, ls_cancel } => JSONBlock::CodeBlock {
                block: CodeBlock::PlayerEvent,
                args: None,
                action: Some(action),
                data: None,
                attribute: ls_cancel.then(|| "LS-CANCEL".to_string()),
                sub_action: None,
                target: None,
            },
            Block::EntityEvent { action, ls_cancel } => JSONBlock::CodeBlock {
                block: CodeBlock::EntityEvent,
                args: None,
                action: Some(action),
                data: None,
                attribute: ls_cancel.then(|| "LS-CANCEL".to_string()),
                sub_action: None,
                target: None,
            },
            Block::Function { args, name } => JSONBlock::CodeBlock {
                block: CodeBlock::Function,
                args: Some(args),
                action: None,
                data: Some(name),
                attribute: None,
                sub_action: None,
                target: None,
            },
            Block::Process { args, name } => JSONBlock::CodeBlock {
                block: CodeBlock::Process,
                args: Some(args),
                action: None,
                data: Some(name),
                attribute: None,
                sub_action: None,
                target: None,
            },
            Block::PlayerAction {
                args,
                action,
                target,
            } => JSONBlock::CodeBlock {
                block: CodeBlock::PlayerAction,
                args: Some(args),
                action: Some(action),
                data: None,
                attribute: None,
                sub_action: None,
                target,
            },
            Block::IfPlayer {
                args,
                action,
                target,
                is_negated,
            } => JSONBlock::CodeBlock {
                block: CodeBlock::IfPlayer,
                args: Some(args),
                action: Some(action),
                data: None,
                attribute: is_negated.then(|| "NOT".to_string()),
                sub_action: None,
                target,
            },
            Block::StartProcess { args, proc } => JSONBlock::CodeBlock {
                block: CodeBlock::StartProcess,
                args: Some(args),
                action: None,
                data: Some(proc),
                attribute: None,
                sub_action: None,
                target: None,
            },
            Block::CallFunction { args, func } => JSONBlock::CodeBlock {
                block: CodeBlock::CallFunction,
                args: Some(args),
                action: None,
                data: Some(func),
                attribute: None,
                sub_action: None,
                target: None,
            },
            Block::Control { args, action } => JSONBlock::CodeBlock {
                block: CodeBlock::Control,
                args: Some(args),
                action: Some(action),
                data: None,
                attribute: None,
                sub_action: None,
                target: None,
            },
            Block::SetVariable { args, action } => JSONBlock::CodeBlock {
                block: CodeBlock::SetVariable,
                args: Some(args),
                action: Some(action),
                data: None,
                attribute: None,
                sub_action: None,
                target: None,
            },
            Block::IfEntity {
                args,
                action,
                target,
                is_negated,
            } => JSONBlock::CodeBlock {
                block: CodeBlock::IfEntity,
                args: Some(args),
                action: Some(action),
                data: None,
                attribute: is_negated.then(|| "NOT".to_string()),
                sub_action: None,
                target,
            },
            Block::EntityAction {
                args,
                action,
                target,
            } => JSONBlock::CodeBlock {
                block: CodeBlock::EntityAction,
                args: Some(args),
                action: Some(action),
                data: None,
                attribute: None,
                sub_action: None,
                target,
            },
            Block::IfVariable {
                args,
                action,
                is_negated,
            } => JSONBlock::CodeBlock {
                block: CodeBlock::IfVariable,
                args: Some(args),
                action: Some(action),
                data: None,
                attribute: is_negated.then(|| "NOT".to_string()),
                sub_action: None,
                target: None,
            },
            Block::SelectObject {
                args,
                action,
                sub_action,
                is_negated,
            } => JSONBlock::CodeBlock {
                block: CodeBlock::SelectObject,
                args: Some(args),
                action: Some(action),
                data: None,
                attribute: is_negated.then(|| "NOT".to_string()),
                sub_action,
                target: None,
            },
            Block::GameAction { args, action } => JSONBlock::CodeBlock {
                block: CodeBlock::GameAction,
                args: Some(args),
                action: Some(action),
                data: None,
                attribute: None,
                sub_action: None,
                target: None,
            },
            Block::Repeat {
                args,
                action,
                sub_action,
                is_negated,
            } => JSONBlock::CodeBlock {
                block: CodeBlock::Repeat,
                args: Some(args),
                action: Some(action),
                data: None,
                attribute: is_negated.then(|| "NOT".to_string()),
                sub_action,
                target: None,
            },
            Block::IfGame {
                args,
                action,
                is_negated,
            } => JSONBlock::CodeBlock {
                block: CodeBlock::IfGame,
                args: Some(args),
                action: Some(action),
                data: None,
                attribute: is_negated.then(|| "NOT".to_string()),
                sub_action: None,
                target: None,
            },
            Block::Else => JSONBlock::CodeBlock {
                block: CodeBlock::Else,
                args: None,
                action: None,
                data: None,
                attribute: None,
                sub_action: None,
                target: None,
            },
            Block::Bracket {
                direction: bracket_direction,
                kind: bracket_type,
            } => JSONBlock::Bracket {
                direction: bracket_direction,
                kind: bracket_type,
            },
        }
    }
}

impl From<JSONArgs> for Args {
    fn from(json: JSONArgs) -> Self {
        Args(
            json.items
                .into_iter()
                .map(|item| (item.slot, item.item))
                .collect(),
        )
    }
}

impl From<Args> for JSONArgs {
    fn from(args: Args) -> Self {
        JSONArgs {
            items: args
                .0
                .into_iter()
                .map(|(slot, item)| SlottedItem { item, slot })
                .collect(),
        }
    }
}

impl Template {
    pub fn new(blocks: Vec<Block>) -> Self {
        Template { blocks }
    }

    pub fn encode(&self) -> Option<String> {
        // gzip then base64
        let json = serde_json::to_string(self).ok()?;

        let mut gz_encoder = GzEncoder::new(Vec::new(), flate2::Compression::default());
        gz_encoder.write_all(json.as_bytes()).ok()?;

        let gzipped = gz_encoder.finish().ok()?;
        let base64 = BASE64_STANDARD.encode(gzipped);

        Some(base64)
    }

    pub fn decode(data: &str) -> Option<Self> {
        // base64 then gunzip
        let gzipped = BASE64_STANDARD.decode(data).ok()?;

        let mut gz_decoder = GzDecoder::new(&gzipped[..]);
        let mut json = String::new();
        gz_decoder.read_to_string(&mut json).ok()?;

        let template: Template = serde_json::from_str(&json).ok()?;

        Some(template)
    }

    pub fn get_name(&self) -> Option<String> {
        if let Some(Block::Function { name, .. }) = self.blocks.first() {
            Some(name.clone())
        } else {
            None
        }
    }

    pub fn add_block(&mut self, block: Block) -> &mut Self {
        self.blocks.push(block);
        self
    }

    pub fn set_var(&mut self, action: impl Into<String>, args: Args) -> &mut Self {
        self.add_block(Block::SetVariable {
            args,
            action: action.into(),
        });
        self
    }

    pub fn set_var_bitwise(
        &mut self,
        bitwise_op: impl Into<String>,
        result: Item,
        a: Item,
        b: Item,
    ) -> &mut Self {
        self.set_var(
            "Bitwise",
            Args::with_tags(
                vec![result, a, b],
                vec![
                    Item::Tag {
                        option: "64-bit".to_string(),
                        tag: "Bit Precision".to_string(),
                        action: "Bitwise".to_string(),
                        block: CodeBlock::SetVariable,
                    },
                    Item::Tag {
                        option: bitwise_op.into(),
                        tag: "Operator".to_string(),
                        action: "Bitwise".to_string(),
                        block: CodeBlock::SetVariable,
                    },
                ],
            ),
        )
    }

    pub fn if_var(&mut self, action: impl Into<String>, args: Args) -> &mut Self {
        self.add_block(Block::IfVariable {
            args,
            action: action.into(),
            is_negated: false,
        });
        self
    }

    pub fn else_block(&mut self) -> &mut Self {
        self.add_block(Block::Else);
        self
    }

    pub fn repeat(&mut self, action: impl Into<String>, args: Args) -> &mut Self {
        self.add_block(Block::Repeat {
            args,
            action: action.into(),
            sub_action: None,
            is_negated: false,
        });
        self
    }

    pub fn open_bracket(&mut self) -> &mut Self {
        self.add_block(Block::Bracket {
            direction: BracketDirection::Open,
            kind: BracketKind::Normal,
        });
        self
    }

    pub fn close_bracket(&mut self) -> &mut Self {
        self.add_block(Block::Bracket {
            direction: BracketDirection::Close,
            kind: BracketKind::Normal,
        });
        self
    }

    pub fn open_bracket_repeat(&mut self) -> &mut Self {
        self.add_block(Block::Bracket {
            direction: BracketDirection::Open,
            kind: BracketKind::Repeat,
        });
        self
    }

    pub fn close_bracket_repeat(&mut self) -> &mut Self {
        self.add_block(Block::Bracket {
            direction: BracketDirection::Close,
            kind: BracketKind::Repeat,
        });
        self
    }

    pub fn control(&mut self, action: impl ToString, args: Args) -> &mut Self {
        self.add_block(Block::Control {
            args,
            action: action.to_string(),
        });
        self
    }

    pub fn call_function(&mut self, func: impl ToString, args: Args) -> &mut Self {
        self.add_block(Block::CallFunction {
            args,
            func: func.to_string(),
        });
        self
    }

    pub fn print(&mut self, args: Args) -> &mut Self {
        self.add_block(Block::PlayerAction {
            args,
            action: "SendMessage".to_string(),
            target: Some(Target::AllPlayers),
        });
        self
    }

    pub fn start_function(name: impl ToString) -> Self {
        Self {
            blocks: vec![Block::Function {
                args: Args::default(),
                name: name.to_string(),
            }],
        }
    }

    pub fn start_function_hidden(name: impl ToString) -> Self {
        Self {
            blocks: vec![Block::Function {
                args: Args::with_tags(
                    vec![],
                    vec![Item::Tag {
                        option: "True".to_string(),
                        tag: "Is Hidden".to_string(),
                        action: "dynamic".to_string(),
                        block: CodeBlock::Function,
                    }],
                ),
                name: name.to_string(),
            }],
        }
    }
}

impl Item {
    pub fn num(name: impl ToString) -> Self {
        Item::Number {
            name: name.to_string(),
        }
    }

    pub fn var(kind: impl Into<String>) -> Self {
        Item::Variable {
            name: kind.into(),
            scope: VariableScope::Game,
        }
    }

    pub fn string(name: impl Into<String>) -> Self {
        Item::String { name: name.into() }
    }
}

//
// -- tests
//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_conversion() {
        let block = Block::PlayerEvent {
            action: "test".to_string(),
            ls_cancel: true,
        };
        let json: JSONBlock = block.clone().into();
        let block2: Block = json.into();
        assert_eq!(block, block2);
    }
}
