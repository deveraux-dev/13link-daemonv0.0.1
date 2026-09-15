RELEASE — FORGE NOSTR DAEMON & RELAY

Sovereign Communications & The Tape Projector
This is my ump based rollback method I use in my repo, I was pretty deadset in switching off git at one point because with my auDHD it is very confusing for a first time operator with 9 months in CS.
23yrs in a trade taught me to respect the tools so when I built this I tried to do the same.
It, like me is not perfect and may not suite your needs as it did mine, there may be dumb shit like hardcodes etc but I am a solo dev who builds too fast and wants to share when things are going well or at the very least my well.. which is sometimes good enough for me
but not everyone. 
Thanks for reading and always down for a chat if you wanna talk shop, I am still new so have a lot to learn. 
This release ships the sovereign Nostr daemon (forge-daemon-door), acting as the localized, deterministic relay for the engine's internal timeline. It operates as a strictly read-only window into the tape.

By default, the relay is sealed (loopback-only). It broadcasts cryptographic proof of the world's state without exposing the core engine to external state mutations.
🛑 Architectural Constraints

    Loopback Default (ARCH-008): Outbound communication defaults to closed. The relay binds strictly to ws://127.0.0.1:13013 unless explicitly overridden per-run via the BeaconValve.

    Tape is Truth: The relay maintains no secondary database. It projects BLAKE3-chained events directly from .forge/timeline.chain.

    Relayed, Never Stored: Ephemeral kinds (like the séance) are broadcast to live listeners at the exact moon-tick and instantly vanish. There is no replay for ephemeral events.

    No Wall-Clock in Core: created_at (UNIX seconds) is stamped only at the egress bridge.

    Zero-Aux Signatures: All events are signed using deterministic BIP-340 Schnorr (k256::schnorr) at the BeatBatch layer.

📻 Protocol & Event Kinds

The daemon translates internal SealedTuple states into standard hex JSON NostrEvent envelopes. It strictly utilizes the following event kinds:

    KIND_TAPE_BEAT (1013): A verified 60-tick engine beat.

    KIND_WORLD_HEAD (30013): The current parameterized state of the world head.

    KIND_SIEVE_13 (21013): Ephemeral events (The Séance of the Second Kind). Broadcast only to active listeners, never stored by the relay.

🛠️ Components Welded
1. Daemon Transport & Egress (nostr_lane.rs)

    Gated behind the FORGE_NOSTR=1 environment variable.

    Upon boot, the daemon mints a seed once to .forge/nostr.seed.

    Implements live self-sign checks and seals every handled call into the timeline audit.

    New Door Verbs: nostr_status (ID 35) and nostr_beat (ID 36) — both strictly read-only and whitelisted.

2. The Envelope (sovereign_comms/envelope.rs)

    Handles OpaqueEnvelope seal, open, and byte serialization.

    Bridges the deterministic integer-only core to the floating JSON web context. The Schnorr signature ensures strangers can verify the payload without trusting the transport.

3. XTASK Integration & Probing

    cargo xtask nostr: New CLI subcommand to probe the daemon on :13013 via DaemonMsg.

    Reads the latest BeatBatch, runs verify, and outputs the telemetry (moon · tick-span · sig validity · valve state).

🚀 Getting Started

To boot the daemon with the Nostr lane active:
Bash

FORGE_NOSTR=1 FORGE_TIMELINE=1 cargo run --bin forgedaemon

To probe the local relay and verify the latest signature:
Bash

cargo xtask nostr

(Note: Public egress via the BeaconValve to external relays or webhooks requires explicit opt-in configuration and is disabled by default.)
