use macroquad::prelude::*;

use macroquad_platformer::*;

use macroquad_tiled as tiled;

struct Player {
    collider: Actor,
    speed: Vec2,
}

mod consts {
    pub const JUMP_SPEED: f32 = -700.0;
    pub const GRAVITY: f32 = 2000.0;
    pub const MOVE_SPEED: f32 = 300.0;
}
#[macroquad::main("Fishgame")]
async fn main() {
    let tileset = load_texture("assets/tileset.png").await.unwrap();
    tileset.set_filter(FilterMode::Nearest);

    let decorations = load_texture("assets/decorations1.png").await.unwrap();
    decorations.set_filter(FilterMode::Nearest);

    let tiled_map_json = load_string("assets/map.json").await.unwrap();
    let tiled_map = tiled::load_map(
        &tiled_map_json,
        &[("tileset.png", tileset), ("decorations1.png", decorations)],
        &[],
    )
    .unwrap();

    let mut static_colliders = vec![];
    for (_x, _y, tile) in tiled_map.tiles("main layer", None) {
        static_colliders.push(tile.is_some());
    }

    let mut world = World::new();
    world.add_static_tiled_layer(
        static_colliders,
        tiled_map.raw_tiled_map.tilewidth as f32,
        tiled_map.raw_tiled_map.tileheight as f32,
        tiled_map.raw_tiled_map.width as _,
        1,
    );

    let mut player = Player {
        collider: world.add_actor(vec2(200.0, 100.0), 36, 66),
        speed: vec2(0., 0.),
    };

    let whale = load_texture("assets/Whale/Whale(76x66)(Orange).png")
        .await
        .unwrap();

    let width = tiled_map.raw_tiled_map.tilewidth * tiled_map.raw_tiled_map.width;
    let height = tiled_map.raw_tiled_map.tileheight * tiled_map.raw_tiled_map.height;

    loop {
        clear_background(BLACK);

        tiled_map.draw_tiles(
            "main layer",
            Rect::new(0.0, 0.0, width as _, height as _),
            None,
        );

        let pos = world.actor_pos(player.collider);

        draw_texture_ex(
            whale,
            pos.x - 20.,
            pos.y,
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(0.0, 0.0, 76., 66.)),
                ..Default::default()
            },
        );

        {
            let pos = world.actor_pos(player.collider);
            let on_ground = world.collide_check(player.collider, pos + vec2(0., 1.));

            if on_ground == false {
                player.speed.y += consts::GRAVITY * get_frame_time();
            }

            if is_key_down(KeyCode::Right) {
                player.speed.x = consts::MOVE_SPEED;
            } else if is_key_down(KeyCode::Left) {
                player.speed.x = -consts::MOVE_SPEED;
            } else {
                player.speed.x = 0.;
            }

            if is_key_pressed(KeyCode::Space) {
                if on_ground {
                    player.speed.y = consts::JUMP_SPEED;
                }
            }

            world.move_h(player.collider, player.speed.x * get_frame_time());
            world.move_v(player.collider, player.speed.y * get_frame_time());
        }

        next_frame().await;
    }
}
