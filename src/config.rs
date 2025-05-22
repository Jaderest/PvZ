use bevy::{ecs::system::command::insert_resource, prelude::*};

pub struct ConfigPlugin;
impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameConfig::default())
            .insert_resource(GameType::default())
            .insert_resource(ControlState::default())
            .insert_resource(PlantType::default())
            .insert_resource(WindowResolution::default())
            .add_systems(Startup, setup_window_size);
        //TODO: 设定一下window size
    }
}

#[derive(Resource, Clone, Copy)]
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
pub enum ControlState {
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

#[derive(Resource, Clone, Copy, PartialEq, Eq, Hash)]
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
        Self::Sunflower
    }
}

#[derive(Resource)]
struct WindowResolution {
    large: Vec2,
    medium: Vec2,
    small: Vec2,
}
impl Default for WindowResolution {
    fn default() -> Self {
        Self {
            large: Vec2::new(1920.0, 1080.0),
            medium: Vec2::new(1200.0, 800.0),
            small: Vec2::new(800.0, 600.0),
        }
    }
}

#[derive(Resource, Clone, Copy)]
pub struct GameConfig {
    pub tile_size: f32,
    pub map_width: u32,
    pub map_height: u32,
    pub sun_size: f32,
    // 用以生成地图
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            tile_size: 160.0,
            map_width: 9,
            map_height: 5,
            sun_size: 64.0,
            // game_type: GameType::default(),
        }
    }
}

fn setup_window_size(mut window: Single<&mut Window>, resolution: Res<WindowResolution>) {
    let res = resolution.medium;
    window.resolution.set(res.x, res.y);
    window.resizable = false;
    window.name = Some("PvZ Rust Bevy Ver.".to_string());
    window.resolution.set_scale_factor(1.2);
}

pub fn grid2pixel(game_config: GameConfig, grid_x: f32, gird_y: f32, z: f32) -> Vec3 {
    let tile_size = game_config.tile_size;
    let lawn_width = game_config.map_width as f32 * tile_size;
    let bottom_edge_of_tile = -tile_size * (game_config.map_height as f32 - 2.0);
    let left_edge_of_tile = 0.0 - lawn_width / 2.0;
    let offset_x = left_edge_of_tile + tile_size / 2.0;
    let offset_y = bottom_edge_of_tile + tile_size / 2.0;
    Vec3 {
        x: offset_x + (grid_x * tile_size),
        y: offset_y + (gird_y * tile_size),
        z,
    }
}
