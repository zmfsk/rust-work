use bevy::prelude::*;
use crate::game::{GameState, Stone, StoneComponent};

// 应用状态枚举
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
}

// 开始按钮组件
#[derive(Component)]
pub struct StartButton;

// 主菜单相机标记组件
#[derive(Component)]
pub struct MainMenuCamera;

// 添加新的组件
#[derive(Component)]
pub struct UsageButton;

#[derive(Component)]
pub struct UsageWindow;

#[derive(Component)]
pub struct CloseButton;

// 设置主菜单
pub fn setup_main_menu(mut commands: Commands, windows: Query<&Window>) {
    // 获取窗口大小
    let window = windows.single();
    let window_width = window.width();

    // 添加主菜单专用相机
    commands.spawn((Camera2dBundle::default(), MainMenuCamera));

    // 添加标题
    commands.spawn(TextBundle {
        text: Text::from_section(
            "Gobang Game",
            TextStyle {
                font_size: 60.0,
                color: Color::rgb(0.2, 0.2, 0.2),
                ..default()
            },
        )
        .with_alignment(TextAlignment::Center),
        style: Style {
            position_type: PositionType::Absolute,
            left: Val::Px(window_width / 2.0 - 150.0),
            top: Val::Px(200.0),
            ..default()
        },
        ..default()
    });

    // 添加开始按钮
    commands
        .spawn((
            ButtonBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(window_width / 2.0 - 100.0),
                    top: Val::Px(300.0),
                    width: Val::Px(200.0),
                    height: Val::Px(65.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                ..default()
            },
            StartButton,
        ))
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "Start Game",
                    TextStyle {
                        font_size: 30.0,
                        color: Color::WHITE,
                        ..default()
                    },
                )
                .with_text_alignment(TextAlignment::Center),
            );
        });

    // 添加Usage按钮
    commands
        .spawn((
            ButtonBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(window_width / 2.0 - 100.0),
                    top: Val::Px(380.0),
                    width: Val::Px(200.0),
                    height: Val::Px(65.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                ..default()
            },
            UsageButton,
        ))
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "How to Play",
                    TextStyle {
                        font_size: 30.0,
                        color: Color::WHITE,
                        ..default()
                    },
                )
                .with_text_alignment(TextAlignment::Center),
            );
        });
}

// 处理开始按钮点击
pub fn handle_start_button(
    mut next_state: ResMut<NextState<AppState>>,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, &mut Style),
        (Changed<Interaction>, With<StartButton>),
    >,
) {
    for (interaction, mut bg_color, mut style) in &mut button_query {
        match *interaction {
            Interaction::Pressed => {
                // 按下状态 - 颜色变深，尺寸略小
                *bg_color = Color::rgb(0.1, 0.1, 0.1).into();
                style.padding = UiRect::all(Val::Px(2.0));
                next_state.set(AppState::InGame);
            }
            Interaction::Hovered => {
                // 悬停状态 - 颜色变亮
                *bg_color = Color::rgb(0.25, 0.25, 0.25).into();
                style.padding = UiRect::all(Val::Px(0.0));
            }
            Interaction::None => {
                // 普通状态 - 恢复默认颜色
                *bg_color = Color::rgb(0.15, 0.15, 0.15).into();
                style.padding = UiRect::all(Val::Px(0.0));
            }
        }
    }
}

// 清理主菜单
pub fn cleanup_main_menu(world: &mut World) {
    // 使用世界直接查询和删除实体

    // 首先收集所有需要删除的实体
    let mut entities_to_despawn = Vec::new();

    // 收集按钮实体
    let button_entities = world
        .query_filtered::<Entity, Or<(With<StartButton>, With<UsageButton>, With<CloseButton>)>>()
        .iter(world)
        .collect::<Vec<_>>();
    entities_to_despawn.extend(button_entities);

    // 收集窗口实体
    let window_entities = world
        .query_filtered::<Entity, With<UsageWindow>>()
        .iter(world)
        .collect::<Vec<_>>();
    entities_to_despawn.extend(window_entities);

    // 收集相机实体
    let camera_entities = world
        .query_filtered::<Entity, With<MainMenuCamera>>()
        .iter(world)
        .collect::<Vec<_>>();
    entities_to_despawn.extend(camera_entities);

    // 收集文本实体（排除已收集的实体）
    let text_entities = world
        .query_filtered::<Entity, With<Text>>()
        .iter(world)
        .filter(|e| !entities_to_despawn.contains(e))
        .collect::<Vec<_>>();
    entities_to_despawn.extend(text_entities);

    // 收集节点实体（排除已收集的实体）
    let node_entities = world
        .query_filtered::<Entity, With<Node>>()
        .iter(world)
        .filter(|e| !entities_to_despawn.contains(e))
        .collect::<Vec<_>>();
    entities_to_despawn.extend(node_entities);

    // 安全地删除所有收集到的实体
    for entity in entities_to_despawn {
        // 检查实体是否仍然存在
        if world.get_entity(entity).is_some() {
            world.despawn(entity);
        }
    }
}

// 添加处理Usage按钮点击的系统
pub fn handle_usage_button(
    mut commands: Commands,
    windows: Query<&Window>,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (
            Changed<Interaction>,
            With<UsageButton>,
            Without<CloseButton>,
        ),
    >,
    usage_window_query: Query<Entity, With<UsageWindow>>,
) {
    // 如果已经有窗口打开，不再创建新窗口
    if !usage_window_query.is_empty() {
        return;
    }

    let window = windows.single();
    let window_width = window.width();

    for (interaction, mut bg_color) in &mut button_query {
        match *interaction {
            Interaction::Pressed => {
                // 按下状态 - 颜色变深
                *bg_color = Color::rgb(0.1, 0.1, 0.1).into();

                // 创建说明窗口
                commands
                    .spawn((
                        NodeBundle {
                            style: Style {
                                position_type: PositionType::Absolute,
                                left: Val::Px(window_width / 2.0 - 250.0),
                                top: Val::Px(150.0),
                                width: Val::Px(500.0),
                                height: Val::Px(400.0),
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                padding: UiRect::all(Val::Px(20.0)),
                                ..default()
                            },
                            background_color: Color::rgb(0.9, 0.9, 0.9).into(),
                            ..default()
                        },
                        UsageWindow,
                    ))
                    .with_children(|parent| {
                        // 添加标题
                        parent.spawn(
                            TextBundle::from_section(
                                "Gobang (Five in a Row) Rules",
                                TextStyle {
                                    font_size: 24.0,
                                    color: Color::rgb(0.2, 0.2, 0.2),
                                    ..default()
                                },
                            )
                            .with_style(Style {
                                margin: UiRect::bottom(Val::Px(20.0)),
                                ..default()
                            })
                            .with_text_alignment(TextAlignment::Center),
                        );

                        // 添加说明文本
                        parent.spawn(
                            TextBundle::from_section(
                                "Gobang is a traditional board game played on a 15x15 grid.\n\n\
                            Rules:\n\
                            1. Black plays first, followed by White.\n\
                            2. Players take turns placing stones on intersections.\n\
                            3. The first player to form an unbroken line of five stones horizontally, vertically, or diagonally wins.\n\
                            4. In this version, you play against an AI opponent.\n\
                            5. You can switch between playing as Black or White.\n\
                            6. Use the Reset button to start a new game.",
                                TextStyle {
                                    font_size: 18.0,
                                    color: Color::rgb(0.2, 0.2, 0.2),
                                    ..default()
                                },
                            )
                            .with_style(Style {
                                margin: UiRect::bottom(Val::Px(20.0)),
                                ..default()
                            }),
                        );

                        // 添加关闭按钮
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: Style {
                                        width: Val::Px(100.0),
                                        height: Val::Px(40.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                                    ..default()
                                },
                                CloseButton,
                            ))
                            .with_children(|parent| {
                                parent.spawn(
                                    TextBundle::from_section(
                                        "Close",
                                        TextStyle {
                                            font_size: 20.0,
                                            color: Color::WHITE,
                                            ..default()
                                        },
                                    )
                                    .with_text_alignment(TextAlignment::Center),
                                );
                            });
                    });
            }
            Interaction::Hovered => {
                // 悬停状态 - 颜色变亮
                *bg_color = Color::rgb(0.25, 0.25, 0.25).into();
            }
            Interaction::None => {
                // 普通状态 - 恢复默认颜色
                *bg_color = Color::rgb(0.15, 0.15, 0.15).into();
            }
        }
    }
}

// 处理关闭按钮点击
pub fn handle_close_button(
    mut commands: Commands,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<CloseButton>),
    >,
    usage_window_query: Query<Entity, With<UsageWindow>>,
) {
    for (interaction, mut bg_color) in &mut button_query {
        match *interaction {
            Interaction::Pressed => {
                // 按下状态 - 颜色变深
                *bg_color = Color::rgb(0.1, 0.1, 0.1).into();

                // 关闭窗口
                for entity in usage_window_query.iter() {
                    commands.entity(entity).despawn_recursive();
                }
            }
            Interaction::Hovered => {
                // 悬停状态 - 颜色变亮
                *bg_color = Color::rgb(0.25, 0.25, 0.25).into();
            }
            Interaction::None => {
                // 普通状态 - 恢复默认颜色
                *bg_color = Color::rgb(0.15, 0.15, 0.15).into();
            }
        }
    }
}

// 添加胜利窗口相关组件
#[derive(Component)]
pub struct VictoryWindow;

#[derive(Component)]
pub struct VictoryCloseButton;

#[derive(Component)]
pub struct PlayAgainButton;

// 显示胜利窗口
pub fn show_victory_window(
    mut commands: Commands,
    windows: Query<&Window>,
    game_state: Res<GameState>,
    victory_window_query: Query<Entity, With<VictoryWindow>>,
) {
    // 如果游戏未结束或已经有窗口，则不创建
    if !game_state.is_game_over || !victory_window_query.is_empty() {
        return;
    }

    let window = windows.single();
    let window_width = window.width();

    // 获取胜利者信息
    let victory_text = match game_state.winner {
        Some(Stone::Black) => "Black Wins!",
        Some(Stone::White) => "White Wins!",
        None => "It's a Draw!",
    };

    // 创建胜利窗口 - 调整位置使其靠右
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(window_width / 2.0 + 100.0), // 向右移动
                    top: Val::Px(300.0),
                    width: Val::Px(300.0),
                    height: Val::Px(200.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    padding: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
                background_color: Color::rgb(0.9, 0.9, 0.9).into(),
                z_index: ZIndex::Global(10), // 确保显示在最上层
                ..default()
            },
            VictoryWindow,
        ))
        .with_children(|parent| {
            // 添加标题
            parent.spawn(
                TextBundle::from_section(
                    "Game Over",
                    TextStyle {
                        font_size: 28.0,
                        color: Color::rgb(0.2, 0.2, 0.2),
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                })
                .with_text_alignment(TextAlignment::Center),
            );

            // 添加胜利者文本
            parent.spawn(
                TextBundle::from_section(
                    victory_text,
                    TextStyle {
                        font_size: 24.0,
                        color: Color::rgb(0.2, 0.2, 0.2),
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::bottom(Val::Px(30.0)),
                    ..default()
                })
                .with_text_alignment(TextAlignment::Center),
            );

            // 添加"再来一局"按钮
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(120.0),
                            height: Val::Px(40.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                        ..default()
                    },
                    PlayAgainButton,
                ))
                .with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section(
                            "Play Again",
                            TextStyle {
                                font_size: 20.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        )
                        .with_text_alignment(TextAlignment::Center),
                    );
                });
        });
}

// 处理胜利窗口关闭按钮点击
pub fn handle_victory_close_button(
    mut commands: Commands,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<VictoryCloseButton>),
    >,
    victory_window_query: Query<Entity, With<VictoryWindow>>,
) {
    for (interaction, mut bg_color) in &mut button_query {
        match *interaction {
            Interaction::Pressed => {
                // 按下状态 - 颜色变深
                *bg_color = Color::rgb(0.1, 0.1, 0.1).into();

                // 关闭胜利窗口
                for entity in victory_window_query.iter() {
                    commands.entity(entity).despawn_recursive();
                }
            }
            Interaction::Hovered => {
                // 悬停状态 - 颜色变亮
                *bg_color = Color::rgb(0.25, 0.25, 0.25).into();
            }
            Interaction::None => {
                // 普通状态 - 恢复默认颜色
                *bg_color = Color::rgb(0.15, 0.15, 0.15).into();
            }
        }
    }
}

// 处理"再来一局"按钮点击
pub fn handle_play_again_button(
    mut commands: Commands,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayAgainButton>),
    >,
    victory_window_query: Query<Entity, With<VictoryWindow>>,
    mut game_state: ResMut<GameState>,
    stone_query: Query<Entity, With<StoneComponent>>,
) {
    for (interaction, mut bg_color) in &mut button_query {
        match *interaction {
            Interaction::Pressed => {
                // 按下状态 - 颜色变深
                *bg_color = Color::rgb(0.1, 0.1, 0.1).into();

                // 关闭胜利窗口
                for entity in victory_window_query.iter() {
                    commands.entity(entity).despawn_recursive();
                }
                
                // 重置游戏状态
                game_state.reset();
                
                // 清除所有棋子
                for entity in stone_query.iter() {
                    commands.entity(entity).despawn_recursive();
                }
            }
            Interaction::Hovered => {
                // 悬停状态 - 颜色变亮
                *bg_color = Color::rgb(0.25, 0.25, 0.25).into();
            }
            Interaction::None => {
                // 普通状态 - 恢复默认颜色
                *bg_color = Color::rgb(0.15, 0.15, 0.15).into();
            }
        }
    }
}
