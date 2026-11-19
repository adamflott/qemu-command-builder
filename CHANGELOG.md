# Changelog
All notable changes to this project will be documented in this file.

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

- TAP script disabling now emits `script=` / `downscript=`. Deprecated `no`
  input remains accepted and is normalized to the empty spelling.
- Option catalog regression checks now use the QEMU 11.2.0-rc0 spec.
- Document Hexagon semihosting availability and the HMP build requirement for
  monitor readline mode.

## [10.0.3-6] - 2026-05-04

- Add -g, -watchdog-action
- Refactor args into `args/`
 
## [10.0.3-5] - 2026-05-04

- Implement many more options (thanks to codex)
- Parse a command line string into an instance than be manipulated and displayed as a runnable command
- Add support for Aarch64 machine type
- Add tons of tests

## [10.0.3-0] - 2025-08-20

### Added

- Initial release
