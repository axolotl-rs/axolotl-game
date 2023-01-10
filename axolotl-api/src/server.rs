use crate::player::Player;
use auto_impl::auto_impl;
#[auto_impl(Arc, &, Box)]
pub trait Server {
    type Player: Player;
}
