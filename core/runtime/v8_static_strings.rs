// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.
macro_rules! v8_static_strings {
  ($($ident:ident = $str:literal),* $(,)?) => {
    $(
      pub static $ident: $crate::FastStaticString = $crate::ascii_str!($str);
    )*
  };
}

v8_static_strings!(
  BUILD_CUSTOM_ERROR = "buildCustomError",
  CALL_CONSOLE = "callConsole",
  CALL_SITE_EVALS = "callSiteEvals",
  CAUSE = "cause",
  CODE = "code",
  CONSOLE = "console",
  CONSTRUCTOR = "constructor",
  CORE = "core",
  DENO = "Deno",
  DIRNAME = "dirname",
  ERR_MODULE_NOT_FOUND = "ERR_MODULE_NOT_FOUND",
  ERRORS = "errors",
  EVENT_LOOP_TICK = "eventLoopTick",
  FILENAME = "filename",
  INSTANTIATE = "instantiate",
  MAIN = "main",
  MESSAGE = "message",
  NAME = "name",
  OPS = "ops",
  RESOLVE = "resolve",
  SET_UP_ASYNC_STUB = "setUpAsyncStub",
  STACK = "stack",
  URL = "url",
  WASM_INSTANTIATE = "wasmInstantiate",
  WEBASSEMBLY = "WebAssembly",
);
