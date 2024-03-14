use serde::Deserialize;
use serde::Serialize;
use serde_json::Number;

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
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LowLevelScene {
    pub name: String,
    pub id: String,
    pub objects: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LowLevelStageSize {
    width: Number,
    height: Number
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