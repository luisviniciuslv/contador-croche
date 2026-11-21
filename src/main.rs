use macroquad::prelude::*;

#[derive(PartialEq)]
enum AppState {
    Setup,
    Counting,
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

fn draw_yarn_ball(cx: f32, cy: f32, radius: f32, time: f32) {
    let yarn_main = Color::from_rgba(244, 169, 194, 255);
    let yarn_shadow = Color::from_rgba(229, 140, 171, 255);
    let yarn_line = Color::from_rgba(255, 224, 235, 255);
    let needle_color = Color::from_rgba(196, 172, 152, 255);
    let needle_tip = Color::from_rgba(235, 215, 195, 255);

    // Agulhas de crochê cruzadas atrás do novelo
    let bob = (time * 2.2).sin() * 2.5;
    draw_line(
        cx - radius * 1.15,
        cy + radius * 0.65 + bob,
        cx + radius * 1.15,
        cy - radius * 0.85 + bob,
        6.0,
        needle_color,
    );
    draw_circle(cx + radius * 1.15, cy - radius * 0.85 + bob, 4.5, needle_tip);

    // Sombra e corpo do novelo
    draw_circle(cx, cy + 4.0 + bob, radius, yarn_shadow);
    draw_circle(cx, cy + bob, radius, yarn_main);

    // Fios cruzados no novelo
    draw_line(
        cx - radius * 0.7,
        cy - radius * 0.4 + bob,
        cx + radius * 0.75,
        cy + radius * 0.35 + bob,
        3.0,
        yarn_line,
    );
    draw_line(
        cx - radius * 0.8,
        cy + radius * 0.1 + bob,
        cx + radius * 0.6,
        cy - radius * 0.55 + bob,
        3.0,
        yarn_line,
    );
    draw_line(
        cx - radius * 0.35,
        cy + radius * 0.7 + bob,
        cx + radius * 0.55,
        cy - radius * 0.1 + bob,
        3.0,
        yarn_line,
    );

    // Olhinhos e bochechas fofas
    let eye_color = Color::from_rgba(92, 67, 79, 255);
    let blush_color = Color::from_rgba(255, 138, 168, 180);
    draw_circle(cx - radius * 0.28, cy + radius * 0.05 + bob, 3.5, eye_color);
    draw_circle(cx + radius * 0.28, cy + radius * 0.05 + bob, 3.5, eye_color);
    draw_circle(cx - radius * 0.44, cy + radius * 0.22 + bob, 5.5, blush_color);
    draw_circle(cx + radius * 0.44, cy + radius * 0.22 + bob, 5.5, blush_color);

    // Sorriso
    draw_line(
        cx - 5.0,
        cy + radius * 0.24 + bob,
        cx,
        cy + radius * 0.30 + bob,
        2.5,
        eye_color,
    );
    draw_line(
        cx,
        cy + radius * 0.30 + bob,
        cx + 5.0,
        cy + radius * 0.24 + bob,
        2.5,
        eye_color,
    );
}

fn draw_centered_text(text: &str, cx: f32, y: f32, font_size: f32, color: Color) {
    let dims = measure_text(text, None, font_size as u16, 1.0);
    draw_text(text, cx - dims.width * 0.5, y, font_size, color);
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut state = AppState::Setup;
    let mut target_input = String::from("12");
    let mut current_count: u32 = 0;
    let mut target_count: u32 = 12;
    let mut pulse_timer: f32 = 0.0;

    // Cores pastéis
    let bg_color = Color::from_rgba(255, 246, 249, 255);
    let card_border = Color::from_rgba(245, 208, 221, 255);
    let card_bg = Color::from_rgba(255, 255, 255, 255);
    let title_color = Color::from_rgba(122, 82, 102, 255);
    let text_muted = Color::from_rgba(165, 132, 148, 255);
    let accent_pink = Color::from_rgba(236, 128, 166, 255);
    let accent_mint = Color::from_rgba(134, 214, 184, 255);
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

        // Entrada de teclado
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
                if is_key_pressed(KeyCode::Space) {
                    if current_count < target_count {
                        current_count += 1;
                        pulse_timer = 1.0;
                    }
                }

                if is_key_pressed(KeyCode::Backspace) {
                    current_count = current_count.saturating_sub(1);
                    pulse_timer = 0.6;
                }

                if is_key_pressed(KeyCode::R) {
                    current_count = 0;
                    pulse_timer = 0.6;
                }

                if is_key_pressed(KeyCode::Escape) {
                    state = AppState::Setup;
                }
            }
        }

        // Desenho
        clear_background(bg_color);

        // Cartão central
        let card_w = (sw - 48.0).min(400.0);
        let card_h = (sh - 64.0).min(510.0);
        let card_x = cx - card_w * 0.5;
        let card_y = (sh - card_h) * 0.5;

        draw_rounded_rect(card_x - 3.0, card_y - 3.0, card_w + 6.0, card_h + 6.0, 28.0, card_border);
        draw_rounded_rect(card_x, card_y, card_w, card_h, 25.0, card_bg);

        // Novelo fofinho no topo do cartão
        draw_yarn_ball(cx, card_y + 78.0, 42.0, time);

        draw_centered_text("Contador de Croche", cx, card_y + 150.0, 28.0, title_color);

        match state {
            AppState::Setup => {
                draw_centered_text(
                    "Quantos pontos tem a carreira?",
                    cx,
                    card_y + 195.0,
                    19.0,
                    text_muted,
                );

                // Caixa do número alvo
                let box_w = 180.0;
                let box_h = 84.0;
                let box_x = cx - box_w * 0.5;
                let box_y = card_y + 225.0;
                draw_rounded_rect(box_x, box_y, box_w, box_h, 18.0, bar_bg);

                let display_val = if target_input.is_empty() {
                    "0"
                } else {
                    target_input.as_str()
                };
                draw_centered_text(display_val, cx, box_y + 58.0, 56.0, accent_pink);

                // Botões - e + clicáveis
                let btn_size = 44.0;
                let minus_x = box_x - btn_size - 14.0;
                let plus_x = box_x + box_w + 14.0;
                let btn_y = box_y + (box_h - btn_size) * 0.5;

                draw_rounded_rect(minus_x, btn_y, btn_size, btn_size, 14.0, card_border);
                draw_centered_text("-", minus_x + btn_size * 0.5, btn_y + 31.0, 32.0, title_color);

                draw_rounded_rect(plus_x, btn_y, btn_size, btn_size, 14.0, card_border);
                draw_centered_text("+", plus_x + btn_size * 0.5, btn_y + 31.0, 32.0, title_color);

                if is_mouse_button_pressed(MouseButton::Left) {
                    let (mx, my) = mouse_position();
                    if mx >= minus_x && mx <= minus_x + btn_size && my >= btn_y && my <= btn_y + btn_size {
                        let val = target_input.parse::<u32>().unwrap_or(0).saturating_sub(1);
                        target_input = val.to_string();
                    }
                    if mx >= plus_x && mx <= plus_x + btn_size && my >= btn_y && my <= btn_y + btn_size {
                        let val = (target_input.parse::<u32>().unwrap_or(0) + 1).min(9999);
                        target_input = val.to_string();
                    }
                }

                // Instruções
                draw_centered_text(
                    "Digite o numero ou use + / -",
                    cx,
                    card_y + 360.0,
                    18.0,
                    text_muted,
                );
                draw_rounded_rect(cx - 135.0, card_y + 390.0, 270.0, 48.0, 16.0, accent_pink);
                draw_centered_text(
                    "ESPACO para comecar",
                    cx,
                    card_y + 421.0,
                    20.0,
                    WHITE,
                );
            }
            AppState::Counting => {
                let done = current_count >= target_count;
                let status_text = if done {
                    "Carreira concluida!"
                } else {
                    "Contando pontos..."
                };
                let status_color = if done { accent_mint } else { text_muted };
                draw_centered_text(status_text, cx, card_y + 190.0, 20.0, status_color);

                // Número grande com leve pulso
                let scale_boost = pulse_timer * 8.0;
                let count_str = format!("{}", current_count);
                draw_centered_text(
                    &count_str,
                    cx,
                    card_y + 290.0,
                    92.0 + scale_boost,
                    if done { accent_mint } else { accent_pink },
                );

                let goal_str = format!("de {} pontos", target_count);
                draw_centered_text(&goal_str, cx, card_y + 332.0, 22.0, title_color);

                // Barra de progresso
                let bar_w = card_w - 72.0;
                let bar_h = 22.0;
                let bar_x = cx - bar_w * 0.5;
                let bar_y = card_y + 362.0;
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

                // Atalhos na parte inferior
                draw_centered_text(
                    "ESPACO: +1 ponto   |   BACKSPACE: -1 ponto",
                    cx,
                    card_y + 432.0,
                    16.0,
                    text_muted,
                );
                draw_centered_text(
                    "R: zerar   |   ESC: mudar meta",
                    cx,
                    card_y + 460.0,
                    16.0,
                    text_muted,
                );
            }
        }

        next_frame().await;
    }
}
