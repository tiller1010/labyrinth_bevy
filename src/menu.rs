use bevy::prelude::*;

use crate::game_state::{
    GameState,
};

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MenuState {
    #[default]
    Main,
}

#[derive(Component)]
pub struct OnMainMenuScreen;

#[derive(Component)]
pub enum MenuButtonAction {
    Play,
    Quit,
}

pub fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main)
}

pub fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>), >,
    mut app_exit_events: EventWriter<AppExit>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => {
                    app_exit_events.send(AppExit::Success);
                }
                MenuButtonAction::Play => {
                    game_state.set(GameState::Game);
                }
            }
        }
    }
}

pub fn main_menu_setup(mut commands: Commands) {
    let button_style = Style {
        width: Val::Px(250.),
        height: Val::Px(65.),
        margin: UiRect::all(Val::Px(20.)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_style = TextStyle {
        font_size: 40.,
        ..default()
    };

    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        },
        OnMainMenuScreen,
    )).with_children(|parent| {

        parent.spawn((
            ButtonBundle {
                style: button_style.clone(),
                ..default()
            },
            MenuButtonAction::Play
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Play",
                button_text_style.clone(),
            ));
        });

        parent.spawn((
            ButtonBundle {
                style: button_style.clone(),
                ..default()
            },
            MenuButtonAction::Quit
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Quit",
                button_text_style.clone(),
            ));
        });

    });
}

