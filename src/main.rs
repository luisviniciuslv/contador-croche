#![windows_subsystem = "windows"]

use macroquad::prelude::*;

#[cfg(target_os = "windows")]
mod win_topmost {
    unsafe extern "system" {
        fn FindWindowA(lp_class_name: *const u8, lp_window_name: *const u8) -> isize;
        fn SetWindowPos(
            h_wnd: isize,
            h_wnd_insert_after: isize,
            x: i32,
            y: i32,
            cx: i32,
            cy: i32,
            u_flags: u32,
        ) -> i32;
    }

    const HWND_TOPMOST: isize = -1;
    const HWND_NOTOPMOST: isize = -2;
    const SWP_NOMOVE: u32 = 0x0002;
    const SWP_NOSIZE: u32 = 0x0001;
    const SWP_SHOWWINDOW: u32 = 0x0040;

    pub fn set_topmost(topmost: bool) {
        let title = b"Contador de Croche\0";
        unsafe {
            let hwnd = FindWindowA(std::ptr::null(), title.as_ptr());
            if hwnd != 0 {
                let insert_after = if topmost { HWND_TOPMOST } else { HWND_NOTOPMOST };
                SetWindowPos(
                    hwnd,
                    insert_after,
                    0,
                    0,
                    0,
                    0,
                    SWP_NOMOVE | SWP_NOSIZE | SWP_SHOWWINDOW,
                );
            }
        }
    }
}

#[cfg(not(target_os = "windows"))]
mod win_topmost {
    pub fn set_topmost(_topmost: bool) {}
}

#[derive(PartialEq, Clone, Copy)]
enum AppState {
    Setup,
    Counting,
    Settings,
}

#[derive(PartialEq, Clone, Copy)]
enum RemapAction {
    Increment,
    Decrement,
    Reset,
    ChangeGoal,
}

struct Controls {
    increment: KeyCode,
    decrement: KeyCode,
    reset: KeyCode,
    change_goal: KeyCode,
}

impl Default for Controls {
    fn default() -> Self {
        Self {
            increment: KeyCode::Space,
            decrement: KeyCode::Backspace,
            reset: KeyCode::R,
            change_goal: KeyCode::Escape,
        }
    }
}

fn key_name(key: KeyCode) -> &'static str {
    match key {
        KeyCode::Space => "ESPACO",
        KeyCode::Backspace => "BACKSPACE",
        KeyCode::Enter => "ENTER",
        KeyCode::Escape => "ESC",
        KeyCode::Tab => "TAB",
        KeyCode::Up => "SETA CIMA",
        KeyCode::Down => "SETA BAIXO",
        KeyCode::Left => "SETA ESQ",
        KeyCode::Right => "SETA DIR",
        KeyCode::A => "A",
        KeyCode::B => "B",
        KeyCode::C => "C",
        KeyCode::D => "D",
        KeyCode::E => "E",
        KeyCode::F => "F",
        KeyCode::G => "G",
        KeyCode::H => "H",
        KeyCode::I => "I",
        KeyCode::J => "J",
        KeyCode::K => "K",
        KeyCode::L => "L",
        KeyCode::M => "M",
        KeyCode::N => "N",
        KeyCode::O => "O",
        KeyCode::P => "P",
        KeyCode::Q => "Q",
        KeyCode::R => "R",
        KeyCode::S => "S",
        KeyCode::T => "T",
        KeyCode::U => "U",
        KeyCode::V => "V",
        KeyCode::W => "W",
        KeyCode::X => "X",
        KeyCode::Y => "Y",
        KeyCode::Z => "Z",
        KeyCode::Key0 | KeyCode::Kp0 => "0",
        KeyCode::Key1 | KeyCode::Kp1 => "1",
        KeyCode::Key2 | KeyCode::Kp2 => "2",
        KeyCode::Key3 | KeyCode::Kp3 => "3",
        KeyCode::Key4 | KeyCode::Kp4 => "4",
        KeyCode::Key5 | KeyCode::Kp5 => "5",
        KeyCode::Key6 | KeyCode::Kp6 => "6",
        KeyCode::Key7 | KeyCode::Kp7 => "7",
        KeyCode::Key8 | KeyCode::Kp8 => "8",
        KeyCode::Key9 | KeyCode::Kp9 => "9",
        KeyCode::Equal | KeyCode::KpAdd => "+",
        KeyCode::Minus | KeyCode::KpSubtract => "-",
        _ => "TECLA",
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Contador de Croche".to_owned(),
        window_width: 480,
        window_height: 600,
        window_resizable: true,
        ..Default::default()
    }
}

fn draw_rounded_rect(x: f32, y: f32, w: f32, h: f32, r: f32, color: Color) {
    let r = r.min(w * 0.5).min(h * 0.5);
    draw_rectangle(x + r, y, w - 2.0 * r, h, color);
    draw_rectangle(x, y + r, w, h - 2.0 * r, color);
    draw_circle(x + r, y + r, r, color);
    draw_circle(x + w - r, y + r, r, color);
    draw_circle(x + r, y + h - r, r, color);
    draw_circle(x + w - r, y + h - r, r, color);
}

fn draw_heart(cx: f32, cy: f32, size: f32, color: Color) {
    let r = size * 0.35;
    draw_circle(cx - r * 0.85, cy - r * 0.3, r, color);
    draw_circle(cx + r * 0.85, cy - r * 0.3, r, color);
    draw_triangle(
        vec2(cx - size * 0.68, cy - r * 0.05),
        vec2(cx + size * 0.68, cy - r * 0.05),
        vec2(cx, cy + size * 0.72),
        color,
    );
}

fn draw_yarn_ball(cx: f32, cy: f32, radius: f32, time: f32) {
    let yarn_main = Color::from_rgba(244, 169, 194, 255);
    let yarn_shadow = Color::from_rgba(229, 140, 171, 255);
    let yarn_line = Color::from_rgba(255, 224, 235, 255);
    let needle_color = Color::from_rgba(196, 172, 152, 255);
    let needle_tip = Color::from_rgba(235, 215, 195, 255);

    let bob = (time * 2.2).sin() * (radius * 0.06);
    draw_line(
        cx - radius * 1.15,
        cy + radius * 0.65 + bob,
        cx + radius * 1.15,
        cy - radius * 0.85 + bob,
        (radius * 0.14).max(2.5),
        needle_color,
    );
    draw_circle(
        cx + radius * 1.15,
        cy - radius * 0.85 + bob,
        (radius * 0.11).max(2.0),
        needle_tip,
    );

    draw_circle(cx, cy + radius * 0.09 + bob, radius, yarn_shadow);
    draw_circle(cx, cy + bob, radius, yarn_main);

    let line_w = (radius * 0.07).max(1.5);
    draw_line(
        cx - radius * 0.7,
        cy - radius * 0.4 + bob,
        cx + radius * 0.75,
        cy + radius * 0.35 + bob,
        line_w,
        yarn_line,
    );
    draw_line(
        cx - radius * 0.8,
        cy + radius * 0.1 + bob,
        cx + radius * 0.6,
        cy - radius * 0.55 + bob,
        line_w,
        yarn_line,
    );
    draw_line(
        cx - radius * 0.35,
        cy + radius * 0.7 + bob,
        cx + radius * 0.55,
        cy - radius * 0.1 + bob,
        line_w,
        yarn_line,
    );

    let eye_color = Color::from_rgba(92, 67, 79, 255);
    let blush_color = Color::from_rgba(255, 138, 168, 180);
    let eye_r = (radius * 0.085).max(1.8);
    let blush_r = (radius * 0.13).max(2.5);
    draw_circle(cx - radius * 0.28, cy + radius * 0.05 + bob, eye_r, eye_color);
    draw_circle(cx + radius * 0.28, cy + radius * 0.05 + bob, eye_r, eye_color);
    draw_circle(cx - radius * 0.44, cy + radius * 0.22 + bob, blush_r, blush_color);
    draw_circle(cx + radius * 0.44, cy + radius * 0.22 + bob, blush_r, blush_color);

    let smile_w = (radius * 0.06).max(1.5);
    draw_line(
        cx - radius * 0.12,
        cy + radius * 0.24 + bob,
        cx,
        cy + radius * 0.30 + bob,
        smile_w,
        eye_color,
    );
    draw_line(
        cx,
        cy + radius * 0.30 + bob,
        cx + radius * 0.12,
        cy + radius * 0.24 + bob,
        smile_w,
        eye_color,
    );
}

fn draw_centered_text(text: &str, cx: f32, y: f32, font_size: f32, color: Color) {
    let dims = measure_text(text, None, font_size as u16, 1.0);
    draw_text(text, cx - dims.width * 0.5, y, font_size, color);
}

fn point_in_rect(px: f32, py: f32, x: f32, y: f32, w: f32, h: f32) -> bool {
    px >= x && px <= x + w && py >= y && py <= y + h
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut state = AppState::Setup;
    let mut prev_state = AppState::Setup;
    let mut controls = Controls::default();
    let mut remapping: Option<RemapAction> = None;
    let mut minimized = false;

    let mut target_input = String::from("12");
    let mut current_count: u32 = 0;
    let mut target_count: u32 = 12;
    let mut pulse_timer: f32 = 0.0;

    // Cores pastéis kawaii
    let bg_color = Color::from_rgba(255, 246, 249, 255);
    let card_border = Color::from_rgba(245, 208, 221, 255);
    let card_bg = Color::from_rgba(255, 255, 255, 255);
    let title_color = Color::from_rgba(122, 82, 102, 255);
    let text_muted = Color::from_rgba(165, 132, 148, 255);
    let accent_pink = Color::from_rgba(236, 128, 166, 255);
    let accent_mint = Color::from_rgba(134, 214, 184, 255);
    let accent_lilac = Color::from_rgba(203, 178, 232, 255);
    let bar_bg = Color::from_rgba(246, 234, 239, 255);

    loop {
        let dt = get_frame_time();
        let time = get_time() as f32;
        if pulse_timer > 0.0 {
            pulse_timer = (pulse_timer - dt * 4.0).max(0.0);
        }

        let sw = screen_width();
        let sh = screen_height();
        let cx = sw * 0.5;
        let mouse_clicked = is_mouse_button_pressed(MouseButton::Left);
        let (mx, my) = mouse_position();

        // Lógica de entrada de teclado
        if minimized {
            if is_key_pressed(controls.increment) && current_count < target_count {
                current_count += 1;
                pulse_timer = 1.0;
            }
            if is_key_pressed(controls.decrement) {
                current_count = current_count.saturating_sub(1);
                pulse_timer = 0.6;
            }
            if is_key_pressed(controls.reset) {
                current_count = 0;
                pulse_timer = 0.6;
            }
            if is_key_pressed(controls.change_goal) {
                minimized = false;
                win_topmost::set_topmost(false);
                request_new_screen_size(480.0, 600.0);
                state = AppState::Setup;
            }
        } else {
            match state {
                AppState::Setup => {
                    while let Some(ch) = get_char_pressed() {
                        if ch.is_ascii_digit() && target_input.len() < 4 {
                            if target_input == "0" {
                                target_input.clear();
                            }
                            target_input.push(ch);
                        }
                    }

                    if is_key_pressed(KeyCode::Backspace) {
                        target_input.pop();
                    }

                    if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter) {
                        let parsed = target_input.parse::<u32>().unwrap_or(0);
                        if parsed > 0 {
                            target_count = parsed;
                            current_count = 0;
                            state = AppState::Counting;
                            pulse_timer = 1.0;
                        }
                    }
                }
                AppState::Counting => {
                    if is_key_pressed(controls.increment) && current_count < target_count {
                        current_count += 1;
                        pulse_timer = 1.0;
                    }

                    if is_key_pressed(controls.decrement) {
                        current_count = current_count.saturating_sub(1);
                        pulse_timer = 0.6;
                    }

                    if is_key_pressed(controls.reset) {
                        current_count = 0;
                        pulse_timer = 0.6;
                    }

                    if is_key_pressed(controls.change_goal) {
                        state = AppState::Setup;
                    }
                }
                AppState::Settings => {
                    if let Some(action) = remapping {
                        if let Some(key) = get_last_key_pressed() {
                            match action {
                                RemapAction::Increment => controls.increment = key,
                                RemapAction::Decrement => controls.decrement = key,
                                RemapAction::Reset => controls.reset = key,
                                RemapAction::ChangeGoal => controls.change_goal = key,
                            }
                            remapping = None;
                        }
                    } else if is_key_pressed(KeyCode::Escape) {
                        state = prev_state;
                    }
                }
            }
        }

        // Desenho
        clear_background(bg_color);

        if minimized {
            // Modo Popup Kawaii Compacto
            let pad = 8.0;
            let card_w = (sw - pad * 2.0).max(160.0);
            let card_h = (sh - pad * 2.0).max(100.0);
            draw_rounded_rect(pad - 2.0, pad - 2.0, card_w + 4.0, card_h + 4.0, 20.0, card_border);
            draw_rounded_rect(pad, pad, card_w, card_h, 18.0, card_bg);

            // Mini novelo no canto esquerdo
            draw_yarn_ball(pad + 32.0, pad + 38.0, 18.0, time);

            // Botão expandir no canto superior direito
            let exp_w = 54.0;
            let exp_h = 22.0;
            let exp_x = pad + card_w - exp_w - 8.0;
            let exp_y = pad + 8.0;
            draw_rounded_rect(exp_x, exp_y, exp_w, exp_h, 10.0, accent_lilac);
            draw_centered_text("abrir", exp_x + exp_w * 0.5, exp_y + 15.0, 13.0, WHITE);
            if mouse_clicked && point_in_rect(mx, my, exp_x, exp_y, exp_w, exp_h) {
                minimized = false;
                win_topmost::set_topmost(false);
                request_new_screen_size(480.0, 600.0);
            }

            let done = current_count >= target_count;
            let count_color = if done { accent_mint } else { accent_pink };
            let boost = pulse_timer * 4.0;
            let label = format!("{} / {}", current_count, target_count);
            draw_centered_text(&label, cx + 10.0, pad + 52.0, 34.0 + boost, count_color);

            // Mini barra de progresso
            let bar_w = card_w - 28.0;
            let bar_h = 10.0;
            let bar_x = pad + 14.0;
            let bar_y = pad + card_h - 38.0;
            draw_rounded_rect(bar_x, bar_y, bar_w, bar_h, 5.0, bar_bg);
            let progress = (current_count as f32 / target_count.max(1) as f32).clamp(0.0, 1.0);
            if progress > 0.0 {
                draw_rounded_rect(
                    bar_x,
                    bar_y,
                    (bar_w * progress).max(10.0),
                    bar_h,
                    5.0,
                    count_color,
                );
            }

            // Mini botões - e + clicáveis
            let btn_w = 34.0;
            let btn_h = 20.0;
            let minus_x = pad + 14.0;
            let plus_x = pad + card_w - btn_w - 14.0;
            let btn_y = pad + card_h - 24.0;
            draw_rounded_rect(minus_x, btn_y, btn_w, btn_h, 8.0, card_border);
            draw_centered_text("-", minus_x + btn_w * 0.5, btn_y + 15.0, 18.0, title_color);

            draw_rounded_rect(plus_x, btn_y, btn_w, btn_h, 8.0, accent_pink);
            draw_centered_text("+", plus_x + btn_w * 0.5, btn_y + 15.0, 18.0, WHITE);

            if mouse_clicked {
                if point_in_rect(mx, my, minus_x, btn_y, btn_w, btn_h) {
                    current_count = current_count.saturating_sub(1);
                    pulse_timer = 0.6;
                }
                if point_in_rect(mx, my, plus_x, btn_y, btn_w, btn_h) && current_count < target_count {
                    current_count += 1;
                    pulse_timer = 1.0;
                }
            }

            if done {
                draw_heart(cx, pad + card_h - 14.0, 12.0, accent_mint);
            } else {
                draw_centered_text("carreiras", cx, pad + card_h - 9.0, 13.0, text_muted);
            }

            next_frame().await;
            continue;
        }

        // Modo Normal
        let card_w = (sw - 48.0).min(400.0);
        let card_h = (sh - 64.0).min(520.0);
        let card_x = cx - card_w * 0.5;
        let card_y = (sh - card_h) * 0.5;

        draw_rounded_rect(card_x - 3.0, card_y - 3.0, card_w + 6.0, card_h + 6.0, 28.0, card_border);
        draw_rounded_rect(card_x, card_y, card_w, card_h, 25.0, card_bg);

        // Botões no topo do cartão: Config (esquerda) e Minimizar (direita)
        if state != AppState::Settings {
            let cfg_x = card_x + 16.0;
            let cfg_y = card_y + 16.0;
            let cfg_w = 72.0;
            let cfg_h = 28.0;
            draw_rounded_rect(cfg_x, cfg_y, cfg_w, cfg_h, 12.0, bar_bg);
            draw_centered_text("config", cfg_x + cfg_w * 0.5, cfg_y + 19.0, 15.0, title_color);
            if mouse_clicked && point_in_rect(mx, my, cfg_x, cfg_y, cfg_w, cfg_h) {
                prev_state = state;
                remapping = None;
                state = AppState::Settings;
            }

            let min_w = 72.0;
            let min_h = 28.0;
            let min_x = card_x + card_w - min_w - 16.0;
            let min_y = card_y + 16.0;
            draw_rounded_rect(min_x, min_y, min_w, min_h, 12.0, accent_lilac);
            draw_centered_text("popup", min_x + min_w * 0.5, min_y + 19.0, 15.0, WHITE);
            if mouse_clicked && point_in_rect(mx, my, min_x, min_y, min_w, min_h) {
                minimized = true;
                win_topmost::set_topmost(true);
                request_new_screen_size(250.0, 130.0);
            }
        }

        // Novelo fofinho no topo do cartão
        draw_yarn_ball(cx, card_y + 78.0, 40.0, time);
        draw_centered_text("Contador de Croche", cx, card_y + 148.0, 27.0, title_color);

        match state {
            AppState::Setup => {
                draw_centered_text(
                    "Quantas carreiras voce vai fazer?",
                    cx,
                    card_y + 192.0,
                    19.0,
                    text_muted,
                );

                let box_w = 180.0;
                let box_h = 84.0;
                let box_x = cx - box_w * 0.5;
                let box_y = card_y + 222.0;
                draw_rounded_rect(box_x, box_y, box_w, box_h, 18.0, bar_bg);

                let display_val = if target_input.is_empty() {
                    "0"
                } else {
                    target_input.as_str()
                };
                draw_centered_text(display_val, cx, box_y + 58.0, 56.0, accent_pink);

                let btn_size = 44.0;
                let minus_x = box_x - btn_size - 14.0;
                let plus_x = box_x + box_w + 14.0;
                let btn_y = box_y + (box_h - btn_size) * 0.5;

                draw_rounded_rect(minus_x, btn_y, btn_size, btn_size, 14.0, card_border);
                draw_centered_text("-", minus_x + btn_size * 0.5, btn_y + 31.0, 32.0, title_color);

                draw_rounded_rect(plus_x, btn_y, btn_size, btn_size, 14.0, card_border);
                draw_centered_text("+", plus_x + btn_size * 0.5, btn_y + 31.0, 32.0, title_color);

                if mouse_clicked {
                    if point_in_rect(mx, my, minus_x, btn_y, btn_size, btn_size) {
                        let val = target_input.parse::<u32>().unwrap_or(0).saturating_sub(1);
                        target_input = val.to_string();
                    }
                    if point_in_rect(mx, my, plus_x, btn_y, btn_size, btn_size) {
                        let val = (target_input.parse::<u32>().unwrap_or(0) + 1).min(9999);
                        target_input = val.to_string();
                    }
                }

                draw_centered_text(
                    "Digite o numero ou use + / -",
                    cx,
                    card_y + 356.0,
                    18.0,
                    text_muted,
                );
                draw_rounded_rect(cx - 135.0, card_y + 388.0, 270.0, 48.0, 16.0, accent_pink);
                draw_centered_text("ESPACO para comecar", cx, card_y + 419.0, 20.0, WHITE);
            }
            AppState::Counting => {
                let done = current_count >= target_count;
                let status_text = if done {
                    "Meta concluida!"
                } else {
                    "Contando carreiras..."
                };
                let status_color = if done { accent_mint } else { text_muted };
                draw_centered_text(status_text, cx, card_y + 188.0, 20.0, status_color);

                if done {
                    let float_y = (time * 3.0).sin() * 4.0;
                    draw_heart(cx - 95.0, card_y + 245.0 + float_y, 16.0, accent_pink);
                    draw_heart(cx + 95.0, card_y + 245.0 - float_y, 16.0, accent_mint);
                }

                let scale_boost = pulse_timer * 8.0;
                let count_str = format!("{}", current_count);
                draw_centered_text(
                    &count_str,
                    cx,
                    card_y + 288.0,
                    92.0 + scale_boost,
                    if done { accent_mint } else { accent_pink },
                );

                let goal_str = format!("de {} carreiras", target_count);
                draw_centered_text(&goal_str, cx, card_y + 330.0, 22.0, title_color);

                let bar_w = card_w - 72.0;
                let bar_h = 22.0;
                let bar_x = cx - bar_w * 0.5;
                let bar_y = card_y + 360.0;
                draw_rounded_rect(bar_x, bar_y, bar_w, bar_h, 11.0, bar_bg);

                let progress = (current_count as f32 / target_count as f32).clamp(0.0, 1.0);
                if progress > 0.0 {
                    let fill_w = (bar_w * progress).max(22.0);
                    draw_rounded_rect(
                        bar_x,
                        bar_y,
                        fill_w,
                        bar_h,
                        11.0,
                        if done { accent_mint } else { accent_pink },
                    );
                }

                let line1 = format!(
                    "{}: +1 carreira  |  {}: -1 carreira",
                    key_name(controls.increment),
                    key_name(controls.decrement)
                );
                let line2 = format!(
                    "{}: zerar  |  {}: mudar meta",
                    key_name(controls.reset),
                    key_name(controls.change_goal)
                );
                draw_centered_text(&line1, cx, card_y + 428.0, 15.0, text_muted);
                draw_centered_text(&line2, cx, card_y + 456.0, 15.0, text_muted);
            }
            AppState::Settings => {
                draw_centered_text("Configurar Controles", cx, card_y + 186.0, 20.0, title_color);
                draw_centered_text(
                    "Clique em um botao e aperte a tecla",
                    cx,
                    card_y + 210.0,
                    15.0,
                    text_muted,
                );

                let rows = [
                    ("+1 carreira", RemapAction::Increment, controls.increment),
                    ("-1 carreira", RemapAction::Decrement, controls.decrement),
                    ("Zerar", RemapAction::Reset, controls.reset),
                    ("Mudar meta", RemapAction::ChangeGoal, controls.change_goal),
                ];

                let row_w = card_w - 64.0;
                let row_x = cx - row_w * 0.5;
                for (i, (label, action, key)) in rows.iter().enumerate() {
                    let ry = card_y + 228.0 + (i as f32) * 52.0;
                    draw_rounded_rect(row_x, ry, row_w, 42.0, 14.0, bar_bg);
                    draw_text(label, row_x + 14.0, ry + 27.0, 18.0, title_color);

                    let btn_w = 130.0;
                    let btn_h = 30.0;
                    let btn_x = row_x + row_w - btn_w - 8.0;
                    let btn_y = ry + 6.0;
                    let is_waiting = remapping == Some(*action);
                    let btn_color = if is_waiting { accent_mint } else { accent_pink };
                    let btn_text = if is_waiting {
                        "Aperte tecla..."
                    } else {
                        key_name(*key)
                    };

                    draw_rounded_rect(btn_x, btn_y, btn_w, btn_h, 10.0, btn_color);
                    draw_centered_text(btn_text, btn_x + btn_w * 0.5, btn_y + 20.0, 15.0, WHITE);

                    if mouse_clicked && point_in_rect(mx, my, btn_x, btn_y, btn_w, btn_h) {
                        remapping = Some(*action);
                    }
                }

                // Botão Voltar
                let back_w = 130.0;
                let back_h = 40.0;
                let back_x = cx - back_w * 0.5;
                let back_y = card_y + 452.0;
                draw_rounded_rect(back_x, back_y, back_w, back_h, 14.0, accent_lilac);
                draw_centered_text("Voltar", cx, back_y + 26.0, 19.0, WHITE);
                if mouse_clicked && point_in_rect(mx, my, back_x, back_y, back_w, back_h) {
                    remapping = None;
                    state = prev_state;
                }
            }
        }

        next_frame().await;
    }
}
