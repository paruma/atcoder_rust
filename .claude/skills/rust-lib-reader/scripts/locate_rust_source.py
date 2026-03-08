# /// script
# requires-python = ">=3.12"
# dependencies = []
# ///
import argparse
import os
from pathlib import Path


def find_std_lib_root() -> Path | None:
    # Use rustc to find the sysroot
    try:
        import subprocess
        sysroot = subprocess.check_output(["rustc", "--print", "sysroot"], text=True).strip()
        base_path = Path(sysroot) / "lib/rustlib/src/rust/library"
        if base_path.exists():
            return base_path
    except Exception:
        pass

    # Fallback to hardcoded path
    rustup_home = "/home/node/.rustup/toolchains"
    if not os.path.exists(rustup_home):
        return None

    toolchains = os.listdir(rustup_home)
    if not toolchains:
        return None

    toolchains.sort()
    base_path = Path(rustup_home) / toolchains[0] / "lib/rustlib/src/rust/library"
    if base_path.exists():
        return base_path
    return None


def find_cargo_registries() -> list[str]:
    cargo_home = "/home/node/.cargo/registry/src"
    if not os.path.exists(cargo_home):
        return []

    return [
        str(Path(cargo_home) / d)
        for d in os.listdir(cargo_home)
        if os.path.isdir(Path(cargo_home) / d)
    ]


def locate_crate(crate_name: str) -> str | None:
    # 1. Check standard library (core, alloc, std)
    if crate_name in ["core", "alloc", "std"]:
        root = find_std_lib_root()
        if root:
            crate_path = root / crate_name
            if crate_path.exists():
                return f"FOUND (std): {crate_path}"

    # 2. Check cargo registry
    registries = find_cargo_registries()
    for registry_path in registries:
        for entry in os.listdir(registry_path):
            # Matches crate name exactly or with version (e.g., "itertools" or "itertools-0.14.0")
            if entry == crate_name or entry.startswith(crate_name + "-"):
                crate_path = Path(registry_path) / entry
                return f"FOUND (crate): {crate_path}"

    return None


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Locate the base directory of a Rust crate (std or third-party)."
    )
    parser.add_argument(
        "crate_name", help="Name of the crate to locate (e.g., std, core, itertools)."
    )

    args = parser.parse_args()

    result = locate_crate(args.crate_name)
    if result:
        print(result)
    else:
        print(f"NOT FOUND: {args.crate_name}")
        exit(1)


if __name__ == "__main__":
    main()
