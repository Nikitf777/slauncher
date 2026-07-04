use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
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
	/// Parse a `.properties` file content into a `ServerProperties` struct.
	/// Unknown keys are silently ignored.
	pub fn from_str(content: &str) -> Self {
		use std::collections::HashMap;

		let mut map: HashMap<&str, &str> = HashMap::new();
		for line in content.lines() {
			let trimmed = line.trim();
			if trimmed.is_empty() || trimmed.starts_with('#') {
				continue;
			}
			if let Some(eq_pos) = trimmed.find('=') {
				let key = &trimmed[..eq_pos];
				let val = &trimmed[eq_pos + 1..];
				map.insert(key, val);
			}
		}

		macro_rules! get_bool {
			($key:expr) => {
				map.get($key).and_then(|v| match *v {
					"true" => Some(true),
					"false" => Some(false),
					_ => None,
				})
			};
		}
		macro_rules! get_int {
			($key:expr) => {
				map.get($key).and_then(|v| v.parse::<i32>().ok())
			};
		}
		macro_rules! get_u16 {
			($key:expr) => {
				map.get($key).and_then(|v| v.parse::<u16>().ok())
			};
		}
		macro_rules! get_str {
			($key:expr) => {
				map.get($key).map(|v| v.to_string())
			};
		}

		ServerProperties {
			accepts_transfers: get_bool!("accepts-transfers"),
			allow_flight: get_bool!("allow-flight"),
			allow_nether: get_bool!("allow-nether"),
			broadcast_console_to_ops: get_bool!("broadcast-console-to-ops"),
			broadcast_rcon_to_ops: get_bool!("broadcast-rcon-to-ops"),
			bug_report_link: get_str!("bug-report-link"),
			chat_spam_threshold_seconds: get_int!("chat-spam-threshold-seconds"),
			command_spam_threshold_seconds: get_int!("command-spam-threshold-seconds"),
			difficulty: get_str!("difficulty"),
			enable_command_block: get_bool!("enable-command-block"),
			enable_jmx_monitoring: get_bool!("enable-jmx-monitoring"),
			enable_query: get_bool!("enable-query"),
			enable_rcon: get_bool!("enable-rcon"),
			enable_status: get_bool!("enable-status"),
			enforce_secure_profile: get_bool!("enforce-secure-profile"),
			enforce_whitelist: get_bool!("enforce-whitelist"),
			entity_broadcast_range_percentage: get_int!("entity-broadcast-range-percentage"),
			force_gamemode: get_bool!("force-gamemode"),
			function_permission_level: get_int!("function-permission-level"),
			gamemode: get_str!("gamemode"),
			generate_structures: get_bool!("generate-structures"),
			generator_settings: get_str!("generator-settings"),
			hardcore: get_bool!("hardcore"),
			hide_online_players: get_bool!("hide-online-players"),
			initial_disabled_packs: get_str!("initial-disabled-packs"),
			initial_enabled_packs: get_str!("initial-enabled-packs"),
			level_name: get_str!("level-name"),
			level_seed: get_str!("level-seed"),
			level_type: get_str!("level-type"),
			log_ips: get_bool!("log-ips"),
			management_server_allowed_origins: get_str!("management-server-allowed-origins"),
			management_server_enabled: get_bool!("management-server-enabled"),
			management_server_host: get_str!("management-server-host"),
			management_server_port: get_int!("management-server-port"),
			management_server_secret: get_str!("management-server-secret"),
			management_server_tls_enabled: get_bool!("management-server-tls-enabled"),
			management_server_tls_keystore: get_str!("management-server-tls-keystore"),
			management_server_tls_keystore_password: get_str!(
				"management-server-tls-keystore-password"
			),
			max_chained_neighbor_updates: get_int!("max-chained-neighbor-updates"),
			max_players: get_int!("max-players"),
			max_tick_time: get_int!("max-tick-time"),
			max_world_size: get_int!("max-world-size"),
			motd: get_str!("motd"),
			network_compression_threshold: get_int!("network-compression-threshold"),
			online_mode: get_bool!("online-mode"),
			op_permission_level: get_int!("op-permission-level"),
			pause_when_empty_seconds: get_int!("pause-when-empty-seconds"),
			player_idle_timeout: get_int!("player-idle-timeout"),
			prevent_proxy_connections: get_bool!("prevent-proxy-connections"),
			query_port: get_u16!("query.port"),
			rate_limit: get_int!("rate-limit"),
			rcon_password: get_str!("rcon.password"),
			rcon_port: get_u16!("rcon.port"),
			region_file_compression: get_str!("region-file-compression"),
			require_resource_pack: get_bool!("require-resource-pack"),
			resource_pack: get_str!("resource-pack"),
			resource_pack_id: get_str!("resource-pack-id"),
			resource_pack_prompt: get_str!("resource-pack-prompt"),
			resource_pack_sha1: get_str!("resource-pack-sha1"),
			server_ip: get_str!("server-ip"),
			server_port: get_u16!("server-port"),
			simulation_distance: get_int!("simulation-distance"),
			spawn_monsters: get_bool!("spawn-monsters"),
			spawn_protection: get_int!("spawn-protection"),
			status_heartbeat_interval: get_int!("status-heartbeat-interval"),
			sync_chunk_writes: get_bool!("sync-chunk-writes"),
			text_filtering_config: get_str!("text-filtering-config"),
			text_filtering_version: get_int!("text-filtering-version"),
			use_native_transport: get_bool!("use-native-transport"),
			view_distance: get_int!("view-distance"),
			white_list: get_bool!("white-list"),
			pvp: get_bool!("pvp"),
		}
	}

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
		push_int!(
			command_spam_threshold_seconds,
			"command-spam-threshold-seconds"
		);
		push_str!(difficulty, "difficulty");
		push_bool!(enable_command_block, "enable-command-block");
		push_bool!(enable_jmx_monitoring, "enable-jmx-monitoring");
		push_bool!(enable_query, "enable-query");
		push_bool!(enable_rcon, "enable-rcon");
		push_bool!(enable_status, "enable-status");
		push_bool!(enforce_secure_profile, "enforce-secure-profile");
		push_bool!(enforce_whitelist, "enforce-whitelist");
		push_int!(
			entity_broadcast_range_percentage,
			"entity-broadcast-range-percentage"
		);
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
		push_str!(
			management_server_allowed_origins,
			"management-server-allowed-origins"
		);
		push_bool!(management_server_enabled, "management-server-enabled");
		push_str!(management_server_host, "management-server-host");
		push_int!(management_server_port, "management-server-port");
		push_str!(management_server_secret, "management-server-secret");
		push_bool!(
			management_server_tls_enabled,
			"management-server-tls-enabled"
		);
		push_str!(
			management_server_tls_keystore,
			"management-server-tls-keystore"
		);
		push_str!(
			management_server_tls_keystore_password,
			"management-server-tls-keystore-password"
		);
		push_int!(max_chained_neighbor_updates, "max-chained-neighbor-updates");
		push_int!(max_players, "max-players");
		push_int!(max_tick_time, "max-tick-time");
		push_int!(max_world_size, "max-world-size");
		push_str!(motd, "motd");
		push_int!(
			network_compression_threshold,
			"network-compression-threshold"
		);
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
