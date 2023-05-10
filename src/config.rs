pub const DIRECTUS_SYSTEM_COLLECTIONS: &[&str] = &[
    "directus_activity",
    "directus_dashboards",
    "directus_fields",
    "directus_flows",
    "directus_folders",
    "directus_migrations",
    "directus_notifications",
    "directus_operations",
    "directus_panels",
    "directus_permissions",
    "directus_presets",
    "directus_relations",
    "directus_revisions",
    "directus_sessions",
    "directus_settings",
    "directus_shares",
    "directus_users",
    "directus_webhooks",
    "directus_collections",
    "directus_files",
    "directus_roles",
];

pub fn get_directus_system_collections() -> Vec<String> {
    return DIRECTUS_SYSTEM_COLLECTIONS
        .iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>(); 
}
