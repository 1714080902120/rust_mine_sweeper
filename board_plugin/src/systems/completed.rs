// mark.rs
use crate::{events::{BoardCompletedEvent}, resources::BoardAssets, Board};
use bevy::prelude::*;

pub fn completed(
    mut commands: Commands,
    mut board: ResMut<Board>,
    mut completed_rdr: EventReader<BoardCompletedEvent>,
    board_assets: Res<BoardAssets>,
) {
    for _ in completed_rdr.iter() {
        
        board.as_mut().set_need_stop_state(true);

        let middle_pos = board.get_middle_pos();
        let board_entry = board.as_ref().entity;
        let board_size = board.get_size();
        let text_bundle = Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: "YOU WIN!!".to_string(),
                    style: TextStyle {
                        color: board_assets.fail_material.color,
                        font: board_assets.bomb_counter_font.clone(),
                        font_size: board_assets.game_over_font_size,
                    },
                }],
                alignment: TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            },
            transform: Transform::from_xyz(middle_pos.x, middle_pos.y, 11.),
            ..Default::default()
        };

        commands.entity(board_entry).with_children(|parent| {
            parent
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color: board_assets.fail_cover_board_material.color,
                        custom_size: Some(board_size),
                        ..Default::default()
                    },
                    texture: board_assets.board_material.texture.clone(),
                    ..Default::default()
                })
                .insert(Name::new("Cover_board")).insert(text_bundle);
        });
    }
}
