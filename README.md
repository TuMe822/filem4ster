# FileM4ster

FileM4ster is a desktop application for safe file migration, verification,
backup and storage management.

The project is being built with a safety-first philosophy: FileM4ster should
plan and preview file operations before executing them, preserve source data
where possible, verify completed transfers and maintain a complete job history.

> **FileM4ster is currently under active development. The current target is v0.1.**

## Why FileM4ster?

Moving large amounts of data between disks, NAS devices, servers and cloud
services often requires a collection of separate command-line tools.

FileM4ster aims to provide a unified graphical interface for these workflows
while using reliable tools such as `rsync` and, later, `rclone` underneath.

The core workflow is:

```text
SCAN
  ↓
ANALYZE
  ↓
PLAN
  ↓
PREVIEW
  ↓
USER APPROVAL
  ↓
EXECUTE
  ↓
VERIFY
  ↓
LOG
```

The goal is not simply to move files from one location to another.

FileM4ster should make potentially large and important file operations
understandable, resumable and verifiable.

---

## v0.1 — Safe Copy

The first release focuses on one task:

> Reliable and understandable file migration.

Planned functionality:

- Source and destination selection
- Preflight checks
- Dry run / preview
- Safe Copy
- Transfer progress
- Current file display
- Transfer speed
- Remaining time estimate
- Safe cancellation
- Resume interrupted transfers
- Verification
- Error logging
- Job history
- Remote execution on TinyMonkey

Destructive synchronization, mirroring and automatic deletion are
intentionally outside the initial v0.1 scope.

---

## Technology

| Component | Technology |
|---|---|
| Desktop framework | Tauri 2 |
| User interface | React |
| Frontend language | TypeScript |
| Core / backend | Rust |
| Initial transfer engine | rsync |
| Future cloud transfer engine | rclone |
| Database | SQLite |
| Version control | Git |
| Remote repository | GitHub |

---

## Architecture

The initial FileM4ster architecture is:

```text
┌──────────────────────────────┐
│       FileM4ster.app         │
│                              │
│   React + TypeScript UI      │
│            │                 │
│          Tauri               │
│            │                 │
│        Rust Core             │
└────────────┬─────────────────┘
             │
             ▼
           rsync
             │
             ▼
        Filesystems
```

### Responsibilities

**React + TypeScript**

Responsible for:

- User interface
- Transfer configuration
- Progress display
- Job status
- User interaction

**Tauri**

Connects the web-based user interface to the native desktop application and
Rust backend.

**Rust**

Responsible for:

- Filesystem operations
- Safety checks
- Path validation
- Job logic
- Process execution
- Transfer control
- Verification
- Communication between the transfer engine and the UI

**rsync**

Used as the initial transfer engine for reliable file copying.

FileM4ster will control how `rsync` is used instead of allowing the frontend to
construct arbitrary shell commands.

---

## Future architecture

Later versions may introduce a dedicated FileM4ster Agent.

```text
                 FileM4ster.app
                       │
                  control plane
                       │
          ┌────────────┼────────────┐
          ▼            ▼            ▼
     TinyMonkey    BackupServer   Other Worker
          │            │            │
          ▼            ▼            ▼
       Storage       Storage       Storage
```

The agent would allow long-running transfer jobs to continue on remote
computers even when the FileM4ster desktop application is closed.

For example:

```text
Source:
Buffalo NAS

Destination:
TinyMonkey

Worker:
TinyMonkey
```

The actual data path would then be:

```text
Buffalo NAS
     │
     ▼
TinyMonkey
```

instead of:

```text
Buffalo NAS
     │
     ▼
Mac
     │
     ▼
TinyMonkey
```

---

## First real-world use case

The reference use case for FileM4ster v0.1 is a real NAS migration.

### Source

```text
Buffalo LS210D
```

The NAS contains approximately:

```text
1.9 TB of data
```

The Buffalo is an older NAS device and currently acts as the source of the
migration.

### Destination

The destination is a Debian server named:

```text
TinyMonkey
```

A new 4 TB storage disk is mounted on TinyMonkey at:

```text
/srv/storage
```

The Buffalo NAS is mounted on TinyMonkey as a read-only source:

```text
/mnt/buffalo
```

The migration path is therefore:

```text
Buffalo LS210D
      │
      │ SMB / read-only source
      ▼
/mnt/buffalo
      │
      │ Safe Copy
      ▼
/srv/storage/Buffalo
      │
      ▼
4 TB ext4 storage
```

The source is deliberately mounted read-only to reduce the risk of accidentally
modifying or deleting the original data during migration.

---

## Current manual migration

The current migration is performed manually with `rsync`.

The transfer runs directly on TinyMonkey instead of moving approximately
1.9 TB of data through the Mac.

The current data path is:

```text
Buffalo
   │
   │ network
   ▼
TinyMonkey
   │
   │ local filesystem
   ▼
4 TB storage disk
```

A long-running transfer is currently executed inside a `tmux` session so that
an SSH connection failure does not terminate the copy operation.

The current transfer concept is approximately:

```bash
rsync -rltvh \
  --info=progress2 \
  --stats \
  --partial \
  --no-perms \
  --no-owner \
  --no-group \
  --log-file=/srv/storage/buffalo-copy.log \
  /mnt/buffalo/ \
  /srv/storage/Buffalo/
```

One of the primary goals of FileM4ster v0.1 is to reproduce this workflow from
a graphical user interface without requiring the user to manually use:

- `mount`
- `tmux`
- `rsync`
- SSH commands

FileM4ster should eventually handle the workflow safely on behalf of the user.

---

## FileM4ster workflow

A transfer job should follow a predictable sequence:

```text
SOURCE
   │
   ▼
DESTINATION
   │
   ▼
PREFLIGHT
   │
   ▼
PREVIEW
   │
   ▼
USER APPROVAL
   │
   ▼
SAFE COPY
   │
   ▼
VERIFY
   │
   ▼
JOB HISTORY
```

### Preflight

Before copying anything, FileM4ster should verify things such as:

- Source exists
- Source is readable
- Destination exists
- Destination is writable
- Destination has enough free space
- Source size can be determined
- File count can be determined
- Potential conflicts can be identified

Nothing should be modified during preflight.

### Preview

The user should be able to see what FileM4ster intends to do before allowing
the operation to start.

Example:

```text
READY TO COPY

Source:
Buffalo

Destination:
TinyMonkey

Files:
200,924

Data:
1.9 TB

Destination free:
3.6 TB

Source protection:
Read only

Nothing has been changed yet.
```

### Safe Copy

Safe Copy is the initial transfer policy.

Its design goals are:

- Do not delete source files
- Do not automatically delete destination files
- Preserve timestamps where appropriate
- Allow interrupted transfers to resume
- Log errors
- Report progress
- Verify completed transfers
- Avoid destructive defaults

---

## Planned transfer progress

During a transfer, FileM4ster should eventually display information similar to:

```text
BUFFALO MIGRATION

████████████████░░░░░░░░  63 %

1.19 TB / 1.89 TB

Files:
126,483 / 200,924

Current file:
Pictures/2019/IMG_4829.JPG

Speed:
74.2 MB/s

Errors:
0

Elapsed:
04:32:18

Remaining:
02:41:07
```

The exact interface will evolve during development.

---

## Verification

Copying data is not enough.

FileM4ster should also be able to verify that the destination contains the data
that was intended to be copied.

Possible verification levels include:

### Quick verification

May compare:

- File path
- File name
- File size
- Timestamp

### Full verification

May additionally compare cryptographic hashes of source and destination files.

A completed migration should eventually produce a clear summary such as:

```text
Migration completed

Files copied:
200,924

Data copied:
1.89 TB

Verified:
200,924

Missing:
0

Failed:
0

Status:
VERIFIED
```

---

## Planned job system

Every significant FileM4ster operation should be represented as a job.

Example:

```text
Job:
Buffalo migration

Source:
/mnt/buffalo

Destination:
/srv/storage/Buffalo

Mode:
Safe Copy

Status:
Running
```

Jobs should eventually support:

- Pending
- Running
- Paused
- Interrupted
- Failed
- Completed
- Verified

---

## Project status

Current baseline:

- [x] Project created
- [x] Tauri 2 configured
- [x] React + TypeScript frontend
- [x] Rust toolchain installed
- [x] macOS development build running
- [x] Git repository created
- [x] GitHub repository created
- [ ] FileM4ster application shell
- [ ] Transfer Job model
- [ ] Source / destination selection
- [ ] Preflight
- [ ] Safe Copy engine
- [ ] Progress reporting
- [ ] Safe stop
- [ ] Resume
- [ ] Verification
- [ ] Job logging
- [ ] SQLite job history
- [ ] TinyMonkey remote worker

---

## Development

### Requirements

The current development environment uses:

- macOS
- Node.js
- npm
- Rust
- Cargo
- Tauri 2

### Install dependencies

From the project directory:

```bash
npm install
```

### Run the development application

```bash
npm run tauri dev
```

This starts the frontend development environment, compiles the Rust backend and
launches the native FileM4ster development application.

---

## Repository structure

The current project is based on the standard Tauri + React structure.

A simplified view is:

```text
filem4ster/
│
├── src/
│   └── React + TypeScript frontend
│
├── src-tauri/
│   └── Rust + Tauri backend
│
├── docs/
│   ├── architecture.md
│   └── roadmap.md
│
├── package.json
├── README.md
└── .gitignore
```

The architecture may be reorganized as FileM4ster grows.

A possible future structure is:

```text
filem4ster/
│
├── apps/
│   └── desktop/
│
├── crates/
│   ├── core/
│   ├── inventory/
│   ├── transfer/
│   ├── verify/
│   └── agent/
│
└── database/
```

This is a future direction and is not required for the first implementation.

---

## Roadmap

The development roadmap is documented in:

[`docs/roadmap.md`](docs/roadmap.md)

Planned major milestones:

| Version | Focus |
|---|---|
| v0.1 | Safe Copy |
| v0.2 | File inventory |
| v0.3 | Duplicate detection |
| v0.4 | File organization |
| v0.5 | Cloud integration |
| v1.0 | Stable release |

---

## Architecture documentation

More detailed architecture documentation is available in:

[`docs/architecture.md`](docs/architecture.md)

---

## Development philosophy

FileM4ster is being developed incrementally.

The project will not attempt to implement every planned feature at once.

The initial development strategy is:

```text
Make it work
     ↓
Make it safe
     ↓
Make it observable
     ↓
Make it resumable
     ↓
Make it verifiable
     ↓
Extend it
```

The first goal is to successfully reproduce the real Buffalo-to-TinyMonkey
migration workflow using FileM4ster.

Only after the core transfer system works reliably will features such as file
inventory, duplicate detection, organization and cloud storage be added.

---

## Safety

FileM4ster is intended to manage potentially important and irreplaceable data.

For that reason, destructive operations should never be the default.

Initial development intentionally avoids automatic:

- File deletion
- Destination mirroring
- Source modification
- Duplicate removal
- Destructive synchronization

Potentially destructive functionality should require explicit user intent,
preflight validation and a clear preview before execution.

---

## License

No public license has been selected yet.