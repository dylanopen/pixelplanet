use bevy::ecs::message::MessageReader;
use bevy::ecs::query::With;
use bevy::ecs::system::Single;
use bevy::ui::widget::Text;

use crate::components::MoneyDisplay;
use crate::messages::SetMoneyMessage;

pub fn update_money_display(
    mut set_money_mr: MessageReader<SetMoneyMessage>,
    mut money_display_text: Single<&mut Text, With<MoneyDisplay>>,
) {
    for msg in set_money_mr.read() {
        **money_display_text = Text::new(format!("${:.2}", msg.new));
    }
}
