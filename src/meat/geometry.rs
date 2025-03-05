use three_d::core::prelude::*;
use three_d::{
    CpuMesh,
    Positions,
    Mesh,
    Gm,
    Context,
    ColorMaterial,
    Srgba,
    RenderStates,
    WriteMask,
    Blend,
};

use crate::meat::color::PrismFacePalette;
pub use macro_hell::rats;

pub fn pain(transform: Mat4, context: &Context, color: Srgba, is_transparent: bool) -> Gm<Mesh, ColorMaterial> {
    let mut excrement = CpuMesh::circle(7);
    excrement.transform(transform).unwrap();
    Gm::new(
        Mesh::new(context, &excrement),
        ColorMaterial {
            color,
            texture: None,
            render_states: 
                RenderStates {
                    blend: Blend::TRANSPARENCY,
                    write_mask:
                        if is_transparent {
                            WriteMask::COLOR
                        } else {
                            WriteMask::COLOR_AND_DEPTH
                        },
                    ..Default::default()
                },
            is_transparent,
        }
    )
}

fn rectangle(min_corner: Vec3, size: Vec2, rotation_from_xy: Mat4) -> CpuMesh {
    let mut ret = CpuMesh::square();
    ret.transform(
        // Mat4::from_translation(Vec3::new(1.0, 1.0, 1.0)) *
        // Mat4::from_nonuniform_scale(size.x / 2.0, size.y / 2.0, 0.5) *
        // rotation_from_xy *
        // Mat4::from_translation(min_corner)
        Mat4::from_translation(min_corner) *
        rotation_from_xy *
        Mat4::from_nonuniform_scale(size.x / 2.0, size.y / 2.0, 0.5) *
        Mat4::from_translation(Vec3::new(1.0, 1.0, 0.0))
    ).unwrap();
    ret
}

pub struct AAPrism {
    min_corner: Vec3, // Point3<f32>,
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
    pub fn new(min_corner: Vec3, size: Vec3, palette: PrismFacePalette) -> AAPrism {
        let sx = Vec3::new(size.x, 0.0, 0.0);
        let sy = Vec3::new(0.0, size.y, 0.0);
        let sz = Vec3::new(0.0, 0.0, size.z);

        let bottom = rectangle(
            min_corner,
            Vec2::new(size.x, size.z),
            Mat4::from_angle_x(Rad::turn_div_4()),
        );

        // NORTH = +Z
        let north = rectangle(
            min_corner + sz,
            Vec2::new(size.x, size.y),
            Mat4::identity(),
        );

        // SOUTH = -Z
        let south = rectangle(
            min_corner,
            Vec2::new(size.x, size.y),
            Mat4::identity(),
        );

        // EAST = +X
        let east = rectangle(
            min_corner + sx,
            Vec2::new(size.z, size.y),
            Mat4::from_angle_y(-Rad::turn_div_4()),
        );
        
        // WEST = -X
        let west = rectangle(
            min_corner,
            Vec2::new(size.z, size.y),
            Mat4::from_angle_y(-Rad::turn_div_4()),
        );

        let top = rectangle(
            min_corner + sy,
            Vec2::new(size.x, size.z),
            Mat4::from_angle_x(Rad::turn_div_4()),
        );

        AAPrism {
            min_corner,
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

