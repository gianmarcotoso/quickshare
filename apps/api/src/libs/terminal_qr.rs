use qrcode::{QrCode, render::unicode::Dense1x2};

const BLACK_ON_WHITE: &str = "\x1b[30;47m";
const RESET: &str = "\x1b[0m";

pub fn terminal_qr(text: &str) -> Option<String> {
    let code = QrCode::new(text.as_bytes()).ok()?;
    let rendered = code.render::<Dense1x2>().quiet_zone(true).build();

    Some(
        rendered
            .lines()
            .map(|line| format!("  {BLACK_ON_WHITE}{line}{RESET}\n"))
            .collect(),
    )
}
