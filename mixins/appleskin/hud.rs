fn appleskin_hud(
    game: &crate::game::Game<'_>,
    layer: &str,
    batch: &mut horizon_client::hud::HudBatch,
) {
    // Only after the layer that holds the food bar; Horizon skips the layer
    // (and this hook) with the HUD hidden (F1), in creative and in spectator.
    if layer == "minecraft:player_health" {
        game.appleskin_bar(batch);
    }
}
