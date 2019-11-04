use crate::intersection::*;
use crate::ray::*;
use crate::tuple::*;

pub mod cone;
pub mod cube;
pub mod cylinder;
pub mod plane;
pub mod sphere;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Geometry {
    Plane,
    Sphere,
    Cube,
    Cone {
        /// Minimum y-value for the cone.
        min: f32,
        /// Maximum y-value for the cone.
        max: f32,
        /// Whether to close the cone on the end.
        closed: bool,
    },
    Cylinder {
        /// Minimum y-value for the cylinder.
        min: f32,
        /// Maximum y-value for the cylinder.
        max: f32,
        /// Whether to close the cylinder on each end.
        closed: bool,
    },
    TestShape,
}

impl Geometry {
    pub fn plane() -> Self {
        Geometry::Plane
    }

    pub fn sphere() -> Self {
        Geometry::Sphere
    }

    pub fn cube() -> Self {
        Geometry::Cube
    }

    pub fn cone() -> Self {
        Geometry::Cone {
            min: -std::f32::INFINITY,
            max: std::f32::INFINITY,
            closed: false,
        }
    }

    pub fn cylinder() -> Self {
        Geometry::Cylinder {
            min: -std::f32::INFINITY,
            max: std::f32::INFINITY,
            closed: false,
        }
    }

    pub fn test() -> Self {
        Geometry::TestShape
    }

    /// Returns the collection of Intersections where the ray intersects the
    /// geometry.
    pub fn intersect(self, ray: Ray) -> Intersections {
        match self {
            Geometry::Plane => plane::intersect(ray),
            Geometry::Sphere => sphere::intersect(ray),
            Geometry::Cube => cube::intersect(ray),
            Geometry::Cone { min, max, closed } => cone::intersect(ray, min, max, closed),
            Geometry::Cylinder { min, max, closed } => cylinder::intersect(ray, min, max, closed),
            Geometry::TestShape => Intersections::new(),
        }
    }

    /// Returns the surface normal at the given point.
    pub fn normal_at(self, point: Tuple4) -> Tuple4 {
        match self {
            Geometry::Plane => plane::normal_at(point),
            Geometry::Sphere => sphere::normal_at(point),
            Geometry::Cube => cube::normal_at(point),
            Geometry::Cone { min, max, closed } => cone::normal_at(point, min, max, closed),
            Geometry::Cylinder { min, max, closed } => cylinder::normal_at(point, min, max, closed),
            Geometry::TestShape => vector3(0., 0., 0.),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_default_minimum_and_maximum_for_a_cylinder() {
        let cyl = Geometry::cylinder();
        if let Geometry::Cylinder {
            min,
            max,
            closed: _,
        } = cyl
        {
            assert_eq!(min, -std::f32::INFINITY);
            assert_eq!(max, std::f32::INFINITY);
        } else {
            panic!();
        }
    }

    #[test]
    fn the_default_closed_value_for_a_cylinder() {
        let cyl = Geometry::cylinder();
        if let Geometry::Cylinder {
            min: _,
            max: _,
            closed,
        } = cyl
        {
            assert_eq!(closed, false);
        } else {
            panic!();
        }
    }
}
