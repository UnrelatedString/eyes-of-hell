use three_d::core::prelude::*;
use three_d::{
    CpuMesh,
    Positions,
    Mesh,
    Gm,
    Context,
    ColorMaterial,
};

use crate::meat::color::PrismFacePalette;

fn rectangle(top_ne: Vec3, size: Vec2, rotation_from_xy: Mat4) -> CpuMesh {
    let mut ret = CpuMesh::square();
    ret.transform(
        // Mat4::from_translation(Vec3::new(1.0, 1.0, 1.0)) *
        // Mat4::from_nonuniform_scale(size.x / 2.0, size.y / 2.0, 0.5) *
        // rotation_from_xy *
        // Mat4::from_translation(top_ne)
        Mat4::from_translation(top_ne) *
        rotation_from_xy *
        Mat4::from_nonuniform_scale(size.x / 2.0, size.y / 2.0, 0.5) *
        Mat4::from_translation(Vec3::new(1.0, 1.0, 0.0))
    ).unwrap();
    ret
}

pub struct AAPrism {
    top_ne: Vec3, // Point3<f32>,
    size: Vec3,
    pub palette: PrismFacePalette,
    top: CpuMesh,
    north: CpuMesh,
    south: CpuMesh,
    east: CpuMesh,
    west: CpuMesh,
    bottom: CpuMesh,
}

impl AAPrism {
    pub fn new(top_ne: Vec3, size: Vec3, palette: PrismFacePalette) -> AAPrism {
        let sx = Vec3::new(size.x, 0.0, 0.0);
        let sy = Vec3::new(0.0, size.y, 0.0);
        let sz = Vec3::new(0.0, 0.0, size.z);

        let top = rectangle(
            top_ne,
            Vec2::new(size.x, size.z), // I cry when no swizzles
            Mat4::from_angle_x(Rad::turn_div_4()),
        );

        let north = rectangle(
            top_ne,
            Vec2::new(size.z, size.y),
            Mat4::identity(),
        );

        let south = rectangle(
            top_ne + sx,
            Vec2::new(size.z, size.y),
            Mat4::identity(),
        );

        let east = rectangle(
            top_ne,
            Vec2::new(size.x, size.y),
            Mat4::from_angle_y(-Rad::turn_div_4()),
        );
        
        let west = rectangle(
            top_ne + sz,
            Vec2::new(size.x, size.y),
            Mat4::from_angle_y(-Rad::turn_div_4()),
        );

        let bottom = rectangle(
            top_ne + sy * 5.0,
            Vec2::new(size.x, size.z),
            Mat4::from_angle_x(Rad::turn_div_4()),
        );

        AAPrism {
            top_ne,
            size,
            palette,
            top,
            north,
            south,
            east,
            west,
            bottom,
        }
    }

    pub fn gms(&self, context: &Context) -> [Gm<Mesh, ColorMaterial>; 6] {[
        Gm::new(
            Mesh::new(context, &self.top),
            ColorMaterial {
                color: self.palette.top,
                ..Default::default()
            },
        ),
        Gm::new(
            Mesh::new(context, &self.north),
            ColorMaterial {
                color: self.palette.ns,
                ..Default::default()
            },
        ),
        Gm::new(
            Mesh::new(context, &self.south),
            ColorMaterial {
                color: self.palette.ns,
                ..Default::default()
            },
        ),
        Gm::new(
            Mesh::new(context, &self.east),
            ColorMaterial {
                color: self.palette.ew,
                ..Default::default()
            },
        ),
        Gm::new(
            Mesh::new(context, &self.west),
            ColorMaterial {
                color: self.palette.ew,
                ..Default::default()
            },
        ),
        Gm::new(
            Mesh::new(context, &self.bottom),
            ColorMaterial {
                color: self.palette.bottom,
                ..Default::default()
            },
        ),
    ]}
}

