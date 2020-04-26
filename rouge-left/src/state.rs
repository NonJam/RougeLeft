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
                    GDSpatial,
                    Renderable { template: template, index: index },
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
        {
            let models = resources.get::<Models<Renderables>>().unwrap();

            data.world.insert(
                (),
                (0..(width * height)).map(|index| {
                    let (x, y) = {
                        let x = index % width;
                        let y = index / width;
                        (x, y)
                    };

                    let (template, index) = if map[index] {
                        models.data_from_t(&Renderables::Wall).unwrap()
                    } else {
                        models.data_from_t(&Renderables::Floor).unwrap()
                    };

                    (
                        GDSpatial,
                        Renderable { template: template, index: index },
                        Position::new(x as f32, y as f32),
                    )
                }
            ));
            let (template, index) = models.data_from_t(&Renderables::Wall).unwrap();
            data.world.insert(
                (),
                (0..width).map(|i| (
                        GDSpatial,
                        Renderable { template: template, index: index },
                        Position::new(i as f32, 0f32),
                    )
            ));
            data.world.insert(
                (),
                (0..width).map(|i| (
                        GDSpatial,
                        Renderable { template: template, index: index },
                        Position::new(i as f32, height as f32 - 1f32),
                    )
            ));
            data.world.insert(
                (),
                (0..height).map(|i| (
                        GDSpatial,
                        Renderable { template: template, index: index },
                        Position::new(0f32, i as f32),
                    )
            ));
            data.world.insert(
                (),
                (0..height).map(|i| (
                        GDSpatial,
                        Renderable { template: template, index: index },
                        Position::new(width as f32 - 1f32, i as f32),
                    )
            ));
        }

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