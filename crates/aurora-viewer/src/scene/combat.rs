use super::{Scene, anchors};
use crate::settings::CombatSettings;
use crate::world::World;
use glam::{Mat4, Vec3};
use std::time::Instant;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct AvatarHitbox<'a> {
    pub avatar: Uuid,
    pub joint: &'a str,
    /// Unit cube transformed to the collision volume's world-space bounds.
    pub transform: Mat4,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AvatarHit {
    pub avatar: Uuid,
    pub joint: String,
    pub distance: f32,
    pub position: Vec3,
}

impl Scene {
    /// Collision volumes from the avatar skeleton, posed in world space.
    /// Avatars without an evaluated pose use their skeleton's rest pose.
    pub fn avatar_hitboxes<'a>(&self, world: &'a World, now: Instant) -> Vec<AvatarHitbox<'a>> {
        self.hitboxes(world, now, None)
    }

    fn hitboxes<'a>(&self, world: &'a World, now: Instant, ray: Option<(Vec3, Vec3, f32)>) -> Vec<AvatarHitbox<'a>> {
        let Some(skeleton) = world.avatar_lib.skeleton.as_ref() else {
            return Vec::new();
        };
        let rig = &world.avatar_lib.rig;
        let mut hitboxes = Vec::new();

        for (idx, object) in world.objects.iter() {
            if !object.is_avatar() {
                continue;
            }
            let Some((position, rotation, _)) = Self::object_transform(world, idx, now, 0) else {
                continue;
            };
            if let Some((origin, direction, max_distance)) = ray {
                let to_avatar = position - origin;
                let along = to_avatar.dot(direction);
                let radius = anchors::skeleton_extent(object);
                if along < -radius || along > max_distance + radius || to_avatar.length_squared() - along * along > radius * radius {
                    continue;
                }
            }
            let model = anchors::skeleton_model(world, &self.meshes, idx, position, rotation, 1.0);
            let pose = world.avatar_poses.get(&object.full_id);

            for (joint_idx, joint) in skeleton.joints.iter().enumerate() {
                if !joint.is_collision_volume {
                    continue;
                }
                if joint_idx >= rig.default_world_inv.len() {
                    continue;
                }
                let posed_delta = pose.and_then(|pose| pose.get(joint_idx)).copied().unwrap_or(Mat4::IDENTITY);
                let transform = model * posed_delta * joint.world;
                if transform.is_finite() && transform.determinant().abs() > 1e-8 {
                    hitboxes.push(AvatarHitbox {
                        avatar: object.full_id,
                        joint: &joint.name,
                        transform,
                    });
                }
            }
        }
        hitboxes
    }

    /// Nearest local avatar collision volume along a ray. This is a local
    /// hit test only; it does not account for world occlusion or server damage.
    pub fn avatar_hit_test(
        &self,
        world: &World,
        settings: &CombatSettings,
        origin: Vec3,
        direction: Vec3,
        max_distance: f32,
        now: Instant,
    ) -> Option<AvatarHit> {
        if !settings.local_hit_detection
            || !origin.is_finite()
            || !direction.is_finite()
            || !direction.length_squared().is_finite()
            || !max_distance.is_finite()
            || max_distance < 0.0
            || direction.length_squared() < 1e-12
        {
            return None;
        }
        let direction = direction.normalize();

        self.hitboxes(world, now, Some((origin, direction, max_distance)))
            .into_iter()
            .filter(|hitbox| hitbox.avatar != world.agent_id)
            .filter_map(|hitbox| {
                let distance = hitbox_distance(origin, direction, max_distance, hitbox.transform)?;
                Some(AvatarHit {
                    avatar: hitbox.avatar,
                    joint: hitbox.joint.to_owned(),
                    distance,
                    position: origin + direction * distance,
                })
            })
            .min_by(|a, b| {
                a.distance
                    .total_cmp(&b.distance)
                    .then_with(|| a.avatar.cmp(&b.avatar))
                    .then_with(|| a.joint.cmp(&b.joint))
            })
    }
}

fn hitbox_distance(origin: Vec3, direction: Vec3, max_distance: f32, transform: Mat4) -> Option<f32> {
    let inverse = transform.inverse();
    if !inverse.is_finite() {
        return None;
    }
    let local_origin = inverse.transform_point3(origin);
    let local_direction = inverse.transform_vector3(direction);
    let distance = super::picking::ray_box(local_origin, local_direction, Vec3::splat(-1.0), Vec3::ONE)?;
    (distance.is_finite() && distance <= max_distance).then_some(distance)
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::Quat;

    #[test]
    fn ray_hits_scaled_rotated_collision_volume_at_world_distance() {
        let transform =
            Mat4::from_scale_rotation_translation(Vec3::new(0.5, 1.0, 0.75), Quat::from_rotation_y(0.4), Vec3::new(0.0, 0.0, 5.0));
        let distance = hitbox_distance(Vec3::ZERO, Vec3::Z, 10.0, transform).unwrap();
        assert!((distance - (5.0 - 0.75 / 0.4f32.cos())).abs() < 0.02);
    }

    #[test]
    fn ray_misses_outside_volume_and_rejects_beyond_range() {
        let transform = Mat4::from_scale_rotation_translation(Vec3::splat(0.5), Quat::IDENTITY, Vec3::Z * 5.0);
        assert!(hitbox_distance(Vec3::new(2.0, 0.0, 0.0), Vec3::Z, 10.0, transform).is_none());
        assert!(hitbox_distance(Vec3::ZERO, Vec3::Z, 4.0, transform).is_none());
    }
}
