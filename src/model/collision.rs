use sdl2::rect::Rect;

pub fn check_collision(rect1: Rect, rect2: Rect) -> bool {
    rect1.has_intersection(rect2)
}