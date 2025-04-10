use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent};
use std::collections::HashMap;

// ECSの基本コンポーネント
#[derive(Debug, Clone)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

// エンティティID
type EntityId = u32;

// コンポーネントストレージ
struct ComponentStorage<T> {
    components: HashMap<EntityId, T>,
}

impl<T> ComponentStorage<T> {
    fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    fn insert(&mut self, entity: EntityId, component: T) {
        self.components.insert(entity, component);
    }

    fn get(&self, entity: EntityId) -> Option<&T> {
        self.components.get(&entity)
    }

    fn get_mut(&mut self, entity: EntityId) -> Option<&mut T> {
        self.components.get_mut(&entity)
    }
}

// ワールド（ECSのメインコンテナ）
pub struct World {
    next_entity_id: EntityId,
    positions: ComponentStorage<Position>,
    sizes: ComponentStorage<Size>,
    colors: ComponentStorage<Color>,
}

impl World {
    pub fn new() -> Self {
        Self {
            next_entity_id: 0,
            positions: ComponentStorage::new(),
            sizes: ComponentStorage::new(),
            colors: ComponentStorage::new(),
        }
    }

    pub fn create_entity(&mut self) -> EntityId {
        let id = self.next_entity_id;
        self.next_entity_id += 1;
        id
    }

    pub fn add_position(&mut self, entity: EntityId, position: Position) {
        self.positions.insert(entity, position);
    }

    pub fn add_size(&mut self, entity: EntityId, size: Size) {
        self.sizes.insert(entity, size);
    }

    pub fn add_color(&mut self, entity: EntityId, color: Color) {
        self.colors.insert(entity, color);
    }
}

// ゲームのメイン構造体
#[wasm_bindgen]
pub struct Game {
    world: World,
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement) -> Result<Game, JsValue> {
        let context = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;

        Ok(Game {
            world: World::new(),
            canvas,
            context,
        })
    }

    pub fn init(&mut self) {
        // マス目の作成
        let grid_size = 8;
        let cell_size = 50.0;

        for y in 0..grid_size {
            for x in 0..grid_size {
                let entity = self.world.create_entity();
                self.world.add_position(
                    entity,
                    Position {
                        x: x as f64 * cell_size,
                        y: y as f64 * cell_size,
                    },
                );
                self.world.add_size(
                    entity,
                    Size {
                        width: cell_size,
                        height: cell_size,
                    },
                );
                self.world.add_color(
                    entity,
                    Color {
                        r: 255,
                        g: 255,
                        b: 255,
                    },
                );
            }
        }
    }

    pub fn render(&self) {
        // キャンバスのクリア
        self.context.clear_rect(
            0.0,
            0.0,
            self.canvas.width() as f64,
            self.canvas.height() as f64,
        );

        // マス目の描画
        for entity in 0..self.world.next_entity_id {
            if let (Some(pos), Some(size), Some(color)) = (
                self.world.positions.get(entity),
                self.world.sizes.get(entity),
                self.world.colors.get(entity),
            ) {
                self.context.set_fill_style(&JsValue::from_str(&format!(
                    "rgb({},{},{})",
                    color.r, color.g, color.b
                )));
                self.context.fill_rect(pos.x, pos.y, size.width, size.height);
                self.context.set_stroke_style(&JsValue::from_str("black"));
                self.context.stroke_rect(pos.x, pos.y, size.width, size.height);
            }
        }
    }

    pub fn handle_click(&mut self, event: MouseEvent) {
        let rect = self.canvas.get_bounding_client_rect();
        let x = event.client_x() as f64 - rect.left();
        let y = event.client_y() as f64 - rect.top();

        // クリックされたマス目を探す
        for entity in 0..self.world.next_entity_id {
            if let (Some(pos), Some(size)) = (
                self.world.positions.get(entity),
                self.world.sizes.get(entity),
            ) {
                if x >= pos.x
                    && x <= pos.x + size.width
                    && y >= pos.y
                    && y <= pos.y + size.height
                {
                    // クリックされたマス目の色を変更
                    if let Some(color) = self.world.colors.get_mut(entity) {
                        color.r = 255 - color.r;
                        color.g = 255 - color.g;
                        color.b = 255 - color.b;
                    }
                }
            }
        }
    }
} 