use amethyst::{
    core::math::{Point2, Point3}, 
    ecs::{World, WorldExt}, prelude::*, 
    renderer::{
        plugins::RenderFlat2D,
        types::DefaultBackend,
        RenderingBundle,
    }, 
    tiles::{RenderTiles2D, Tile}
};

#[derive(Clone, Debug, Default)]
pub struct GroundTile {
    sprites: Option<usize>,
    //occupied: bool
}

impl Tile for GroundTile {
    fn sprite(&self, coordinates: Point3<u32>, world: &World) -> Option<usize> {
        self.sprites
    }
}

#[derive(Clone, Debug, Default)]
pub struct RoadTile {
    sprites: Option<usize>,

}

impl Tile for RoadTile {
    fn sprite(&self, coordinates: Point3<u32>, world: &World) -> Option<usize> {
        self.sprites
    }
}