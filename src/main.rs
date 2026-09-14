//! Standalone runner for link-core, kept OUTSIDE the F:\v3 workspace on
//! purpose (Sean 2026-08-19: don't drag deployment bloat into v3) --
//! depends on link-core by path, builds its own target dir here.

use link_core::{bridge, config::Config, connection, tls};
use std::sync::Arc;
use std::time::Duration;

fn main() {
    let config = Config::load().expect("load/create config");
    println!("device_id: {}", config.connection.device_id);
    println!("device_name: {}", config.connection.device_name);

    let (cert_path, key_path) = tls::ensure_tls_cert(&Config::config_dir()).expect("tls cert");
    let fingerprint = tls::cert_fingerprint(&std::fs::read(&cert_path).unwrap());
    println!("cert fingerprint (DER-hash pinning uses a different hash of the same cert -- this is the PEM display one): {fingerprint}");

    let tls_config = tls::load_server_config(&cert_path, &key_path).expect("load tls config");

    connection::spawn_discovery_broadcaster(
        config.connection.listen_port,
        config.connection.discovery_port,
        Duration::from_secs(2),
    )
    .expect("spawn discovery broadcaster");
    println!("discovery: broadcasting {{\"port\":{}}} on :{} every 2s", config.connection.listen_port, config.connection.discovery_port);

    let subs = connection::subscribers();
    let table = Arc::new(bridge::PairingTable::new());
    let on_packet = bridge::wire_on_packet(table, subs.clone());

    let addr = format!("0.0.0.0:{}", config.connection.listen_port);
    println!("listening on {addr}");
    connection::accept_loop(&addr, tls_config, subs, on_packet).expect("accept loop");
}
