.PHONY: codegen
codegen: 
	substreams protogen substreams.yaml --exclude-paths="sf/substreams,google"

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: stream
stream: build
	substreams run -e polygon.streamingfast.io:443 \
		substreams.yaml \
		map_user_operations \
		--start-block 45666325 \
		--stop-block +2

.PHONY: stream_gui
stream_gui: build
	substreams gui -e polygon.streamingfast.io:443 \
		substreams.yaml \
		db_out \
		--start-block 45645882 \
		--stop-block +1000

.PHONY: setup_db
setup_db:
	substreams-sink-postgres setup "psql://gtms:gtms@localhost:5432/substream_try?sslmode=disable" schema.sql

.PHONY: stream_db
stream_db: build
	substreams run -e polygon.streamingfast.io:443 substreams.yaml map_user_operations --start-block 45666325  --stop-block +2

.PHONY: stream_db2
stream_db2: build
	substreams-sink-postgres run "psql://gtms:gtms@localhost:5432/substream_try?sslmode=disable" "polygon.streamingfast.io:443" "substreams.yaml" db_out "45645882:45677882"

.PHONY: protogen
protogen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"
