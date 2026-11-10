use super::picking::SceneRayHit;
use super::{Scene, anchors};
use crate::settings::CombatSettings;
use crate::world::World;
use glam::{Mat4, Quat, Vec3};
use std::time::Instant;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct AvatarHitbox {
    pub avatar: Uuid,
    /// Simulator-reported root position, before local avatar shape offsets.
    pub position: Vec3,
    pub rotation: Quat,
    /// Simulator-reported collision dimensions, before viewer-side animation.
    pub size: Vec3,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AvatarHit {
    pub avatar: Uuid,
    pub joint: String,
    pub distance: f32,
    pub position: Vec3,
}

/// A world surface or the terrain that may stop a combat ray.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CombatSceneHit {
    pub distance: f32,
    pub point: Vec3,
    pub object: Option<usize>,
    pub terrain: bool,
}

impl From<SceneRayHit> for CombatSceneHit {
    fn from(hit: SceneRayHit) -> Self {
        Self {
            distance: hit.distance,
            point: hit.point,
            object: hit.object,
            terrain: hit.terrain,
        }
    }
}

/// What the local combat ray meets first. `Miss` is the ray's end at
/// `max_distance`, for the overlay.
#[derive(Clone, Debug, PartialEq)]
pub enum CombatRayHit {
    Avatar(AvatarHit),
    Scene(CombatSceneHit),
    Miss { point: Vec3 },
}

/// The screen-center combat ray and its nearest local result.
#[derive(Clone, Debug)]
pub struct CombatRay {
    pub origin: Vec3,
    pub max_distance: f32,
    pub hit: CombatRayHit,
}

impl CombatRay {
    /// Where the ray stops: the hit point, or its end on a miss.
    pub fn point(&self) -> Vec3 {
        match &self.hit {
            CombatRayHit::Avatar(hit) => hit.position,
            CombatRayHit::Scene(hit) => hit.point,
            CombatRayHit::Miss { point } => *point,
        }
    }

    pub fn distance(&self) -> f32 {
        match &self.hit {
            CombatRayHit::Avatar(hit) => hit.distance,
            CombatRayHit::Scene(hit) => hit.distance,
            CombatRayHit::Miss { .. } => self.max_distance,
        }
    }
}

impl Scene {
    /// Simulator-reported avatar bounds, matching DebugRenderHitboxes
    /// (indra/newview/lldrawpoolavatar.cpp, originally LGPL 2.1). This box is
    /// independent of viewer-side animation and is not a damage simulation.
    pub fn avatar_hitboxes(&self, world: &World, now: Instant) -> Vec<AvatarHitbox> {
        world
            .objects
            .avatars()
            .filter_map(|(idx, object)| {
                let (position, rotation, _) = Self::object_transform_unshifted(world, idx, now)?;
                let size = object.scale;
                (position.is_finite() && rotation.is_finite() && size.is_finite() && size.min_element() > 0.0).then_some(AvatarHitbox {
                    avatar: object.full_id,
                    position,
                    rotation,
                    size,
                })
            })
            .collect()
    }

    fn skeleton_hitboxes<'a>(&self, world: &'a World, now: Instant, ray: Option<(Vec3, Vec3, f32)>) -> Vec<(&'a str, Uuid, Mat4)> {
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
                    hitboxes.push((joint.name.as_str(), object.full_id, transform));
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
        if !settings.local_hit_detection || !valid_ray(origin, direction, max_distance) {
            return None;
        }
        self.avatar_hit_test_along(world, origin, direction.normalize(), max_distance, now)
    }

    /// The local combat ray: the avatar skeleton test plus, when
    /// `settings.scene_objects_block_rays` is on, the nearest world surface
    /// or terrain. Local only: no hit event leaves the viewer and the
    /// simulator keeps its own weapon collision.
    pub fn combat_ray(
        &self,
        world: &World,
        settings: &CombatSettings,
        origin: Vec3,
        direction: Vec3,
        max_distance: f32,
        now: Instant,
    ) -> Option<CombatRay> {
        if !valid_ray(origin, direction, max_distance) {
            return None;
        }
        let direction = direction.normalize();
        let scene = settings
            .scene_objects_block_rays
            .then(|| self.scene_ray_hit(world, (origin, direction), max_distance, now))
            .flatten()
            .map(CombatSceneHit::from);
        let avatar = self.avatar_hit_test_along(world, origin, direction, scene.map_or(max_distance, |hit| hit.distance), now);
        Some(CombatRay {
            origin,
            max_distance,
            hit: combat_hit(avatar, scene, origin, direction, max_distance),
        })
    }

    /// `avatar_hit_test` without the preference gate: the raycast diagnostic
    /// reports avatar hits even when only `show_raycast` is on.
    fn avatar_hit_test_along(&self, world: &World, origin: Vec3, direction: Vec3, max_distance: f32, now: Instant) -> Option<AvatarHit> {
        self.skeleton_hitboxes(world, now, Some((origin, direction, max_distance)))
            .into_iter()
            .filter(|(_, avatar, _)| *avatar != world.agent_id)
            .filter_map(|(joint, avatar, transform)| {
                let distance = hitbox_distance(origin, direction, max_distance, transform)?;
                Some(AvatarHit {
                    avatar,
                    joint: joint.to_owned(),
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

fn valid_ray(origin: Vec3, direction: Vec3, max_distance: f32) -> bool {
    origin.is_finite()
        && direction.is_finite()
        && direction.length_squared().is_finite()
        && max_distance.is_finite()
        && max_distance >= 0.0
        && direction.length_squared() >= 1e-12
}

/// Nearest of the avatar and world hits; a world surface at the same
/// distance wins, like the ray meeting the wall before the body behind it.
pub(crate) fn combat_hit(avatar: Option<AvatarHit>, scene: Option<CombatSceneHit>, origin: Vec3, direction: Vec3, max_distance: f32) -> CombatRayHit {
    match (avatar, scene) {
        (Some(avatar), Some(scene)) if avatar.distance < scene.distance => CombatRayHit::Avatar(avatar),
        (_, Some(scene)) => CombatRayHit::Scene(scene),
        (Some(avatar), None) => CombatRayHit::Avatar(avatar),
        (None, None) => CombatRayHit::Miss {
            point: origin + direction * max_distance,
        },
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

    fn avatar_hit(distance: f32) -> AvatarHit {
        AvatarHit {
            avatar: Uuid::nil(),
            joint: "Chest".to_owned(),
            distance,
            position: Vec3::Z * distance,
        }
    }

    fn scene_hit(distance: f32, terrain: bool) -> CombatSceneHit {
        CombatSceneHit {
            distance,
            point: Vec3::Z * distance,
            object: (!terrain).then_some(0),
            terrain,
        }
    }

    #[test]
    fn combat_ray_rejects_invalid_rays() {
        assert!(!valid_ray(Vec3::ZERO, Vec3::ZERO, 10.0));
        assert!(!valid_ray(Vec3::ZERO, Vec3::Z, -1.0));
        assert!(!valid_ray(Vec3::new(f32::NAN, 0.0, 0.0), Vec3::Z, 10.0));
        assert!(valid_ray(Vec3::ZERO, Vec3::Z, 0.0));
    }

    #[test]
    fn combat_ray_reports_the_avatar_when_scene_blocking_is_off() {
        // With `scene_objects_block_rays` off the world query returns no
        // scene hit, so the avatar behind the wall is still the result.
        assert!(matches!(
            combat_hit(Some(avatar_hit(8.0)), None, Vec3::ZERO, Vec3::Z, 10.0),
            CombatRayHit::Avatar(hit) if hit.distance == 8.0
        ));
    }

    #[test]
    fn combat_ray_prefers_the_nearest_hit_and_lets_the_wall_win_a_tie() {
        let ray = (Vec3::ZERO, Vec3::Z, 10.0);
        assert!(matches!(
            combat_hit(Some(avatar_hit(2.0)), Some(scene_hit(5.0, false)), ray.0, ray.1, ray.2),
            CombatRayHit::Avatar(_)
        ));
        assert!(matches!(
            combat_hit(Some(avatar_hit(7.0)), Some(scene_hit(3.0, true)), ray.0, ray.1, ray.2),
            CombatRayHit::Scene(hit) if hit.terrain
        ));
        assert!(matches!(
            combat_hit(Some(avatar_hit(4.0)), Some(scene_hit(4.0, false)), ray.0, ray.1, ray.2),
            CombatRayHit::Scene(_)
        ));
        assert!(matches!(combat_hit(None, None, ray.0, ray.1, ray.2), CombatRayHit::Miss { point } if point == Vec3::Z * 10.0));
    }
}
