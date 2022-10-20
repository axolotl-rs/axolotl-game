pub trait Material {
    fn is_flammable(&self) -> bool;

    fn collides(&self) -> bool;
}

pub struct GenericMaterial {}
