const child_process = require('child_process');
const fs = require('fs');

function set_env(name, val) {
  fs.appendFileSync(process.env['GITHUB_ENV'], `${name}=${val}\n`)
}

// Needed for now to get 1.24.2 which fixes a bug in 1.24.1 that causes issues
// on Windows.
if (process.platform === 'win32') {
  child_process.execFileSync('rustup', ['self', 'update']);
}

// Save disk space by avoiding incremental compilation, and also we don't use
// any caching so incremental wouldn't help anyway.
set_env("CARGO_INCREMENTAL", "0");

// Turn down debuginfo from 2 to 1 to help save disk space
set_env("CARGO_PROFILE_DEV_DEBUG", "1");
set_env("CARGO_PROFILE_TEST_DEBUG", "1");

if (process.platform === 'darwin') {
  set_env("CARGO_PROFILE_DEV_SPLIT_DEBUGINFO", "unpacked");
  set_env("CARGO_PROFILE_TEST_SPLIT_DEBUGINFO", "unpacked");
}
