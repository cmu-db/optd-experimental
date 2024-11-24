#!/bin/bash
rm -rf ~/pgdata
mkdir ~/pgdata
initdb ~/pgdata
pg_ctl -D ~/pgdata start
# default_user is used for cargo run --bin optd-perftest
psql -d postgres -c "CREATE USER default_user WITH SUPERUSER PASSWORD 'password';"
# test_user is used for cargo test --package optd-perftest
psql -d postgres -c "CREATE USER test_user WITH SUPERUSER PASSWORD 'password';"
# Need to apply PGtune before restarting the server. Check https://pgtune.leopard.in.ua/ for your own configuration.
pgtune -i ~/pgdata/postgresql.conf -o ~/pgdata/postgresql.conf
pg_ctl -D ~/pgdata restart
