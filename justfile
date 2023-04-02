hello:
  @echo "hello world"

install:
  cargo build --release && cp target/release/wm ~/wm

local:
  cargo build && ./scripts/xephyr.sh
