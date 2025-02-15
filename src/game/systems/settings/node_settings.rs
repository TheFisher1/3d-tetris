use bevy::prelude::{default, AlignItems, JustifyContent, Node, PositionType, UiRect, Val};

#[derive(Default)]
#[allow(unused)]
pub struct NodeSettings {
    pub width: f32,
    pub height: f32,
    pub border: UiRect,
    pub align_items: AlignItems,
    pub justify_content: JustifyContent,
    pub position_type: PositionType,
    pub bottom: f32,
    pub right: f32,
    pub font_size: f32,
}

#[derive(Default)]
#[allow(unused)]
pub struct NodeSettingsBuilder {
    width: f32,
    height: f32,
    border: Option<UiRect>,
    align_items: Option<AlignItems>,
    justify_content: Option<JustifyContent>,
    position_type: Option<PositionType>,
    bottom: Option<f32>,
    right: Option<f32>,
    font_size: Option<f32>,
}

#[allow(unused)]
impl NodeSettingsBuilder {
    pub fn border(mut self, border: UiRect) -> Self {
        self.border = Some(border);
        self
    }

    pub fn align_items(mut self, align_items: AlignItems) -> Self {
        self.align_items = Some(align_items);
        self
    }

    pub fn justify_content(mut self, justify_content: JustifyContent) -> Self {
        self.justify_content = Some(justify_content);
        self
    }

    pub fn position_type(mut self, position_type: PositionType) -> Self {
        self.position_type = Some(position_type);
        self
    }

    pub fn bottom(mut self, bottom: f32) -> Self {
        self.bottom = Some(bottom);
        self
    }

    pub fn right(mut self, right: f32) -> Self {
        self.right = Some(right);
        self
    }

    pub fn font_size(mut self, font_size: f32) -> Self {
        if font_size > 0.0 {
            self.font_size = Some(font_size);
            self
        } else {
            self
        }
    }

    pub fn build(mut self) -> NodeSettings {
        NodeSettings {
            width: self.width,
            height: self.height,
            border: *self.border.get_or_insert_with(|| UiRect::all(Val::Px(5.0))),
            justify_content: *self.justify_content.get_or_insert_default(),
            align_items: *self.align_items.get_or_insert_default(),
            position_type: *self.position_type.get_or_insert_default(),
            bottom: *self.bottom.get_or_insert_default(),
            right: *self.right.get_or_insert_default(),
            ..default()
        }
    }
}

impl NodeSettings {
    pub fn builder(width: f32, height: f32) -> NodeSettingsBuilder {
        NodeSettingsBuilder {
            width,
            height,
            ..Default::default()
        }
    }

    pub fn node(&self) -> Node {
        Node {
            width: Val::Px(self.width),
            height: Val::Px(self.height),
            border: self.border,
            justify_content: self.justify_content,
            align_items: self.align_items,
            position_type: self.position_type,
            bottom: Val::Px(self.bottom),
            right: Val::Px(self.right),
            ..default()
        }
    }
}
