use low_level::{LowLevelAbility, LowLevelBlock, LowLevelObject, LowLevelProject, LowLevelRule, LowLevelScene};

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
    pub fn from_low_level(low_level_project: LowLevelProject) -> Self {
        let scenes = low_level_project.scenes.iter().map(|low_level_scene| Scene::from_low_level(&low_level_scene, &low_level_project)).collect();
        Self {scenes}
    }
}

#[derive(Debug)]
pub struct Scene {
    objects: Vec<Object>,
    name: String
}

impl Scene {
    pub fn from_low_level(low_level_scene: &LowLevelScene, low_level_project: &LowLevelProject) -> Self {
        let name = &low_level_scene.name;
        let objects = low_level_scene.objects.iter().map(|object_id| {
            match low_level_project.object_with_id(&object_id) {
                None => Object::new(),
                Some(object) => Object::from_low_level(object, low_level_project)
            }
        }).collect();
        Self { name: name.clone(), objects }
    }
}

#[derive(Debug)]
pub struct Object {
    before_game_starts_blocks: Vec<Block>,
    // Custom rules to be implemented later
    // Currently `Object` could be used for custom rules (but not customRuleInstances) but in the future this will be more complete and distinct from custom rules
    rules: Vec<Rule>
}

impl Object {
    pub fn new() -> Self {
        Self { before_game_starts_blocks: vec!(), rules: vec!() }
    }
    pub fn from_low_level(low_level_object: &LowLevelObject, low_level_project: &LowLevelProject) -> Self {
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
        Self {before_game_starts_blocks, rules}
    }
}

#[derive(Debug)]
pub struct Rule {
    event: Option<Block>,
    blocks: Vec<Block>
}
impl Rule {
    pub fn new() -> Self {
        Self {event: None, blocks: vec!()}
    }
    pub fn from_low_level(low_level_rule: &LowLevelRule, low_level_project: &LowLevelProject) -> Self {
        //TODO: Create a block here for event (they aren't in the json here in the same format as everywehr else)
        let event: Option<Block> = None;
        let blocks = match low_level_project.ability_with_id(&low_level_rule.ability_id) {
            None => vec!(),
            Some(low_level_ability) => Block::vec_from_low_level_ability(low_level_ability, low_level_project)
        };
        Self {event, blocks}
    }
}

#[derive(Debug)]
pub struct Block {
    hs_type: BlockType
}

impl Block {
    pub fn vec_from_low_level_ability(low_level_ability: &LowLevelAbility, low_level_project: &LowLevelProject) -> Vec<Self> {
        low_level_ability.blocks.iter().map(|low_level_block| Block::from_low_level(low_level_block, low_level_project)).collect()
    }
    pub fn from_low_level(low_level_block: &LowLevelBlock, low_level_project: &LowLevelProject) -> Self {
        let hs_type = BlockType::ArbitraryID(low_level_block.hs_type.to_string());
        Self {hs_type}
    }
}

#[derive(Debug)]
enum BlockType {
    ArbitraryID(String)
}