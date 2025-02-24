use macroquad::prelude::*;

pub fn draw_quad(v1: Vec2, v2: Vec2, v3: Vec2, v4: Vec2, color: Color) {
    // It's probably better to use the graphics context directly 
    // but for a proof of concept this is fine
    // ...and also I can't find get_context() so it might not even be part of the API lol
    // maybe I will switch to miniquad a bit later
    
    // I think I also prefer how miniquad makes you pass context around explicitly
    // instead of having stuff mutate global state without even thinking about it 🙃
    draw_triangle(v1, v2, v4, color);
    draw_triangle(v2, v3, v4, color);
}
