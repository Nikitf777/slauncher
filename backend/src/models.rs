use serde::{Deserialize, Serialize};

use crate::entities::server::ServerType;

#[derive(Deserialize)]
pub struct CreateServerRequest {
    /// Display name – must match the folder name (alphanumeric + hyphens).
    pub name: String,
    /// Mod loader / server type.
    pub server_type: ServerType,
    /// Minecraft version, e.g. "1.20.1".
    pub minecraft_version: String,
    /// Loader / mod-loader version, e.g. "47.2.0" (Forge) or "0.19.3" (Fabric).
    pub loader_version: String,
}

#[derive(Serialize)]
pub struct ServerResponse {
    pub id: i32,
    pub name: String,
    pub server_type: ServerType,
    pub minecraft_version: String,
    pub loader_version: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ServerProperties {
    pub accepts_transfers: Option<bool>,
    pub allow_flight: Option<bool>,
    pub allow_nether: Option<bool>,
    pub broadcast_console_to_ops: Option<bool>,
    pub broadcast_rcon_to_ops: Option<bool>,
    pub bug_report_link: Option<String>,
    pub chat_spam_threshold_seconds: Option<i32>,
    pub command_spam_threshold_seconds: Option<i32>,
    pub difficulty: Option<String>,
    pub enable_command_block: Option<bool>,
    pub enable_jmx_monitoring: Option<bool>,
    pub enable_query: Option<bool>,
    pub enable_rcon: Option<bool>,
    pub enable_status: Option<bool>,
    pub enforce_secure_profile: Option<bool>,
    pub enforce_whitelist: Option<bool>,
    pub entity_broadcast_range_percentage: Option<i32>,
    pub force_gamemode: Option<bool>,
    pub function_permission_level: Option<i32>,
    pub gamemode: Option<String>,
    pub generate_structures: Option<bool>,
    pub generator_settings: Option<String>,
    pub hardcore: Option<bool>,
    pub hide_online_players: Option<bool>,
    pub initial_disabled_packs: Option<String>,
    pub initial_enabled_packs: Option<String>,
    pub level_name: Option<String>,
    pub level_seed: Option<String>,
    pub level_type: Option<String>,
    pub log_ips: Option<bool>,
    pub management_server_allowed_origins: Option<String>,
    pub management_server_enabled: Option<bool>,
    pub management_server_host: Option<String>,
    pub management_server_port: Option<i32>,
    pub management_server_secret: Option<String>,
    pub management_server_tls_enabled: Option<bool>,
    pub management_server_tls_keystore: Option<String>,
    pub management_server_tls_keystore_password: Option<String>,
    pub max_chained_neighbor_updates: Option<i32>,
    pub max_players: Option<i32>,
    pub max_tick_time: Option<i32>,
    pub max_world_size: Option<i32>,
    pub motd: Option<String>,
    pub network_compression_threshold: Option<i32>,
    pub online_mode: Option<bool>,
    pub op_permission_level: Option<i32>,
    pub pause_when_empty_seconds: Option<i32>,
    pub player_idle_timeout: Option<i32>,
    pub prevent_proxy_connections: Option<bool>,
    #[serde(rename = "query.port")]
    pub query_port: Option<u16>,
    pub rate_limit: Option<i32>,
    #[serde(rename = "rcon.password")]
    pub rcon_password: Option<String>,
    #[serde(rename = "rcon.port")]
    pub rcon_port: Option<u16>,
    pub region_file_compression: Option<String>,
    pub require_resource_pack: Option<bool>,
    pub resource_pack: Option<String>,
    pub resource_pack_id: Option<String>,
    pub resource_pack_prompt: Option<String>,
    pub resource_pack_sha1: Option<String>,
    pub server_ip: Option<String>,
    pub server_port: Option<u16>,
    pub simulation_distance: Option<i32>,
    pub spawn_monsters: Option<bool>,
    pub spawn_protection: Option<i32>,
    pub status_heartbeat_interval: Option<i32>,
    pub sync_chunk_writes: Option<bool>,
    pub text_filtering_config: Option<String>,
    pub text_filtering_version: Option<i32>,
    pub use_native_transport: Option<bool>,
    pub view_distance: Option<i32>,
    pub white_list: Option<bool>,
    pub pvp: Option<bool>,
}

impl ServerProperties {
    pub fn validate(&self) -> Result<(), String> {
        let ports: [(&str, Option<u16>); 3] = [
            ("server-port", self.server_port),
            ("rcon.port", self.rcon_port),
            ("query.port", self.query_port),
        ];
        for (name, value) in &ports {
            if let Some(v) = value {
                if *v == 0 {
                    return Err(format!("{name} must be between 1 and 65535"));
                }
            }
        }
        Ok(())
    }

    /// Collect non-None fields into key–value string pairs ready for
    /// serialisation into a `.properties` file.
    pub fn to_updates(&self) -> Vec<(&'static str, String)> {
        let mut updates: Vec<(&'static str, String)> = Vec::new();

        macro_rules! push_bool {
            ($field:ident, $key:expr) => {
                if let Some(v) = self.$field {
                    updates.push(($key, if v { "true".into() } else { "false".into() }));
                }
            };
        }
        macro_rules! push_int {
            ($field:ident, $key:expr) => {
                if let Some(v) = self.$field {
                    updates.push(($key, v.to_string()));
                }
            };
        }
        macro_rules! push_str {
            ($field:ident, $key:expr) => {
                if let Some(ref v) = self.$field {
                    updates.push(($key, v.clone()));
                }
            };
        }

        push_bool!(accepts_transfers, "accepts-transfers");
        push_bool!(allow_flight, "allow-flight");
        push_bool!(allow_nether, "allow-nether");
        push_bool!(broadcast_console_to_ops, "broadcast-console-to-ops");
        push_bool!(broadcast_rcon_to_ops, "broadcast-rcon-to-ops");
        push_str!(bug_report_link, "bug-report-link");
        push_int!(chat_spam_threshold_seconds, "chat-spam-threshold-seconds");
        push_int!(command_spam_threshold_seconds, "command-spam-threshold-seconds");
        push_str!(difficulty, "difficulty");
        push_bool!(enable_command_block, "enable-command-block");
        push_bool!(enable_jmx_monitoring, "enable-jmx-monitoring");
        push_bool!(enable_query, "enable-query");
        push_bool!(enable_rcon, "enable-rcon");
        push_bool!(enable_status, "enable-status");
        push_bool!(enforce_secure_profile, "enforce-secure-profile");
        push_bool!(enforce_whitelist, "enforce-whitelist");
        push_int!(entity_broadcast_range_percentage, "entity-broadcast-range-percentage");
        push_bool!(force_gamemode, "force-gamemode");
        push_int!(function_permission_level, "function-permission-level");
        push_str!(gamemode, "gamemode");
        push_bool!(generate_structures, "generate-structures");
        push_str!(generator_settings, "generator-settings");
        push_bool!(hardcore, "hardcore");
        push_bool!(hide_online_players, "hide-online-players");
        push_str!(initial_disabled_packs, "initial-disabled-packs");
        push_str!(initial_enabled_packs, "initial-enabled-packs");
        push_str!(level_name, "level-name");
        push_str!(level_seed, "level-seed");
        push_str!(level_type, "level-type");
        push_bool!(log_ips, "log-ips");
        push_str!(management_server_allowed_origins, "management-server-allowed-origins");
        push_bool!(management_server_enabled, "management-server-enabled");
        push_str!(management_server_host, "management-server-host");
        push_int!(management_server_port, "management-server-port");
        push_str!(management_server_secret, "management-server-secret");
        push_bool!(management_server_tls_enabled, "management-server-tls-enabled");
        push_str!(management_server_tls_keystore, "management-server-tls-keystore");
        push_str!(management_server_tls_keystore_password, "management-server-tls-keystore-password");
        push_int!(max_chained_neighbor_updates, "max-chained-neighbor-updates");
        push_int!(max_players, "max-players");
        push_int!(max_tick_time, "max-tick-time");
        push_int!(max_world_size, "max-world-size");
        push_str!(motd, "motd");
        push_int!(network_compression_threshold, "network-compression-threshold");
        push_bool!(online_mode, "online-mode");
        push_int!(op_permission_level, "op-permission-level");
        push_int!(pause_when_empty_seconds, "pause-when-empty-seconds");
        push_int!(player_idle_timeout, "player-idle-timeout");
        push_bool!(prevent_proxy_connections, "prevent-proxy-connections");
        push_int!(query_port, "query.port");
        push_int!(rate_limit, "rate-limit");
        push_str!(rcon_password, "rcon.password");
        push_int!(rcon_port, "rcon.port");
        push_str!(region_file_compression, "region-file-compression");
        push_bool!(require_resource_pack, "require-resource-pack");
        push_str!(resource_pack, "resource-pack");
        push_str!(resource_pack_id, "resource-pack-id");
        push_str!(resource_pack_prompt, "resource-pack-prompt");
        push_str!(resource_pack_sha1, "resource-pack-sha1");
        push_str!(server_ip, "server-ip");
        push_int!(server_port, "server-port");
        push_int!(simulation_distance, "simulation-distance");
        push_bool!(spawn_monsters, "spawn-monsters");
        push_int!(spawn_protection, "spawn-protection");
        push_int!(status_heartbeat_interval, "status-heartbeat-interval");
        push_bool!(sync_chunk_writes, "sync-chunk-writes");
        push_str!(text_filtering_config, "text-filtering-config");
        push_int!(text_filtering_version, "text-filtering-version");
        push_bool!(use_native_transport, "use-native-transport");
        push_int!(view_distance, "view-distance");
        push_bool!(white_list, "white-list");
        push_bool!(pvp, "pvp");

        updates
    }
}
