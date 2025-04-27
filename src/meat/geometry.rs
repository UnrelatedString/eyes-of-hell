use three_d::core::prelude::*;
use three_d::{
    CpuMesh,
    Mesh,
    Gm,
    Context,
    ColorMaterial,
    Srgba,  
};

use crate::meat::color::{ Color, PrismFacePalette };
pub use macro_hell::rats;

pub type LevelCoordinate = f32;

pub enum Terrain {
    Floor,
    Wall,
}

use Terrain::*;

pub fn pain(transform: Mat4, context: &Context, color: Srgba, is_transparent: bool) -> Gm<Mesh, ColorMaterial> {
    let mut excrement = CpuMesh::circle(7);
    excrement.transform(transform).unwrap();
    let good_color = Color(color);
    Gm::new(
        Mesh::new(context, &excrement),
        if is_transparent {good_color.transparent_material()} else {good_color.opaque_material()}
    )
}

fn rectangle_mesh(min_corner: Point3<f32>, size: Vec2, rotation_from_xy: Mat4) -> CpuMesh {
    let mut ret = CpuMesh::square();
    ret.transform(
        Mat4::from_translation(min_corner.to_vec()) *
        rotation_from_xy *
        Mat4::from_nonuniform_scale(size.x / 2.0, size.y / 2.0, 0.5) *
        Mat4::from_translation(Vec3::new(1.0, 1.0, 0.0))
    ).unwrap();
    ret
}

pub struct AAPrism {
    meshes: AAPrismMeshes,
    terrain: AAPrismFaces<TerrainQuad>,
}

impl AAPrism {
    pub fn new(min_and_size: [Vec3; 2], palette: PrismFacePalette) -> AAPrism {
        let [min_corner, size] = min_and_size;
        AAPrism {
            meshes: AAPrismMeshes::new(
                Point3::from_vec(min_corner),
                size,
                palette,
            ),
            terrain: AAPrismFaces::new(
                Point3::from_vec(min_corner),
                size,
                TerrainQuad::from_rect,
            ),
        }
    }

    pub fn gms(&self, context: &Context) -> [Gm<Mesh, ColorMaterial>; 6] {
        self.meshes.gms(context)
    }

    pub fn get_terrain(&self) -> &AAPrismFaces<TerrainQuad> {
        &self.terrain
    }
}

pub struct AAPrismFaces<T> {
    pub top: T,
    pub north: T,
    pub south: T,
    pub east: T,
    pub west: T,
    pub bottom: T,
}

pub struct AAPrismMeshes {
    pub palette: PrismFacePalette,
    faces: AAPrismFaces<CpuMesh>
}

pub struct TerrainQuad {
    to_unit_square: Mat4,
}

impl <T> AAPrismFaces<T> {
    pub fn new<F>(min_corner: Point3<f32>, size: Vec3, face_factory: F) -> AAPrismFaces<T>
        where F: Fn(Point3<f32>, Vec2, Mat4) -> T
    {
        let sx = Vec3::new(size.x, 0.0, 0.0);
        let sy = Vec3::new(0.0, size.y, 0.0);
        let sz = Vec3::new(0.0, 0.0, size.z);

        let top = face_factory(
            min_corner + sy,
            Vec2::new(size.x, size.z),
            Mat4::from_angle_x(Rad::turn_div_4()),
        );

        // NORTH = +Z
        let north = face_factory(
            min_corner + sz,
            Vec2::new(size.x, size.y),
            Mat4::identity(),
        );

        // SOUTH = -Z
        let south = face_factory(
            min_corner,
            Vec2::new(size.x, size.y),
            Mat4::identity(),
        );

        // EAST = +X
        let east = face_factory(
            min_corner + sx,
            Vec2::new(size.z, size.y),
            Mat4::from_angle_y(-Rad::turn_div_4()),
        );
        
        // WEST = -X
        let west = face_factory(
            min_corner,
            Vec2::new(size.z, size.y),
            Mat4::from_angle_y(-Rad::turn_div_4()),
        );
        
        let bottom = face_factory(
            min_corner,
            Vec2::new(size.x, size.z),
            Mat4::from_angle_x(Rad::turn_div_4()),
        );

        AAPrismFaces {
            top,
            north,
            south,
            east,
            west,
            bottom,
        }
    }
}

impl AAPrismMeshes {
    pub fn new(min_corner: Point3<f32>, size: Vec3, palette: PrismFacePalette) -> AAPrismMeshes {
        AAPrismMeshes {
            palette,
            faces: AAPrismFaces::new(min_corner, size, rectangle_mesh),
        }
    }

    pub fn gms(&self, context: &Context) -> [Gm<Mesh, ColorMaterial>; 6] {[
        Gm::new(
            Mesh::new(context, &self.faces.top),
            self.palette.top.opaque_material(),
        ),
        Gm::new(
            Mesh::new(context, &self.faces.north),
            self.palette.ns.opaque_material(),
        ),
        Gm::new(
            Mesh::new(context, &self.faces.south),
            self.palette.ns.opaque_material(),
        ),
        Gm::new(
            Mesh::new(context, &self.faces.east),
            self.palette.ew.opaque_material(),
        ),
        Gm::new(
            Mesh::new(context, &self.faces.west),
            self.palette.ew.opaque_material(),
        ),
        Gm::new(
            Mesh::new(context, &self.faces.bottom),
            self.palette.bottom.opaque_material(),
        ),
    ]}
}

impl TerrainQuad {
    pub fn from_rect(min_corner: Point3<f32>, size: Vec2, rotation_from_xy: Mat4) -> TerrainQuad {
        
        let from_unit_square_3d =
            Mat4::from_translation(min_corner.to_vec()) *
            rotation_from_xy *
            Mat4::from_nonuniform_scale(size.x, size.y, 1.0);
        
        //panic!("{:?} should be {:?}", min_corner, from_unit_square_3d.transform_point(Point3::new(0.0, 0.0, 0.0)));

        TerrainQuad {
            to_unit_square: from_unit_square_3d.invert().unwrap(),
        }
    }

    pub fn onto_plane(&self, point: Point2<f32>, inverse_camera: Mat4) -> Point3<f32> {
        // dummy z coordinate because MAAAAAATH! I MATHEDED >:3
        let p = Point3::new(point.x, point.y, 0.0);
        let transform = self.to_unit_square * inverse_camera;
        let out_z = transform.row(2).dot(Vec4::new(point.x, point.y, 0.0, 1.0));
        let in_z = -out_z / transform.z.z;
        let p2 = Point3::new(point.x, point.y, in_z);
        let relative = transform.transform_point(p2);
        println!("{:?}", relative);
        relative
    }

    pub fn contains(&self, point: Point2<f32>, inverse_camera: Mat4) -> bool {
        let relative = self.onto_plane(point, inverse_camera);
        (0.0 ..= 1.0).contains(&relative.x) && (0.0 ..= 1.0).contains(&relative.y)
    }
}
