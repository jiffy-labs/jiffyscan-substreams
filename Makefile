.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: stream
stream: build
	substreams run -e polygon.streamingfast.io:443 \
		substreams.yaml \
		graph_out \
		--start-block 45645882 \
		--stop-block +1000

.PHONY: stream_db
stream_db: build
	substreams run -e polygon.streamingfast.io:443 substreams.yaml db_out --start-block 45645882 --stop-block +10000

.PHONY: stream_db2
stream_db2: build
	substreams-sink-postgres run "psql://<username>:<password>@localhost:5432/substream_try?sslmode=disable" "polygon.streamingfast.io:443" "substreams.yaml" db_out

.PHONY: protogen
protogen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"
