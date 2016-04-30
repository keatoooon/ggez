use std::time::Duration;

use GameError;

pub trait State {
    fn init(&self) -> Result<(), GameError>;
    fn update(&self, dt: Duration) -> Result<(), GameError>;
    fn draw(&self) -> Result<(), GameError>;
}
