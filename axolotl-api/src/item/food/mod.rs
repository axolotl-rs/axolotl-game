use crate::item::Item;

pub trait Food: Item {
    fn food_points(&self) -> f32;
    fn saturation(&self) -> f32;
    fn effective_quality(&self) -> f32;
    fn saturation_ratio(&self) -> f32;
}
