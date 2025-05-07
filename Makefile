run:
	cargo run --release dynamicrank_dynamicnet
	cargo run --release dynamicrank_staticnet
	cargo run --release staticrank_dynamicnet
	cargo run --release staticrank_staticnet
	cargo run --release population/20
	cargo run --release population/50
	cargo run --release population/100
	cargo run --release population/200
	cargo run --release population/500
