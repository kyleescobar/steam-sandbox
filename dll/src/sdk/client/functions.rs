use crate::function;

function! {
    pub FN_DRAW_SCENE: extern "fastcall" fn(client: *mut u64, scene: *mut u64, camera_x: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32) -> *mut u64
        = "osclient.exe"@ "48 8B D9 48 8B CA 48 89 15";

    pub FN_DRAW_TILE: extern "fastcall" fn(scene: *mut u64, tile: *mut u64, a3: bool) -> *mut u64
        = "osclient.exe"@ "48 8B F1 48 89 5D 88";

    pub FN_GET_VISIBLE_TILES: extern "fastcall" fn(tileX: u32, tileY: u32) -> *mut u64
        = "osclient.exe"@ "'4C 8D 05 82 FC E4 FF";
}