# Beet uses Just for common tasks
#
# ```rust 
# cargo binstall just
# just --list
# just test-all
# ```
#
#💡 Init
set windows-shell := ["C:/tools/cygwin/bin/sh.exe","-c"]
set dotenv-load
crates := 'beet beet_spatial beet_flow'

default:
	just --list --unsorted

init-repo:
	just assets-pull
	mkdir -p crates/beet_ml/assets/ml && cp ./assets/ml/default-bert.ron crates/beet_ml/assets/ml/default.bert.ron
# just test-site
# just export-scenes

#💡 CLI

cli *args:
	cargo run -p beet-cli -- {{args}}

run-ws example *args:
	just watch 'just run-ci {{example}} {{args}}'

run-ci example *args:
	cargo run --example {{example}} {{args}}

run crate example *args:
	just watch cargo run -p {{crate}} --example {{example}} {{args}}


doc crate *args:
	just watch cargo doc -p {{crate}} --open {{args}}

fmt *args:
	just watch 'just leptosfmt {{args}}'

leptosfmt *args:
	leptosfmt 												\
	crates/beet_rsx/**/*.rs 					\
	crates/beet_rsx/**/**/*.rs 				\
	crates/beet_rsx/**/**/**/*.rs 		\
	crates/beet_router/**/*.rs 				\
	crates/beet_router/**/**/*.rs 		\
	crates/beet_router/**/**/**/*.rs 	\
	crates/beet_site/**/*.rs 					\
	{{args}}

#💡 e2e examples

run-reactive:
	cp ./index.html ./target/index.html
	sweet serve ./target | \
	just watch 'just build-wasm beet dom_renderer'

run-test-site:
	cargo run -p beet_router --example routes_mod
	cargo run -p beet_router --example templates
	cargo run -p beet_router --example html
	sweet serve target/test_site


run-beet-site:
	just cli serve \
	-p beet_site \
	--src crates/beet_site/src \
	--serve-dir target/client


## common
cmd *args:
	cd /cygdrive/c/work/beet && {{args}}

export-scenes *args:
	cargo run --example export_scenes {{args}}

app *args:
	cargo run --example app {{args}}

# blocked on #https://github.com/bevyengine/bevy/issues/14300
hello-world:
	just app \
	./scenes/beet-debug.json \
	../bevyhub/scenes/camera-2d.json \
	../bevyhub/scenes/ui-terminal-input.json \
	./scenes/hello-world.json

test-all *args:
	cargo fmt 				--check
	just leptosfmt 		--check
	cargo test --workspace										 --features=_doctest							{{args}}
	cargo test 																 --all-features -p beet_flow 			{{args}}
	cargo test 																 --all-features -p beet_rsx 			{{args}}
	cargo test --target wasm32-unknown-unknown --all-features -p beet_flow 			{{args}}
	cargo test --target wasm32-unknown-unknown --all-features -p beet_spatial 	{{args}}
	cargo test --target wasm32-unknown-unknown --all-features -p beet_rsx 			{{args}}
# no space left on device
# cargo test 																 --all-features -p beet_spatial 	{{args}} 

#cargo test -p beet_spatial
#cargo test -p beet_sim
#cargo test -p beet_ml
# cargo test --workspace -- {{args}}
# cargo test --workspace --all-features -- {{args}}

test-doc crate *args:
	just watch 'cargo test -p {{crate}} --doc --features=_doctest {{args}}'
# copied from sweet
test crate *args:
	just watch 'cargo test -p {{crate}} --lib -- --watch {{args}}'
test-e2e crate test_name *args:
	just watch 'cargo test -p {{crate}} --test {{test_name}} -- --watch {{args}}'
test-feat crate *args:
	just watch 'cargo test -p {{crate}} --lib --all-features -- {{args}}'
test-wasm crate *args:
	just watch 'cargo test -p {{crate}} --lib --target wasm32-unknown-unknown -- --watch {{args}}'
test-wasm-feat crate *args:
	just watch 'cargo test -p {{crate}} --lib --target wasm32-unknown-unknown --all-features -- {{args}}'
test-wasm-e2e crate test_name *args:
	just watch 'cargo test -p {{crate}} --test {{test_name}} --target wasm32-unknown-unknown -- --watch {{args}}'

serve-web:
	just serve-wasm

clean-repo:
	cargo clean
	rm -rf ./target
# rm -rf ./Cargo.lock

clean-analyzer:
	rm -rf $CARGO_TARGET_DIR/rust-analyzer

pws *args:
	just --shell powershell.exe --shell-arg -c {{args}}

tree:
	cargo tree --depth=2 -e=no-dev

#### WEB EXAMPLES #####################################################


# Build wasm files, pass --no-build to just update scenes and registries
bevyhub-build *args:
	just export-scenes
	bevyhub build \
	--example app \
	--release \
	--copy-local ../bevyhub-apps \
	--copy-scenes scenes \
	--copy-registries target/registries {{args}}
	bevyhub build \
	--example app_ml \
	--release \
	--copy-local ../bevyhub-apps {{args}}


build-wasm crate example *args:
	cargo build -p {{crate}} --example {{example}} --target wasm32-unknown-unknown {{args}}
	wasm-bindgen \
	--out-dir ./target/wasm \
	--out-name bindgen \
	--target web \
	--no-typescript \
	~/.cargo_target/wasm32-unknown-unknown/debug/examples/{{example}}.wasm

### MISC

expand crate example *args:
	just watch 'cargo expand -p {{crate}} --example {{example}} {{args}}'

patch:
	cargo set-version --bump patch

publish crate *args:
	cargo publish -p {{crate}} --allow-dirty --no-verify {{args}}
	sleep 2

publish-all *args:
	just publish beet_flow_macros     {{args}} || true
	just publish beet_flow            {{args}} || true
	just publish beet_spatial         {{args}} || true
	just publish beet_ml              {{args}} || true
	just publish beet_sim          		{{args}} || true
	just publish beet_rsx_parser      {{args}} || true
	just publish beet_rsx_macros      {{args}} || true
	just publish beet_rsx             {{args}} || true
	just publish beet_router_parser   {{args}} || true
	just publish beet_router          {{args}} || true
	just publish beet                 {{args}} || true
# just publish beet_examples        {{args}} || true

watch *command:
	forky watch \
	-w '**/*.rs' \
	-i '{.git,target,html}/**' \
	-i '**/mod.rs' \
	-- {{command}}

copy-web-assets:
	mkdir -p target/wasm/assets || true
	cp -r ./assets/* target/wasm/assets


copy-wasm-assets:
	rm -rf ./target/static/assets
	mkdir -p ./target/static/assets || true
	
serve-wasm *args:
	cd ./target/static && forky serve {{args}}

watch-assets:
	just watch-web 'just copy-wasm-assets'

watch-web *command:
	forky watch \
	-w '**/*/assets/**/*' \
	-- {{command}}

assets-push:
	aws s3 sync ./assets s3://bevyhub-public/assets --delete
	tar -czvf ./assets.tar.gz ./assets
	aws s3 cp ./assets.tar.gz s3://bevyhub-public/assets.tar.gz
	rm ./assets.tar.gz

assets-pull:
	curl -o ./assets.tar.gz https://bevyhub-public.s3.us-west-2.amazonaws.com/assets.tar.gz
	tar -xzvf ./assets.tar.gz
	rm ./assets.tar.gz



### TEST SCENE LOADS

test-fetch:
	cargo run --example app_ml \
	../bevyhub/scenes/ui-terminal-input.json \
	../bevyhub/scenes/lighting-3d.json \
	../bevyhub/scenes/ground-3d.json \
	./scenes/beet-debug.json \
	./scenes/fetch-scene.json \
	./scenes/fetch-npc.json \


test-flock:
	cargo run --example app \
	../bevyhub/scenes/camera-2d.json \
	../bevyhub/scenes/space-scene.json \
	./scenes/beet-debug.json \
	./scenes/flock.json \

test-seek:
	cargo run --example app \
	../bevyhub/scenes/camera-2d.json \
	../bevyhub/scenes/space-scene.json \
	./scenes/beet-debug.json \
	./scenes/seek.json \

test-frozen-lake-train:
	cargo run --example app_ml \
	../bevyhub/scenes/lighting-3d.json \
	./scenes/frozen-lake-scene.json \
	./scenes/frozen-lake-train.json \

test-frozen-lake-run:
	cargo run --example app_ml \
	../bevyhub/scenes/lighting-3d.json \
	./scenes/frozen-lake-scene.json \
	./scenes/frozen-lake-run.json \

test-hello-animation:
	cargo run --example app \
	../bevyhub/scenes/ui-terminal.json \
	../bevyhub/scenes/lighting-3d.json \
	../bevyhub/scenes/ground-3d.json \
	./scenes/beet-debug.json \
	./scenes/hello-animation.json \

test-hello-ml:
	cargo run --example app_ml \
	../bevyhub/scenes/camera-2d.json \
	../bevyhub/scenes/ui-terminal-input.json \
	./scenes/beet-debug.json \
	./scenes/hello-ml.json \

test-hello-world:
	cargo run --example app \
	../bevyhub/scenes/camera-2d.json \
	../bevyhub/scenes/ui-terminal.json \
	./scenes/beet-debug.json \
	./scenes/hello-world.json \

test-seek-3d:
	cargo run --example app \
	../bevyhub/scenes/ui-terminal.json \
	../bevyhub/scenes/lighting-3d.json \
	../bevyhub/scenes/ground-3d.json \
	./scenes/beet-debug.json \
	./scenes/seek-3d.json \


# https://gist.github.com/stephenhardy/5470814
# 1. Remove the history
# 2. recreate the repos from the current content only
# 3. push to the github remote repos ensuring you overwrite history
very-scary-purge-commit-history:
	rm -rf .git

	git init
	git add .
	git commit -m "Initial commit"

	git remote add origin git@github.com:mrchantey/beet.git
	git push -u --force origin main