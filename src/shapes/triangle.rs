use super::vertex::Vertex;

pub struct Triangle(Vec<Vertex>);

impl Triangle {
    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) -> Vec<Vertex> {
        vec![
            Vertex::new(x1, y1),
            Vertex::new(x2, y2),
            Vertex::new(x3, y3),
        ]
    }
}
