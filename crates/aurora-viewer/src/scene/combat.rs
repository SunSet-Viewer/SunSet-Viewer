use super::picking::SceneRayHit;
use super::{Scene, anchors};
use crate::settings::{CombatAimMode, CombatSettings, Rgba};
use crate::world::World;
use glam::{Mat4, Quat, Vec3};
use std::collections::HashMap;
use std::time::Instant;
use uuid::Uuid;

pub const AUTO_AIM_HEAD_SHOTS: u32 = 5;
pub const AUTO_AIM_CHEST_SHOTS: u32 = 1;
pub const AUTO_AIM_CYCLE_SHOTS: u32 = AUTO_AIM_HEAD_SHOTS + AUTO_AIM_CHEST_SHOTS;

const COMBAT_LEAD_MAX_SECONDS: f32 = 0.35;
const COMBAT_LEAD_MAX_OFFSET: f32 = 3.0;
const COMBAT_LEAD_SPEED_DEADBAND: f32 = 0.15;
const COMBAT_LEAD_ACCEL_DEADBAND: f32 = 0.5;
const COLOR_MATCH_TOLERANCE: u8 = 2;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CombatAimPart {
    Head,
    Chest,
}

/// Returns which body region to target for `shot_index` in the repeating
/// `5 Head -> 1 Chest` cycle (never arms).
pub fn combat_aim_part(shot_index: u32) -> CombatAimPart {
    if shot_index % AUTO_AIM_CYCLE_SHOTS < AUTO_AIM_HEAD_SHOTS {
        CombatAimPart::Head
    } else {
        CombatAimPart::Chest
    }
}

/// Returns the target region for `shot_index` respecting the configured
/// [`CombatAimMode`].
pub fn combat_aim_part_for_mode(shot_index: u32, mode: CombatAimMode) -> CombatAimPart {
    match mode {
        CombatAimMode::HeadAndBody => combat_aim_part(shot_index),
        CombatAimMode::HeadOnly => CombatAimPart::Head,
    }
}

/// Converts live round-trip ping (`ping_ms`) and the user's multiplier into
/// forward prediction seconds for moving targets.
pub fn combat_lead_seconds(ping_ms: u32, lead_compensation: f32) -> f32 {
    if !lead_compensation.is_finite() || lead_compensation <= 0.0 {
        return 0.0;
    }
    let base_seconds = if ping_ms > 0 { ping_ms as f32 * 0.001 * 0.85 + 0.025 } else { 0.03 };
    (base_seconds * lead_compensation.clamp(0.0, 2.0)).clamp(0.0, COMBAT_LEAD_MAX_SECONDS)
}

/// Friendly-fire exclusion policy applied during target discovery, cycling, and
/// active lock maintenance.
#[derive(Clone, Copy, Debug)]
pub struct CombatTargetFilter<'a> {
    pub enabled: bool,
    pub preset: Option<&'a str>,
    pub excluded_color: Rgba,
    pub minimap_marks: Option<&'a HashMap<Uuid, egui::Color32>>,
}

impl<'a> CombatTargetFilter<'a> {
    #[cfg(test)]
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            preset: None,
            excluded_color: crate::settings::DEFAULT_COMBAT_FRIENDLY_FIRE_COLOR,
            minimap_marks: None,
        }
    }

    pub fn from_settings(settings: &'a CombatSettings, minimap_marks: Option<&'a HashMap<Uuid, egui::Color32>>) -> Self {
        Self {
            enabled: settings.friendly_fire_protection,
            preset: settings.friendly_fire_preset.as_deref(),
            excluded_color: settings.friendly_fire_color,
            minimap_marks,
        }
    }

    /// Returns `true` when `avatar` should be skipped to prevent friendly fire:
    /// checks membership in `preset` when configured and present in
    /// `world.contact_sets`, otherwise matches the resolved hitbox RGB colour
    /// against `excluded_color`.
    pub fn is_excluded(&self, world: &World, avatar: &Uuid) -> bool {
        if !self.enabled {
            return false;
        }
        if let Some(name) = self.preset.map(str::trim).filter(|name| !name.is_empty())
            && let Some(set) = world.contact_sets.set(name)
        {
            return set.friends.contains(avatar);
        }
        let resolved = crate::ui::minimap::avatar_hitbox_color(world, self.minimap_marks, avatar);
        let adapted = crate::ui::colors::c(self.excluded_color);
        let raw = egui::Color32::from_rgb(self.excluded_color[0], self.excluded_color[1], self.excluded_color[2]);
        rgb_matches(resolved, adapted) || rgb_matches(resolved, raw)
    }
}

fn rgb_matches(a: egui::Color32, b: egui::Color32) -> bool {
    a.r().abs_diff(b.r()) <= COLOR_MATCH_TOLERANCE
        && a.g().abs_diff(b.g()) <= COLOR_MATCH_TOLERANCE
        && a.b().abs_diff(b.b()) <= COLOR_MATCH_TOLERANCE
}

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

    /// Precise head aim point centered high inside the simulator head zone
    /// (`0.39..=0.45 * size.z`, default `0.42 * size.z`) and tightly clamped to
    /// the central spine axis so idle animations do not deflect into arms.
    pub fn head_aim_point(&self, posed_head: Option<Vec3>) -> Vec3 {
        let inv_rot = self.rotation.inverse();
        let default_local = Vec3::new(0.0, 0.0, self.size.z * 0.42);
        let local = if let Some(head) = posed_head.filter(|p| p.is_finite()) {
            let rel = inv_rot * (head - self.position);
            Vec3::new(
                rel.x.clamp(-self.size.x * 0.06, self.size.x * 0.06),
                rel.y.clamp(-self.size.y * 0.06, self.size.y * 0.06),
                rel.z.clamp(self.size.z * 0.39, self.size.z * 0.45),
            )
        } else {
            default_local
        };
        self.position + self.rotation * local
    }

    /// Center-chest aim point inside the simulator physics torso zone
    /// (`+0.14 * size.z` to `+0.22 * size.z`), tightly centered on the spine
    /// axis to stay clear of upper-arm hit regions.
    pub fn chest_aim_point(&self, posed_chest: Option<Vec3>) -> Vec3 {
        let inv_rot = self.rotation.inverse();
        let default_local = Vec3::new(0.0, 0.0, self.size.z * 0.18);
        let local = if let Some(chest) = posed_chest.filter(|p| p.is_finite()) {
            let rel = inv_rot * (chest - self.position);
            Vec3::new(
                rel.x.clamp(-self.size.x * 0.08, self.size.x * 0.08),
                rel.y.clamp(-self.size.y * 0.08, self.size.y * 0.08),
                rel.z.clamp(self.size.z * 0.14, self.size.z * 0.22),
            )
        } else {
            default_local
        };
        self.position + self.rotation * local
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

    fn avatar_collision_volume_center(&self, world: &World, avatar: Uuid, volume_name: &str, now: Instant) -> Option<Vec3> {
        let index = world.objects.index_of_uuid(&avatar)?;
        let (position, rotation, _) = Self::object_transform_unshifted(world, index, now)?;
        let skeleton = world.avatar_lib.skeleton.as_ref()?;
        let rig = &world.avatar_lib.rig;
        let model = anchors::skeleton_model(world, &self.meshes, index, position, rotation, 1.0);
        let pose = world.avatar_poses.get(&avatar);
        for (joint_idx, joint) in skeleton.joints.iter().enumerate() {
            if joint.is_collision_volume && joint.name == volume_name && joint_idx < rig.default_world_inv.len() {
                let posed_delta = pose.and_then(|pose| pose.get(joint_idx)).copied().unwrap_or(Mat4::IDENTITY);
                let transform = model * posed_delta * joint.world;
                if transform.is_finite() && transform.determinant().abs() > 1e-8 {
                    let center = transform.transform_point3(Vec3::ZERO);
                    if center.is_finite() {
                        return Some(center);
                    }
                }
            }
        }
        None
    }

    /// World-space center of an avatar's posed `"HEAD"` collision volume (or
    /// fallback posed `"mHead"` joint raised toward skull center) in
    /// unshifted simulator coordinates.
    pub fn avatar_head_center(&self, world: &World, avatar: Uuid, now: Instant) -> Option<Vec3> {
        if let Some(center) = self.avatar_collision_volume_center(world, avatar, "HEAD", now) {
            return Some(center);
        }
        let index = world.objects.index_of_uuid(&avatar)?;
        let (_, rotation, _) = Self::object_transform_unshifted(world, index, now)?;
        self.joint_world_unshifted(world, &avatar, "mHead", now)
            .map(|p| p + rotation * Vec3::Z * 0.10)
    }

    /// World-space center of an avatar's posed `"CHEST"` collision volume (or
    /// fallback posed `"mChest"` / `"mTorso"` joint) in unshifted simulator
    /// coordinates.
    pub fn avatar_chest_center(&self, world: &World, avatar: Uuid, now: Instant) -> Option<Vec3> {
        self.avatar_collision_volume_center(world, avatar, "CHEST", now)
            .or_else(|| self.joint_world_unshifted(world, &avatar, "mChest", now))
            .or_else(|| self.joint_world_unshifted(world, &avatar, "mTorso", now))
    }

    /// Precise head target coordinate in world space for `hitbox`.
    pub fn avatar_head_aim_point(&self, world: &World, hitbox: &AvatarHitbox, now: Instant) -> Vec3 {
        hitbox.head_aim_point(self.avatar_head_center(world, hitbox.avatar, now))
    }

    /// Chest target coordinate in world space for `hitbox`.
    pub fn avatar_chest_aim_point(&self, world: &World, hitbox: &AvatarHitbox, now: Instant) -> Vec3 {
        hitbox.chest_aim_point(self.avatar_chest_center(world, hitbox.avatar, now))
    }

    /// Velocity/acceleration lead displacement for a moving target evaluated at
    /// `lead_seconds` ahead of current client prediction.
    pub fn combat_lead_offset(world: &World, avatar: Uuid, lead_seconds: f32) -> Vec3 {
        if !lead_seconds.is_finite() || lead_seconds <= 0.0 {
            return Vec3::ZERO;
        }
        let Some(index) = world.objects.index_of_uuid(&avatar) else {
            return Vec3::ZERO;
        };
        let Some(object) = world.objects.get(index) else {
            return Vec3::ZERO;
        };
        if world.objects.held_pose(object).is_some() {
            return Vec3::ZERO;
        }
        // Seated avatars ride their linkset root vehicle; resolve up the parent
        // chain so vehicle velocity/acceleration drives lead when seat vel == 0.
        let mut mover = object;
        for _ in 0..16 {
            if mover.parent_id == 0 {
                break;
            }
            let Some(parent_idx) = world.objects.parent_of(mover) else {
                return Vec3::ZERO;
            };
            let Some(parent) = world.objects.get(parent_idx) else {
                return Vec3::ZERO;
            };
            if world.objects.held_pose(parent).is_some() {
                return Vec3::ZERO;
            }
            mover = parent;
        }
        if mover.parent_id != 0 {
            return Vec3::ZERO;
        }
        let rate = world.objects.motion_rate(mover.key.region);
        if !rate.is_finite() || rate <= 0.0 {
            return Vec3::ZERO;
        }
        let (raw_vel, raw_acc) = mover.linear_motion();
        let v0 = if raw_vel.is_finite() { raw_vel } else { Vec3::ZERO };
        let mut acc = if raw_acc.is_finite() { raw_acc } else { Vec3::ZERO };
        // Walking avatars on SL frequently carry gravity acceleration along -Z
        // while grounded; suppress vertical acceleration lead BEFORE integrating
        // elapsed time so grounded -9.8 m/s^2 cannot manufacture downward speed.
        if v0.z.abs() < 0.2 {
            acc.z = 0.0;
        }
        let elapsed = mover.motion_elapsed(world.objects.motion_time(mover.key.region));
        let vel = v0 + acc * elapsed;
        if vel.length_squared() < COMBAT_LEAD_SPEED_DEADBAND * COMBAT_LEAD_SPEED_DEADBAND
            && acc.length_squared() < COMBAT_LEAD_ACCEL_DEADBAND * COMBAT_LEAD_ACCEL_DEADBAND
        {
            return Vec3::ZERO;
        }
        let t = (lead_seconds * rate).clamp(0.0, COMBAT_LEAD_MAX_SECONDS);
        let offset = vel * t + acc * (0.5 * t * t);
        if !offset.is_finite() {
            return Vec3::ZERO;
        }
        let len = offset.length();
        if len > COMBAT_LEAD_MAX_OFFSET {
            offset * (COMBAT_LEAD_MAX_OFFSET / len)
        } else {
            offset
        }
    }

    /// Target coordinate in world space for `hitbox` according to `shot_index`,
    /// configured `mode`, and moving-target latency `lead_seconds`.
    pub fn avatar_combat_aim_point_with_lead(
        &self,
        world: &World,
        hitbox: &AvatarHitbox,
        shot_index: u32,
        mode: CombatAimMode,
        lead_seconds: f32,
        now: Instant,
    ) -> Vec3 {
        let base = match combat_aim_part_for_mode(shot_index, mode) {
            CombatAimPart::Head => self.avatar_head_aim_point(world, hitbox, now),
            CombatAimPart::Chest => self.avatar_chest_aim_point(world, hitbox, now),
        };
        base + Self::combat_lead_offset(world, hitbox.avatar, lead_seconds)
    }

    /// Target coordinate in world space for `hitbox` according to the active
    /// `shot_index` in the `5 Head -> 1 Chest` cycle (without lead).
    #[cfg(test)]
    pub fn avatar_combat_aim_point(&self, world: &World, hitbox: &AvatarHitbox, shot_index: u32, now: Instant) -> Vec3 {
        self.avatar_combat_aim_point_with_lead(world, hitbox, shot_index, CombatAimMode::HeadAndBody, 0.0, now)
    }

    /// Other eligible avatars within `max_range` meters of `origin`, sorted
    /// nearest-first and excluding allies matching `filter`.
    pub fn combat_targets_in_range_with_filter(
        &self,
        world: &World,
        origin: Vec3,
        max_range: f32,
        filter: &CombatTargetFilter<'_>,
        now: Instant,
    ) -> Vec<AvatarHitbox> {
        if !origin.is_finite() || !max_range.is_finite() || max_range <= 0.0 {
            return Vec::new();
        }
        let max_sq = max_range * max_range;
        let mut targets: Vec<AvatarHitbox> = self
            .avatar_hitboxes(world, now)
            .into_iter()
            .filter(|hb| {
                hb.avatar != world.agent_id && hb.position.distance_squared(origin) <= max_sq && !filter.is_excluded(world, &hb.avatar)
            })
            .collect();
        targets.sort_by(|a, b| {
            a.position
                .distance_squared(origin)
                .total_cmp(&b.position.distance_squared(origin))
                .then_with(|| a.avatar.cmp(&b.avatar))
        });
        targets
    }

    /// Other avatars within `max_range` meters of `origin`, sorted nearest-first.
    #[cfg(test)]
    pub fn combat_targets_in_range(&self, world: &World, origin: Vec3, max_range: f32, now: Instant) -> Vec<AvatarHitbox> {
        self.combat_targets_in_range_with_filter(world, origin, max_range, &CombatTargetFilter::disabled(), now)
    }

    /// Select the nearest eligible target within `max_range`, or advance to the
    /// next-nearest eligible target when `current` is already selected.
    pub fn cycle_combat_target_with_filter(
        &self,
        world: &World,
        origin: Vec3,
        max_range: f32,
        current: Option<Uuid>,
        filter: &CombatTargetFilter<'_>,
        now: Instant,
    ) -> Option<Uuid> {
        let targets = self.combat_targets_in_range_with_filter(world, origin, max_range, filter, now);
        if targets.is_empty() {
            return None;
        }
        if let Some(cur) = current
            && let Some(idx) = targets.iter().position(|hb| hb.avatar == cur)
        {
            return Some(targets[(idx + 1) % targets.len()].avatar);
        }
        Some(targets[0].avatar)
    }

    /// Select the nearest combat target within `max_range`, or advance to the
    /// next-nearest target when `current` is already selected (wrapping at end).
    #[cfg(test)]
    pub fn cycle_combat_target(&self, world: &World, origin: Vec3, max_range: f32, current: Option<Uuid>, now: Instant) -> Option<Uuid> {
        self.cycle_combat_target_with_filter(world, origin, max_range, current, &CombatTargetFilter::disabled(), now)
    }

    /// Current hitbox of `target` if still present, within `max_range` meters,
    /// and not excluded by friendly-fire `filter`.
    pub fn combat_target_hitbox_with_filter(
        &self,
        world: &World,
        target: Uuid,
        origin: Vec3,
        max_range: f32,
        filter: &CombatTargetFilter<'_>,
        now: Instant,
    ) -> Option<AvatarHitbox> {
        if target == world.agent_id
            || !origin.is_finite()
            || !max_range.is_finite()
            || max_range <= 0.0
            || filter.is_excluded(world, &target)
        {
            return None;
        }
        let max_sq = max_range * max_range;
        self.avatar_hitboxes(world, now)
            .into_iter()
            .find(|hb| hb.avatar == target && hb.position.distance_squared(origin) <= max_sq)
    }

    /// Current hitbox of `target` if still present and within `max_range` meters of `origin`.
    #[cfg(test)]
    pub fn combat_target_hitbox(&self, world: &World, target: Uuid, origin: Vec3, max_range: f32, now: Instant) -> Option<AvatarHitbox> {
        self.combat_target_hitbox_with_filter(world, target, origin, max_range, &CombatTargetFilter::disabled(), now)
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
    fn head_bounds_and_aim_point_target_upper_head_region() {
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

        // Fallback aim point sits centered high inside the simulator skull region.
        let default_pt = hb.head_aim_point(None);
        let default_local = rot.inverse() * (default_pt - pos);
        assert!((default_local.z - size.z * 0.42).abs() < 1e-5);
        assert!(default_local.z >= size.z * 0.39 && default_local.z <= size.z * 0.45);

        // Extreme out-of-bounds posed head coordinates clamp tightly onto the spine core.
        let clamped_pt = hb.head_aim_point(Some(pos + rot * Vec3::new(4.0, -4.0, 10.0)));
        let clamped_local = rot.inverse() * (clamped_pt - pos);
        assert!(clamped_local.x <= size.x * 0.06 + 1e-5 && clamped_local.x >= -size.x * 0.06 - 1e-5);
        assert!(clamped_local.y <= size.y * 0.06 + 1e-5 && clamped_local.y >= -size.y * 0.06 - 1e-5);
        assert!((clamped_local.z - size.z * 0.45).abs() < 1e-5);
    }

    #[test]
    fn combat_aim_part_repeats_five_head_one_chest_without_arm() {
        let expected = [
            CombatAimPart::Head,
            CombatAimPart::Head,
            CombatAimPart::Head,
            CombatAimPart::Head,
            CombatAimPart::Head,
            CombatAimPart::Chest,
            CombatAimPart::Head,
            CombatAimPart::Head,
            CombatAimPart::Head,
            CombatAimPart::Head,
            CombatAimPart::Head,
            CombatAimPart::Chest,
        ];
        for (shot, part) in expected.into_iter().enumerate() {
            assert_eq!(combat_aim_part(shot as u32), part, "unexpected part at shot {shot}");
            assert_eq!(
                combat_aim_part_for_mode(shot as u32, CombatAimMode::HeadOnly),
                CombatAimPart::Head,
                "head-only mode must always select Head at shot {shot}"
            );
        }
    }

    #[test]
    fn chest_aim_point_clamps_tightly_inside_central_torso() {
        let pos = Vec3::new(10.0, 20.0, 25.0);
        let rot = Quat::from_rotation_z(0.5);
        let size = Vec3::new(0.45, 0.60, 1.90);
        let hb = AvatarHitbox {
            avatar: Uuid::from_u128(2),
            position: pos,
            rotation: rot,
            size,
        };

        let default_chest = rot.inverse() * (hb.chest_aim_point(None) - pos);
        assert!(default_chest.x.abs() < 1e-5 && default_chest.y.abs() < 1e-5);
        assert!((default_chest.z - size.z * 0.18).abs() < 1e-5);

        let clamped_chest = rot.inverse() * (hb.chest_aim_point(Some(pos + rot * Vec3::new(4.0, -4.0, 10.0))) - pos);
        assert!((clamped_chest.x - size.x * 0.08).abs() < 1e-5);
        assert!((clamped_chest.y + size.y * 0.08).abs() < 1e-5);
        assert!((clamped_chest.z - size.z * 0.22).abs() < 1e-5);
    }

    #[test]
    fn lead_prediction_leads_moving_targets_and_ignores_grounded_gravity() {
        let mut world = World::new(std::sync::Arc::new(crate::scene::avatar::AvatarLibrary::load()));
        world.agent_id = crate::demo::DEMO_AGENT;
        world.main_region = Some(1);
        let mut peer = crate::demo::poser::peer();
        let target_id = Uuid::from_u128(0x3001);
        peer.full_id = target_id;
        peer.local_id = 9850;
        // Nearly stationary target stays deadbanded to zero offset.
        peer.velocity = Vec3::new(0.05, 0.0, 0.0);
        peer.acceleration = Vec3::ZERO;
        world.apply(aurora_net::NetEvent::ObjectUpdates {
            handle: 1,
            objects: vec![peer.clone()],
        });
        let lead_s = combat_lead_seconds(120, 1.0);
        assert!(lead_s > 0.10 && lead_s < 0.20);
        assert_eq!(combat_lead_seconds(120, 0.0), 0.0);
        assert_eq!(Scene::combat_lead_offset(&world, target_id, lead_s), Vec3::ZERO);

        // Moving target running sideways with spurious grounded -9.8 Z acceleration
        // must lead horizontally without pulling head aim downwards.
        peer.velocity = Vec3::new(0.0, 4.0, 0.0);
        peer.acceleration = Vec3::new(0.0, 0.0, -9.8);
        world.apply(aurora_net::NetEvent::ObjectUpdates {
            handle: 1,
            objects: vec![peer],
        });
        let offset = Scene::combat_lead_offset(&world, target_id, 0.15);
        assert!((offset.y - 0.60).abs() < 1e-5, "expected +0.60 m lateral lead, got {offset:?}");
        assert!(offset.z.abs() < 1e-5, "grounded gravity must not pull vertical lead down");
    }

    #[test]
    fn friendly_fire_filter_excludes_preset_or_predefined_hitbox_color() {
        let mut world = World::new(std::sync::Arc::new(crate::scene::avatar::AvatarLibrary::load()));
        world.agent_id = crate::demo::DEMO_AGENT;
        world.main_region = Some(1);
        let self_av = crate::demo::action_avatar(false);
        let origin = self_av.position;

        let spawn_peer = |idx: u128, dist: f32, color_name: &str| {
            let mut o = crate::demo::poser::peer();
            o.full_id = Uuid::from_u128(0x4000 + idx);
            o.local_id = 9860 + idx as u32;
            o.position = origin + Vec3::new(dist, 0.0, 0.0);
            o.name_values = format!("FirstName STRING RW SV {color_name}\nLastName STRING RW SV Tester");
            o
        };
        let ally_mafia = spawn_peer(1, 10.0, "MafiaAlly");
        let enemy = spawn_peer(2, 20.0, "EnemyTarget");
        let friend_default = spawn_peer(3, 15.0, "RegularFriend");
        let id_mafia = ally_mafia.full_id;
        let id_enemy = enemy.full_id;
        let id_friend = friend_default.full_id;

        world.apply(aurora_net::NetEvent::ObjectUpdates {
            handle: 1,
            objects: vec![self_av, ally_mafia, friend_default, enemy],
        });

        world.contact_sets.add_set("Mafia");
        world.contact_sets.set_color("Mafia", [0.1, 0.4, 0.95, 1.0]);
        world.contact_sets.add_to_set(&[id_mafia], "Mafia", |_| false);
        world.social.friends.push(crate::world::social::Friend {
            id: id_friend,
            online: true,
            rights_given: 0,
            rights_has: 0,
        });

        let scene = Scene::new(Default::default(), world.avatar_lib.clone());
        let now = Instant::now();

        // When preset "Mafia" is selected, only members of "Mafia" are skipped.
        let preset_settings = CombatSettings {
            friendly_fire_protection: true,
            friendly_fire_preset: Some("Mafia".to_owned()),
            ..CombatSettings::default()
        };
        let preset_filter = CombatTargetFilter::from_settings(&preset_settings, None);
        let ids_by_preset: Vec<Uuid> = scene
            .combat_targets_in_range_with_filter(&world, origin, 100.0, &preset_filter, now)
            .into_iter()
            .map(|h| h.avatar)
            .collect();
        assert_eq!(ids_by_preset, vec![id_friend, id_enemy]);
        assert!(
            scene
                .combat_target_hitbox_with_filter(&world, id_mafia, origin, 100.0, &preset_filter, now)
                .is_none()
        );

        // When no preset is chosen, default predefined ally color (#818CF8 / map_friend)
        // excludes friends, and a custom avatar_marks color can also exclude another hitbox.
        let color_settings = CombatSettings {
            friendly_fire_protection: true,
            friendly_fire_preset: None,
            ..CombatSettings::default()
        };
        let color_filter = CombatTargetFilter::from_settings(&color_settings, None);
        let ids_by_color: Vec<Uuid> = scene
            .combat_targets_in_range_with_filter(&world, origin, 100.0, &color_filter, now)
            .into_iter()
            .map(|h| h.avatar)
            .collect();
        assert!(!ids_by_color.contains(&id_friend));
        assert!(ids_by_color.contains(&id_enemy));

        // Marking `id_enemy` with the protected color immediately unlocks it.
        world.avatar_marks.insert(
            id_enemy,
            egui::Color32::from_rgb(
                color_settings.friendly_fire_color[0],
                color_settings.friendly_fire_color[1],
                color_settings.friendly_fire_color[2],
            ),
        );
        assert!(
            scene
                .combat_target_hitbox_with_filter(&world, id_enemy, origin, 100.0, &color_filter, now)
                .is_none()
        );

        // Disabling friendly-fire protection restores targeting all peers.
        let off_settings = CombatSettings {
            friendly_fire_protection: false,
            ..preset_settings
        };
        let off_filter = CombatTargetFilter::from_settings(&off_settings, None);
        let all_ids: Vec<Uuid> = scene
            .combat_targets_in_range_with_filter(&world, origin, 100.0, &off_filter, now)
            .into_iter()
            .map(|h| h.avatar)
            .collect();
        assert_eq!(all_ids, vec![id_mafia, id_friend, id_enemy]);
    }

    #[test]
    fn cycle_combat_target_sorts_by_distance_within_100m_and_wraps() {
        let mut world = World::new(std::sync::Arc::new(crate::scene::avatar::AvatarLibrary::load()));
        world.agent_id = crate::demo::DEMO_AGENT;
        world.main_region = Some(1);
        let self_av = crate::demo::action_avatar(false);
        let origin = self_av.position;

        let spawn_peer = |idx: u128, dist: f32| {
            let mut o = crate::demo::poser::peer();
            o.full_id = Uuid::from_u128(0x1000 + idx);
            o.local_id = 9800 + idx as u32;
            o.position = origin + Vec3::new(dist, 0.0, 0.0);
            o
        };
        let av_85 = spawn_peer(1, 85.0);
        let av_15 = spawn_peer(2, 15.0);
        let av_130 = spawn_peer(3, 130.0);
        let av_40 = spawn_peer(4, 40.0);

        let (id_85, id_15, id_130, id_40) = (av_85.full_id, av_15.full_id, av_130.full_id, av_40.full_id);
        world.apply(aurora_net::NetEvent::ObjectUpdates {
            handle: 1,
            objects: vec![self_av, av_85, av_15, av_130, av_40],
        });

        let scene = Scene::new(Default::default(), world.avatar_lib.clone());
        let now = Instant::now();
        let in_range: Vec<Uuid> = scene
            .combat_targets_in_range(&world, origin, 100.0, now)
            .into_iter()
            .map(|h| h.avatar)
            .collect();
        assert_eq!(in_range, vec![id_15, id_40, id_85]);
        assert!(!in_range.contains(&id_130));
        assert!(!in_range.contains(&world.agent_id));

        assert_eq!(scene.cycle_combat_target(&world, origin, 100.0, None, now), Some(id_15));
        assert_eq!(scene.cycle_combat_target(&world, origin, 100.0, Some(id_15), now), Some(id_40));
        assert_eq!(scene.cycle_combat_target(&world, origin, 100.0, Some(id_40), now), Some(id_85));
        assert_eq!(scene.cycle_combat_target(&world, origin, 100.0, Some(id_85), now), Some(id_15));
        // Out-of-range previous target resets to the nearest valid target.
        assert_eq!(scene.cycle_combat_target(&world, origin, 100.0, Some(id_130), now), Some(id_15));
    }

    #[test]
    fn unshifted_aim_ignores_visual_shape_hover_water_and_keeps_tilted_rotation() {
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
        let base_head = scene.avatar_head_center(&world, target_id, now).expect("base head");
        let base_chest = scene.avatar_chest_center(&world, target_id, now).expect("base chest");

        // Add large viewer-only vertical shape offset: neither unshifted hitboxes
        // nor unshifted collision-volume / joint aim centers may move by shape_dz.
        world.avatar_shape_dz.insert(target_id, 0.37);
        let shifted_head = scene.avatar_head_center(&world, target_id, now).expect("shifted head");
        let shifted_chest = scene.avatar_chest_center(&world, target_id, now).expect("shifted chest");
        assert!(
            (shifted_head - base_head).length() < 1e-5,
            "avatar_head_center must stay unshifted under avatar_shape_dz: base={base_head:?}, got={shifted_head:?}"
        );
        assert!(
            (shifted_chest - base_chest).length() < 1e-5,
            "avatar_chest_center must stay unshifted under avatar_shape_dz: base={base_chest:?}, got={shifted_chest:?}"
        );

        // Viewer-space ray test evaluates the shifted visual avatar at its rendered height.
        let visual_hit = scene.avatar_hit_test_along(&world, base_head + Vec3::new(-4.0, 0.0, 0.37), Vec3::X, 10.0, now);
        assert_eq!(visual_hit.map(|h| h.avatar), Some(target_id));
        assert_eq!(base_hitbox.position, Vec3::new(128.0, 128.0, 25.0));
    }

    #[test]
    fn combat_lead_offset_follows_vehicle_parent_acceleration_freeze_and_rate() {
        let mut world = World::new(std::sync::Arc::new(crate::scene::avatar::AvatarLibrary::load()));
        world.agent_id = crate::demo::DEMO_AGENT;
        world.main_region = Some(1);
        let now = Instant::now();

        let mut vehicle = crate::demo::poser::peer();
        vehicle.pcode = aurora_prim::params::LL_PCODE_VOLUME;
        vehicle.full_id = Uuid::from_u128(0x6001);
        vehicle.local_id = 9910;
        vehicle.velocity = Vec3::new(4.0, 0.0, 0.0);
        vehicle.acceleration = Vec3::new(2.0, 0.0, 0.0);

        let mut rider = crate::demo::poser::peer();
        let rider_id = Uuid::from_u128(0x6002);
        rider.full_id = rider_id;
        rider.local_id = 9911;
        rider.parent_id = vehicle.local_id;
        rider.velocity = Vec3::ZERO;
        rider.acceleration = Vec3::ZERO;

        world.apply(aurora_net::NetEvent::ObjectUpdates {
            handle: 1,
            objects: vec![vehicle.clone(), rider.clone()],
        });

        // Advance region motion clock by 0.5 s: rider follows vehicle vel(0.5) = 4 + 2*0.5 = 5 m/s.
        world.objects.advance_motion(1, 0.5);
        world.objects.note_motion_tick(now, 1, 1.0);
        let lead = Scene::combat_lead_offset(&world, rider_id, 0.2);
        let expected_x = 5.0 * 0.2 + 0.5 * 2.0 * 0.2 * 0.2;
        assert!(
            (lead.x - expected_x).abs() < 1e-4,
            "expected vehicle lead {expected_x}, got {lead:?}"
        );

        // Half time-dilation scales lead lookahead linearly.
        world.objects.note_motion_tick(now, 1, 0.5);
        let half_lead = Scene::combat_lead_offset(&world, rider_id, 0.2);
        let expected_half_x = 5.0 * 0.1 + 0.5 * 2.0 * 0.1 * 0.1;
        assert!(
            (half_lead.x - expected_half_x).abs() < 1e-4,
            "expected scaled lead {expected_half_x}, got {half_lead:?}"
        );

        // Frozen simulation rate zeroes lead.
        world.objects.note_motion_tick(now, 1, 0.0);
        assert_eq!(Scene::combat_lead_offset(&world, rider_id, 0.2), Vec3::ZERO);

        // Grounded stationary avatar with gravity acc.z = -9.8 over elapsed clock stays at ZERO.
        let mut grounded = crate::demo::poser::peer();
        let grounded_id = Uuid::from_u128(0x6003);
        grounded.full_id = grounded_id;
        grounded.local_id = 9912;
        grounded.velocity = Vec3::ZERO;
        grounded.acceleration = Vec3::new(0.0, 0.0, -9.8);
        world.apply(aurora_net::NetEvent::ObjectUpdates {
            handle: 1,
            objects: vec![grounded],
        });
        world.objects.note_motion_tick(now, 1, 1.0);
        world.objects.advance_motion(1, 0.5);
        assert_eq!(Scene::combat_lead_offset(&world, grounded_id, 0.2), Vec3::ZERO);

        // Photo-mode hold freezes vehicle lead to ZERO.
        world.objects.hold_poses(now, []);
        assert_eq!(Scene::combat_lead_offset(&world, rider_id, 0.2), Vec3::ZERO);
    }
}
