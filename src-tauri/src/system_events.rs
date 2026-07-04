use std::time::Duration;
use tauri::{AppHandle, Emitter};
use windows::Win32::Media::Audio::Endpoints::IAudioEndpointVolume;
use windows::Win32::Media::Audio::{eConsole, eRender, IMMDeviceEnumerator, MMDeviceEnumerator};
use windows::Win32::System::Com::{CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_MULTITHREADED};
use windows::Win32::System::Power::{GetSystemPowerStatus, SYSTEM_POWER_STATUS};

pub fn start_monitor(app: AppHandle) {
    std::thread::spawn(move || {
        // 初始化 COM 接口以获取音频
        unsafe { let _ = CoInitializeEx(None, COINIT_MULTITHREADED); }
        
        let mut last_volume = get_system_volume().unwrap_or(-1.0);
        let mut last_power_state = get_ac_line_status().unwrap_or(255); // 255 = 未知

        loop {
            std::thread::sleep(Duration::from_millis(800)); // 每 800ms 检查一次

            // 1. 检查音量变化
            if let Some(current_volume) = get_system_volume() {
                // 如果音量变化超过 0.01 (1%)
                if (current_volume - last_volume).abs() > 0.01 && last_volume != -1.0 {
                    let vol_percent = (current_volume * 100.0).round() as i32;
                    let _ = app.emit("system-event", format!("当前系统音量 {}%", vol_percent));
                }
                last_volume = current_volume;
            }

            // 2. 检查电源状态变化 (0 = 使用电池, 1 = 插入电源)
            if let Some(current_power) = get_ac_line_status() {
                if current_power != last_power_state && last_power_state != 255 {
                    if current_power == 1 {
                        let _ = app.emit("system-event", "已连接交流电源");
                    } else if current_power == 0 {
                        let _ = app.emit("system-event", "正在使用电池供电");
                    }
                }
                last_power_state = current_power;
            }
        }
    });
}

// 辅助函数：获取 Windows 系统音量 (0.0 到 1.0)
fn get_system_volume() -> Option<f32> {
    unsafe {
        let enumerator: IMMDeviceEnumerator = CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL).ok()?;
        let device = enumerator.GetDefaultAudioEndpoint(eRender, eConsole).ok()?;
        let endpoint_volume: IAudioEndpointVolume = device.Activate(CLSCTX_ALL, None).ok()?;
        
        // 0.58.0 版本的 windows crate 直接返回 f32，不需要传入 &mut 变量
        let volume = endpoint_volume.GetMasterVolumeLevelScalar().ok()?;
        Some(volume)
    }
}

// 辅助函数：获取电源插入状态
fn get_ac_line_status() -> Option<u8> {
    unsafe {
        let mut status: SYSTEM_POWER_STATUS = std::mem::zeroed();
        if GetSystemPowerStatus(&mut status).is_ok() {
            Some(status.ACLineStatus)
        } else {
            None
        }
    }
}