use crate::prelude::*;

pub fn load_renderables(mut renderables: Models<Renderables>) -> Models<Renderables> {
    renderables.insert(None, Some(Renderables::Wall), load_scene("wall"), Template::Scene);
    renderables.insert(None, Some(Renderables::Floor), load_scene("floor"), Template::Scene);
    renderables.insert(None, Some(Renderables::Player), load_scene("player"), Template::Scene);

    renderables
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Renderables {
    Player,
    Enemy,
    Wall,
    Floor,
}