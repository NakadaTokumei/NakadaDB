use std::thread;
use std::error::Error;

pub trait NakadaThread
{
    fn create_thread(&mut self, func : fn()) -> Result<(), Box<dyn Error>>;
    fn start_all(self) -> thread::Result<()>;
    fn stop_all(&mut self) -> Result<(), Box<dyn Error>>;
}