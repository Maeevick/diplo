//! Words and numbers drawn with the pixel font, crisp at any zoom where Bevy's own text would blur.

use bevy::{prelude::*, transform::TransformSystems};

use crate::pictures::Pictures;

/// From one letter's left edge to the next one's.
const ADVANCE: f32 = 6.0;

#[derive(Component)]
#[require(Transform, Visibility)]
pub struct PixelText {
    pub text: String,
    pub color: Color,
}

impl PixelText {
    pub fn new(text: impl Into<String>, color: Color) -> Self {
        Self {
            text: text.into(),
            color,
        }
    }

    /// Width in world units before the entity's scale.
    pub fn width(&self) -> f32 {
        self.text.chars().count() as f32 * ADVANCE - 1.0
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(PostUpdate, draw.before(TransformSystems::Propagate));
}

/// Redraws a text's letters, centred on its entity, whenever it changes.
fn draw(
    mut commands: Commands,
    pictures: Res<Pictures>,
    texts: Query<(Entity, &PixelText), Changed<PixelText>>,
) {
    for (entity, text) in &texts {
        let left = -text.width() / 2.0 + 2.5;
        commands
            .entity(entity)
            .despawn_related::<Children>()
            .with_children(|parent| {
                for (i, letter) in text.text.chars().enumerate() {
                    if let Some(glyph) = pictures.glyph(letter) {
                        parent.spawn((
                            Sprite {
                                image: glyph,
                                color: text.color,
                                ..default()
                            },
                            Transform::from_xyz(left + i as f32 * ADVANCE, 0.0, 0.0),
                        ));
                    }
                }
            });
    }
}
