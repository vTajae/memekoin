//! Base service traits and utilities
//! 
//! This module contains common service patterns and base implementations
//! that can be shared across different service modules.

use crate::error::AppError;

/// Base trait for services that need initialization
pub trait BaseService {
    /// Initialize the service with necessary dependencies
    fn initialize(&self) -> Result<(), AppError> {
        Ok(())
    }
}

/// Common service result type
pub type ServiceResult<T> = Result<T, AppError>;