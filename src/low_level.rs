use std::time::Duration;
use std::time::SystemTime;

use serde::Deserialize;
use serde::Serialize;
use serde_json::Number;
use uuid::Uuid;

use crate::Object;

pub fn deserialize_project(serialized: &str) -> Result<LowLevelProject, serde_json::Error> {
    Ok(serde_json::from_str(&serialized)?)
}

pub fn serialize_project(deserialized: &LowLevelProject) ->  Result<String, serde_json::Error> {
    Ok(serde_json::to_string(&deserialized)?)
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LowLevelProject {
    pub scenes: Vec<LowLevelScene>,
    pub stage_size: LowLevelStageSize,
    pub player_version: String,
    pub version: Number,
    pub abilities: Vec<LowLevelAbility>,
    //TODO: customObjects
    pub font_size: Number,
    pub custom_rules: Vec<LowLevelCustomRule>,
    pub objects: Vec<LowLevelObject>,
    pub variables: Vec<LowLevelVariable>,
    pub custom_rule_instances: Vec<LowLevelCustomRuleInstance>,
    pub event_parameters: Vec<LowLevelEventParameters>,
    pub scene_references: Vec<LowLevelSceneReference>,
    #[serde(rename(serialize = "requires_beta_editor", deserialize = "requires_beta_editor"))]
    pub requires_beta_editor: bool,
    pub rules: Vec<LowLevelRule>,
}

impl LowLevelProject {
    pub fn object_with_id(self: &Self, object_id: &String) -> Option<&LowLevelObject> {
        self.objects.iter().find(|object| &object.object_id == object_id)
    }
    pub fn rule_with_id(self: &Self, rule_id: &String) -> Option<&LowLevelRule> {
        self.rules.iter().find(|rule| &rule.id == rule_id)
    }
    pub fn ability_with_id(self: &Self, ability_id: &String) -> Option<&LowLevelAbility> {
        self.abilities.iter().find(|ability| &ability.ability_id == ability_id)
    }
    pub fn new(stage_size: LowLevelStageSize, player_version: String, version: Number, font_size: Number, requires_beta_editor: bool) -> Self {
        Self {
            scenes: vec!(),
            abilities: vec!(),
            custom_rules: vec!(),
            objects: vec!(),
            variables: vec!(),
            custom_rule_instances: vec!(),
            event_parameters: vec!(),
            scene_references: vec!(),
            rules: vec!(),
            player_version, stage_size, version, font_size, requires_beta_editor
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LowLevelScene {
    pub name: String,
    pub id: String,
    pub objects: Vec<String>,
}

impl LowLevelScene {
    pub fn new(name: String, objects: Vec<String>) -> Self {
        Self {
            name, objects, id: id()
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LowLevelStageSize {
    pub width: Number,
    pub height: Number
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LowLevelAbility {
    pub name: Option<String>,
    #[serde(rename(serialize = "abilityID", deserialize = "abilityID"))]
    pub ability_id: String,
    pub created_at: Number,
    pub parameters: Option<Vec<LowLevelParameter>>,
    pub blocks: Vec<LowLevelBlock>,
}

impl LowLevelAbility {
    pub fn new() -> Self {
        Self {
            name: None,
            ability_id: id(),
            created_at: LowLevelAbility::created_at(),
            parameters: None,
            blocks: vec!(),
        }
    }
    fn created_at() -> Number {
        //TODO: Better way to get SystemTime for 1/1/01
        let number = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH + Duration::from_secs(978307200)).unwrap_or(Duration::from_secs(0)).as_secs() as f64;
        Number::from_f64(number).unwrap_or(Number::from_f64(0.0).unwrap())
    }
}
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LowLevelCustomRule {
    name: String,
    id: String,
    #[serde(rename(deserialize = "abilityID", serialize = "abilityID"))]
    ability_id: String,
    parameters: Vec<LowLevelParameter>,
    rules: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LowLevelObject {
    #[serde(rename(serialize = "objectID", deserialize = "objectID"))]
    pub object_id: String,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub hs_type: Number,
    pub filename: String,
    pub width: String,
    pub height: String,
    pub name: String,
    pub rules: Vec<String>,
    pub x_position: String,
    pub y_position: String,
    pub resize_scale: String,
    pub rotation: String,
    pub text: Option<String>,
    #[serde(rename(deserialize = "abilityID", serialize = "abilityID"))]
    pub ability_id: String
}

impl LowLevelObject {
    pub fn new(hs_type: Number, filename: String, width: String, height: String, name: String, rules: Vec<String>, x_position: String, y_position: String, resize_scale: String, rotation: String, ability_id: String, text: Option<String>) -> Self {
        Self {
            object_id: id(),
            hs_type, filename, width, height, name, rules, x_position, y_position, resize_scale, rotation, text, ability_id
        }
    }
}
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LowLevelVariable {
    object_id_string: String,
    #[serde(rename(deserialize = "type", serialize = "type"))]
    hs_type: Number,
    name: String
}
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LowLevelCustomRuleInstance {
    id: String,
    #[serde(rename(deserialize = "customRuleID", serialize = "customRuleID"))]
    custom_rule_id: String,
    parameters: Vec<LowLevelParameter>,
}
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LowLevelEventParameters {
    id: String,
    block_type: Number,
    description: String,
}
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LowLevelSceneReference {
    id: String,
    block_type: Number,
    description: String,
    scene: String,
}
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LowLevelParameter {
    key: String,
    default_value: String,
    value: Option<String>,
    #[serde(rename(deserialize = "type", serialize = "type"))]
    hs_type: Number,
    // datum: Option<LowLevelParameterBlock>
}
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LowLevelRule {
    pub id: String,
    pub rule_block_type: Number,
    #[serde(rename(deserialize = "objectID", serialize = "objectID"))]
    pub object_id: Option<String>,
    #[serde(rename(deserialize = "abilityID", serialize = "abilityID"))]
    pub ability_id: String,
    pub parameters: Vec<LowLevelParameter>,
}

impl LowLevelRule {
    pub fn new(hs_type: Number, ability_id: String) -> Self {
        Self {
            id: id(),
            rule_block_type: hs_type,
            object_id: None,
            parameters: vec!(),
            ability_id
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LowLevelBlock {
    #[serde(rename(deserialize = "type", serialize = "type"))]
    pub hs_type: Number,
    pub description: String,
    #[serde(rename(deserialize = "block_class", serialize = "block_class"))]
    pub block_class: String,
    pub parameters: Vec<LowLevelParameter>,
}
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LowLevelParameterBlock {
    // This should also allow the other forms, which should be different types
    #[serde(rename(deserialize = "type", serialize = "type"))]
    hs_type: Number,
    description: String,
    #[serde(rename(deserialize = "block_class", serialize = "block_class"))]
    block_class: String,
    params: Vec<LowLevelParameter>,
}

fn id() -> String {
    Uuid::new_v4().to_string()
}