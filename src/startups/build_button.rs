use bevy::color::Color;
use bevy::ecs::children;
use bevy::ecs::spawn::SpawnRelated;
use bevy::ecs::system::Commands;
use bevy::text::{TextColor, TextFont};
use bevy::ui::widget::Text;
use bevy::ui::{percent, px, AlignItems, BackgroundColor, BorderColor, JustifyContent, Node, UiRect};
use bevy::utils::default;

pub fn spawn_build_button(mut commands: Commands) {
    let container = Node {
        width: percent(100.0),
        height: percent(100.0),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..default()
    };

    let button = (
        BorderColor::all(Color::BLACK),
        BackgroundColor(Color::srgb(0.12, 0.12, 0.12)),
        Node {
            width: px(150.0),
            height: px(65.0),
            border: UiRect::all(px(5.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        children![(
            Text::new("Build"),
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            TextFont::default().with_font_size(40.0),
        )],
    );

    let button_entity = commands.spawn(button).id();

    commands.spawn(container).add_children(&[button_entity]);
}

