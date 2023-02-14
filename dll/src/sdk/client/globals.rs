/*
 * === GLOBAL OFFSETS ===
 */

use crate::*;

global! {
    pub extern LOGIN_STATE: u32 = "osclient.exe"% "8B 0D ? ? ? ? 85 C9 0F 85 A8 03 00 00";
    pub extern CAMERA_TILE_Y_MIN: u32 = "osclient.exe"% "8B 2D ? ? ? ? 3B 2D";
    pub extern CAMERA_TILE_Y_MAX: u32 = "osclient.exe"% "3B 2D ? ? ? ? 0F 8D 0C 01 00 00";
    pub extern CAMERA_TILE_X_MIN: u32 = "osclient.exe"% "44 8B 35 ?? ?? ?? ?? 44 3B 35";
    pub extern CAMERA_TILE_X_MAX: u32 = "osclient.exe"% "44 3B 35 ?? ?? ?? ?? 0F 8D 2E 01 00 00";
    pub extern CAMERA_TILE_X: u32 = "osclient.exe"% "44 8B 05 ?? ?? ?? ?? 44 8B 0D ?? ?? ?? ?? E9 0D 03 00 00";
    pub extern CAMERA_TILE_Y: u32 = "osclient.exe"% "8B 15 ?? ?? ?? ?? 44 8B 05 ?? ?? ?? ?? 44 8B 0D ?? ?? ?? ?? E9 0D 03 00 00";
    pub extern DRAW_DISTANCE: u32 = "osclient.exe"% "44 8B 2D ?? ?? ?? ?? 44 8B A4 24";
}