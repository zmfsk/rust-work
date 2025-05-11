use crate::game::{GameState, Stone, StoneComponent};
use crate::agent::SmartAgent; // Add this import for SmartAgent
use bevy::prelude::*;

// Add these constants at the top of the file
const BOARD_OFFSET: f32 = -200.0;
const GRID_SIZE: usize = 14; // Make sure this matches the value in your game.rs
const CELL_SIZE: f32 = 40.0; // Make sure this matches the value in your game.rs

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

// 添加难度选择相关组件
#[derive(Component)]
pub struct DifficultySelector;

#[derive(Component)]
pub struct DifficultyDropdown {
    pub is_open: bool,
}

#[derive(Component)]
pub struct DifficultyOption {
    pub level: u32,
    pub label: String,
}

// 创建难度选择下拉菜单
pub fn setup_difficulty_selector(mut commands: Commands, ai: Res<SmartAgent>) {
    // 难度按钮位置
    let button_x = BOARD_OFFSET + (GRID_SIZE as f32 * CELL_SIZE) + 100.0;
    let button_y = 50.0;
    
    // 获取当前难度
    let current_level = ai.get_difficulty();
    let difficulty_label = match current_level {
        1 => "Easy",
        3 => "Medium",
        2 => "Hard",
        _ => "Medium",
    };
    
    // 创建难度选择器按钮
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(button_x),
                    top: Val::Px(button_y),
                    width: Val::Px(150.0),
                    height: Val::Px(40.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            DifficultySelector,
        ))
        .with_children(|parent| {
            // 主按钮
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Px(40.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::rgb(0.3, 0.3, 0.3).into(),
                        ..default()
                    },
                    DifficultyDropdown { is_open: false },
                ))
                .with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section(
                            format!("Difficulty: {} ", difficulty_label),
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

// 处理难度下拉菜单点击
pub fn handle_difficulty_dropdown(
    mut commands: Commands,
    mut dropdown_query: Query<(Entity, &mut DifficultyDropdown, &mut BackgroundColor, &Children), (Changed<Interaction>, With<Button>)>,
    mut text_query: Query<&mut Text>,
    interaction_query: Query<&Interaction, With<Button>>,
    ai: Res<SmartAgent>,
) {
    for (entity, mut dropdown, mut bg_color, children) in dropdown_query.iter_mut() {
        let interaction = interaction_query.get(entity).unwrap_or(&Interaction::None);
        
        match *interaction {
            Interaction::Pressed => {
                // 切换下拉菜单状态
                dropdown.is_open = !dropdown.is_open;
                
                // 更新按钮颜色
                *bg_color = if dropdown.is_open {
                    Color::rgb(0.4, 0.4, 0.4).into()
                } else {
                    Color::rgb(0.3, 0.3, 0.3).into()
                };
                
                // 如果打开下拉菜单，创建选项
                if dropdown.is_open {
                    // 获取按钮文本
                    let text_entity = children.iter().next().unwrap();
                    if let Ok(mut text) = text_query.get_mut(*text_entity) {
                        // 获取当前难度并更改箭头方向
                        let current_level =ai.get_difficulty();
                        let difficulty_label = match current_level {
                            1 => "Easy",
                            3 => "Medium",
                            2 => "Hard",
                            _ => "Medium",
                        };
                        text.sections[0].value = format!("Difficulty: {} ", difficulty_label);
                    }
                    
                    // 创建下拉选项
                    commands.entity(entity).with_children(|parent| {
                        // 创建选项容器
                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                    position_type: PositionType::Absolute,
                                    top: Val::Px(40.0),
                                    width: Val::Percent(100.0),
                                    flex_direction: FlexDirection::Column,
                                    ..default()
                                },
                                background_color: Color::rgb(0.2, 0.2, 0.2).into(),
                                z_index: ZIndex::Global(100),
                                ..default()
                            })
                            .with_children(|parent| {
                                // 添加三个难度选项
                                let options = [
                                    ("Easy", 1, Color::rgb(0.2, 0.6, 0.2)),
                                    ("Medium", 3, Color::rgb(0.6, 0.6, 0.2)),
                                    ("Hard", 2, Color::rgb(0.6, 0.2, 0.2)),
                                ];
                                
                                for (label, level, color) in options {
                                    parent
                                        .spawn((
                                            ButtonBundle {
                                                style: Style {
                                                    width: Val::Percent(100.0),
                                                    height: Val::Px(30.0),
                                                    justify_content: JustifyContent::Center,
                                                    align_items: AlignItems::Center,
                                                    ..default()
                                                },
                                                background_color: color.with_a(0.7).into(),
                                                ..default()
                                            },
                                            DifficultyOption {
                                                level,
                                                label: label.to_string(),
                                            },
                                        ))
                                        .with_children(|parent| {
                                            parent.spawn(
                                                TextBundle::from_section(
                                                    label,
                                                    TextStyle {
                                                        font_size: 18.0,
                                                        color: Color::WHITE,
                                                        ..default()
                                                    },
                                                )
                                                .with_text_alignment(TextAlignment::Center),
                                            );
                                        });
                                }
                            });
                    });
                } else {
                    // 关闭下拉菜单，移除选项
                    let text_entity = children.iter().next().unwrap();
                    if let Ok(mut text) = text_query.get_mut(*text_entity) {
                        // 显示当前选择的难度
                        let current_level = ai.get_difficulty();
                        let difficulty_label = match current_level {
                            1 => "Easy",
                            3 => "Medium",
                            2 => "Hard",
                            _ => "Medium",
                        };
                        text.sections[0].value = format!("Difficulty: {} ", difficulty_label);
                    }
                    
                    // 移除所有子元素（除了第一个文本元素）
                    for &child in children.iter().skip(1) {
                        commands.entity(child).despawn_recursive();
                    }
                }
            }
            Interaction::Hovered => {
                // 悬停效果
                *bg_color = Color::rgb(0.35, 0.35, 0.35).into();
            }
            Interaction::None => {
                // 恢复默认颜色
                if !dropdown.is_open {
                    *bg_color = Color::rgb(0.3, 0.3, 0.3).into();
                }
            }
        }
    }
}

// 处理难度选项点击
pub fn handle_difficulty_options(
    mut commands: Commands,
    mut option_query: Query<(&Interaction, &DifficultyOption, &Parent), (Changed<Interaction>, With<Button>)>,
    dropdown_query: Query<(Entity, &Children), With<DifficultyDropdown>>,
    mut text_query: Query<&mut Text>,
    mut ai: ResMut<SmartAgent>,
) {
    for (interaction, option, parent) in option_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            // 设置AI难度
            ai.set_depth(option.level);
            
            // 更新下拉菜单文本并关闭菜单
            for (dropdown_entity, children) in dropdown_query.iter() {
                // 更新按钮文本
                if let Some(&text_entity) = children.iter().next() {
                    if let Ok(mut text) = text_query.get_mut(text_entity) {
                        text.sections[0].value = format!("Difficulty: {} ", option.label);
                    }
                }
                
                // 移除所有子元素（除了第一个文本元素）
                for &child in children.iter().skip(1) {
                    commands.entity(child).despawn_recursive();
                }
                
                // 更新下拉菜单状态
                commands.entity(dropdown_entity).insert(DifficultyDropdown { is_open: false });
            }
            
            println!("AI difficulty set to: {}", option.level);
        }
    }
}
