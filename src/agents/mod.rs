use amethyst::{ecs::Entity, prelude::*};


type Population = u32;
pub struct Housing {
    pub size: Population,
    
}


pub trait Consumption {
        
}

pub struct Worker {
    pub population: Population,
    pub money: f32,
    //pub needs: Vec<(Goods, f32)>
    //pub goods: Goods
}

pub struct Unemployment {
    pub population: Population,
    pub money: f32,
    //pub goods: Vec<(Goods, f32)>
}

pub struct Student {
    pub population: Population,
    //pub needs: (Goods, f32)
    //pub goods: (Goods, f32)
}