use std::str::FromStr;

use low_level::{serialize_project, LowLevelAbility, LowLevelBlock, LowLevelObject, LowLevelProject, LowLevelRule, LowLevelScene, LowLevelStageSize};
use serde_json::from_str;

mod low_level;

pub fn project_from_json(json: &String) -> Result<Project, serde_json::Error> {
    let low_level_project = low_level::deserialize_project(&json)?;
    Ok(Project::from_low_level(low_level_project))
}

#[derive(Debug)]
pub struct Project {
    scenes: Vec<Scene>
}

impl Project {
    fn from_low_level(low_level_project: LowLevelProject) -> Self {
        let scenes = low_level_project.scenes.iter().map(|low_level_scene| Scene::from_low_level(&low_level_scene, &low_level_project)).collect();
        Self {scenes}
    }
    pub fn blocks_iter_mut(self: &mut Self) -> impl Iterator<Item = &mut Block> {
        self.scenes.iter_mut().flat_map(|scene| scene.blocks_iter_mut())
    }
    pub fn jsonify(self: &Self) -> Result<String, serde_json::Error> {
        let low_level_project = self.to_low_level();
        return serialize_project(&low_level_project);
    }
    fn to_low_level(self: &Self) -> LowLevelProject {
        //TODO: Store stage size
        let stage_size = LowLevelStageSize {width: serde_json::Number::from_f64(1024.0).unwrap(), height: serde_json::Number::from_f64(768.0).unwrap()};
        let mut result = LowLevelProject::new(stage_size, "3.0.0".to_owned(), serde_json::Number::from_f64(34.0).unwrap(), serde_json::Number::from_f64(80.0).unwrap(), false);
        self.scenes.iter().for_each(|scene| scene.to_low_level_in(&mut result));
        return result
    }
}

#[derive(Debug)]
pub struct Scene {
    objects: Vec<Object>,
    name: String
}

impl Scene {
    fn from_low_level(low_level_scene: &LowLevelScene, low_level_project: &LowLevelProject) -> Self {
        let name = &low_level_scene.name;
        let objects = low_level_scene.objects.iter().map(|object_id| {
            match low_level_project.object_with_id(&object_id) {
                None => Object::new(ObjectType { hs_type: 0.0, filename: "monkey.png".to_string(), width: 150.0, height: 150.0 }, object_id.clone()), //TODO: use ObjectType::Monkey or whatever exists when that exists
                Some(object) => Object::from_low_level(object, low_level_project)
            }
        }).collect();
        Self { name: name.clone(), objects }
    }
    pub fn blocks_iter_mut(self: &mut Self) -> impl Iterator<Item = &mut Block> {
        self.objects.iter_mut().flat_map(|object| object.blocks_iter_mut() )
    }
    fn to_low_level_in(self: &Self, mut low_level_project: &mut LowLevelProject) {
        let low_level_object_ids: Vec<ObjectID> = self.objects.iter().map(|object| object.to_low_level_id_in(&mut low_level_project)).collect();
        LowLevelScene::new(self.name.clone(), low_level_object_ids);
    }
}

#[derive(Debug)]
pub struct Object {
    before_game_starts_blocks: Vec<Block>,
    // Custom rules to be implemented later
    // Currently `Object` could be used for custom rules (but not customRuleInstances) but in the future this will be more complete and distinct from custom rules
    rules: Vec<Rule>,
    hs_type: ObjectType,
    x_position: f64,
    y_position: f64,
    resize_scale: f64,
    rotation: f64,
    name: String
}

type ObjectID = String;
impl Object {
    pub fn new(hs_type: ObjectType, name: String) -> Self {
        Self { before_game_starts_blocks: vec!(), rules: vec!(), hs_type, x_position: 0.0, y_position: 0.0, resize_scale: 1.0, rotation: 0.0, name}
    }
    fn from_low_level(low_level_object: &LowLevelObject, low_level_project: &LowLevelProject) -> Self {
        let before_game_starts_blocks = match low_level_project.ability_with_id(&low_level_object.ability_id) {
            None => vec!(),
            Some(ability) => Block::vec_from_low_level_ability(ability, low_level_project)
        };
        let rules = low_level_object.rules.iter().map(|rule_id| {
            match low_level_project.rule_with_id(rule_id) {
                None => Rule::new(),
                Some(low_level_rule) => Rule::from_low_level(low_level_rule, low_level_project)
            }
        }).collect();
        let hs_type = ObjectType { hs_type: low_level_object.hs_type.as_f64().unwrap_or(0.0), filename: low_level_object.filename.clone(), width: from_str(&low_level_object.width).unwrap_or(150.0), height: from_str(&low_level_object.height).unwrap_or(150.0) };
        let x_position: f64 = from_str(low_level_object.x_position.as_str()).unwrap_or(0.0);
        let y_position: f64 = from_str(low_level_object.y_position.as_str()).unwrap_or(0.0);
        let resize_scale: f64 = from_str(low_level_object.resize_scale.as_str()).unwrap_or(0.0);
        let rotation: f64 = from_str(low_level_object.rotation.as_str()).unwrap_or(0.0);
        let name = low_level_object.name.clone();
        Self {before_game_starts_blocks, rules, hs_type, x_position, y_position, resize_scale, rotation, name}
    }
    pub fn blocks_iter_mut(self: &mut Self) -> impl Iterator<Item = &mut Block> {
        self.before_game_starts_blocks.iter_mut().chain(
            self.rules.iter_mut().flat_map(|rule| rule.blocks_iter_mut() )
        )
    }
    fn to_low_level_id_in(self: &Self, mut low_level_project: &mut LowLevelProject) -> ObjectID {
        let before_game_starts_ability_id = Block::vec_to_low_level_ability_id_in(&mut low_level_project, &self.before_game_starts_blocks);
        let rule_ids: Vec<RuleID> = self.rules.iter().map(|rule| rule.to_low_level_id_in_project(&mut low_level_project)).collect();
        //TODO: Don't expect here
        let object = LowLevelObject::new(serde_json::Number::from_f64(self.hs_type.hs_type).expect("Object type"), self.hs_type.filename.clone(), self.hs_type.width.to_string(), self.hs_type.height.to_string(), self.name.clone(), rule_ids, self.x_position.to_string(), self.y_position.to_string(), self.resize_scale.to_string(), self.rotation.to_string(), before_game_starts_ability_id, None);
        let object_id = object.object_id.clone();
        low_level_project.objects.push(object);
        return object_id
    }
}

#[derive(Debug)]
pub struct Rule {
    event: Option<Block>,
    blocks: Vec<Block>
}
type RuleID = String;
impl Rule {
    pub fn new() -> Self {
        Self {event: None, blocks: vec!()}
    }
    fn from_low_level(low_level_rule: &LowLevelRule, low_level_project: &LowLevelProject) -> Self {
        let event = Block {hs_type: BlockType::ArbitraryID(low_level_rule.rule_block_type.as_f64().unwrap_or(-10140001.0))}; //magic id for "soemething is broken" – maybe don't do this?
        let blocks = match low_level_project.ability_with_id(&low_level_rule.ability_id) {
            None => vec!(),
            Some(low_level_ability) => Block::vec_from_low_level_ability(low_level_ability, low_level_project)
        };
        Self {event: Some(event), blocks}
    }
    pub fn blocks_iter_mut(self: &mut Self) -> impl Iterator<Item = &mut Block> {
        self.event.iter_mut().chain(self.blocks.iter_mut())
    }
    fn to_low_level_id_in_project(self: &Self, mut project: &mut LowLevelProject) -> RuleID {
        let ability_id = Block::vec_to_low_level_ability_id_in(&mut project, &self.blocks);
        let block_type_number = match &self.event {
            // In some cases (ae mod) the "None" block actually does something. Not without parameters, but it is worth noting
            None => serde_json::Number::from_f64(22.0).unwrap(), //TODO: Figure out what to do here, or at least use the known block types enum instead of just a literal 22 (representing HSBlockType.None)
            Some(block) => block.hs_type.to_low_level().unwrap_or(serde_json::Number::from_f64(-10140001.0).unwrap()) //magic id for "soemething is broken" – maybe don't do this?
        };
        let rule = LowLevelRule::new(block_type_number, ability_id);
        //TODO: Add parameters
        let rule_id = rule.id.clone();
        project.rules.push(rule);
        return rule_id;
    }
}

#[derive(Debug)]
pub struct Block {
    pub hs_type: BlockType
}

type AbilityID = String;
impl Block {
    fn vec_from_low_level_ability(low_level_ability: &LowLevelAbility, low_level_project: &LowLevelProject) -> Vec<Self> {
        low_level_ability.blocks.iter().map(|low_level_block| Block::from_low_level(low_level_block, low_level_project)).collect()
    }
    fn from_low_level(low_level_block: &LowLevelBlock, low_level_project: &LowLevelProject) -> Self {
        let hs_type = BlockType::ArbitraryID(low_level_block.hs_type.as_f64().unwrap_or(-10140001.0)); //magic id for "soemething is broken" – maybe don't do this?
        Self {hs_type}
    }
    fn vec_to_low_level_ability_id_in(mut project: &mut LowLevelProject, blocks: &Vec<Block>) -> AbilityID {
        let mut ability = LowLevelAbility::new();
        ability.blocks = blocks.iter().map(|block| block.to_low_level_in(&mut project)).collect();
        let id = ability.ability_id.clone();
        project.abilities.push(ability);
        return id
    }
    fn to_low_level_in(self: &Self, low_level_project: &mut LowLevelProject) -> LowLevelBlock {
        let hs_type_number = self.hs_type.to_low_level().unwrap_or(serde_json::Number::from_f64(-10140001.0).unwrap()); //magic id for "soemething is broken" – maybe don't do this?
        LowLevelBlock {
            hs_type: hs_type_number,
            description: "TODO: Description".to_owned(),
            block_class: "method".to_owned(), //TODO: Block class,
            parameters: vec!()
        }
    }
}

type HSTypeID = f64;
#[derive(Debug)]
pub enum BlockType {
    ArbitraryID(HSTypeID)
}

impl BlockType {
    fn to_low_level(self: &Self) -> Option<serde_json::Number> {
        match self {
            BlockType::ArbitraryID(id) => serde_json::Number::from_f64(*id)
        }
    }
}


#[derive(Debug)]
pub struct ObjectType {
    hs_type: HSTypeID,
    filename: String,
    width: f64,
    height: f64
}