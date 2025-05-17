use bevy::prelude::*;

pub struct ConfigPlugin;
impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameConfig::default())
            .insert_resource(GameType::default())
            .insert_resource(ControlState::default())
            .insert_resource(PlantType::default());
    }
}

#[derive(Resource)]
enum GameType {
    DayTimeGrass,
    NightTimeGrass,
    DayTimeWater,
    NightTimeWater,
}

impl Default for GameType {
    fn default() -> Self {
        Self::DayTimeGrass
    }
}

#[derive(Resource)]
enum ControlState {
    /// 可以点击阳光，可以点击种子，可以点击工具
    Normal,
    /// 选择植物
    SelectPlant,
    /// 选择工具
    Shovel,
}
impl Default for ControlState {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Resource, Clone, Copy)]
pub enum PlantType {
    /// 豌豆射手
    PeaShooter,
    /// 向日葵
    Sunflower,
    /// 坚果墙
    WallNut,
}
impl Default for PlantType {
    fn default() -> Self {
        Self::PeaShooter
    }
}

#[derive(Resource)]
pub struct GameConfig {
    pub tile_size: f32,
    pub map_width: u32,
    pub map_height: u32,
    /// 用以生成地图
    pub game_type: GameType,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            tile_size: 128.0,
            map_width: 9,
            map_height: 5,
            game_type: GameType::default(),
        }
    }
}