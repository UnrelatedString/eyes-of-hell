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
        let sx = Vec3::new(size.x, 0.0, 0.0); // size * Vec3::unit_x();
        let sy = Vec3::new(0.0, size.y, 0.0); // size * Vec3::unit_y();
        let sz = Vec3::new(0.0, 0.0, size.z); // size * Vec3::unit_z();

        let top = CpuMesh {
            positions: Positions::F32(vec![
                top_ne, top_ne + sz,
                top_ne + sz + sx, top_ne + sx,
            ]),
            ..Default::default()
        };
        let north = CpuMesh {
            positions: Positions::F32(vec![
                top_ne, top_ne + sz,
                top_ne + sz + sy, top_ne + sy,
            ]),
            ..Default::default()
        };
        let south = CpuMesh {
            positions: Positions::F32(vec![
                top_ne + sx, top_ne + sx + sz,
                top_ne + size, top_ne + sx + sy,
            ]),
            ..Default::default()
        };
        let east = CpuMesh {
            positions: Positions::F32(vec![
                top_ne, top_ne + sx,
                top_ne + sx + sy, top_ne + sy,
            ]),
            ..Default::default()
        };
        let west = CpuMesh {
            positions: Positions::F32(vec![
                top_ne + sz, top_ne + sx + sz,
                top_ne + size, top_ne + sy + sz,
            ]),
            ..Default::default()
        };
        let bottom = CpuMesh {
            positions: Positions::F32(vec![
                top_ne + sy, top_ne + sy + sz,
                top_ne + size, top_ne + sx + sy,
            ]),
            ..Default::default()
        };

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

    pub fn gms(&self, context: &Context) -> [Gm<Mesh, ColorMaterial>; 1] {[
        Gm::new(
            Mesh::new(context, &self.top),
            ColorMaterial {
                color: self.palette.top,
                ..Default::default()
            },
        )
    ]}
}

