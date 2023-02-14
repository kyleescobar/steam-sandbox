mod client;
mod hooks;

use std::borrow::Borrow;
use std::future::IntoFuture;
use lazy_static::lazy_static;
use once_cell::sync::OnceCell;
use tokio::runtime::Runtime;
use tokio::task::JoinHandle;
use crate::sdk::client::Client;
use crate::sdk::hooks::Hooks;

lazy_static! {
    pub static ref SDK: SandboxSdk = SandboxSdk::setup();
}

pub trait Global: Send + Sync + Sized {
    fn cell() -> &'static OnceCell<Self>;
    fn create() -> Self;
    fn get_or_create() -> &'static Self {
        Self::cell().get_or_init(|| Self::create())
    }
}

pub type Sandbox = std::sync::Arc<SandboxSdk>;

pub struct SandboxSdk {
    client: &'static Client,
    hooks: &'static Hooks,
    scheduler: Runtime,
}

impl SandboxSdk {
    //noinspection RsFieldInitShorthand
    pub fn setup() -> Self {
        unsafe {
            log::info!("Setting up Sandbox.");

            let client = Client::get_or_create();
            let hooks = Hooks::get_or_create();
            let scheduler = Runtime::new().unwrap();

            log::info!("Sandbox has been initialized.");
            log::info!("Module Base: 0x{:x}", client.get_module().dll_base as usize);

            Self {
                client,
                hooks,
                scheduler
            }
        }
    }

    pub fn get_client(&self) -> &'static Client {
        self.client
    }

    fn get_hooks(&self) -> &'static Hooks {
        self.hooks
    }

    pub fn spawn<T>(&self, future: T) -> JoinHandle<<T as IntoFuture>::Output>
    where
    T: IntoFuture,
    T::IntoFuture: Send + Sync + 'static,
    T::Output: Send + Sync + 'static,
    {
        self.scheduler.spawn(future.into_future())
    }

    pub fn run_until_shutdown(&self) {
        self.scheduler.block_on(async move {
            loop {
                tokio::task::yield_now().await;
            }
        });
    }
}