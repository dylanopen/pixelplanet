use bevy::color::Color;
use bevy::ecs::children;
use bevy::ecs::spawn::SpawnRelated;
use bevy::ecs::system::Commands;
use bevy::text::{TextColor, TextFont};
use bevy::ui::widget::Text;
use bevy::ui::{
    AlignItems, BackgroundColor, BorderColor, JustifyContent, Node, PositionType, UiRect, px,
};
use bevy::utils::default;

use crate::components::BuildContainer;
use crate::components::build_button::BuildButton;

pub fn spawn_build_button(mut commands: Commands) {
    let button = (
        BuildContainer,
        BorderColor::all(Color::BLACK),
        BackgroundColor(Color::srgb(0.12, 0.12, 0.12)),
        Node {
            left: px(0.0),
            bottom: px(0.0),
            border: UiRect::all(px(2.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            position_type: PositionType::Absolute,
            ..default()
        },
        children![(
            BuildButton,
            Text::new(" Build "),
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            TextFont::default().with_font_size(24.0),
        )],
    );

    let _ = commands.spawn(button).id();
}
