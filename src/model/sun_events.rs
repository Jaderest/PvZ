use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct PickupSunEvent {
    pub amount: u32,
}
