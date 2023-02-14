use faithe::internal::{get_module_information, ModuleInfo};
use once_cell::sync::OnceCell;
use crate::sdk::Global;

pub struct Client {
    module: ModuleInfo
}

impl Global for Client {
    fn cell() -> &'static OnceCell<Self> {
        static INSTANCE: OnceCell<Client> = OnceCell::new();
        &INSTANCE
    }

    fn create() -> Self {
        Client {
            module: get_module_information("osclient.exe").unwrap()
        }
    }
}

impl Client {

    pub fn get_module(&self) -> &ModuleInfo {
        return &self.module;
    }
}

unsafe impl Send for Client {}
unsafe impl Sync for Client {}