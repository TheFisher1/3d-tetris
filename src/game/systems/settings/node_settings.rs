use bevy::prelude::{default, AlignItems, JustifyContent, Node, PositionType, UiRect, Val};

#[derive(Default)]
pub struct NodeSettings {
    pub width: f32,
    pub height: f32,
    pub border: UiRect,
    pub align_items: AlignItems,
    pub justify_content: JustifyContent,
    pub position_type: PositionType,
    pub bottom: f32,
    pub right: f32,
}

impl NodeSettings {
    pub fn new(width: f32, height: f32) -> Self {
        NodeSettings {
            width,
            height,
            ..Default::default()
        }
    }

    pub fn border(mut self, border: UiRect) -> Self {
        self.border = border;
        self
    }

    pub fn align_items(mut self, align_items: AlignItems) -> Self {
        self.align_items = align_items;
        self
    }

    pub fn justify_content(mut self, justify_content: JustifyContent) -> Self {
        self.justify_content = justify_content;
        self
    }

    pub fn position_type(mut self, position_type: PositionType) -> Self {
        self.position_type = position_type;
        self
    }

    pub fn bottom(mut self, bottom: f32) -> Self {
        self.bottom = bottom;
        self
    }

    pub fn right(mut self, right: f32) -> Self {
        self.right = right;
        self
    }

    pub fn build(self) -> Node {
        Node {
            width: Val::Px(self.width),
            height: Val::Px(self.height),
            border: UiRect::all(Val::Px(5.0)),
            justify_content: self.justify_content,
            align_items: self.align_items,
            position_type: PositionType::Absolute,
            bottom: Val::Px(self.bottom),
            right: Val::Px(self.right),
            ..default()
        }
    }
}
