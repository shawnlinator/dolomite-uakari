use std::collections::HashSet;

use anyhow::{Error, bail};
use openssl::ssl::{SslAcceptor, SslMethod};

use pbs_api_types::NodeConfig;
use proxmox_http::ProxyConfig;
use proxmox_product_config::replace_privileged_config;
use proxmox_schema::ApiType;

use pbs_buildcfg::configdir;

use crate::{BackupLockGuard, open_backup_lockfile};

const CONF_FILE: &str = configdir!("/node.cfg");
const LOCK_FILE: &str = configdir!("/.node.lck");

pub fn lock() -> Result<BackupLockGuard, Error> {
    open_backup_lockfile(LOCK_FILE, None, true)
}

/// Read the Node Config.
pub fn config() -> Result<(NodeConfig, [u8; 32]), Error> {
    let content = proxmox_sys::fs::file_read_optional_string(CONF_FILE)?.unwrap_or_default();

    let digest = openssl::sha::sha256(content.as_bytes());
    let data: NodeConfig = proxmox_simple_config::from_str(&content, &NodeConfig::API_SCHEMA)?;

    Ok((data, digest))
}

/// Write the Node Config, requires the write lock to be held.
pub fn save_config(config: &NodeConfig) -> Result<(), Error> {
    let mut domains = HashSet::new();
    for domain in config.acme_domains() {
        let domain = domain?;
        if !domains.insert(domain.domain.to_lowercase()) {
            bail!("duplicate domain '{}' in ACME config", domain.domain);
        }
    }
    let mut dummy_acceptor = SslAcceptor::mozilla_intermediate_v5(SslMethod::tls()).unwrap();
    if let Some(ciphers) = config.ciphers_tls_1_3.as_deref() {
        dummy_acceptor.set_ciphersuites(ciphers)?;
    }
    if let Some(ciphers) = config.ciphers_tls_1_2.as_deref() {
        dummy_acceptor.set_cipher_list(ciphers)?;
    }

    let raw = proxmox_simple_config::to_bytes(config, &NodeConfig::API_SCHEMA)?;
    replace_privileged_config(CONF_FILE, &raw)
}

pub fn node_http_proxy_config() -> Result<Option<ProxyConfig>, Error> {
    let (node_config, _digest) = self::config()?;
    Ok(node_config.http_proxy())
}

#[cfg(test)]
mod tests {
    use pbs_api_types::NodeConfig;
    use proxmox_schema::ApiType;

    #[test]
    fn node_config_roundtrip() {
        const NODE_CONFIG: &str = "\
            acme: account=pebble\n\
            acmedomain0: test1.invalid.local,plugin=power\n\
            acmedomain1: test2.invalid.local\n\
        ";

        let data: NodeConfig =
            proxmox_simple_config::from_str(NODE_CONFIG, &NodeConfig::API_SCHEMA)
                .expect("failed to parse node config");

        let raw = proxmox_simple_config::to_bytes(&data, &NodeConfig::API_SCHEMA)
            .expect("failed to serialize node config");

        assert_eq!(raw, NODE_CONFIG.as_bytes());
    }

    #[test]
    fn node_config_with_description_roundtrip() {
        const NODE_INPUT: &str = "\
            #this should\n\
            #be included\n\
            acme: account=pebble\n\
            # this should not\n\
            acmedomain0: test1.invalid.local,plugin=power\n\
            acmedomain1: test2.invalid.local\n\
        ";

        const NODE_OUTPUT: &str = "\
            #this should\n\
            #be included\n\
            acme: account=pebble\n\
            acmedomain0: test1.invalid.local,plugin=power\n\
            acmedomain1: test2.invalid.local\n\
        ";

        let data: NodeConfig = proxmox_simple_config::from_str(NODE_INPUT, &NodeConfig::API_SCHEMA)
            .expect("failed to parse node config with description");

        let raw = proxmox_simple_config::to_bytes(&data, &NodeConfig::API_SCHEMA)
            .expect("failed to serialize node config with description");

        assert_eq!(raw, NODE_OUTPUT.as_bytes());
    }
}
