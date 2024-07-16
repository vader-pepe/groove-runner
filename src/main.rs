use raylib::{
    color::Color,
    math::{Rectangle, Vector2},
};
use std::{env::current_dir, os::unix::raw::time_t, path::PathBuf};

use raylib::prelude::*;
mod game;
mod player;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(128 * 3, 256 * 3)
        .title("Groove Gunner")
        .build();

    let backgrounds_path = "/assets/textures/SpaceShooterAssetPack_BackGrounds.png";
    let uis_path = "/assets/textures/SpaceShooterAssetPack_IU.png";
    let projectiles_path = "/assets/textures/SpaceShooterAssetPack_Projectiles.png";
    let characters_path = "/assets/textures/SpaceShooterAssetPack_Characters.png";
    let misc_path = "/assets/textures/SpaceShooterAssetPack_Miscellaneous.png";
    let ships_path = "/assets/textures/SpaceShooterAssetPack_Ships.png";
    let font_path = "/assets/fonts/Minecraft.ttf";

    let font = &rl
        .load_font(&thread, pathbuf_into_str(path_utils(font_path)).as_str())
        .expect("failed to load font");
    let backgrounds = &rl
        .load_texture(
            &thread,
            pathbuf_into_str(path_utils(backgrounds_path)).as_str(),
        )
        .expect("cannot load texture!");
    let uis = &rl
        .load_texture(&thread, pathbuf_into_str(path_utils(uis_path)).as_str())
        .expect("cannot load texture!");
    let projectiles = &rl
        .load_texture(
            &thread,
            pathbuf_into_str(path_utils(projectiles_path)).as_str(),
        )
        .expect("cannot load texture!");
    let characters = &rl
        .load_texture(
            &thread,
            pathbuf_into_str(path_utils(characters_path)).as_str(),
        )
        .expect("cannot load texture!");
    let misc = &rl
        .load_texture(&thread, pathbuf_into_str(path_utils(misc_path)).as_str())
        .expect("cannot load texture!");
    let player = &rl
        .load_texture(&thread, pathbuf_into_str(path_utils(ships_path)).as_str())
        .expect("cannot load texture!");

    // let mut pulse_frames: Vec<f32> = vec![3.0; 12];
    let mut frame_counter: i64 = 0;
    let mut current_frame: i64 = 0;
    let frame_speed: i64 = 4;
    rl.set_target_fps(60);
    while !rl.window_should_close() {
        let elapsed_time = &rl.get_time();
        let time_text = format!("{:.2} seconds", elapsed_time);
        frame_counter += 1;
        if frame_counter >= (60 / frame_speed) {
            frame_counter = 0;
            current_frame += 1;
            if current_frame >= 4 {
                current_frame = 0;
            }
        }
        // let frame_time = rl.get_frame_time();
        let mut d = rl.begin_drawing(&thread);
        d.draw_text_ex(
            font,
            &time_text,
            Vector2 { x: 50.0, y: 50.0 },
            50.0,
            0.0,
            Color::WHITE,
        );
        play_pulse_anim(&mut d, current_frame, misc, 1);
        // for (i, frame) in pulse_frames.iter_mut().enumerate() {
        //     if i % 2 == 0 {
        //         play_pulse_anim(&mut d, frame, misc, frame_time, i as i32, 2);
        //     } else {
        //         play_pulse_anim(&mut d, frame, misc, frame_time, i as i32, 3);
        //     }
        // }
        d.draw_texture_pro(
            player,
            Rectangle {
                x: 8.0,
                y: 0.0,
                height: 8.0,
                width: 8.0,
            },
            Rectangle {
                x: ((128 * 3 / 2) - 8) as f32,
                y: ((256 * 3) - 30) as f32,
                height: 8.0 * 4.0,
                width: 8.0 * 4.0,
            },
            Vector2 { x: 0.0, y: 0.0 },
            0.0,
            Color::WHITE,
        );
        d.clear_background(Color::BLACK);
    }
}

fn play_pulse_anim(
    d: &mut RaylibDrawHandle,
    current_frame: i64,
    pulse_texture: &Texture2D,
    // frame_time: f32,
    which_grid: i32,
    // beat: i32,
) {
    // let frame_speed = frame_time * 3.0 * (beat as f32);

    d.draw_texture_pro(
        pulse_texture,
        Rectangle {
            x: (40 + (current_frame * 16)) as f32,
            y: 32.0,
            width: 16.0,
            height: 16.0,
        },
        Rectangle {
            x: (which_grid * 32) as f32,
            y: 0.0,
            width: (16 * 2) as f32,
            height: (16 * 2) as f32,
        },
        Vector2 { x: 0.0, y: 0.0 },
        0.0,
        Color::WHITE,
    );
}

fn path_utils(s: &str) -> PathBuf {
    let root = current_dir().unwrap();
    let mut p = root.into_os_string();
    p.push(s);
    p.into()
}

fn pathbuf_into_str(p: PathBuf) -> String {
    p.into_os_string().into_string().unwrap()
}
