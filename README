# Core Utils Guard

## Description

There were several incidents that invovled bad scripts running `rm $var/*` with empty `$var` which lead to deletion of directory.
This tool helps preventing it.

## Installation

* Build with cargo build
* Copy (or hardlink) executable to `/usr/local/bin` (for brevity we'll assume root looks there too)
* Symlink `rm` to `/usr/local/bin/rm.unsafe`
* Optionally restart the shell
* Test by running `rm _GUARD_DEBUG_ENTRY_` -- this is a test entry which is prevented alongside with other entries, you should get error

```console
thread 'main' panicked at '/usr/local/sbin/rm: Argument contain invalid arg _GUARD_DEBUG_ENTRY_', src/main.rs:46:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```


## List of forbidden items 

* "..",
* "/bin",
* "/boot",
* "/dev",
* "/efi",
* "/etc",
* "/home",
* "/lib",
* "/lib64",
* "/mnt",
* "/opt",
* "/proc",
* "/root",
* "/run",
* "/sbin",
* "/srv",
* "/sys",
* "/tmp",
* "/usr",
* "/var",
* "_GUARD_DEBUG_ENTRY_"

_GUARD_DEBUG_ENTRY_ is used for testing as even if `rm /tmp` will not delete the directory due to it being a directory

List is hard-coded as the tool is intended for my parnoid self.

