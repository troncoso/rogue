use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[read_component(Weapon)]
#[write_component(Health)]
#[write_component(Item)]
#[write_component(Carried)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    if let Some(key) = key {
        let mut players = <(Entity, &Point)>::query().filter(component::<Player>());

        let delta = match key {
            VirtualKeyCode::A => Point::new(-1, 0),
            VirtualKeyCode::D => Point::new(1, 0),
            VirtualKeyCode::W => Point::new(0, -1),
            VirtualKeyCode::S => Point::new(0, 1),
            VirtualKeyCode::G => {
                let (player, player_pos) = players
                    .iter(ecs)
                    .map(|(entity, pos)| (*entity, *pos))
                    .next()
                    .unwrap();

                let mut items = <(Entity, &Item, &Point)>::query();
                items
                    .iter(ecs)
                    .filter(|(_, _, &item_pos)| item_pos == player_pos)
                    .for_each(|(entity, _, _)| {
                        commands.remove_component::<Point>(*entity);
                        commands.add_component(*entity, Carried(player));

                        if let Ok(e) = ecs.entry_ref(*entity) {
                            if e.get_component::<Weapon>().is_ok() {
                                <(Entity, &Carried, &Weapon)>::query()
                                    .iter(ecs)
                                    .filter(|(_, c, _)| c.0 == player)
                                    .for_each(|(e, _, _)| {
                                        commands.remove(*e);
                                    })
                            }
                        }
                    });

                Point::new(0, 0)
            }
            VirtualKeyCode::Key1 => use_item(0, ecs, commands),
            VirtualKeyCode::Key2 => use_item(1, ecs, commands),
            VirtualKeyCode::Key3 => use_item(2, ecs, commands),
            VirtualKeyCode::Key4 => use_item(3, ecs, commands),
            VirtualKeyCode::Key5 => use_item(4, ecs, commands),
            VirtualKeyCode::Key6 => use_item(5, ecs, commands),
            VirtualKeyCode::Key7 => use_item(6, ecs, commands),
            VirtualKeyCode::Key8 => use_item(7, ecs, commands),
            VirtualKeyCode::Key9 => use_item(8, ecs, commands),
            _ => Point::zero(),
        };

        let (player_entity, dest) = players
            .iter(ecs)
            .map(|(entity, pos)| (*entity, *pos + delta))
            .next()
            .unwrap();

        let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
        if delta.x != 0 || delta.y != 0 {
            let mut hit_something = false;
            enemies
                .iter(ecs)
                .filter(|(_, pos)| **pos == dest)
                .for_each(|(entity, _)| {
                    hit_something = true;
                    commands.push((
                        (),
                        WantsToAttack {
                            attacker: player_entity,
                            victim: *entity,
                        },
                    ));
                });

            if !hit_something {
                commands.push((
                    (),
                    WantsToMove {
                        entity: player_entity,
                        dest,
                    },
                ));
            }
        };

        *turn_state = TurnState::PlayerTurn;
    }
}

fn use_item(n: usize, ecs: &mut SubWorld, commands: &mut CommandBuffer) -> Point {
    let player_entity = <(Entity, &Player)>::query().iter(ecs).next().unwrap().0;
    let item_entity = <(Entity, &Item, &Carried)>::query()
        .iter(ecs)
        .filter(|(_, _, carried)| carried.0 == *player_entity)
        .enumerate()
        .filter(|(item_count, (_, _, _))| *item_count == n)
        .map(|(_, (entity, _, _))| *entity)
        .next();

    if let Some(item_entity) = item_entity {
        commands.push((
            (),
            ActivateItem {
                used_by: *player_entity,
                item: item_entity,
            },
        ));
    }

    Point::zero()
}
