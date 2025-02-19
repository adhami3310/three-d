mod tone_mapping;
pub use tone_mapping::*;

mod color_space;
pub use color_space::*;

use crate::core::*;

///
/// Represents a camera used for viewing 2D and 3D objects.
///
#[derive(Clone, Debug)]
pub struct Camera {
    camera: three_d_asset::Camera,
    /// This tone mapping is applied to the final color of renders using this camera.
    pub tone_mapping: ToneMapping,
    /// The final color of renders using this camera is converted to this color space.
    pub target_color_space: ColorSpace,
}

impl Camera {
    ///
    /// New camera which projects the world with an orthographic projection.
    ///
    pub fn new_orthographic(
        viewport: Viewport,
        position: Vec3,
        target: Vec3,
        up: Vec3,
        height: f32,
        z_near: f32,
        z_far: f32,
    ) -> Self {
        Self {
            camera: three_d_asset::Camera::new_orthographic(
                viewport, position, target, up, height, z_near, z_far,
            ),
            tone_mapping: ToneMapping::default(),
            target_color_space: ColorSpace::default(),
        }
    }

    ///
    /// New camera which projects the world with a perspective projection.
    ///
    pub fn new_perspective(
        viewport: Viewport,
        position: Vec3,
        target: Vec3,
        up: Vec3,
        field_of_view_y: impl Into<Radians>,
        z_near: f32,
        z_far: f32,
    ) -> Self {
        Self {
            camera: three_d_asset::Camera::new_perspective(
                viewport,
                position,
                target,
                up,
                field_of_view_y,
                z_near,
                z_far,
            ),
            tone_mapping: ToneMapping::default(),
            target_color_space: ColorSpace::default(),
        }
    }

    ///
    /// Returns an orthographic camera for viewing 2D content.
    /// The camera is placed at the center of the given viewport.
    /// The (0, 0) position is at the bottom left corner and the
    /// (`viewport.width`, `viewport.height`) position is at the top right corner.
    ///
    pub fn new_2d(viewport: Viewport) -> Self {
        Self::new_orthographic(
            viewport,
            vec3(
                viewport.width as f32 * 0.5,
                viewport.height as f32 * 0.5,
                1.0,
            ),
            vec3(
                viewport.width as f32 * 0.5,
                viewport.height as f32 * 0.5,
                0.0,
            ),
            vec3(0.0, 1.0, 0.0),
            viewport.height as f32,
            0.0,
            10.0,
        )
    }
}

use std::ops::Deref;
impl Deref for Camera {
    type Target = three_d_asset::Camera;
    fn deref(&self) -> &Self::Target {
        &self.camera
    }
}

impl std::ops::DerefMut for Camera {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.camera
    }
}
