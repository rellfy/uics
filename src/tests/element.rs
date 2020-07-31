use nalgebra::{Vector2};
use crate::element::*;

#[test]
fn exists_doesnt_throw() {
    let element: Element = Element::new(
        (0.5, 0.5),
        (0.1, 0.1)
    );
}

#[test]
fn hover_true() {
    let element: Element = Element::new(
        (0.5, 0.5),
        (0.1, 0.1)
    );
    let mouse_position: Vector2<f32> = Vector2::new(0.45, 0.45);
    let is_mouse_over_element: bool = element.is_mouse_over(mouse_position);
    assert_eq!(is_mouse_over_element, true);
}