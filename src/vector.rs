#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Vector
{
    pub x: i64,
    pub y: i64
}


impl Vector
{
    pub fn new(x: i64, y: i64) -> Vector
    {
        Vector{x, y}
    }

    pub fn add(&self, other: &Vector) -> Vector
    {
        Vector{x: self.x + other.x, y: self.y + other.y}
    }

    pub fn sub(&self, other: &Vector) -> Vector
    {
        Vector{x: self.x - other.x, y: self.y - other.y}
    }

    pub fn calc_orthogonal(&self) -> Vector
    {
        if self.y == 0
        {
            return Vector{x: 0, y: -self.x}
        }
        Vector{x: self.y, y: 0}
    }

    pub fn multiply(&self, c: i64) -> Vector
    {
        Vector{x: self.x * c, y: self.y * c}
    }

    pub fn length(&self) -> f64
    {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
    }
}