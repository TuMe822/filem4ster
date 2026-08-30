# FileM4ster Architecture

This document describes the technical architecture, design principles and planned evolution of FileM4ster.

FileM4ster is being developed incrementally. Some components described here already exist, while others represent the intended direction of the project.

> **Current development target: FileM4ster v0.1 — Safe Copy**

---

## 1. Architecture goals

FileM4ster is designed around a small number of core goals:

1. **Safety before convenience**
2. **Preview before execution**
3. **Source data should remain untouched whenever possible**
4. **Long-running operations should be resumable**
5. **Completed transfers should be verifiable**
6. **Operations should be observable and logged**
7. **The user interface and transfer execution should remain separated**
8. **Remote transfers should not require data to pass through the GUI computer**
9. **Destructive operations must never be the default**
10. **The architecture should support future expansion without rewriting the entire application**

---

## 2. Core workflow

The fundamental FileM4ster workflow is:

```text
SCAN
  │
  ▼
DATABASE
  │
  ▼
ANALYZE
  │
  ▼
PLAN
  │
  ▼
PREVIEW / DRY RUN
  │
  ▼
USER APPROVAL
  │
  ▼
EXECUTE
  │
  ▼
VERIFY
  │
  ▼
LOG
```

Not every v0.1 operation requires every stage immediately.

For example, the first Safe Copy implementation may initially use:

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
SAFE COPY
  │
  ▼
VERIFY
  │
  ▼
JOB HISTORY
```

The larger workflow describes the direction of the complete FileM4ster system.

---

## 3. High-level architecture

The initial desktop architecture is:

```text
┌──────────────────────────────────────┐
│          FileM4ster.app              │
│                                      │
│  ┌────────────────────────────────┐  │
│  │ React + TypeScript UI          │  │
│  └───────────────┬────────────────┘  │
│                  │                   │
│                Tauri                 │
│                  │                   │
│  ┌───────────────▼────────────────┐  │
│  │ Rust Core                      │  │
│  │                                │  │
│  │ Jobs                           │  │
│  │ Safety                         │  │
│  │ Validation                     │  │
│  │ Transfer control               │  │
│  │ Verification                   │  │
│  └───────────────┬────────────────┘  │
└──────────────────┼───────────────────┘
                   │
                   ▼
                rsync
                   │
                   ▼
              Filesystems
```

The user interacts with React.

React communicates through Tauri with the Rust backend.

Rust controls filesystem operations and external transfer tools.

---

## 4. Technology stack

| Layer | Technology | Responsibility |
|---|---|---|
| Desktop framework | Tauri 2 | Native desktop application shell |
| User interface | React | Application UI |
| Frontend language | TypeScript | Typed frontend logic |
| Core / backend | Rust | Native logic, safety and execution |
| Initial transfer engine | rsync | Reliable file transfers |
| Future transfer engine | rclone | Cloud and remote storage |
| Database | SQLite | Jobs, inventory and history |
| Version control | Git | Source history |
| Remote repository | GitHub | Collaboration, issues and releases |

---

## 5. Frontend responsibilities

The React + TypeScript frontend is responsible for presentation and user interaction.

Examples include:

- Application navigation
- Source selection
- Destination selection
- Transfer configuration
- Preflight results
- Preview screens
- Progress display
- Current file display
- Transfer speed
- Estimated remaining time
- Warnings
- Error presentation
- Job history
- Storage views
- Connection management
- User settings

The frontend should **not** directly execute arbitrary shell commands.

It should communicate intent to the Rust backend.

For example:

```text
User presses:

START SAFE COPY
```

The frontend should conceptually request:

```text
Create and execute this TransferJob
```

instead of constructing:

```bash
rsync ...
```

itself.

This separation is important for safety.

---

## 6. Tauri responsibilities

Tauri connects the React frontend with native Rust functionality.

Conceptually:

```text
React
   │
   │ Tauri command
   ▼
Rust
```

and in the opposite direction:

```text
Rust
   │
   │ Tauri event
   ▼
React
```

### Commands

Commands are useful for actions initiated by the frontend.

Examples:

```text
scan_source
validate_destination
create_transfer_job
start_transfer
stop_transfer
verify_transfer
```

### Events

Events are useful for asynchronous information coming from Rust.

Examples:

```text
transfer_started
transfer_progress
current_file_changed
transfer_warning
transfer_failed
transfer_completed
verification_progress
verification_completed
```

This model allows long-running backend work without blocking the UI.

---

## 7. Rust Core responsibilities

Rust is the trusted core of FileM4ster.

The Rust layer should be responsible for:

- Path validation
- Source validation
- Destination validation
- Free-space checks
- Job creation
- Transfer policies
- External process execution
- Process lifecycle management
- Progress parsing
- Error handling
- Verification
- Logging
- Database access
- Remote execution logic
- Safety rules

The frontend should not be trusted to enforce important safety guarantees.

For example:

```text
React asks:
Copy A → B

Rust checks:
Is A valid?
Is B valid?
Is B writable?
Is B accidentally inside A?
Is there enough space?
Is this operation allowed by the selected policy?
```

Only after validation should execution begin.

---

## 8. Transfer Job model

FileM4ster operations should be represented as jobs.

A simplified conceptual model:

```text
TransferJob

ID
Source
Destination
Worker
Mode
Verification mode
Status
Created time
Started time
Finished time
Bytes total
Bytes transferred
Files total
Files transferred
Warnings
Errors
```

Example:

```text
Job ID:
job-0042

Source:
/mnt/buffalo

Destination:
/srv/storage/Buffalo

Worker:
TinyMonkey

Mode:
Safe Copy

Verification:
Full

Status:
Running
```

This model allows the UI, database and execution engine to work with the same logical operation.

---

## 9. Job states

A transfer job may eventually move through states such as:

```text
CREATED
   │
   ▼
PREFLIGHT
   │
   ▼
READY
   │
   ▼
RUNNING
   │
   ├──────────────► PAUSED
   │                    │
   │                    ▼
   │                  RUNNING
   │
   ├──────────────► INTERRUPTED
   │                    │
   │                    ▼
   │                  RUNNING
   │
   ├──────────────► FAILED
   │
   ▼
TRANSFER_COMPLETE
   │
   ▼
VERIFYING
   │
   ├──────────────► VERIFY_FAILED
   │
   ▼
VERIFIED
```

The exact state machine may evolve during implementation.

---

## 10. Safe Copy policy

FileM4ster v0.1 is centered around **Safe Copy**.

Safe Copy is intentionally conservative.

### Safe Copy should

- Copy files recursively
- Preserve timestamps where appropriate
- Allow interrupted transfers to continue
- Report progress
- Log errors
- Avoid modifying the source
- Avoid automatic deletion
- Support verification

### Safe Copy should not automatically

- Delete source files
- Delete destination files
- Mirror a source destructively
- Remove duplicates
- Overwrite unrelated data without policy checks
- Execute arbitrary user-generated shell commands

The goal is that the safest useful behavior is the default behavior.

---

## 11. Transfer engine

The first FileM4ster transfer engine is `rsync`.

Architecture:

```text
TransferJob
     │
     ▼
Transfer Policy
     │
     ▼
Rust rsync adapter
     │
     ▼
rsync process
     │
     ▼
Filesystem
```

Rust should construct the allowed rsync invocation from FileM4ster policies.

The frontend should not build command strings.

For example:

```text
FileM4ster policy:

Mode:
Safe Copy

Resume:
Enabled

Preserve timestamps:
Enabled

Delete destination extras:
Disabled
```

may internally translate into a controlled rsync invocation.

---

## 12. Why rsync?

`rsync` is used initially because it already provides many important transfer capabilities:

- Recursive copying
- Incremental operation
- Resume-friendly transfers
- Metadata handling
- Progress output
- Network support
- Mature error behavior
- Efficient re-runs
- Reliable handling of large directory trees

FileM4ster does not need to reimplement all of this functionality from scratch in Rust.

Instead, Rust can provide:

```text
Safety
Policy
Validation
Control
Progress parsing
Logging
Verification
User experience
```

while rsync performs the low-level transfer.

---

## 13. Transfer progress

FileM4ster should convert raw transfer-engine output into structured progress information.

Conceptually:

```text
rsync output
     │
     ▼
Rust parser
     │
     ▼
TransferProgress
     │
     ▼
Tauri event
     │
     ▼
React UI
```

Example structured progress:

```text
Bytes total:
1.89 TB

Bytes transferred:
1.19 TB

Progress:
63 %

Files total:
200,924

Files transferred:
126,483

Current file:
Pictures/2019/IMG_4829.JPG

Speed:
74.2 MB/s

Errors:
0
```

The UI should not need to understand raw rsync terminal output.

---

## 14. Preflight

Before execution, FileM4ster performs preflight checks.

Possible checks include:

### Source

- Exists
- Reachable
- Readable
- Source type recognized
- Read-only status known
- Size available
- File count available

### Destination

- Exists
- Reachable
- Writable
- Free space available
- Filesystem detected
- Destination does not create an unsafe recursive path

### Transfer

- Source and destination are different
- Destination has enough capacity
- Required transfer engine exists
- Required permissions exist
- Connection is stable enough to begin
- Potential conflicts have been identified

Nothing should be modified during preflight.

---

## 15. Preview / Dry Run

The preview stage gives the user an understandable description of the planned operation.

Example:

```text
READY TO COPY

Source:
Buffalo LS210D

Source path:
/mnt/buffalo

Destination:
TinyMonkey

Destination path:
/srv/storage/Buffalo

Files:
200,924

Data:
1.89 TB

Destination free:
3.6 TB

Source:
READ ONLY

Transfer mode:
SAFE COPY

Verification:
FULL

Nothing has been changed yet.
```

The user explicitly approves the operation after preview.

---

## 16. Verification

A successful copy process does not automatically mean that the migration should be considered complete.

FileM4ster should support verification.

### Quick verification

Possible checks:

- Path exists
- File exists
- File size matches
- Timestamp matches where applicable

### Full verification

Possible checks:

- Source hash
- Destination hash
- Cryptographic comparison

Conceptually:

```text
SOURCE FILE
     │
     ▼
    HASH
     │
     ├──────── compare ────────┐
     │                         │
DESTINATION FILE               │
     │                         │
     ▼                         │
    HASH ──────────────────────┘
```

A migration can then be marked:

```text
TRANSFER COMPLETE
```

and later:

```text
VERIFIED
```

These are intentionally separate concepts.

---

## 17. Logging and observability

Every important job should generate structured information.

Examples:

```text
Job created
Preflight started
Preflight completed
Transfer started
File copied
Warning generated
Transfer interrupted
Transfer resumed
Transfer completed
Verification started
Verification completed
Job verified
```

Logs are useful for:

- Troubleshooting
- Audit history
- Resume
- User confidence
- Development
- Diagnostics

Logs should eventually be visible through the GUI instead of requiring direct access to terminal log files.

---

## 18. SQLite

SQLite is planned as the local FileM4ster database.

The initial database may contain tables such as:

```text
jobs
job_events
```

Later versions may add:

```text
files
directories
volumes
connections
scans
hashes
duplicates
operations
workers
```

A conceptual relationship:

```text
JOB
 │
 ├── JOB EVENTS
 │
 ├── TRANSFER ITEMS
 │
 └── VERIFICATION RESULTS
```

SQLite allows FileM4ster to preserve state between application launches.

---

## 19. File inventory

FileM4ster v0.2 is planned to introduce persistent inventory.

Instead of repeatedly scanning terabytes of data, FileM4ster can store scan results.

Example:

```text
Filesystem
    │
    ▼
Scanner
    │
    ▼
Metadata
    │
    ▼
SQLite inventory
```

Stored information may include:

- Path
- Filename
- Extension
- Size
- Modified time
- File type
- Parent directory
- Hash status
- Bundle status
- Media metadata

This inventory later enables:

- Search
- Storage analysis
- Duplicate detection
- Organization planning

---

## 20. Context-aware organization

FileM4ster should not blindly organize files based only on file extension.

For example:

```text
Metallica/
├── 01 Enter Sandman.flac
├── 02 Sad But True.flac
├── cover.jpg
├── folder.jpg
└── album.cue
```

should be treated as a music album.

It should not become:

```text
Music/
├── 01 Enter Sandman.flac
└── 02 Sad But True.flac

Pictures/
├── cover.jpg
└── folder.jpg

Documents/
└── album.cue
```

Likewise:

```text
Interstellar/
├── Interstellar.mkv
├── Interstellar.srt
├── poster.jpg
└── movie.nfo
```

should remain a logical movie package.

This is why FileM4ster eventually requires context-aware analysis rather than simple extension-based classification.

---

## 21. Bundles and special directories

macOS and application data often contain directories that logically behave as single objects.

Examples include:

```text
.app
.photoslibrary
.sparsebundle
.logicx
```

FileM4ster should detect these structures and avoid incorrectly reorganizing their internal contents.

For example:

```text
Elina - MacBook Pro.sparsebundle
```

should normally be treated as one logical object.

---

## 22. Duplicate detection

Duplicate detection is planned for a later release.

The process may use multiple stages.

### Candidate detection

Fast comparison using:

```text
File size
Filename
Metadata
```

### Verification

More expensive comparison using:

```text
Cryptographic hashes
```

Conceptually:

```text
Files
  │
  ▼
Size grouping
  │
  ▼
Candidate duplicates
  │
  ▼
Hash comparison
  │
  ▼
Confirmed duplicates
```

FileM4ster should not automatically delete confirmed duplicates.

The user should review and approve cleanup operations.

---

## 23. Local execution

The first implementation can execute jobs locally on the same computer running FileM4ster.

```text
FileM4ster.app
      │
      ▼
Rust Core
      │
      ▼
rsync
      │
      ▼
Local filesystem
```

This is the simplest execution model and is useful for developing the transfer engine.

---

## 24. Remote execution

FileM4ster also needs to support transfers where another computer is the optimal place to execute the job.

The first remote implementation may use SSH.

Example:

```text
FileM4ster.app
      │
      │ SSH control
      ▼
TinyMonkey
      │
      │ rsync
      ▼
Storage
```

The Mac acts as the control interface.

TinyMonkey performs the actual work.

---

## 25. Why remote execution matters

Consider this transfer:

```text
Buffalo NAS
     │
     ▼
TinyMonkey
```

If the FileM4ster desktop application on the Mac performed the transfer itself, the data path could become:

```text
Buffalo
   │
   ▼
Mac
   │
   ▼
TinyMonkey
```

For approximately 1.9 TB of data, this is unnecessary.

The better architecture is:

```text
            FileM4ster.app
                  │
                  │ control only
                  ▼
Buffalo ─────► TinyMonkey
      actual data path
```

This reduces:

- Network traffic
- Dependency on the Mac
- Risk from Mac sleep
- Risk from GUI crashes
- Unnecessary transfer hops

---

## 26. First real-world architecture

The current reference migration is:

```text
┌────────────────────┐
│ Buffalo LS210D     │
│                    │
│ ~1.9 TB data       │
└──────────┬─────────┘
           │
           │ SMB1
           │ read only
           ▼
┌────────────────────┐
│ TinyMonkey         │
│                    │
│ /mnt/buffalo       │
│       │            │
│       │ rsync      │
│       ▼            │
│ /srv/storage/      │
│ Buffalo            │
│                    │
│ 4 TB ext4          │
└────────────────────┘
```

The transfer currently runs manually on TinyMonkey.

This serves as the primary integration scenario for FileM4ster v0.1.

---

## 27. FileM4ster Agent

A later architecture may introduce a dedicated worker service:

```text
filem4ster-agent
```

The agent would be installed on machines capable of executing FileM4ster jobs.

Example:

```text
                 FileM4ster.app
                       │
                       │ control
          ┌────────────┼────────────┐
          ▼            ▼            ▼
     TinyMonkey    BackupBox    Workstation
          │            │            │
          ▼            ▼            ▼
       Storage       Storage       Storage
```

The same agent could eventually run on:

- Linux
- Windows
- macOS

depending on platform support.

---

## 28. Agent responsibilities

A future FileM4ster Agent may be responsible for:

- Advertising worker capabilities
- Receiving jobs
- Validating paths
- Starting transfer processes
- Monitoring transfers
- Sending progress events
- Managing resumable state
- Performing verification
- Writing local logs
- Reporting disk capacity
- Reporting available transfer engines
- Maintaining jobs after the desktop application disconnects

Example:

```text
FileM4ster.app closes

       │
       X

filem4ster-agent
       │
       │ continues
       ▼
running transfer
```

This removes the need for tools such as `tmux` from the user-facing workflow.

---

## 29. Worker selection

In the future, FileM4ster may automatically recommend the best worker.

Example:

```text
Source:
Google Drive

Destination:
TinyMonkey

Recommended worker:
TinyMonkey
```

Another example:

```text
Source:
NAS-A

Destination:
BackupBox

Recommended worker:
BackupBox
```

A useful principle is:

> Execute the transfer as close to the destination or data path as practical.

---

## 30. Cloud architecture

Cloud support is planned through `rclone`.

Conceptually:

```text
FileM4ster Job
      │
      ▼
Rust Core
      │
      ▼
rclone adapter
      │
      ▼
Cloud provider
```

Potential supported services include:

- Google Drive
- OneDrive
- Nextcloud
- WebDAV
- S3-compatible storage

FileM4ster should use a common job model regardless of the underlying transfer engine.

For example:

```text
Safe Copy Job
```

may internally use:

```text
rsync
```

or:

```text
rclone
```

depending on the source and destination.

---

## 31. Transfer engine abstraction

Long term, FileM4ster should not make the rest of the application dependent on one specific transfer engine.

Conceptually:

```text
                 TransferJob
                      │
                      ▼
               Transfer Engine
                abstraction
                      │
          ┌───────────┴───────────┐
          ▼                       ▼
        rsync                   rclone
          │                       │
          ▼                       ▼
Local / NAS storage         Cloud storage
```

The GUI should not care which engine performs the transfer.

---

## 32. Security model

FileM4ster may eventually manage:

- Local files
- NAS credentials
- SSH connections
- Cloud credentials
- Remote workers

Security therefore needs to be considered from the beginning.

Core principles include:

- Do not store plaintext passwords in project files
- Do not commit secrets to Git
- Use operating-system credential storage where possible
- Validate all paths
- Avoid arbitrary shell construction
- Minimize privilege escalation
- Do not run the entire application as root
- Require explicit approval for destructive operations
- Treat remote inputs as untrusted
- Keep network communication authenticated

---

## 33. Privilege model

FileM4ster should not normally run as:

```text
root
```

or with unrestricted administrative privileges.

If elevated privileges are required for a specific operation, elevation should be limited to that operation where possible.

Preferred model:

```text
Normal application
      │
      ├── normal filesystem operations
      │
      └── explicitly elevated operation
             only when required
```

instead of:

```text
Entire FileM4ster application
running permanently as root
```

---

## 34. Error handling

Errors should be treated as structured information.

Example categories:

```text
SourceUnavailable
DestinationUnavailable
PermissionDenied
InsufficientSpace
AuthenticationFailed
TransferInterrupted
TransferEngineMissing
VerificationFailed
NetworkFailure
UnknownError
```

The UI should present useful explanations rather than only raw system errors.

Example:

```text
Transfer interrupted

Reason:
Source connection lost

Copied:
1.28 TB

Verified:
1.26 TB

The completed data has not been deleted.

[ RETRY ]
```

---

## 35. Resume philosophy

Long-running transfers must assume that interruptions can happen.

Possible causes include:

- Network failure
- NAS reboot
- Server reboot
- Laptop sleep
- Application restart
- SSH disconnect
- Temporary storage failure

FileM4ster should be designed around the assumption that:

> An interrupted job should normally be recoverable without restarting the entire operation.

---

## 36. Application areas

The desktop application may eventually contain primary sections such as:

```text
Transfers
Jobs
Storage
Inventory
Duplicates
Organize
Connections
Settings
```

For v0.1, only a subset is required.

A likely initial navigation structure is:

```text
FileM4ster

├── New Transfer
├── Jobs
├── Connections
└── Settings
```

---

## 37. Current repository structure

The current project uses the standard Tauri + React structure.

Simplified:

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

This structure is appropriate for the beginning of the project.

---

## 38. Possible future repository structure

As FileM4ster grows, the project may later move toward a workspace structure.

For example:

```text
filem4ster/
│
├── apps/
│   └── desktop/
│
├── crates/
│   ├── core/
│   │   ├── jobs
│   │   ├── planner
│   │   ├── paths
│   │   └── safety
│   │
│   ├── inventory/
│   │   ├── scanner
│   │   ├── metadata
│   │   └── hashing
│   │
│   ├── transfer/
│   │   ├── rsync
│   │   └── rclone
│   │
│   ├── verify/
│   │
│   └── agent/
│
└── database/
    └── migrations
```

This is **not required for v0.1**.

The project should only be reorganized when the additional complexity provides a clear benefit.

---

## 39. Version evolution

### v0.1 — Safe Copy

Focus:

```text
Reliable file migration
```

Architecture emphasis:

- Tauri desktop application
- React interface
- Rust job model
- rsync execution
- Progress
- Safe stop
- Resume
- Verification
- Job history
- TinyMonkey remote execution

### v0.2 — Inventory

Focus:

```text
Persistent understanding of storage contents
```

Adds:

- Scanner
- SQLite inventory
- Metadata
- Search
- Storage statistics
- Bundle detection

### v0.3 — Duplicates

Focus:

```text
Safe duplicate detection
```

Adds:

- Candidate detection
- Hash comparison
- Duplicate review
- Cleanup planning

### v0.4 — Organize

Focus:

```text
Context-aware file organization
```

Adds:

- Classification
- Sidecar awareness
- Media context
- Bundle awareness
- Preview
- Undo

### v0.5 — Cloud

Focus:

```text
Cloud and remote storage
```

Adds:

- rclone
- Cloud connections
- Scheduled backup jobs
- Nextcloud
- Google Drive
- OneDrive
- S3-compatible storage

### v1.0 — Stable

Focus:

```text
Reliable everyday use
```

The exact v1.0 feature set will be decided based on development experience and testing.

---

## 40. Non-goals for v0.1

FileM4ster v0.1 will intentionally not attempt to solve every storage problem.

The following are outside the initial scope:

- Automatic duplicate deletion
- Automatic filesystem cleanup
- Destructive mirroring
- Full cloud integration
- Intelligent organization
- Multi-platform agents
- Advanced scheduling
- Complete backup automation
- Full NAS management
- Custom native file-transfer protocol

Keeping the scope controlled allows the core transfer system to become reliable first.

---

## 41. Development philosophy

FileM4ster is developed incrementally.

The preferred order is:

```text
Make it work
     │
     ▼
Make it safe
     │
     ▼
Make it observable
     │
     ▼
Make it resumable
     │
     ▼
Make it verifiable
     │
     ▼
Extend it
```

Complexity should be added only when the simpler implementation has demonstrated its limitations.

---

## 42. First architecture milestone

The first major architectural milestone is reached when FileM4ster can reproduce the current Buffalo migration workflow from the GUI.

Today:

```text
User
 │
 ▼
SSH
 │
 ▼
TinyMonkey
 │
 ├── mount
 ├── tmux
 └── rsync
```

FileM4ster v0.1 target:

```text
User
 │
 ▼
FileM4ster.app
 │
 ▼
New Transfer
 │
 ▼
Preview
 │
 ▼
Start Safe Copy
 │
 ▼
TinyMonkey executes transfer
 │
 ▼
Progress appears in FileM4ster
 │
 ▼
Verification
 │
 ▼
Completed job stored in history
```

The user should not need to manually use:

```text
mount
tmux
rsync
```

for the normal workflow.

---

## 43. Architectural rule of thumb

A useful rule for FileM4ster development is:

> The UI describes what the user wants.  
> The Rust core decides whether it is safe.  
> The transfer engine performs the operation.  
> Verification determines whether the result can be trusted.

In simplified form:

```text
INTENT
  │
  ▼
VALIDATE
  │
  ▼
EXECUTE
  │
  ▼
VERIFY
```

This separation should remain one of the central architectural principles of FileM4ster.

---

## 44. Architecture status

Current implementation status:

- [x] Tauri 2 project created
- [x] React + TypeScript frontend configured
- [x] Rust backend configured
- [x] macOS development build working
- [x] Git repository configured
- [x] GitHub repository configured
- [x] Project README created
- [ ] FileM4ster application shell
- [ ] TransferJob model
- [ ] Preflight engine
- [ ] Transfer engine adapter
- [ ] rsync integration
- [ ] Progress event pipeline
- [ ] Verification engine
- [ ] SQLite persistence
- [ ] Remote SSH worker
- [ ] FileM4ster Agent

---

## 45. Related documentation

Project overview:

[`../README.md`](../README.md)

Development roadmap:

[`roadmap.md`](roadmap.md)

---

## 46. Document status

This architecture document evolves together with FileM4ster.

Architectural decisions may change as implementation reveals better solutions.

Significant architectural changes should be documented here and committed to Git so that the reasoning behind the project structure remains visible over time.