## On both
- Allow a specific binary to run on high ports: `sudo setcap CAP_NET_BIND_SERVICE=+eip /path/to/binary`

# On Dev
- Make sure the host is set as the server's IP (IP for the client to reference) -> HOST=SERVER_IP (once DNS works, this can instead be the host's name instead of IP)
- Build client: `pnpm build`
- Build server: `cargo build --release`
- Copy binary to prod: `scp target/release/bin root@134.122.15.165:/home/bin`

I feel like working on deployment
- git push -> build -> deploy -> public IP
- learn how to create a database backup

DATABASE_URI=postgresql://jessewright:pass@192.168.0.178:5432/howido-db