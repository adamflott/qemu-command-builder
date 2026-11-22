# Changelog
All notable changes to this project will be documented in this file.

Release dates follow the [published version history](https://docs.rs/crate/qemu-command-builder/latest).
Historical entries were reconstructed from Git history and published versions.

## [Unreleased]

### Added

- QEMU 11.2 `-compat` input policy `warn` and `insecure-input`, with independently
  selectable and combined compatibility policies.
- s390x command parsing/building with `MachineS390x`, `CpuS390x`, and
  `QemuInstanceForS390x`, including `boot-certs.N.path` and `secure-boot`.
- Hexagon command parsing/building with `MachineHexagon`, `CpuHexagon`, and
  `QemuInstanceForHexagon`, including `-semihosting` and `-semihosting-config`.
- `Object::igvm_cfg` helper and support for its optional `cmdline` property,
  including literal commas in object property values.

### Changed

- Add a repeatable cargo-release process and repair release workflow tagging and packaging.
- TAP script disabling now emits `script=` / `downscript=`. Deprecated `no`
  input remains accepted and is normalized to the empty spelling.
- Option catalog regression checks now use the QEMU 11.2.0-rc0 spec.
- Document Hexagon semihosting availability and the HMP build requirement for
  monitor readline mode.

## [11.1.0-1] - 2026-08-26

### Added

- QEMU 11.1 option support for chardev, display, fsdev, and virtfs.
- Missing top-level options and aliases, structured option properties, and
  validation and command round-trip regression tests.

### Fixed

- Incorrect command generation, structured option handling, and option
  repeatability/cardinality mismatches.

### Changed

- Update dependencies and the QEMU option specification to 11.1.0.

## [11.0.0-1] - 2026-05-05

### Added

- Confidential guest support in machine configuration.

## [11.0.0-0] - 2026-05-05

### Added

- QEMU 11.0 option support for machine, accelerator, netdev, and run-with settings.
- `parse_qemu_command` utility to parse command files and display results.

### Changed

- Update the QEMU option specification to 11.0.0.

## [10.0.3-6] - 2026-05-05

- Add -g, -watchdog-action
- Refactor args into `args/`
 
## [10.0.3-5] - 2026-05-04

- Implement many more options (thanks to codex)
- Parse a command line string into an instance than be manipulated and displayed as a runnable command
- Add support for Aarch64 machine type
- Add tons of tests

## [10.0.3-4] - 2025-11-06

### Fixed

- Generate machine arguments for all supported machine types, including AArch64.
- Remove the implicit conversion requirement on the CPU builder field.

## [10.0.3-3] - 2025-11-06

### Added

- Initial AArch64 machine support and additional struct/enum derives.

## [10.0.3-2] - 2025-11-01

### Added

- AArch64 CPU types, byte memory units, and a chardev ID getter.

### Fixed

- Allow repeated `-mon` options and treat `-initrd` as a file path.
- Expand accelerator derives and fix failing tests.

## [10.0.3-1] - 2025-11-01

### Added

- `-object` support.

### Changed

- Update dependencies and checkout action.

## [10.0.3-0] - 2025-08-20

### Added

- Initial release
