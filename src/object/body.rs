use super::collider_set::ColliderHandle;
use crate::Vec2;

/// Describes a body.
///  
/// It functions as a container for colliders.
#[derive(Clone, Debug)]
pub struct Body {
    pub position: Vec2,
    /// static body CAN have velocity - it just behaves as if it had infinite mass  
    /// (this might change with introduction of kinematic body that pushes other objects)  
    /// and doesn't collide with other static bodies
    pub velocity: Vec2,
    /// Type of body - `static` or `kinematic`
    pub status: BodyStatus,
    /// Whether colliders of the same body should collide
    pub self_collide: bool,
    // cached list of colliders belonging to body
    pub(crate) colliders: Vec<ColliderHandle>,
    // the distance body will want to cover during the next step
    pub(crate) movement: Vec2,
}

impl Body {
    pub fn new(position: Vec2, velocity: Vec2, status: BodyStatus, self_collide: bool) -> Self {
        Self {
            position,
            velocity,
            status,
            self_collide,
            colliders: Vec::new(),
            movement: Vec2::zero(),
        }
    }
}
/// Status of the body, determines how it's affected by other bodies.
#[derive(Copy, Clone, Debug)]
pub enum BodyStatus {
    /// Even when it moves it never collides with anything.
    Static,
    /// Collides with both static and kinematic bodies.
    Kinematic,
}
