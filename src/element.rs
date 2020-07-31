use nalgebra::{Vector2};

pub struct Element {
    position: (f32, f32),
    size: (f32, f32)
}

impl Element {
    pub fn new(position: (f32, f32), size: (f32, f32)) -> Element {
        Element {
            position,
            size
        }
    }

    pub fn is_mouse_over(&self, mouse_position: Vector2<f32>) -> bool {
        let bounds = self.bounds();

        if mouse_position.x < (bounds.0).0 || mouse_position.x > (bounds.1).0 {
            return false;
        }

        if mouse_position.y < (bounds.0).1 || mouse_position.y > (bounds.1).1 {
            return false;
        }

        return true;
    }

    pub fn bounds(&self) -> ((f32, f32), (f32, f32)) {
        let lower = (
            self.position.0 - self.size.0/2.,
            self.position.1 - self.size.1/2.
        );

        (
            lower,
            (
                lower.0 + self.size.0,
                lower.1 + self.size.1
            )
        )
    }
}