use bevy::color::Color;
use bevy::ecs::system::Commands;
use bevy::text::{Justify, TextColor, TextLayout};
use bevy::ui::widget::Text;
use bevy::ui::{px, Node, PositionType};
use bevy::utils::default;
use bevy::{ecs::system::Res, asset::AssetServer};

use crate::resources::Money;

pub fn spawn_money_display(mut commands: Commands, assets: Res<AssetServer>, money: Res<Money>) {
  commands.spawn((
    Node {
      position_type: PositionType::Absolute,
      top: px(8.0),
      left: px(8.0),
      ..default()
    },
    Text::new(format!("{}", *money)),
    TextColor(Color::WHITE),
    TextLayout::new_with_justify(Justify::Center),
  ));
}
