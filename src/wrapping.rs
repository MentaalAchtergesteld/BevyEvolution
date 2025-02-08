use bevy::prelude::*;

pub struct WrappingPlugin;

impl Plugin for WrappingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, wrap_transforms);
    }
}

#[derive(Resource)]
pub struct WrappingRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32
}

fn wrap_position(position: Vec2, rect: &WrappingRect) -> Vec2 {
    let mut wrapped_position = position.clone();
    if position.x < rect.x {
        wrapped_position.x += rect.width;
    } else if position.x > rect.x + rect.width {
        wrapped_position.x -= rect.width;
    }

    if position.y < rect.y {
        wrapped_position.y += rect.height;
    } else if position.y > rect.y + rect.height {
        wrapped_position.y -= rect.height;
    }

    wrapped_position
}

fn wrap_transforms(
    mut query: Query<&mut Transform>,
    rect: Res<WrappingRect>
) {
    for mut transform in &mut query {
        let position = transform.translation.xy();
        transform.translation = wrap_position(position, &rect).extend(0.);
    }
}

pub fn wrapping_delta(a: Vec2, b: Vec2, rect: &WrappingRect) -> Vec2 {
    let wrapped_a = wrap_position(a, rect);
    let wrapped_b = wrap_position(b, rect);

    let mut delta = wrapped_b - wrapped_a;

    if delta.x.abs() > rect.width * 0.5 {
        if delta.x > 0.0 {
            delta.x -= rect.width;
        } else {
            delta.x += rect.width;
        }
    }

    if delta.y.abs() > rect.width * 0.5 {
        if delta.y > 0.0 {
            delta.y -= rect.height;
        } else {
            delta.y += rect.height;
        }
    }

    delta
}

pub fn wrapping_length_squared(a: Vec2, b: Vec2, rect: &WrappingRect) -> f32 {
    wrapping_delta(a, b, rect).length_squared()
}

pub fn wrapping_length(a: Vec2, b: Vec2, rect: &WrappingRect) -> f32 {
    wrapping_delta(a, b, rect).length()
}