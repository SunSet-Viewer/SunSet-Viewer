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

impl AvatarHitbox {
    /// Center and full dimensions of the upper head hitbox zone inside the
    /// simulator-reported avatar bounds (`+0.26 * size.z` to `+0.50 * size.z`).
    pub fn head_bounds(&self) -> (Vec3, Vec3) {
        let head_size = Vec3::new(self.size.x * 0.85, self.size.y * 0.85, self.size.z * 0.24);
        let head_center = self.position + self.rotation * Vec3::new(0.0, 0.0, self.size.z * 0.38);
        (head_center, head_size)
    }
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
        self.avatar_hitboxes_in(world, now, false)
    }

    fn avatar_hitboxes_in(&self, world: &World, now: Instant, visual: bool) -> Vec<AvatarHitbox> {
        world
            .objects
            .avatars()
            .filter_map(|(idx, object)| {
                let (position, rotation, _) = if visual {
                    Self::object_transform(world, idx, now, 0)?
                } else {
                    Self::object_transform_unshifted(world, idx, now)?
                };
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
        // `skeleton_hitboxes` evaluates in visual space (`object_transform`),
        // so evaluate companion head/zone boxes in visual space too.
        let head_boxes = self.avatar_hitboxes_in(world, now, true);
        let classify_zone = |avatar: Uuid, point: Vec3, raw_joint: &str| -> String {
            for hb in &head_boxes {
                if hb.avatar != avatar {
                    continue;
                }
                let rel = hb.rotation.inverse() * (point - hb.position);
                if rel.x.abs() <= hb.size.x * 0.45 && rel.y.abs() <= hb.size.y * 0.45 {
                    if rel.z >= hb.size.z * 0.12 && rel.z < hb.size.z * 0.26 {
                        if rel.y >= hb.size.y * 0.16 {
                            return "L_UPPER_ARM".to_owned();
                        }
                        if rel.y <= -hb.size.y * 0.16 {
                            return "R_UPPER_ARM".to_owned();
                        }
                        return "CHEST".to_owned();
                    }
                    if rel.z >= hb.size.z * 0.26 && rel.z <= hb.size.z * 0.50 {
                        return "HEAD".to_owned();
                    }
                }
            }
            raw_joint.to_owned()
        };
        let skel_hits = self
            .skeleton_hitboxes(world, now, Some((origin, direction, max_distance)))
            .into_iter()
            .filter(|(_, avatar, _)| *avatar != world.agent_id)
            .filter_map(|(joint, avatar, transform)| {
                let distance = hitbox_distance(origin, direction, max_distance, transform)?;
                let position = origin + direction * distance;
                let joint = classify_zone(avatar, position, joint);
                Some(AvatarHit {
                    avatar,
                    joint,
                    distance,
                    position,
                })
            });
        let head_box_hits = head_boxes.iter().filter(|hb| hb.avatar != world.agent_id).filter_map(|hb| {
            let (head_center, head_size) = hb.head_bounds();
            let transform = Mat4::from_scale_rotation_translation(head_size * 0.5, hb.rotation, head_center);
            let distance = hitbox_distance(origin, direction, max_distance, transform)?;
            Some(AvatarHit {
                avatar: hb.avatar,
                joint: "HEAD".to_owned(),
                distance,
                position: origin + direction * distance,
            })
        });
        skel_hits.chain(head_box_hits).min_by(|a, b| {
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
pub(crate) fn combat_hit(
    avatar: Option<AvatarHit>,
    scene: Option<CombatSceneHit>,
    origin: Vec3,
    direction: Vec3,
    max_distance: f32,
) -> CombatRayHit {
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

    #[test]
    fn head_bounds_target_upper_head_region() {
        let pos = Vec3::new(10.0, 20.0, 25.0);
        let rot = Quat::from_rotation_z(0.5);
        let size = Vec3::new(0.45, 0.60, 1.90);
        let hb = AvatarHitbox {
            avatar: Uuid::from_u128(1),
            position: pos,
            rotation: rot,
            size,
        };
        let (head_center, head_size) = hb.head_bounds();
        assert!((head_center - (pos + rot * Vec3::new(0.0, 0.0, size.z * 0.38))).length() < 1e-5);
        assert!((head_size - Vec3::new(size.x * 0.85, size.y * 0.85, size.z * 0.24)).length() < 1e-5);
    }

    #[test]
    fn unshifted_hitboxes_ignore_visual_shape_hover_water_and_keep_tilted_rotation() {
        let mut world = World::new(std::sync::Arc::new(crate::scene::avatar::AvatarLibrary::load()));
        world.agent_id = crate::demo::DEMO_AGENT;
        world.main_region = Some(1);
        let mut peer = crate::demo::poser::peer();
        let target_id = Uuid::from_u128(0x5001);
        peer.full_id = target_id;
        peer.local_id = 9901;
        peer.position = Vec3::new(128.0, 128.0, 25.0);
        peer.rotation = glam::Quat::from_rotation_x(0.45);
        world.apply(aurora_net::NetEvent::ObjectUpdates {
            handle: 1,
            objects: vec![peer],
        });

        let scene = Scene::new(Default::default(), world.avatar_lib.clone());
        let now = Instant::now();
        let base_hitbox = scene
            .avatar_hitboxes(&world, now)
            .into_iter()
            .find(|h| h.avatar == target_id)
            .expect("peer hitbox");
        let (base_head, _) = base_hitbox.head_bounds();

        // Add large viewer-only vertical shape offset: simulator hitboxes must stay unshifted.
        world.avatar_shape_dz.insert(target_id, 0.37);
        let shifted_hitbox = scene
            .avatar_hitboxes(&world, now)
            .into_iter()
            .find(|h| h.avatar == target_id)
            .expect("peer hitbox after shape offset");
        assert_eq!(base_hitbox.position, Vec3::new(128.0, 128.0, 25.0));
        assert_eq!(shifted_hitbox.position, base_hitbox.position);
        assert_eq!(shifted_hitbox.rotation, base_hitbox.rotation);

        // Viewer-space ray test evaluates the shifted visual avatar at its rendered height.
        let visual_hit = scene.avatar_hit_test_along(&world, base_head + Vec3::new(-4.0, 0.0, 0.37), Vec3::X, 10.0, now);
        assert_eq!(visual_hit.map(|h| h.avatar), Some(target_id));
    }
}
