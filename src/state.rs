use amethyst::{
    prelude::*
};

use super::tile::{
    GroundTile,
    RoadTile,
};

pub struct RunningState;

impl SimpleState for RunningState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        
    }
}
