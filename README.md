## Package Manager

Bootstrap seed is WASM runtime.
Compilers are compiled to wasm executed by the bootstrap wasm runtime to compile themself again.
Zigs boostraping is reused but with the our own WASM runtime

Ipfs/Iroh is used for content adressed storing
Nushell is used to create packages specifically nushell is embeded to have full control over the context,
so that extensions become avaiable only when in use with nuka.
Packages call the derivation function at its core which creates a package ir in the nuon (nushell object notation) format.
The hash of that package ir (PIR) determines the path in the local file store.
The local file store is a virtual filesystem over the content addressed store, which is as said before addressed by the hash of the package ir.
The content addressed store also maintains a list of mappings from one cid to many pid.
Therefore when invoking a package installation if the pid is found inside that list, the package contents can directly be downloaded from the cid.

## Scope

For the project to actually have any benefit, it must stand out apart from its content addressed storage which will be implemented by nix in the future anyway.
Nix currently has most problems with its integration in existing tools.
E.g. programs that require a certain FHS directory structure or applications written with language package managers that allow recursive dependencies.
Additionally packages are usually not built with all cpu extensions that are available on the machine.
Therefore the scope of this pm should be to better integrate into existing workflows.
Instead of the approach used by nix, where usually the approach is to replace the chosen package manager of an application in order to make the application
compatible to the strict rules (e.g. no recursive dependencies) and also to create extra environments for applications that require e.g. FHS compatibility,
the approach here should be to reuse the existing language package managers.

Therefore this pm will use an approach with is a mixture of flatpak and nix. Flatpak uses a common freedesktop runtime, which is among others the base for other applications.
Similarly this pm will only package system packages itself which are not packaged by other package managers and are required for the base runtime to run the bare system.
The base runtime will be used for the nuka exec command  which tries to run an external program. But a specific runtime can also be defined
For packages an FHS compatible environment will be created based on language package managers runtimes. These runtimes will have the base runtime included in them as well
as packages needed for the language compilers/interpreter/package managers to work and a list of system packages that are required but not packaged by these packages will be provided as well.

Concretly for a python application using pytorch (assuming pytorch doesnt bundle libtorch), the building steps will be to look up system packages required and finding therefore
libtorch. With the python runtime and libtorch then an FHS environment will be created in where the application can run.
Using xdg-portals similar to flatpak permissions will be enforced.

But some language package managers may also be compatible with the strict rules imposed. E.g. cargo/rust uses static linking and therefore is perfectly compatible.

To further narrowing the scope only programs that adhere to a high standard of memory safety are allowed to directly run inside the system runtime environment. Applications written in C
e.g. which do not adhere to these standards must be packaged with a wasm runtime using emscripten.

Also this pm should be cross platform, so the base runtime must provide only system packages that are available on all major platforms (linux, freebsd, mac, windows) and further system packages
that are only allowed on certain kinds of os e.g. unix-like must be marked as such.


```nu
# modules as variables
module test { export def hello [] { echo "test" } }
const cst = test
use $cst hello
hello
> test

def derivation [name: string, version: string, src: src, packages: list<modules>, build: closure<derivations(build from packages), src>]
```

## Services

Apart from packages also configurations for services and alike should be able to be written. 

## Security

Different from Nix, users should also be able to subscribe to security updates, meaning that they will get notified if a package is not secure anymore and needs updating. The updating process should be manual.

## Local store

The local store could be potentially implemented by a custom filesystem which is implemented underyling by the CAS store and is addressed on the outside by the pid.


## Architecture

daemons: peer 2 peer 
cli: peer with less capabilites

cli <-> system daemon <-> net daemons