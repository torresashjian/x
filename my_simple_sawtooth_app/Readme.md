# Setup sawtooth examples

1.- Clone sawtooth rust from https://github.com/hyperledger/sawtooth-sdk-rust
2.- From within sawtooth-sdk-rust directory run
```
docker build -f examples/intkey_rust/Dockerfile-installed-bionic -t sawtooth-intkey-tp-rust-local .
```
3.- Clone dovetail-rust-lib rust from https://github.com/TIBCOSoftware/dovetail-rust-lib
4.- Download https://sawtooth.hyperledger.org/docs/core/releases/latest/app_developers_guide/sawtooth-default.yaml and run
```
docker-compose -f sawtooth-default.yaml up
```

# Setup dovetail sawtooth app

1.- Clone dovetail-rust-lib rust from https://github.com/TIBCOSoftware/dovetail-rust-lib
2.- From within dovetail-rust-lib directory run
```
docker build -f my_simple_sawtooth_app/Dockerfile-installed-bionic -t sawtooth-intkey-tp-rust-local .
```
3.- From within dovetail-rust-lib/my_simple_sawtooth_app run
```
docker-compose -f sawtooth-default.yaml up
```

## To test the intkey app in sawtooth

1.- Go to the shell container
```
docker exec -it sawtooth-shell-default bash
```
2.- within the shell container execute
```
intkey set a 1 --url http://rest-api:8008
```
3.- Check that the value has been set
```
intkey list --url http://rest-api:8008
```



To test the app:

APP_CONFIG_PATH=src/WASM_fromDovetail.json cargo test -- --nocapture