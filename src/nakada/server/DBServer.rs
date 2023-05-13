
use std::thread;
use std::net::TcpListener;
use std::error::Error;
use std::thread::JoinHandle;

use crate::nakada::server::Thread::{NakadaThread};

pub struct ServerThread
{
    server_thread : Vec<JoinHandle<()>>
}

impl NakadaThread for ServerThread
{
    fn create_thread(&mut self, func : fn()) -> Result<(), Box<dyn Error>> {
        let thread = std::thread::spawn(move || { func() });
        self.server_thread.push(thread);
        Ok(())
    }

    fn start_all(self) -> std::thread::Result<()> {
        let mut iter = self.server_thread.into_iter();
        for thread in iter {
            thread.join().unwrap();
        }

        Ok(())
    }

    fn stop_all(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

pub struct Server
{
    tcp_listener : TcpListener,
    server_thread : ServerThread
}

impl Server
{
    pub fn bind(
        ip_port : &str
    ) -> Result<Server, Box<dyn Error>>
    {
        println!("Try to open NakadaDB Server... {ip_port}\n");

        let listener = TcpListener::bind(ip_port)?;
        let thread = ServerThread {
            server_thread: vec![],
        };

        Ok(Server
        {
            tcp_listener: listener,
            server_thread : thread
        })
    }
}