
//! A module contains implementation of ease functions.

use std::num::{
    Float,
};

#[allow(missing_docs)]
#[derive(Copy, Clone, PartialEq)]
pub enum EaseFunction {
    QuadraticIn,
    QuadraticOut,
    QuadraticInOut,

    CubicIn,
    CubicOut,
    CubicInOut,

    QuarticIn,
    QuarticOut,
    QuarticInOut,

    QuinticIn,
    QuinticOut,
    QuinticInOut,

    SineIn,
    SineOut,
    SineInOut,

    CircularIn,
    CircularOut,
    CircularInOut,

    ExponentialIn,
    ExponentialOut,
    ExponentialInOut,

    ElasticIn,
    ElasticOut,
    ElasticInOut,

    BackIn,
    BackOut,
    BackInOut,

    BounceIn,
    BounceOut,
    BounceInOut,
}

#[allow(missing_docs)]
pub trait Ease {
    /// Calculate the eased value, normalized
    fn calc(self, f: EaseFunction) -> Self;

    fn quadratic_in(self) -> Self;
    fn quadratic_out(self) -> Self;
    fn quadratic_in_out(self) -> Self;

    fn cubic_in(self) -> Self;
    fn cubic_out(self) -> Self;
    fn cubic_in_out(self) -> Self;

    fn quartic_in(self) -> Self;
    fn quartic_out(self) -> Self;
    fn quartic_in_out(self) -> Self;

    fn quintic_in(self) -> Self;
    fn quintic_out(self) -> Self;
    fn quintic_in_out(self) -> Self;

    fn sine_in(self) -> Self;
    fn sine_out(self) -> Self;
    fn sine_in_out(self) -> Self;

    fn circular_in(self) -> Self;
    fn circular_out(self) -> Self;
    fn circular_in_out(self) -> Self;

    fn exponential_in(self) -> Self;
    fn exponential_out(self) -> Self;
    fn exponential_in_out(self) -> Self;

    fn elastic_in(self) -> Self;
    fn elastic_out(self) -> Self;
    fn elastic_in_out(self) -> Self;

    fn back_in(self) -> Self;
    fn back_out(self) -> Self;
    fn back_in_out(self) -> Self;

    fn bounce_in(self) -> Self;
    fn bounce_out(self) -> Self;
    fn bounce_in_out(self) -> Self;
}

macro_rules! impl_ease_trait_for {
    ($T: ident) => (
        impl Ease for $T {
            fn calc(self, f: EaseFunction) -> Self {
                match f {
                    EaseFunction::QuadraticIn => self.quadratic_in(),
                    EaseFunction::QuadraticOut => self.quadratic_out(),
                    EaseFunction::QuadraticInOut => self.quadratic_in_out(),

                    EaseFunction::CubicIn => self.cubic_in(),
                    EaseFunction::CubicOut => self.cubic_out(),
                    EaseFunction::CubicInOut => self.cubic_in_out(),

                    EaseFunction::QuarticIn => self.quartic_in(),
                    EaseFunction::QuarticOut => self.quartic_out(),
                    EaseFunction::QuarticInOut => self.quartic_in_out(),

                    EaseFunction::QuinticIn => self.quintic_in(),
                    EaseFunction::QuinticOut => self.quintic_out(),
                    EaseFunction::QuinticInOut => self.quintic_in_out(),

                    EaseFunction::SineIn => self.sine_in(),
                    EaseFunction::SineOut => self.sine_out(),
                    EaseFunction::SineInOut => self.sine_in_out(),

                    EaseFunction::CircularIn => self.circular_in(),
                    EaseFunction::CircularOut => self.circular_out(),
                    EaseFunction::CircularInOut => self.circular_in_out(),

                    EaseFunction::ExponentialIn => self.exponential_in(),
                    EaseFunction::ExponentialOut => self.exponential_out(),
                    EaseFunction::ExponentialInOut => self.exponential_in_out(),

                    EaseFunction::ElasticIn => self.elastic_in(),
                    EaseFunction::ElasticOut => self.elastic_out(),
                    EaseFunction::ElasticInOut => self.elastic_in_out(),

                    EaseFunction::BackIn => self.back_in(),
                    EaseFunction::BackOut => self.back_out(),
                    EaseFunction::BackInOut => self.back_in_out(),

                    EaseFunction::BounceIn => self.bounce_in(),
                    EaseFunction::BounceOut => self.bounce_out(),
                    EaseFunction::BounceInOut => self.bounce_in_out(),
                }
            }

            fn quadratic_in(self) -> Self {
                let p = normalized(self);
                p * p
            }

            fn quadratic_out(self) -> Self {
                let p = normalized(self);
                -(p * (p - 2.0))
            }

            fn quadratic_in_out(self) -> Self {
                let p = normalized(self);
                if p < 0.5 {
                    2.0 * p * p
                } else {
                    (-2.0 * p * p) + (4.0 * p) - 1.0
                }
            }


            fn cubic_in(self) -> Self {
                let p = normalized(self);
                p * p * p
            }

            fn cubic_out(self) -> Self {
                let p = normalized(self);
                let f = p - 1.0;
                f * f * f + 1.0
            }

            fn cubic_in_out(self) -> Self {
                let p = normalized(self);
                if p < 0.5 {
                    4.0 * p * p * p
                } else {
                    let f = (2.0 * p) - 2.0;
                    0.5 * f * f * f + 1.0
                }
            }


            fn quartic_in(self) -> Self {
                let p = normalized(self);
                p * p * p * p
            }

            fn quartic_out(self) -> Self {
                let p = normalized(self);
                let f = p - 1.0;
                f * f * f * (1.0 - p) + 1.0
            }

            fn quartic_in_out(self) -> Self {
                let p = normalized(self);
                if p < 0.5 {
                    8.0 * p * p * p * p
                } else {
                    let f = p - 1.0;
                    -8.0 * f * f * f * f + 1.0
                }
            }


            fn quintic_in(self) -> Self {
                let p = normalized(self);
                p * p * p * p * p
            }

            fn quintic_out(self) -> Self {
                let p = normalized(self);
                let f = p - 1.0;
                f * f * f * f * f + 1.0
            }

            fn quintic_in_out(self) -> Self {
                let p = normalized(self);
                if p < 0.5  {
                    16.0 * p * p * p * p * p
                } else {
                    let f = (2.0 * p) - 2.0;
                    0.5 * f * f * f * f * f + 1.0
                }
            }


            fn sine_in(self) -> Self {
                use std::$T::consts::PI_2;
                let p = normalized(self);
                ((p - 1.0) * PI_2).sin() + 1.0
            }

            fn sine_out(self) -> Self {
                use std::$T::consts::PI_2;
                let p = normalized(self);
                (p * PI_2).sin()
            }

            fn sine_in_out(self) -> Self {
                use std::$T::consts::PI;
                let p = normalized(self);
                0.5 * (1.0 - (p * PI).cos())
            }


            fn circular_in(self) -> Self {
                let p = normalized(self);
                1.0 - (1.0 - (p * p)).sqrt()
            }

            fn circular_out(self) -> Self {
                let p = normalized(self);
                ((2.0 - p) * p).sqrt()
            }

            fn circular_in_out(self) -> Self {
                let p = normalized(self);
                if p < 0.5 {
                    0.5 * (1.0 - (1.0 - 4.0 * (p * p)).sqrt())
                } else {
                    0.5 * ((-((2.0 * p) - 3.0) * ((2.0 * p) - 1.0)).sqrt() + 1.0)
                }
            }


            fn exponential_in(self) -> Self {
                let p = normalized(self);
                if p == 0.0 {
                    p
                } else {
                    2.0.powf(10.0 * (p - 1.0))
                }
            }

            fn exponential_out(self) -> Self {
                let p = normalized(self);
                if p == 1.0 {
                    p
                } else {
                    1.0 - 2.0.powf(-10.0 * p)
                }
            }

            fn exponential_in_out(self) -> Self {
                let p = normalized(self);
                if p == 0.0 || p == 1.0 {
                    return p;
                }

                if p < 0.5  {
                    0.5 * 2.0.powf((20.0 * p) - 10.0)
                } else {
                    -0.5 * 2.0.powf((-20.0 * p) + 10.0) + 1.0
                }
            }


            fn elastic_in(self) -> Self {
                use std::$T::consts::PI_2;
                let p = normalized(self);
                (13.0 * PI_2 * p).sin() * 2.0.powf(10.0 * (p - 1.0))
            }

            fn elastic_out(self) -> Self {
                use std::$T::consts::PI_2;
                let p = normalized(self);
                (-13.0 * PI_2 * (p + 1.0)).sin() * 2.0.powf(-10.0 * p) + 1.0
            }

            fn elastic_in_out(self) -> Self {
                use std::$T::consts::PI_2;
                let p = normalized(self);
                if p < 0.5 {
                    0.5 * (13.0 * PI_2 * (2.0 * p)).sin() * 2.0.powf(10.0 * ((2.0 * p) - 1.0))
                } else {
                    0.5 * ((-13.0 * PI_2 * ((2.0 * p - 1.0) + 1.0)).sin() * 2.0.powf(-10.0 * (2.0 * p - 1.0)) + 2.0)
                }
            }


            fn back_in(self) -> Self {
                use std::$T::consts::PI;
                let p = normalized(self);
                p * p * p - p * (p * PI).sin()
            }

            fn back_out(self) -> Self {
                use std::$T::consts::PI;
                let p = normalized(self);
                let f = 1.0 - p;
                1.0 - (f * f * f - f * (f * PI).sin())
            }

            fn back_in_out(self) -> Self {
                use std::$T::consts::PI;
                let p = normalized(self);
                if p < 0.5 {
                    let f = 2.0 * p;
                    0.5 * (f * f * f - f * (f * PI).sin())
                } else {
                    let f = 1.0 - (2.0 * p - 1.0);
                    0.5 * (1.0 - (f * f * f - f * (f * PI).sin())) + 0.5
                }
            }


            fn bounce_in(self) -> Self {
                let p = normalized(self);
                1.0 - Ease::bounce_out(1.0 - p)
            }

            fn bounce_out(self) -> Self {
                let p = normalized(self);
                if p < 4.0 / 11.0 {
                    (121.0 * p * p) / 16.0
                } else if p < 8.0 / 11.0 {
                    (363.0 / 40.0 * p * p) - (99.0 / 10.0 * p) + 17.0 / 5.0
                } else if p < 9.0 / 10.0 {
                    (4356.0 / 361.0 * p * p) - (35442.0 / 1805.0 * p) + 16061.0 / 1805.0
                } else {
                    (54.0 / 5.0 * p * p) - (513.0 / 25.0 * p) + 268.0 / 25.0
                }
            }

            fn bounce_in_out(self) -> Self {
                let p = normalized(self);
                if p < 0.5 {
                    0.5 * Ease::bounce_in(p * 2.0)
                } else {
                    0.5 * Ease::bounce_out(p * 2.0 - 1.0) + 0.5
                }
            }
        }
    )
}

impl_ease_trait_for!(f32);
impl_ease_trait_for!(f64);

fn normalized<T: Float>(p: T) -> T {
    if p > Float::one() {
        Float::one()
    } else if p < Float::zero() {
        Float::zero()
    } else {
        p
    }
}
