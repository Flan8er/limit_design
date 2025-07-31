/// Holds the current state of a translating object.
///
/// This includes the object's current position, its velocity,
/// and the direction of movement (upward or downward).
#[derive(Clone)]
pub struct TranslationState {
    /// The current vertical position of the object.
    pub position: f32,

    /// The current velocity applied to the object per update tick.
    pub velocity: f32,

    /// The direction of motion.
    /// `true` means moving upward (negative direction),
    /// `false` means moving downward (positive direction).
    pub moving_up: bool,
}

/// Configuration parameters for translation and easing behavior.
///
/// Used to control how fast and how smoothly an object translates,
/// and where it should reverse direction.
#[derive(Clone)]
pub struct TranslationConfig {
    /// The target speed of translation in units per tick.
    /// This value is eased toward over time.
    pub target_speed: f32,

    /// The easing factor (typically between 0.05 and 0.2).
    /// Lower values result in smoother, slower changes.
    pub easing_factor: f32,

    /// The maximum absolute value for the position
    /// before the object reverses direction.
    pub max_translation: f32,

    /// A small buffer zone to allow reversal near the limit
    /// without requiring an exact match. Prevents stalling.
    pub threshold: f32,
}

pub fn translate_object(state: &mut TranslationState, config: &TranslationConfig) {
    let target_velocity = if state.moving_up {
        -config.target_speed
    } else {
        config.target_speed
    };

    // Ease the velocity
    state.velocity += (target_velocity - state.velocity) * config.easing_factor;
    state.position += state.velocity;

    // Handle bounds with threshold
    if state.moving_up && state.position <= -config.max_translation + config.threshold {
        state.position = -config.max_translation;
        state.velocity = 0.0;
        state.moving_up = false;
    } else if !state.moving_up && state.position >= config.max_translation - config.threshold {
        state.position = config.max_translation;
        state.velocity = 0.0;
        state.moving_up = true;
    }
}
