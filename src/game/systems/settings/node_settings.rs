use bevy::prelude::{default, AlignItems, JustifyContent, Node, PositionType, UiRect, Val};
use bevy::ui::FlexDirection;

#[derive(Default, Clone)]
#[allow(unused)]
pub struct NodeSettings {
    pub width: Val,
    pub height: Val,
    pub border: UiRect,
    pub align_items: AlignItems,
    pub justify_content: JustifyContent,
    pub position_type: PositionType,
    pub bottom: Val,
    pub right: Val,
    pub font_size: f32,
    pub flex_direction: FlexDirection,
    pub row_gap: Val,
    pub padding: UiRect,
}

#[derive(Default, Clone)]
#[allow(unused)]
pub struct NodeSettingsBuilder {
    width: Val,
    height: Val,
    border: Option<UiRect>,
    align_items: Option<AlignItems>,
    justify_content: Option<JustifyContent>,
    position_type: Option<PositionType>,
    bottom: Option<Val>,
    right: Option<Val>,
    font_size: Option<f32>,
    flex_direction: Option<FlexDirection>,
    row_gap: Option<Val>,
    padding: Option<UiRect>,
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

    pub fn bottom(mut self, bottom: Val) -> Self {
        self.bottom = Some(bottom);
        self
    }

    pub fn right(mut self, right: Val) -> Self {
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

    pub fn row_gap(mut self, row_gap: Val) -> Self {
        self.row_gap = Some(row_gap);
        self
    }

    pub fn flex_direction(mut self, flex_direction: FlexDirection) -> Self {
        self.flex_direction = Some(flex_direction);
        self
    }

    pub fn padding(mut self, padding: UiRect) -> Self {
        self.padding = Some(padding);
        self
    }

    pub fn build(mut self) -> NodeSettings {
        NodeSettings {
            width: self.width,
            height: self.height,
            border: *self.border.get_or_insert_with(|| UiRect::all(Val::Px(5.0))),
            justify_content: self.justify_content.unwrap_or_default(),
            align_items: self.align_items.unwrap_or_default(),
            position_type: self.position_type.unwrap_or_default(),
            bottom: self.bottom.unwrap_or_default(),
            right: self.right.unwrap_or_default(),
            font_size: self.font_size.unwrap_or_default(),
            flex_direction: self.flex_direction.unwrap_or_default(),
            row_gap: self.row_gap.unwrap_or_default(),
            padding: self.padding.unwrap_or_default(),
            ..default()
        }
    }
}

impl NodeSettings {
    pub fn builder(width: Val, height: Val) -> NodeSettingsBuilder {
        NodeSettingsBuilder {
            width,
            height,
            ..Default::default()
        }
    }

    pub fn node(&self) -> Node {
        Node {
            width: self.width,
            height: self.height,
            border: self.border,
            justify_content: self.justify_content,
            align_items: self.align_items,
            position_type: self.position_type,
            bottom: self.bottom,
            right: self.right,
            flex_direction: self.flex_direction,
            padding: self.padding,
            row_gap: self.row_gap,
            ..default()
        }
    }
}
