use anyhow::Result as AnyResult;

pub struct Notifier;

impl Notifier {
    /// 发送控制台通知
    pub fn send_console(message: &str) {
        println!("\n{}", "=".repeat(60));
        println!("⏰ 提醒通知");
        println!("{}", "=".repeat(60));
        println!("{}", message);
        println!("{}", "=".repeat(60));
        println!();
    }
    
    /// 发送 Windows 系统通知
    #[cfg(windows)]
    pub fn send_windows(title: &str, message: &str) -> AnyResult<()> {
        use winrt_notification::{Toast, Duration as ToastDuration};
        
        Toast::new(Toast::POWERSHELL_APP_ID)
            .title(title)
            .text1(message)
            .duration(ToastDuration::Short)
            .show()?;
        
        Ok(())
    }
    
    #[cfg(not(windows))]
    pub fn send_windows(_title: &str, _message: &str) -> AnyResult<()> {
        // 非 Windows 系统不支持
        Ok(())
    }
    
    /// 根据配置发送通知
    pub fn send(message: &str, notification_type: &str) -> AnyResult<()> {
        match notification_type {
            "console" => {
                Self::send_console(message);
            }
            "windows" => {
                Self::send_windows("TodoList 提醒", message)?;
            }
            "both" => {
                Self::send_console(message);
                Self::send_windows("TodoList 提醒", message)?;
            }
            _ => {
                Self::send_console(message);
            }
        }
        Ok(())
    }
}

