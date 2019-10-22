use crate::geometry::*;
use crate::material::*;
use crate::transform::*;

pub type ObjectId = usize;

pub struct Object<'a> {
    pub transform: Transform,
    pub geometry: Geometry,
    pub material: Material<'a>,
    pub parent: Option<ObjectId>,
}

impl<'a> Object<'a> {
    pub fn new() -> Object<'a> {
        Object {
            transform: Transform::new(),
            geometry: Geometry::sphere(),
            material: Material::new(),
            parent: None,
        }
    }

    pub fn transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }

    pub fn geometry(mut self, geometry: Geometry) -> Self {
        self.geometry = geometry;
        self
    }

    pub fn material(mut self, material: Material<'a>) -> Self {
        self.material = material;
        self
    }

    pub fn parent(mut self, parent: ObjectId) -> Self {
        self.parent = Some(parent);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_objects_default_transformation() {
        let o = Object::new();
        assert_eq!(o.transform, Transform::new());
    }

    #[test]
    fn changing_an_objects_transformation() {
        let o = Object::new().transform(Transform::new().translate(2., 3., 4.));
        assert_eq!(o.transform, Transform::new().translate(2., 3., 4.));
    }

    #[test]
    fn an_object_has_a_default_material() {
        let s = Object::new();
        assert_eq!(s.material, Material::new());
    }

    #[test]
    fn an_object_may_be_assigned_a_material() {
        let m = Material::new().ambient(1.);
        let s = Object::new().material(m);
        assert_eq!(s.material, m);
    }
}
