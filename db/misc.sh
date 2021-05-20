# FOR LINUX
# pg_hba.conf (host based configuration)
host    all             all              0.0.0.0/0                       md5
host    all             all              ::/0                            md5

# Set in /etc/postgresql/12/main/postgresql.conf
listen_addresses = '0.0.0.0'		# what IP address(es) to listen on;

# Restart postgres
systemctl restart postgresql.service
# OR (manual):
pg_ctl -D /usr/local/var/postgres start
pg_ctl -D /usr/local/var/postgres restart

# FOR MACOS
brew update
brew install postgresql
brew services start postgresql

# BOTH
initdb /usr/local/var/postgres

createdb howido-db

psql howido-db
