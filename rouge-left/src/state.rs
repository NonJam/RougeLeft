use crate::prelude::*;

enum LevelState {
    Generation,
    TurnManager,
    PlayerInput,
    TurnSimulator,
}

pub struct Level {
    fsm: LevelState,
}

impl State for Level {
    fn on_push(&mut self, data: &mut StateData, resources: &mut Resources) {
        data.world.insert(
            (),
            (0..1).map(|_| {
                let models = resources.get::<Models<Renderables>>().unwrap();
                let (template, index) = models.data_from_t(&Renderables::Player).unwrap();

                (
                    Renderable::new(Position::default(), index, template),
                    Position::new(20f32, 20f32),
                )
            })
        );
    }

    fn update(&mut self, data: &mut StateData, resources: &mut Resources) {
        let next_state: Option<LevelState> = match self.fsm {
            LevelState::Generation => { self.generation(data, resources) }
            LevelState::TurnManager => { self.turn_manager(data, resources) }
            LevelState::PlayerInput => { self.player_input(data, resources) }
            LevelState::TurnSimulator => { self.turn_simulator(data, resources) }
        };

        if let Some(next_state) = next_state {
            self.fsm = next_state;
        }
    }
}

impl Level {
    pub fn new() -> Self {
        Level {
            fsm: LevelState::Generation,
        }
    }

    fn generation(&mut self, data: &mut StateData, resources: &mut Resources) -> Option<LevelState> {
        const width: usize = 50;
        const height: usize = 50;
        const passes: usize = 5;

        let mut map = vec![false; width * height];
        let mut map_two = vec![false; width * height];

        use rand::prelude::*;

        let mut rng = rand::thread_rng();

        // Populate map
        {
            for cell in map.iter_mut() {
                *cell = rng.gen::<f32>() > 0.5f32;
            }
        }

        // Run CA        
        {
            fn get_in_scope(index: usize, map: &Vec<bool>) -> usize {
                let mut count = 0;

                let mut indices: Vec<usize> = vec![];

                let (x, y) = {
                    let x = index % width;
                    let y = index / width;
                    (x, y)
                };

                // Left Right Up Down
                if x != 0 {
                    indices.push(index - 1);
                }
                if x != width - 1  {
                    indices.push(index + 1);
                }
                if y != 0 {
                    indices.push(index - height);
                }
                if y != height - 1 {
                    indices.push(index + height);
                }

                // TopLeft TopRight BottomLeft BottomRight
                if x != 0 && y != 0 {
                    indices.push(index - 1 - height);
                }
                if x != width - 1 && y != 0 {
                    indices.push(index + 1 - height);
                }
                if x != 0 && y != height - 1 {
                    indices.push(index - 1 + height);
                }
                if x != width - 1 && y != height -1 {
                    indices.push(index + 1 + height);
                }

                // Add walls to count
                for i in indices.iter() {
                    if map[*i] {
                        count += 1;
                    }
                }
                // Add off-map to count
                count += 8 - indices.len();
                // Add self to count
                if map[index] {
                    count += 1;
                }

                count
            }

            // Run CA
            let mut map_r;
            let mut map_w;
            for i in 1..=passes {

                if i % 2 == 1 {
                    map_r = &map;
                    map_w = &mut map_two;
                } else {
                    map_r = &map_two;
                    map_w = &mut map;
                }

                for i in 0..(width * height) {
                    let amount = get_in_scope(i, map_r);

                    let is_wall: bool = amount >= 5;

                    map_w[i] = is_wall;
                }
            }
        }

        // Get what map has the final data on
        let map = if passes % 2 == 1 {
            map_two
        } else {
            map
        };

        // Create entites
        let create_multi = || {
            let models = resources.get::<Models<Renderables>>().unwrap();
            let (wall_t, wall_i) = models.data_from_t(&Renderables::Wall).unwrap();
            let (floor_t, floor_i) = models.data_from_t(&Renderables::Floor).unwrap();
            let mut multi_renderable = Renderable::default();

            for i in 0..(width * height) {
                let (x, y) = {
                    let x = i % width;
                    let y = i / width;
                    (x, y)
                };
                
                let (template, index) = if map[i] {
                    (wall_t, wall_i)
                } else {
                    (floor_t, floor_i)
                };

                multi_renderable.push_child(Renderable::new(
                    Position::new(x as f32, y as f32),
                    index,
                    template,
                ));
            }

            let (template, index) = (wall_t, wall_i);
            for i in 0..width {
                multi_renderable.push_child(Renderable::new(
                    Position::new(i as f32, 0f32), index, template,
                ));
                multi_renderable.push_child(Renderable::new(
                    Position::new(i as f32, height as f32 - 1f32), index, template,
                ));
            }
            for i in 0..height {
                multi_renderable.push_child(Renderable::new(
                    Position::new(0f32, i as f32), index, template,
                ));
                multi_renderable.push_child(Renderable::new(
                    Position::new(width as f32 - 1f32, i as f32), index, template,
                ));
            }
            multi_renderable
        };

        data.world.insert(
            (),
            (0..1).map(move |_| (
                create_multi(),
            ))
        );

        resources.insert(Map(map));

        Some(LevelState::TurnManager)
    }

    fn turn_manager(&mut self, data: &mut StateData, resources: &mut Resources) -> Option<LevelState> {
        
        None
    }

    fn player_input(&mut self, data: &mut StateData, resources: &mut Resources) -> Option<LevelState> {
        
        None
    }

    fn turn_simulator(&mut self, data: &mut StateData, resources: &mut Resources) -> Option<LevelState> {
        
        None
    }
}

pub struct Map(Vec<bool>);