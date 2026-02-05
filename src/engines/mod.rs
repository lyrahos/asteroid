//! Engine implementations for Asteroid Browser.
//!
//! This module provides concrete implementations of the BrowserEngine trait.
//! Engine selection is done at compile time via feature flags.

pub mod gecko;
pub mod servo;

#[cfg(feature = "gecko-engine")]
pub use gecko::GeckoEngine as DefaultEngine;

#[cfg(feature = "servo-engine")]
pub use servo::ServoEngine as DefaultEngine;

/// Create the default engine based on compile-time feature flags.
pub fn create_default_engine() -> Box<dyn crate::core::engine::BrowserEngine> {
    #[cfg(feature = "gecko-engine")]
    {
        Box::new(gecko::GeckoEngine::new())
    }

    #[cfg(feature = "servo-engine")]
    {
        Box::new(servo::ServoEngine::new())
    }

    #[cfg(not(any(feature = "gecko-engine", feature = "servo-engine")))]
    {
        compile_error!("At least one engine feature must be enabled (gecko-engine or servo-engine)");
    }
}
