# Rust程序设计语言 2025春 课程设计——PVZ

## 项目设计

### 框架使用
- 代码&框架版本
```
cargo 1.86.0 (adf9b6ad1 2025-02-28)
bevy = "0.16.0"
rand = "0.9.1"
```

### 设计思路
- 使用bevy作为游戏引擎，使用bevy的ecs架构来管理游戏对象（同时使用event驱动游戏各个系统），同时加入了MVC设计模式来更好地管理代码
- **在接下来模块的介绍中仅描述系统的行为，模块之间的关联会用括号指出，同时可以扩展的地方也会用括号指出，起到一个引导看代码的作用，因为我bevy文档没一个看明白的我也不知道怎么写（**
- 代码文件结构及解释
``` 
pvz/src
├── app.rs
├── config.rs
├── game.rs
├── main.rs
├── model                        // 基本的组件：阳光、植物、投射物、僵尸
│   ├── components.rs                // 全局游戏属性：游戏状态、地图栅格坐标、UiTimer（动画播放辅助器）、
│   ├── events.rs                    // 事件：投射物->僵尸、僵尸->植物等耦合的事件
│   ├── level.rs                     // 关卡：关卡设计
│   ├── mod.rs                       // 模块入口
│   ├── plant.rs                     // 植物
│   ├── plant_events.rs              // 植物事件：生成植物；铲除植物；成功/失败生成植物事件（主要用来game中的ui的功能）
│   ├── projectile.rs                // 投射物
│   ├── projectile_events.rs         // 投射物事件：生成投射物
│   ├── sun.rs                       // 阳光
│   ├── sun_events.rs                // 阳光事件：自然生成阳光；植物生成阳光；阳光资源改变
│   ├── tile.rs                      // 地图栅格：用以辅助植物定位、检测植物重新放置、检测地图块是否可以放植物（虽然还没实现水池地图块）
│   ├── zombie.rs                    // 僵尸
│   ├── zombie_events.rs             // 僵尸事件：生成僵尸；僵尸防具破坏
│   └── zombie_pole_vaulting.rs      // 硬编码了撑杆僵尸的逻辑，，这个想不到很漂亮的实现方式
├── systems                      // 游戏逻辑：阳光生成掉落、植物逻辑、僵尸逻辑、投射物逻辑、投射物->僵尸逻辑、僵尸->植物逻辑、关卡逻辑
│   ├── camera.rs
│   ├── collision_check.rs           // 碰撞检测：投射物->僵尸、僵尸->植物；
│   ├── keyboard_control.rs          // 调试使用的键盘控制：生成僵尸
│   ├── lawn.rs                      // 对tile.rs的使用
│   ├── level_manage.rs              // 关卡管理：关卡开始、关卡结束、成功失败判定
│   ├── mod.rs
│   ├── mouse_control.rs             // **核心游玩逻辑**：鼠标控制：阳光拾取、植物种植、植物铲除
│   ├── plant_manage.rs              // 植物管理：植物生成、植物逻辑（植物种植、植物铲除、植物攻击）
│   ├── projectile_manage.rs         // 投射物管理：投射物生成、投射物逻辑（投射物飞行）
│   ├── sun_manage.rs
│   └── zombie_manage.rs
└── view                         // 视觉效果：对bevy加载资源稍作封装、
    ├── back_ground.rs               // 实现了game中的背景
    ├── get_sprites.rs               // 封装bevy加载资源，（不过可复用性不高）
    ├── menu_ui.rs                   // 菜单UI：主菜单、关卡选择
    ├── mod.rs
    ├── play_animation.rs            // 实现了游戏中所有动画的播放逻辑
    ├── pvz_ui.rs                    // 游戏UI：SunBank、植物卡槽、铲子
    └── result_ui.rs                 // 结果UI：成功/失败界面
```

#### 阳光系统
- 涉及`sun.rs`，`sun_events.rs`，`sun_manage.rs`
- 阳光组件设计：
  - 阳光类型，区分自然落下的阳光、向日葵产生的阳光，交给不同的动画系统（自然落下是匀速落到终点，向日葵产生的阳光是以抛物线落到向日葵前方）
  - 全局阳光Timer——用于阳光的自然生成
  - 掉落Timer——用于阳光的掉落动画，经过多久完成动画并停在目标位置
  - 阳光消失Timer——阳光超时自动消失
  - 阳光数量（作为资源存储在全局中）
- 阳光系统：
  - 自然生成阳光`sun_produce_sun`——GlobalSunTimer计时，计时完成后spawn一个自然阳光实体 （可以完成：设计一个关卡状态，晚上就不对它进行计时）
  - 向日葵生产阳光`sunflower_produce_sun`——监听向日葵系统发送的事件，在事件描述的位置生成一个向日葵阳光实体
  - 阳光资源增加/消耗`sun_add/sun_consume`——改变全局阳光数量，发送阳光资源改变事件（UI会监听这个事件，更新sunbank的UI）
  - 阳光消失`sun_despawn_with_time`——阳光实体消失计时器结束，despawn阳光实体
  - 自然阳光掉落系统`sun_fall_system`——获取自然生成阳光实体，根据它目标位置计算当前位置，并修改阳光的transformer以实现运动
  - 向日葵阳光掉落系统`flower_sun_fall_system`——获取向日葵生成阳光实体，根据它目标位置以及Timer计时完成的百分比计算当前位置（抛物线轨迹），并修改阳光的transformer以实现运动

#### 植物模块
- 涉及`plant.rs`，`plant_events.rs`，`plant_manage.rs`
- 植物组件设计：
  - 血量
  - 地图栅格坐标
  - 植物类型
  - 开火伤害&间隔
  - 阳光生产数量&间隔
  - 植物阳光消耗作为资源存储在全局中（可以更新、这里可以用作以后的扩展设计、平衡调整等）
- 植物系统：
  - 生成植物`spawn_plant`——创建实体、检测阳光、获取点击坐标并发送植物生成成功/失败事件 （阳光系统会监听这个事件，更新阳光资源）
  - 铲除植物`shovel_plant`——将植物生命设置为-1，等待其他系统自动回收
  - 植物伤害机制`plant_receive_damage`——植物生命值减少
  - 向日葵生产阳光`sunflower_produce`——向日葵每隔一段时间生产阳光，阳光系统会监听这个事件，生成一个阳光实体
  - 豌豆射手射击`peashooter_shoot`——检测僵尸是否在同一行，发送生成投射物事件

#### 投射物模块
- 涉及`projectile.rs`，`projectile_events.rs`，`projectile_manage.rs`
- 投射物组件设计：
  - 投射物类型（目前只实现了豌豆，后续可以再扩展标记组件）
  - 投射物伤害
  - 投射物回收Timer（用于投射物一定时间回收，节省系统资源）
  - 投射物飞行速度（实现了x、y方向的速度，这样可以扩展杨桃的子弹）
- 投射物系统：
  - 生成投射物`spawn_pea`——监听植物系统发送的事件，生成豌豆投射物实体
  - 豌豆飞行`move_pea`——根据pea的速度和时间变化更新pea位置      （是时间相关类型游戏）

#### 僵尸模块
- 涉及`zombie.rs`，`zombie_events.rs`，`zombie_manage.rs`，`zombie_pole_vaulting.rs`，`collision_check.rs`
- 僵尸组件设计：
  - 僵尸类型（目前实现了普通僵尸、路障(智障bushi)僵尸、撑杆僵尸）
  - 僵尸生命
  - 僵尸移动速度
  - 僵尸攻击力
  - 僵尸防具（路障、可以实现铁桶）
  - 僵尸行为——攻击/移动（系统可以根据这个类型决定僵尸的数据的行为）
- 僵尸系统
  - 生成僵尸`spawn_zombie`——监听关卡系统发送的事件，生成僵尸实体
  - 僵尸行走系统`zombie_move`——根据僵尸的速度和时间变化更新僵尸位置
  - 僵尸似了`despawn_zombie`——僵尸生命值<=0时，despawn僵尸实体
  - 僵尸状态切换系统（攻击->行走/行走->攻击）——读取僵尸target（植物）消失的事件，将僵尸行为切换为行走/读取僵尸碰撞到植物的事件，将僵尸行为切换为攻击，同时更新僵尸的贴图，换为攻击的贴图
    - `zombie_recover_walk_system`——普通/路障/其他有防具的僵尸
    - `zombie_pole_vaulting_recover_walk_system`——撑杆僵尸，贴图不一样，硬编码了一个
  - 僵尸防具系统`break_zombie_defender`——监听僵尸防具破坏事件，更新僵尸贴图

#### 投射物对僵尸造成伤害模块
- 涉及`collision_check.rs`
- `detect_pea_zombie_collision`——使用bevy的BoundingCircle和Aabb2d求交实现检测投射物与僵尸的碰撞，发送僵尸接收到伤害的事件，僵尸系统监听这个事件，更新僵尸生命值
- `handle_pea_hit_zombie`——处理投射物对僵尸造成伤害事件，更新僵尸生命值

#### 僵尸对植物造成伤害模块
- 涉及`collision_check.rs`
- `detect_zombie_plant_collision`——使用bevy的BoundingCircle和Aabb2d求交实现检测僵尸与植物的碰撞，发送植物接收到伤害的事件，植物系统监听这个事件，更新植物生命值
- `handle_zombie_collide_plant`——处理僵尸与植物碰撞事件，如果僵尸非攻击状态则将僵尸状态设置为攻击，并设置僵尸的目标植物（用于僵尸攻击切换到行走状态），如果僵尸为攻击状态则计时攻击计时器，到时间发送僵尸攻击植物事件，植物系统监听这个事件，更新植物生命值

#### 撑杆僵尸模块
- 涉及`zombie_pole_vaulting.rs`，`zombie_pole_vaulting.rs`，`play_animation.rs`
  - 组件还是那些组件
  - 由于素材的缘故，将撑杆僵尸的跳跃分为了两个阶段播放动画，phase1为撑杆跳起，phase2为撑杆落地
  - 播放动画由事件串联，开跳为一个事件（存储血量、transformer等），上跳完成为一个事件（存储血量，transformer等），落地为一个事件（存储transformer等），最后根据落地事件重新spawn一个撑杆僵尸实体，**同时还原了原版中跳起时不能被豌豆打到的机制**
- 系统处理了
  - 撑杆僵尸碰到植物，检测是否跳过，如果未跳过则播放跳起动画，跳过则进入攻击状态
  - 跳起动画是一个连贯的系统
  - 其余行为与僵尸一致

#### 关卡模块
- 涉及`level.rs`，`level_manage.rs`
- 关卡组件设计：
  - Level作为资源存储在全局中，其中存储ZombieWave
  - ZombieWave存储一波中的僵尸类型、数量、生成间隔（可以完成：更便捷的关卡添加与设计）
- 关卡系统：
  - 关卡开始前会调用level的start_timer，计时完成则开始从level中取出ZombieWave组件，将它生成一个组件仅有ZombieWave的实体
  - ZombieWave系统会查询ZombieWave实体，获取僵尸类型、数量、生成间隔等信息，并根据它生成僵尸实体
  - 胜利判定系统：
    - Level队列清空
    - 僵尸实体数量为0
    - 发送关卡成功事件
  - 失败判定系统：
    - 僵尸到达底线则发送关卡失败事件