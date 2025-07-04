use dioxus::prelude::*;
use web_sys::{window, HtmlCanvasElement, CanvasRenderingContext2d};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use std::collections::HashSet;
use gloo_timers::callback::Interval;

#[wasm_bindgen(inline_js = "export function now_ts() { return Date.now() / 1000; } export function get_elem_size(id) { let e = document.getElementById(id); return e ? [e.offsetWidth, e.offsetHeight] : [0,0]; }")]
extern "C" {
    pub fn now_ts() -> f64;
    pub fn get_elem_size(id: &str) -> js_sys::Array;
}

// 游戏配置常量
// 期望的单元格像素大小（可调整）
const DESIRED_TILE_SIZE: u32 = 24; // 期望每个格子的像素宽高
// 玩家视野半径（格子数，决定可探索范围）
const PLAYER_VISION: i32 = 5; // 玩家视野半径
// 迷雾颜色（未探索区域的遮罩颜色）
const FOG_COLOR: &str = "rgba(15, 15, 40, 0.95)"; // 迷雾遮罩颜色
// 迷雾符号（未探索区域显示的符号）
const FOG_SYMBOL: &str = "░"; // 迷雾符号
// 已探索区域标记符号
const DISCOVERED_SYMBOL: &str = "◌"; // 已探索区域符号
// 已探索区域标记颜色
const DISCOVERED_COLOR: &str = "rgba(100, 100, 180, 0.2)"; // 已探索区域符号颜色
// 玩家符号
const PLAYER_SYMBOL: &str = "@"; // 玩家在地图上的显示符号
// 玩家颜色
const PLAYER_COLOR: &str = "#ffcc00"; // 玩家符号颜色
// 动物符号集合
const ANIMAL_SYMBOLS: &[char] = &['$', '&', '#', '%', '*', '¥', '§', '?']; // 动物实体可用的符号
// 动物颜色
const ANIMAL_COLOR: &str = "#ff6666"; // 动物符号颜色
// 植物符号集合
const PLANT_SYMBOLS: &[char] = &['*', '#', '%', '&', '+', '-', '=', '√', '∞', '≈']; // 植物实体可用的符号
// 植物颜色
const PLANT_COLOR: &str = "#66cc66"; // 植物符号颜色
// 物体符号集合
const OBJECT_SYMBOLS: &[char] = &['!', '?', '~', '#', '$', '%', '&', '*', '+', '-', '=', '<', '>', '|', '^', '°', '○', '《', '》', '“', '”', '￥']; // 物体实体可用的符号
// 物体颜色
const OBJECT_COLOR: &str = "#6699ff"; // 物体符号颜色

#[derive(Clone, Copy, PartialEq)]
enum Terrain {
    Water,
    Mountain,
    Forest,
    Plain,
}

impl Terrain {
    fn symbol(&self) -> &'static str {
        match self {
            Terrain::Water => "≈",
            Terrain::Mountain => "^",
            Terrain::Forest => "↑",
            Terrain::Plain => ".",
        }
    }
    fn color(&self) -> &'static str {
        match self {
            Terrain::Water => "#3366cc",
            Terrain::Mountain => "#888888",
            Terrain::Forest => "#336633",
            Terrain::Plain => "#444466",
        }
    }
}

#[derive(Clone)]
enum FoodKind {
    Apple,   // 🍎 +5
    Banana,  // 🍌 +8
    Bread,   // 🍞 +10
    Chicken, // 🍗 +15
    Cake,    // 🍰 +20
}

impl FoodKind {
    fn symbol(&self, eaten: bool) -> char {
        match (self, eaten) {
            (FoodKind::Apple, false) => '🍎',
            (FoodKind::Apple, true) => '🍏',
            (FoodKind::Banana, false) => '🍌',
            (FoodKind::Banana, true) => '🍌', // 香蕉皮可用别的符号
            (FoodKind::Bread, false) => '🍞',
            (FoodKind::Bread, true) => '🥖',
            (FoodKind::Chicken, false) => '🍗',
            (FoodKind::Chicken, true) => '🍖',
            (FoodKind::Cake, false) => '🍰',
            (FoodKind::Cake, true) => '🍮',
        }
    }
    fn color(&self, eaten: bool) -> &'static str {
        match (self, eaten) {
            (FoodKind::Apple, false) => "#ff3333",
            (FoodKind::Apple, true) => "#99ff99",
            (FoodKind::Banana, false) => "#ffe066",
            (FoodKind::Banana, true) => "#cccc66",
            (FoodKind::Bread, false) => "#e0b97d",
            (FoodKind::Bread, true) => "#bfa76f",
            (FoodKind::Chicken, false) => "#ffbb66",
            (FoodKind::Chicken, true) => "#d1a05a",
            (FoodKind::Cake, false) => "#ffb3e6",
            (FoodKind::Cake, true) => "#ffe6b3",
        }
    }
    fn stamina(&self) -> u32 {
        match self {
            FoodKind::Apple => 5,
            FoodKind::Banana => 8,
            FoodKind::Bread => 10,
            FoodKind::Chicken => 15,
            FoodKind::Cake => 20,
        }
    }
    fn name(&self) -> &'static str {
        match self {
            FoodKind::Apple => "苹果",
            FoodKind::Banana => "香蕉",
            FoodKind::Bread => "面包",
            FoodKind::Chicken => "鸡腿",
            FoodKind::Cake => "蛋糕",
        }
    }
}

#[derive(Clone)]
enum EntityType {
    Animal,
    Plant,
    Object { collected: bool },
    Food { eaten: bool, kind: FoodKind },
    Chest { opened: bool },
}

#[derive(Clone)]
struct Entity {
    entity_type: EntityType,
    x: usize,
    y: usize,
    symbol: char,
    color: &'static str,
}

#[derive(Clone)]
struct GameState {
    player_x: usize,
    player_y: usize,
    discovered: HashSet<(usize, usize)>,
    entities: Vec<Entity>,
    items_collected: usize,
    start_time: u64,
    map: Vec<Vec<Terrain>>,
    map_width: usize,
    map_height: usize,
    stamina: u32,
    score: u32,
    message: Option<String>,
}

impl GameState {
    fn new(map_width: usize, map_height: usize) -> Self {
        let mut map = vec![vec![Terrain::Plain; map_width]; map_height];
        
        // 生成地图
        for y in 0..map_height {
            for x in 0..map_width {
                let r = js_sys::Math::random();
                if r.is_nan() || r.is_infinite() {
                    map[y][x] = Terrain::Plain;
                } else {
                map[y][x] = if r < 0.05 {
                    Terrain::Water
                } else if r < 0.1 {
                    Terrain::Mountain
                } else if r < 0.2 {
                    Terrain::Forest
                } else {
                    Terrain::Plain
                };
            }
        }
        }
        
        let mut entities = vec![];
        let mut used_positions = std::collections::HashSet::new();
        
        // 生成动物 (map_width * map_height / 30)
        let animal_target = (map_width * map_height / 30).min(map_width * map_height);
        let mut animal_count = 0;
        while animal_count < animal_target {
            let rand_x = js_sys::Math::random();
            let rand_y = js_sys::Math::random();
            let rand_symbol = js_sys::Math::random();
            
            if rand_x.is_nan() || rand_x.is_infinite() || rand_y.is_nan() || rand_y.is_infinite() || rand_symbol.is_nan() || rand_symbol.is_infinite() {
                continue;
            }
            
            let x = ((rand_x * map_width as f64) as usize).min(map_width - 1);
            let y = ((rand_y * map_height as f64) as usize).min(map_height - 1);
            if used_positions.contains(&(x, y)) { continue; }
            
            let symbol_idx = ((rand_symbol * ANIMAL_SYMBOLS.len() as f64) as usize).min(ANIMAL_SYMBOLS.len() - 1);
            let symbol = ANIMAL_SYMBOLS[symbol_idx];
            
            entities.push(Entity {
                entity_type: EntityType::Animal,
                x,
                y,
                symbol,
                color: ANIMAL_COLOR,
            });
            used_positions.insert((x, y));
            animal_count += 1;
        }
        
        // 生成植物 (map_width * map_height / 40)
        let plant_target = (map_width * map_height / 40).min(map_width * map_height - animal_count);
        let mut plant_count = 0;
        while plant_count < plant_target {
            let rand_x = js_sys::Math::random();
            let rand_y = js_sys::Math::random();
            let rand_symbol = js_sys::Math::random();
            
            if rand_x.is_nan() || rand_x.is_infinite() || rand_y.is_nan() || rand_y.is_infinite() || rand_symbol.is_nan() || rand_symbol.is_infinite() {
                continue;
            }
            
            let x = ((rand_x * map_width as f64) as usize).min(map_width - 1);
            let y = ((rand_y * map_height as f64) as usize).min(map_height - 1);
            if used_positions.contains(&(x, y)) { continue; }
            
            let symbol_idx = ((rand_symbol * PLANT_SYMBOLS.len() as f64) as usize).min(PLANT_SYMBOLS.len() - 1);
            let symbol = PLANT_SYMBOLS[symbol_idx];
            
            entities.push(Entity {
                entity_type: EntityType::Plant,
                x,
                y,
                symbol,
                color: PLANT_COLOR,
            });
            used_positions.insert((x, y));
            plant_count += 1;
        }
        
        // 生成物体 (map_width * map_height / 24)
        let object_target = (map_width * map_height / 24).min(map_width * map_height - animal_count - plant_count);
        let mut object_count = 0;
        while object_count < object_target {
            let rand_x = js_sys::Math::random();
            let rand_y = js_sys::Math::random();
            let rand_symbol = js_sys::Math::random();
            
            if rand_x.is_nan() || rand_x.is_infinite() || rand_y.is_nan() || rand_y.is_infinite() || rand_symbol.is_nan() || rand_symbol.is_infinite() {
                continue;
            }
            
            let x = ((rand_x * map_width as f64) as usize).min(map_width - 1);
            let y = ((rand_y * map_height as f64) as usize).min(map_height - 1);
            if used_positions.contains(&(x, y)) { continue; }
            
            let symbol_idx = ((rand_symbol * OBJECT_SYMBOLS.len() as f64) as usize).min(OBJECT_SYMBOLS.len() - 1);
            let symbol = OBJECT_SYMBOLS[symbol_idx];
            
            entities.push(Entity {
                entity_type: EntityType::Object { collected: false },
                x,
                y,
                symbol,
                color: OBJECT_COLOR,
            });
            used_positions.insert((x, y));
            object_count += 1;
        }
        
        // 生成食物 (map_width * map_height / 50)
        let food_target = (map_width * map_height / 50).max(2);
        let mut food_count = 0;
        let food_kinds = [FoodKind::Apple, FoodKind::Banana, FoodKind::Bread, FoodKind::Chicken, FoodKind::Cake];
        while food_count < food_target {
            let rand_x = js_sys::Math::random();
            let rand_y = js_sys::Math::random();
            let rand_kind = js_sys::Math::random();
            if rand_x.is_nan() || rand_x.is_infinite() || rand_y.is_nan() || rand_y.is_infinite() || rand_kind.is_nan() || rand_kind.is_infinite() {
                continue;
            }
            let x = ((rand_x * map_width as f64) as usize).min(map_width - 1);
            let y = ((rand_y * map_height as f64) as usize).min(map_height - 1);
            if used_positions.contains(&(x, y)) { continue; }
            let kind_idx = ((rand_kind * food_kinds.len() as f64) as usize).min(food_kinds.len() - 1);
            let kind = food_kinds[kind_idx].clone();
            entities.push(Entity {
                entity_type: EntityType::Food { eaten: false, kind: kind.clone() },
                x,
                y,
                symbol: kind.symbol(false),
                color: kind.color(false),
            });
            used_positions.insert((x, y));
            food_count += 1;
        }
        
        // 生成宝箱 (map_width * map_height / 80)
        let chest_target = (map_width * map_height / 80).max(1);
        let mut chest_count = 0;
        while chest_count < chest_target {
            let rand_x = js_sys::Math::random();
            let rand_y = js_sys::Math::random();
            if rand_x.is_nan() || rand_x.is_infinite() || rand_y.is_nan() || rand_y.is_infinite() {
                continue;
            }
            let x = ((rand_x * map_width as f64) as usize).min(map_width - 1);
            let y = ((rand_y * map_height as f64) as usize).min(map_height - 1);
            if used_positions.contains(&(x, y)) { continue; }
            entities.push(Entity {
                entity_type: EntityType::Chest { opened: false },
                x,
                y,
                symbol: '□',
                color: "#ffcc66",
            });
            used_positions.insert((x, y));
            chest_count += 1;
        }
        
        let mut state = Self {
            player_x: map_width / 2,
            player_y: map_height / 2,
            discovered: HashSet::new(),
            entities,
            items_collected: 0,
            start_time: now_ts() as u64,
            map,
            map_width,
            map_height,
            stamina: 20,
            score: 0,
            message: None,
        };
        state.update_discovered_area();
        state
    }
    
    fn update_discovered_area(&mut self) {
        for dy in -PLAYER_VISION..=PLAYER_VISION {
            for dx in -PLAYER_VISION..=PLAYER_VISION {
                let x = self.player_x as isize + dx as isize;
                let y = self.player_y as isize + dy as isize;
                
                if x >= 0 && x < self.map_width as isize && y >= 0 && y < self.map_height as isize {
                    if (dx*dx + dy*dy) as f64 <= (PLAYER_VISION as f64).powi(2) {
                        self.discovered.insert((x as usize, y as usize));
                    }
                }
            }
        }
    }
    
    fn move_player(&mut self, dx: isize, dy: isize) {
        if self.stamina == 0 { return; }
        let new_x = self.player_x as isize + dx;
        let new_y = self.player_y as isize + dy;
        
        if new_x >= 0 && new_x < self.map_width as isize && new_y >= 0 && new_y < self.map_height as isize {
            self.player_x = new_x as usize;
            self.player_y = new_y as usize;
            self.update_discovered_area();
            self.check_for_items();
            if self.stamina > 0 {
                self.stamina -= 1;
            }
        }
    }
    
    fn check_for_items(&mut self) {
        for entity in &mut self.entities {
            match &mut entity.entity_type {
                EntityType::Object { collected } => {
                    if !*collected && entity.x == self.player_x && entity.y == self.player_y {
                        *collected = true;
                        self.items_collected += 1;
                    }
                },
                EntityType::Food { eaten, kind } => {
                    if !*eaten && entity.x == self.player_x && entity.y == self.player_y {
                        *eaten = true;
                        self.stamina += kind.stamina();
                        self.message = Some(format!("你吃了{}，体力+{}！", kind.name(), kind.stamina()));
                    }
                },
                EntityType::Chest { opened } => {
                    if !*opened && entity.x == self.player_x && entity.y == self.player_y {
                        *opened = true;
                        self.score += 10;
                        self.message = Some("你打开了宝箱，获得10分！".to_string());
                    }
                },
                _ => {}
            }
        }
    }
    
    fn get_progress(&self) -> u32 {
        let total_objects = self.entities.iter().filter(|e| matches!(e.entity_type, EntityType::Object { .. })).count();
        if total_objects == 0 {
            100
        } else {
            ((self.items_collected as f64 / total_objects as f64) * 100.0).min(100.0) as u32
        }
    }
}

#[component]
pub fn Game() -> Element {
    // 信号统一声明
    let state = use_signal(|| {
        // 默认初始大小
        GameState::new(25, 20)
    });
    let redraw = use_signal(|| 0u64);
    let game_time = use_signal(|| "00:00".to_string());
    let canvas_width = use_signal(|| 600u32);
    let canvas_height = use_signal(|| 500u32);
    let map_width = use_signal(|| (canvas_width() / DESIRED_TILE_SIZE).max(5) as usize);
    let map_height = use_signal(|| (canvas_height() / DESIRED_TILE_SIZE).max(5) as usize);
    let canvas_container_id = "game-canvas-container";
    let panel_visible = use_signal(|| true);

    // 首次挂载时设置canvas宽高为视口（移动端适配）
    {
        let mut canvas_width = canvas_width.clone();
        let mut canvas_height = canvas_height.clone();
        let mut map_width = map_width.clone();
        let mut map_height = map_height.clone();
        let mut state = state.clone();
        use_effect(move || {
            let arr = get_elem_size(canvas_container_id);
            let w = arr.get(0).as_f64().unwrap_or(600.0) as u32;
            let h = arr.get(1).as_f64().unwrap_or(500.0) as u32;
            if w > 0 && h > 0 {
                canvas_width.set(w);
                canvas_height.set(h);
                let mw = (w / DESIRED_TILE_SIZE).max(5) as usize;
                let mh = (h / DESIRED_TILE_SIZE).max(5) as usize;
                map_width.set(mw);
                map_height.set(mh);
                state.set(GameState::new(mw, mh));
            }
            ()
        });
    }

    // 游戏时间定时器
    {
        let state = state.clone();
        let mut game_time = game_time.clone();
        use_effect(move || {
            let interval = Interval::new(1000, move || {
                let current_time = now_ts() as u64;
                let elapsed = current_time.saturating_sub(state().start_time);
                let minutes = elapsed / 60;
                let seconds = elapsed % 60;
                game_time.set(format!("{:02}:{:02}", minutes, seconds));
            });
            (move || drop(interval))()
        });
    }

    // 监听窗口resize，自动重置游戏
    {
        let mut canvas_width = canvas_width.clone();
        let mut canvas_height = canvas_height.clone();
        let mut map_width = map_width.clone();
        let mut map_height = map_height.clone();
        let mut state = state.clone();
        use_effect(move || {
            let closure = Closure::wrap(Box::new(move || {
                let arr = get_elem_size("game-canvas-container");
                let w = arr.get(0).as_f64().unwrap_or(600.0) as u32;
                let h = arr.get(1).as_f64().unwrap_or(500.0) as u32;
                canvas_width.set(w);
                canvas_height.set(h);
                map_width.set((w / DESIRED_TILE_SIZE).max(5) as usize);
                map_height.set((h / DESIRED_TILE_SIZE).max(5) as usize);
                state.set(GameState::new(map_width(), map_height()));
            }) as Box<dyn FnMut()>);
            window().unwrap().add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref()).unwrap();
            closure.forget();
            ()
        });
    }

    // 处理canvas绘制
    {
        let state = state.clone();
        let redraw = redraw.clone();
        let canvas_width = canvas_width.clone();
        let canvas_height = canvas_height.clone();
        let map_width = map_width.clone();
        let map_height = map_height.clone();
        use_effect(move || {
            let _ = (state(), redraw(), canvas_width(), canvas_height(), map_width(), map_height());
            let window = match window() {
                Some(w) => w,
                None => return,
            };
            let document = match window.document() {
                Some(d) => d,
                None => return,
            };
            let canvas = document.get_element_by_id("gameCanvas")
                .and_then(|el| el.dyn_into::<HtmlCanvasElement>().ok());
            if let Some(canvas) = canvas {
                canvas.set_width(canvas_width());
                canvas.set_height(canvas_height());
                let ctx = match canvas.get_context("2d") {
                    Ok(Some(ctx)) => match ctx.dyn_into::<CanvasRenderingContext2d>() {
                        Ok(ctx) => ctx,
                        Err(_) => return,
                    },
                    _ => return,
                };
                let game_state = state();
                let tile_size = (canvas_width() as f64 / map_width() as f64)
                    .min(canvas_height() as f64 / map_height() as f64);
                ctx.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
                for y in 0..map_height() {
                    for x in 0..map_width() {
                        let draw_x = x as f64 * tile_size;
                        let draw_y = y as f64 * tile_size;
                        if !game_state.discovered.contains(&(x, y)) {
                            ctx.set_fill_style_str(FOG_COLOR);
                            ctx.fill_rect(draw_x, draw_y, tile_size, tile_size);
                            ctx.set_fill_style_str("rgba(80, 80, 130, 0.8)");
                            ctx.set_font(&format!("{}px monospace", tile_size));
                            ctx.set_text_align("center");
                            ctx.set_text_baseline("middle");
                            ctx.fill_text(FOG_SYMBOL, draw_x + tile_size / 2.0, draw_y + tile_size / 2.0).ok();
                            continue;
                        }
                        // 优先级：玩家 > 实体 > 地形
                        if game_state.player_x == x && game_state.player_y == y {
                            ctx.set_fill_style_str(PLAYER_COLOR);
                            ctx.set_font(&format!("{}px monospace", tile_size));
                            ctx.set_text_align("center");
                            ctx.set_text_baseline("middle");
                            ctx.fill_text(
                                PLAYER_SYMBOL,
                                draw_x + tile_size / 2.0,
                                draw_y + tile_size / 2.0,
                            ).ok();
                            continue;
                        }
                        if let Some(entity) = game_state.entities.iter().find(|e| e.x == x && e.y == y && match e.entity_type { EntityType::Object { collected } => !collected, EntityType::Food { eaten: _, kind: _ } => true, EntityType::Chest { opened } => true, _ => true }) {
                            // 食物和宝箱根据状态显示不同符号
                            let (symbol, color) = match &entity.entity_type {
                                EntityType::Food { eaten, kind } => {
                                    (kind.symbol(*eaten), kind.color(*eaten))
                                },
                                EntityType::Chest { opened } => {
                                    if *opened {
                                        ('▣', "#cccc99")
                        } else {
                                        ('□', "#ffcc66")
                                    }
                                },
                                _ => (entity.symbol, entity.color)
                            };
                            ctx.set_fill_style_str(color);
                            ctx.set_font(&format!("{}px monospace", tile_size));
                            ctx.set_text_align("center");
                            ctx.set_text_baseline("middle");
                            ctx.fill_text(&symbol.to_string(), draw_x + tile_size / 2.0, draw_y + tile_size / 2.0).ok();
                            continue;
                        }
                        // 地形
                        let terrain = game_state.map[y][x];
                        ctx.set_fill_style_str("#0a0a15");
                        ctx.fill_rect(draw_x, draw_y, tile_size, tile_size);
                        ctx.set_fill_style_str(terrain.color());
                        ctx.set_font(&format!("{}px monospace", tile_size));
                                ctx.set_text_align("center");
                                ctx.set_text_baseline("middle");
                        ctx.fill_text(terrain.symbol(), draw_x + tile_size / 2.0, draw_y + tile_size / 2.0).ok();
                        ctx.set_fill_style_str(DISCOVERED_COLOR);
                        ctx.set_font(&format!("{}px monospace", tile_size));
                        ctx.fill_text(DISCOVERED_SYMBOL, draw_x + tile_size / 2.0, draw_y + tile_size / 2.0).ok();
                    }
                }
            }
            ()
        });
    }

    // 方向键和按钮事件
    {
        let state = state.clone();
        let redraw = redraw.clone();
        use_effect(move || {
            let mut state = state.clone();
            let mut redraw = redraw.clone();
            let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
                let mut s = state();
                match event.key().as_str() {
                    "ArrowUp" => s.move_player(0, -1),
                    "ArrowDown" => s.move_player(0, 1),
                    "ArrowLeft" => s.move_player(-1, 0),
                    "ArrowRight" => s.move_player(1, 0),
                    "r" | "R" => {
                        state.set(GameState::new(map_width(), map_height()));
                        let val = *redraw.read();
                        redraw.set(val + 1);
                        return;
                    },
                    _ => return,
                }
                state.set(s);
                let val = *redraw.read();
                redraw.set(val + 1);
            }) as Box<dyn FnMut(_)>);
            let window = match window() {
                Some(w) => w,
                None => return,
            };
            if let Err(_) = window.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref()) {
                return;
            }
            closure.forget();
            ()
        });
    }

    // 移动玩家函数
    let mut move_player = {
        let mut state = state.clone();
        let mut redraw = redraw.clone();
        move |dx: isize, dy: isize| {
            let mut s = state();
            s.move_player(dx, dy);
                state.set(s);
                let val = *redraw.read();
                redraw.set(val + 1);
        }
    };

    // 重新开始函数
    let mut restart = {
        let mut state = state.clone();
        let map_width = map_width.clone();
        let map_height = map_height.clone();
        move || {
            state.set(GameState::new(map_width(), map_height()));
        }
    };

    // 切换面板显示
    let mut toggle_panel = {
        let mut panel_visible = panel_visible.clone();
        move || {
            panel_visible.set(!panel_visible());
        }
    };

    // 监听面板显示状态变化，重新计算地图尺寸
    {
        let mut canvas_width = canvas_width.clone();
        let mut canvas_height = canvas_height.clone();
        let mut map_width = map_width.clone();
        let mut map_height = map_height.clone();
        let mut state = state.clone();
        let panel_visible = panel_visible.clone();
        use_effect(move || {
            let _ = panel_visible();
            // 延迟一点时间让DOM更新完成
            let window = match window() {
                Some(w) => w,
                None => return,
            };
            let closure = Closure::wrap(Box::new(move || {
                let arr = get_elem_size(canvas_container_id);
                let w = arr.get(0).as_f64().unwrap_or(600.0) as u32;
                let h = arr.get(1).as_f64().unwrap_or(500.0) as u32;
                if w > 0 && h > 0 {
                    canvas_width.set(w);
                    canvas_height.set(h);
                    let mw = (w / DESIRED_TILE_SIZE).max(5) as usize;
                    let mh = (h / DESIRED_TILE_SIZE).max(5) as usize;
                    map_width.set(mw);
                    map_height.set(mh);
                    state.set(GameState::new(mw, mh));
                }
            }) as Box<dyn FnMut()>);
            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().unchecked_ref(),
                100
            );
            closure.forget();
            ()
        });
    }

    // 弹窗自动消失
    {
        let mut state = state.clone();
        use_effect(move || {
            if state().message.is_some() {
                let window = match window() { Some(w) => w, None => return };
                let closure = Closure::wrap(Box::new(move || {
                    let mut s = state();
                    s.message = None;
                    state.set(s);
                }) as Box<dyn FnMut()>);
                let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                    closure.as_ref().unchecked_ref(),
                    1500
                );
                closure.forget();
            }
            ()
        });
    }

    // UI 数据
    let progress = {
        let state = state.clone();
        move || format!("{}%", state().get_progress())
    };
    let items = {
        let state = state.clone();
        move || state().items_collected.to_string()
    };
    let stamina = {
        let state = state.clone();
        move || state().stamina.to_string()
    };
    let score = {
        let state = state.clone();
        move || state().score.to_string()
    };
    let message = {
        let state = state.clone();
        move || state().message.clone()
    };

    let message_node = {
        if let Some(msg) = message() {
    rsx! {
                div { class: "fixed left-1/2 top-10 -translate-x-1/2 bg-[#222244] text-[#ffcc00] px-6 py-3 rounded shadow-lg z-50 border border-[#ffcc00] text-lg animate-bounce", {msg} }
            }
        } else {
            rsx! {}
        }
    };

    rsx! {
        div { class: "flex flex-col md:flex-row h-screen overflow-hidden bg-gradient-to-br from-[#1a1a2e] to-[#16213e]",
            // 左侧flex-1容器，canvas自适应父容器
            div { id: canvas_container_id, class: "flex-1 h-full relative",
                    canvas {
                        id: "gameCanvas",
                    width: canvas_width(),
                    height: canvas_height(),
                    class: "block w-full h-full bg-[#0a0a15]",
                    style: "width: 100%; height: 100%;",
                }
                // 移动端控制按钮 - 仅在移动端显示
                div { class: "md:hidden absolute bottom-4 left-1/2 transform -translate-x-1/2 flex flex-col items-center gap-2",
                    // 上按钮
                    button { 
                        class: "w-12 h-12 bg-gradient-to-b from-[#33335f] to-[#1a1a3a] border border-[#55558f] rounded-full text-2xl shadow-lg cursor-pointer select-none flex items-center justify-center text-white", 
                        onclick: move |_| move_player(0, -1), 
                        "↑" 
                    }
                    // 中间行按钮
                    div { class: "flex gap-2",
                        button { 
                            class: "w-12 h-12 bg-gradient-to-b from-[#33335f] to-[#1a1a3a] border border-[#55558f] rounded-full text-2xl shadow-lg cursor-pointer select-none flex items-center justify-center text-white", 
                            onclick: move |_| move_player(-1, 0), 
                            "←" 
                        }
                        button { 
                            class: "w-12 h-12 bg-gradient-to-b from-[#33335f] to-[#1a1a3a] border border-[#55558f] rounded-full text-2xl shadow-lg cursor-pointer select-none flex items-center justify-center text-white", 
                            onclick: move |_| move_player(0, 1), 
                            "↓" 
                        }
                        button { 
                            class: "w-12 h-12 bg-gradient-to-b from-[#33335f] to-[#1a1a3a] border border-[#55558f] rounded-full text-2xl shadow-lg cursor-pointer select-none flex items-center justify-center text-white", 
                            onclick: move |_| move_player(1, 0), 
                            "→" 
                        }
                    }
                }
                // 移动端面板切换按钮（已移除）
            }
            // 右侧状态栏 - 仅桌面端显示
            div {
                class: "ui-panel hidden md:flex md:w-[300px] md:h-full md:bg-[#141428cc] md:border-l-2 border-[#44447f] shadow-xl z-10 flex-col p-3 md:p-5 rounded-lg md:rounded-none transition-all duration-300 ease-in-out",
                div { class: "panel-title text-[#ff9900] text-lg md:text-[22px] mb-3 md:mb-4 text-center border-b border-[#44447f] pb-2", "游戏状态" }
                div { class: "stats mb-4 md:mb-6 text-white text-sm md:text-base",
                    div { class: "stat-item flex justify-between py-1 md:py-2 border-b border-dashed border-[#33335f]", span {"探索进度:"} span { id: "progress", {progress()} } }
                    div { class: "stat-item flex justify-between py-1 md:py-2 border-b border-dashed border-[#33335f]", span {"已发现物品:"} span { id: "items", {items()} } }
                    div { class: "stat-item flex justify-between py-1 md:py-2 border-b border-dashed border-[#33335f]", span {"体力:"} span { id: "stamina", {stamina()} } }
                    div { class: "stat-item flex justify-between py-1 md:py-2 border-b border-dashed border-[#33335f]", span {"分数:"} span { id: "score", {score()} } }
                    div { class: "stat-item flex justify-between py-1 md:py-2 border-b border-dashed border-[#33335f]", span {"游戏时间:"} span { id: "time", {game_time()} } }
                }
                div { class: "controls mb-4 md:mb-6 hidden md:block",
                    p { class: "mb-2 text-[#a0a0c0]", "移动控制:" }
                    div { class: "key-row flex justify-center mb-2", button { class: "key w-[50px] h-[50px] flex items-center justify-center bg-gradient-to-b from-[#33335f] to-[#1a1a3a] border border-[#55558f] rounded-[6px] text-2xl shadow cursor-pointer select-none mr-1", onclick: move |_| move_player(0, -1), id: "up", "↑" } }
                    div { class: "key-row flex justify-center gap-2", 
                        button { class: "key w-[50px] h-[50px] flex items-center justify-center bg-gradient-to-b from-[#33335f] to-[#1a1a3a] border border-[#55558f] rounded-[6px] text-2xl shadow cursor-pointer select-none", onclick: move |_| move_player(-1, 0), id: "left", "←" }
                        button { class: "key w-[50px] h-[50px] flex items-center justify-center bg-gradient-to-b from-[#33335f] to-[#1a1a3a] border border-[#55558f] rounded-[6px] text-2xl shadow cursor-pointer select-none", onclick: move |_| move_player(0, 1), id: "down", "↓" }
                        button { class: "key w-[50px] h-[50px] flex items-center justify-center bg-gradient-to-b from-[#33335f] to-[#1a1a3a] border border-[#55558f] rounded-[6px] text-2xl shadow cursor-pointer select-none", onclick: move |_| move_player(1, 0), id: "right", "→" }
                    }
                }
                div { class: "flex-1 legend mt-2 md:mt-4 text-white overflow-y-auto text-xs md:text-sm",
                    p { class: "mb-2", "符号图例:" }
                    div { class: "legend-item flex items-center mb-1 md:mb-2 p-1 rounded bg-[#1e1e32]/50", div { class: "symbol player-symbol w-[20px] h-[20px] md:w-[30px] md:h-[30px] flex items-center justify-center text-sm md:text-xl mr-2 bg-black/30 rounded text-[#ffcc00]", "@" } span { "玩家" } }
                    div { class: "legend-item flex items-center mb-1 md:mb-2 p-1 rounded bg-[#1e1e32]/50", div { class: "symbol animal-symbol w-[20px] h-[20px] md:w-[30px] md:h-[30px] flex items-center justify-center text-sm md:text-xl mr-2 bg-black/30 rounded text-[#ff6666]", "$" } span { "动物" } }
                    div { class: "legend-item flex items-center mb-1 md:mb-2 p-1 rounded bg-[#1e1e32]/50", div { class: "symbol plant-symbol w-[20px] h-[20px] md:w-[30px] md:h-[30px] flex items-center justify-center text-sm md:text-xl mr-2 bg-black/30 rounded text-[#66cc66]", "%" } span { "植物" } }
                    div { class: "legend-item flex items-center mb-1 md:mb-2 p-1 rounded bg-[#1e1e32]/50", div { class: "symbol object-symbol w-[20px] h-[20px] md:w-[30px] md:h-[30px] flex items-center justify-center text-sm md:text-xl mr-2 bg-black/30 rounded text-[#6699ff]", "!" } span { "物体" } }
                    div { class: "legend-item flex items-center mb-1 md:mb-2 p-1 rounded bg-[#1e1e32]/50", div { class: "symbol food-symbol w-[20px] h-[20px] md:w-[30px] md:h-[30px] flex items-center justify-center text-sm md:text-xl mr-2 bg-black/30 rounded text-[#ff3333]", "🍎" } span { "苹果（+5体力）" } }
                    div { class: "legend-item flex items-center mb-1 md:mb-2 p-1 rounded bg-[#1e1e32]/50", div { class: "symbol food-symbol w-[20px] h-[20px] md:w-[30px] md:h-[30px] flex items-center justify-center text-sm md:text-xl mr-2 bg-black/30 rounded text-[#ffe066]", "🍌" } span { "香蕉（+8体力）" } }
                    div { class: "legend-item flex items-center mb-1 md:mb-2 p-1 rounded bg-[#1e1e32]/50", div { class: "symbol food-symbol w-[20px] h-[20px] md:w-[30px] md:h-[30px] flex items-center justify-center text-sm md:text-xl mr-2 bg-black/30 rounded text-[#e0b97d]", "🍞" } span { "面包（+10体力）" } }
                    div { class: "legend-item flex items-center mb-1 md:mb-2 p-1 rounded bg-[#1e1e32]/50", div { class: "symbol food-symbol w-[20px] h-[20px] md:w-[30px] md:h-[30px] flex items-center justify-center text-sm md:text-xl mr-2 bg-black/30 rounded text-[#ffbb66]", "🍗" } span { "鸡腿（+15体力）" } }
                    div { class: "legend-item flex items-center mb-1 md:mb-2 p-1 rounded bg-[#1e1e32]/50", div { class: "symbol food-symbol w-[20px] h-[20px] md:w-[30px] md:h-[30px] flex items-center justify-center text-sm md:text-xl mr-2 bg-black/30 rounded text-[#ffb3e6]", "🍰" } span { "蛋糕（+20体力）" } }
                    div { class: "legend-item flex items-center mb-1 md:mb-2 p-1 rounded bg-[#1e1e32]/50", div { class: "symbol chest-symbol w-[20px] h-[20px] md:w-[30px] md:h-[30px] flex items-center justify-center text-sm md:text-xl mr-2 bg-black/30 rounded text-[#ffcc66]", "□" } span { "宝箱（获得分数）" } }
                    div { class: "legend-item flex items-center mb-1 md:mb-2 p-1 rounded bg-[#1e1e32]/50", div { class: "symbol tree-symbol w-[20px] h-[20px] md:w-[30px] md:h-[30px] flex items-center justify-center text-sm md:text-xl mr-2 bg-black/30 rounded text-[#336633]", "↑" } span { "树" } }
                    div { class: "legend-item flex items-center mb-1 md:mb-2 p-1 rounded bg-[#1e1e32]/50", div { class: "symbol river-symbol w-[20px] h-[20px] md:w-[30px] md:h-[30px] flex items-center justify-center text-sm md:text-xl mr-2 bg-black/30 rounded text-[#3366cc]", "≈" } span { "河流" } }
                    div { class: "legend-item flex items-center mb-1 md:mb-2 p-1 rounded bg-[#1e1e32]/50", div { class: "symbol mountain-symbol w-[20px] h-[20px] md:w-[30px] md:h-[30px] flex items-center justify-center text-sm md:text-xl mr-2 bg-black/30 rounded text-[#888888]", "^" } span { "山" } }
                    div { class: "legend-item flex items-center mb-1 md:mb-2 p-1 rounded bg-[#1e1e32]/50", div { class: "symbol discovered w-[20px] h-[20px] md:w-[30px] md:h-[30px] flex items-center justify-center text-sm md:text-xl mr-2 bg-black/30 rounded text-[#cc99ff]", "◌" } span { "已探索区域" } }
                    div { class: "legend-item flex items-center mb-1 md:mb-2 p-1 rounded bg-[#1e1e32]/50", div { class: "symbol undiscovered w-[20px] h-[20px] md:w-[30px] md:h-[30px] flex items-center justify-center text-sm md:text-xl mr-2 bg-black/30 rounded text-[#666699]", "░" } span { "未探索区域" } }
                }
                div { class: "message bg-black/60 border-l-4 border-[#ffcc00] p-2 md:p-3 mt-2 md:mt-4 text-xs md:text-sm rounded-r-lg text-gray-100", "探索迷雾区域可以发现各种符号物体！收集它们以完成你的冒险。" }
                button { class: "action-btn mt-3 md:mt-4 w-full bg-gradient-to-b from-[#4d4d99] to-[#333366] text-white py-2 rounded shadow hover:from-[#5d5da9] hover:to-[#434376] transition text-sm md:text-base", onclick: move |_| restart(), id: "restartBtn", "重新开始游戏" }
                {message_node}
            }
        }
    }
} 