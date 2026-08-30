# FileM4ster Roadmap

This document describes the planned development path of FileM4ster.

The roadmap is intentionally incremental. FileM4ster will first become a reliable and understandable file transfer application before expanding into inventory, duplicate detection, organization, cloud storage and distributed workers.

> **Current development target: FileM4ster v0.1 — Safe Copy**

---

## 1. Development philosophy

FileM4ster follows a simple development principle:

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

The project should avoid implementing many partially working features at the same time.

Each major capability should become reliable before the next layer of complexity is added.

---

## 2. Long-term product vision

FileM4ster is intended to become a desktop application for:

- Safe file migration
- Large data transfers
- Backup workflows
- NAS migration
- Storage analysis
- Transfer verification
- File inventory
- Duplicate detection
- Context-aware organization
- Remote workers
- Cloud storage
- Scheduled jobs
- Transfer history
- Recovery from interrupted operations

The long-term workflow is:

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
PREVIEW
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

---

# v0.1 — Safe Copy

## Goal

The goal of v0.1 is simple:

> FileM4ster must be able to perform a reliable file migration from a graphical interface.

The first release should reproduce the current real-world Buffalo-to-TinyMonkey migration without requiring the user to manually use `mount`, `tmux`, `rsync` or repeated SSH commands during normal operation.

Reference workflow:

```text
Buffalo LS210D
      │
      │ read-only source
      ▼
TinyMonkey
/mnt/buffalo
      │
      │ Safe Copy
      ▼
/srv/storage/Buffalo
```

---

## v0.1 milestone 1 — Application foundation

### Goal

Replace the default Tauri demo with the first real FileM4ster application shell.

### Tasks

- [ ] Remove default Tauri demo content
- [ ] Create FileM4ster application layout
- [ ] Create main navigation
- [ ] Add application header
- [ ] Add New Transfer view
- [ ] Add Jobs view
- [ ] Add Connections view
- [ ] Add Settings placeholder
- [ ] Establish basic UI component structure
- [ ] Establish application styling

### Initial navigation

```text
FileM4ster

├── New Transfer
├── Jobs
├── Connections
└── Settings
```

### Completion criteria

The milestone is complete when:

- FileM4ster launches as a recognizable application
- The default Tauri demo is gone
- Navigation works
- New Transfer has its own view
- The application has a stable visual foundation for later features

---

## v0.1 milestone 2 — Transfer Job model

### Goal

Represent every transfer as a structured FileM4ster Job.

### Tasks

- [ ] Create `TransferJob` model in Rust
- [ ] Define unique Job ID
- [ ] Define source
- [ ] Define destination
- [ ] Define worker
- [ ] Define transfer mode
- [ ] Define verification mode
- [ ] Define job status
- [ ] Define timestamps
- [ ] Define byte counters
- [ ] Define file counters
- [ ] Define warnings
- [ ] Define errors

### Example

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
Created
```

### Initial job states

- [ ] Created
- [ ] Preflight
- [ ] Ready
- [ ] Running
- [ ] Interrupted
- [ ] Failed
- [ ] Transfer Complete
- [ ] Verifying
- [ ] Verified

### Completion criteria

The milestone is complete when:

- A transfer can be represented as structured data
- The frontend can create or request a Transfer Job
- Rust can validate the basic model
- Job state can be displayed in the UI

---

## v0.1 milestone 3 — Source and destination selection

### Goal

Allow the user to define what should be copied and where it should go.

### Tasks

- [ ] Add Source selector
- [ ] Add Destination selector
- [ ] Support local paths
- [ ] Display selected paths
- [ ] Validate empty paths
- [ ] Detect identical source and destination
- [ ] Prevent obviously unsafe recursive destinations
- [ ] Display available destination storage
- [ ] Add worker selector placeholder

### Example UI

```text
SOURCE

/Users/tume/TestSource

[ Browse ]


DESTINATION

/Users/tume/TestDestination

[ Browse ]
```

### Completion criteria

The milestone is complete when:

- The user can select valid source and destination paths
- Invalid path combinations are rejected
- Selected paths reach the Rust backend safely

---

## v0.1 milestone 4 — Preflight engine

### Goal

Determine whether a transfer appears safe and possible before anything is copied.

### Source checks

- [ ] Source exists
- [ ] Source is readable
- [ ] Source type detected
- [ ] Source size calculated
- [ ] Source file count calculated
- [ ] Source read-only status detected where possible

### Destination checks

- [ ] Destination exists
- [ ] Destination is writable
- [ ] Destination filesystem detected
- [ ] Free space calculated
- [ ] Capacity compared with source size

### Transfer checks

- [ ] Source and destination differ
- [ ] Destination is not recursively inside source
- [ ] Required transfer engine exists
- [ ] Required permissions exist
- [ ] Basic collision detection
- [ ] Basic connection validation

### Example result

```text
PREFLIGHT

Source reachable          ✓
Source readable           ✓
Destination reachable     ✓
Destination writable      ✓
Source size               1.89 TB
Destination free          3.60 TB
Required capacity         OK
Potential conflicts       3

STATUS
READY FOR PREVIEW
```

### Completion criteria

The milestone is complete when:

- FileM4ster can inspect both ends before copying
- It can stop obviously unsafe operations
- Nothing is modified during preflight

---

## v0.1 milestone 5 — Preview / Dry Run

### Goal

Show exactly what FileM4ster intends to do before execution.

### Tasks

- [ ] Create Preview view
- [ ] Show source
- [ ] Show destination
- [ ] Show worker
- [ ] Show transfer mode
- [ ] Show file count
- [ ] Show total bytes
- [ ] Show destination free space
- [ ] Show warnings
- [ ] Show detected conflicts
- [ ] Clearly state that no data has been modified
- [ ] Require explicit user approval before execution

### Example

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
3.60 TB

Mode:
SAFE COPY

Source protection:
READ ONLY

Nothing has been changed yet.

[ CANCEL ]       [ START SAFE COPY ]
```

### Completion criteria

The milestone is complete when:

- Every transfer requires a preview
- The preview is understandable without reading terminal commands
- Execution begins only after explicit approval

---

## v0.1 milestone 6 — Local Safe Copy engine

### Goal

Perform the first real FileM4ster-controlled copy.

### Initial engine

```text
rsync
```

### Tasks

- [ ] Detect `rsync`
- [ ] Create Rust rsync adapter
- [ ] Convert FileM4ster policy into controlled rsync arguments
- [ ] Start rsync from Rust
- [ ] Capture stdout
- [ ] Capture stderr
- [ ] Capture exit status
- [ ] Record process ID
- [ ] Handle normal completion
- [ ] Handle errors
- [ ] Prevent arbitrary frontend-generated shell commands

### Safe Copy requirements

- [ ] Recursive copy
- [ ] Preserve timestamps where appropriate
- [ ] Partial transfer support
- [ ] No automatic source deletion
- [ ] No destination mirror deletion
- [ ] No destructive default behavior
- [ ] Re-running should be safe
- [ ] Errors must be logged

### Completion criteria

The milestone is complete when:

- FileM4ster can copy a test directory
- The operation is initiated from the GUI
- Rust controls the process
- No terminal command is required by the user

---

## v0.1 milestone 7 — Live transfer progress

### Goal

Turn raw transfer output into understandable real-time progress.

### Tasks

- [ ] Parse rsync progress
- [ ] Track total bytes
- [ ] Track transferred bytes
- [ ] Calculate percentage
- [ ] Track total files
- [ ] Track completed files
- [ ] Detect current file
- [ ] Calculate transfer speed
- [ ] Calculate elapsed time
- [ ] Calculate estimated remaining time
- [ ] Track warnings
- [ ] Track errors
- [ ] Send structured Tauri events to React
- [ ] Update the UI in real time

### Example UI

```text
BUFFALO MIGRATION

████████████████░░░░░░░░  63 %

1.19 TB / 1.89 TB

Files:
126,483 / 200,924

Current:
Pictures/2019/IMG_4829.JPG

Speed:
74.2 MB/s

Elapsed:
04:32:18

Remaining:
02:41:07

Errors:
0
```

### Completion criteria

The milestone is complete when:

- The user no longer needs raw rsync output
- Progress is updated continuously
- Transfer status remains understandable throughout the job

---

## v0.1 milestone 8 — Safe stop and interruption handling

### Goal

Allow a transfer to be stopped without corrupting the entire workflow.

### Tasks

- [ ] Add Stop button
- [ ] Define safe stop behavior
- [ ] Send controlled termination signal
- [ ] Preserve partial data where appropriate
- [ ] Mark job as Interrupted
- [ ] Store interruption reason
- [ ] Handle network loss
- [ ] Handle source disappearance
- [ ] Handle destination disappearance
- [ ] Handle external process failure

### Example

```text
TRANSFER INTERRUPTED

Reason:
Source connection lost

Copied:
1.28 TB

Files completed:
143,291

Partial data preserved:
Yes

[ RETRY ]
```

### Completion criteria

The milestone is complete when:

- A user-requested stop does not destroy completed work
- Unexpected interruptions produce a recoverable job state

---

## v0.1 milestone 9 — Resume

### Goal

Continue interrupted transfers without restarting everything from zero.

### Tasks

- [ ] Detect resumable job
- [ ] Validate source before resume
- [ ] Validate destination before resume
- [ ] Re-run preflight where necessary
- [ ] Continue partial transfers
- [ ] Skip already completed content safely
- [ ] Preserve original Job ID
- [ ] Add resume event to job history

### Completion criteria

The milestone is complete when:

- A transfer can be interrupted
- FileM4ster can reopen or retry it
- Previously completed work is not unnecessarily repeated

---

## v0.1 milestone 10 — Verification

### Goal

Confirm that the migration result can be trusted.

### Quick verification

- [ ] Destination path exists
- [ ] Expected files exist
- [ ] File sizes match
- [ ] Basic metadata matches where appropriate
- [ ] Missing files detected

### Full verification

- [ ] Generate source hash
- [ ] Generate destination hash
- [ ] Compare hashes
- [ ] Track verification progress
- [ ] Report mismatches
- [ ] Store verification results

### Verification states

```text
TRANSFER COMPLETE
       │
       ▼
   VERIFYING
       │
       ├──────► VERIFY FAILED
       │
       ▼
    VERIFIED
```

### Example result

```text
MIGRATION COMPLETED

Files copied:
200,924

Data copied:
1.89 TB

Files verified:
200,924

Missing:
0

Corrupt:
0

Failed:
0

STATUS:
VERIFIED
```

### Completion criteria

The milestone is complete when:

- Transfer completion and verification are separate concepts
- FileM4ster can clearly report whether copied data matches expectations

---

## v0.1 milestone 11 — Job logging

### Goal

Keep a readable history of what happened during each operation.

### Events to store

- [ ] Job created
- [ ] Preflight started
- [ ] Preflight completed
- [ ] Preview approved
- [ ] Transfer started
- [ ] Warning
- [ ] Error
- [ ] Transfer interrupted
- [ ] Transfer resumed
- [ ] Transfer completed
- [ ] Verification started
- [ ] Verification completed
- [ ] Job verified
- [ ] Job failed

### Completion criteria

The milestone is complete when:

- A completed or failed job can be inspected later
- Important events no longer exist only in terminal output

---

## v0.1 milestone 12 — SQLite job history

### Goal

Persist FileM4ster jobs between application launches.

### Initial database tables

```text
jobs
job_events
```

### Planned job fields

- [ ] Job ID
- [ ] Source
- [ ] Destination
- [ ] Worker
- [ ] Mode
- [ ] Verification mode
- [ ] Status
- [ ] Created time
- [ ] Started time
- [ ] Finished time
- [ ] Total bytes
- [ ] Completed bytes
- [ ] Total files
- [ ] Completed files
- [ ] Warning count
- [ ] Error count

### Jobs UI

- [ ] Active jobs
- [ ] Completed jobs
- [ ] Failed jobs
- [ ] Interrupted jobs
- [ ] Verified jobs
- [ ] Job details
- [ ] Job event log

### Completion criteria

The milestone is complete when:

- FileM4ster can be closed and reopened
- Previous jobs remain visible
- Important status information survives application restarts

---

## v0.1 milestone 13 — TinyMonkey connection

### Goal

Allow FileM4ster to recognize TinyMonkey as a remote execution target.

### Initial approach

```text
SSH
```

### Tasks

- [ ] Define Connection model
- [ ] Add TinyMonkey connection
- [ ] Test SSH connectivity
- [ ] Detect operating system
- [ ] Detect available storage
- [ ] Detect rsync
- [ ] Detect rclone if installed
- [ ] Detect source paths
- [ ] Detect destination paths
- [ ] Display connection status

### Example

```text
TinyMonkey

Status:
ONLINE

Operating system:
Debian 13

rsync:
Available

Storage:
/srv/storage

Free:
3.6 TB
```

### Completion criteria

The milestone is complete when:

- FileM4ster can query TinyMonkey
- TinyMonkey can be selected as a worker

---

## v0.1 milestone 14 — Remote execution

### Goal

Execute a transfer directly on TinyMonkey while controlling it from the Mac.

### Architecture

```text
FileM4ster.app
      │
      │ control
      ▼
TinyMonkey
      │
      │ rsync
      ▼
Storage
```

### Tasks

- [ ] Start remote transfer
- [ ] Track remote process
- [ ] Receive remote progress
- [ ] Receive remote errors
- [ ] Stop remote transfer safely
- [ ] Resume remote transfer
- [ ] Run remote verification
- [ ] Store remote job history locally
- [ ] Handle SSH disconnect without corrupting job state

### Completion criteria

The milestone is complete when:

- FileM4ster can control a real job on TinyMonkey
- The transfer data does not pass through the Mac

---

## v0.1 milestone 15 — Buffalo integration test

### Goal

Reproduce the current real-world migration through FileM4ster.

### Reference environment

```text
Source:
Buffalo LS210D

Mounted on TinyMonkey:
/mnt/buffalo

Destination:
/srv/storage/Buffalo

Worker:
TinyMonkey

Transfer mode:
Safe Copy
```

### Required workflow

```text
Open FileM4ster
      │
      ▼
New Transfer
      │
      ▼
Select Buffalo source
      │
      ▼
Select TinyMonkey destination
      │
      ▼
Preflight
      │
      ▼
Preview
      │
      ▼
Start Safe Copy
      │
      ▼
TinyMonkey performs transfer
      │
      ▼
Progress visible in FileM4ster
      │
      ▼
Verify
      │
      ▼
Job stored in history
```

### Success criteria

The milestone is complete when the normal workflow no longer requires the user to manually run:

```text
mount
tmux
rsync
```

for this migration scenario.

---

# v0.1 release criteria

FileM4ster v0.1 can be considered complete when all critical requirements below are met.

## Core functionality

- [ ] Source can be selected
- [ ] Destination can be selected
- [ ] Transfer Job is created
- [ ] Preflight runs
- [ ] Preview is shown
- [ ] Safe Copy executes
- [ ] Progress is visible
- [ ] Errors are visible
- [ ] Transfer can be stopped safely
- [ ] Transfer can be resumed
- [ ] Verification works
- [ ] Job history persists
- [ ] TinyMonkey can execute remote transfers

## Safety

- [ ] No automatic source deletion
- [ ] No automatic destination deletion
- [ ] No destructive mirror mode
- [ ] Source and destination are validated
- [ ] Recursive unsafe paths are rejected
- [ ] Destination capacity is checked
- [ ] Dangerous operations require explicit approval

## User experience

- [ ] No terminal required for normal Safe Copy
- [ ] Errors are understandable
- [ ] Progress is understandable
- [ ] Completed jobs show clear results

---

# v0.2 — Inventory

## Goal

Build a persistent understanding of the contents of storage locations.

### Planned features

- [ ] File scanner
- [ ] Directory scanner
- [ ] SQLite inventory
- [ ] Persistent scan results
- [ ] File type detection
- [ ] File size statistics
- [ ] Directory statistics
- [ ] Modified timestamps
- [ ] Storage usage analysis
- [ ] Search
- [ ] Filtering
- [ ] Bundle detection
- [ ] Media metadata foundation

### Architecture

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
SQLite
    │
    ▼
Inventory UI
```

### Goal example

Instead of repeatedly scanning 1.9 TB:

```text
NAS
 │
 ▼
Full scan every time
```

FileM4ster should be able to use:

```text
NAS
 │
 ▼
Scan
 │
 ▼
SQLite inventory
 │
 ├── Search
 ├── Statistics
 ├── Duplicates
 └── Organize
```

---

# v0.3 — Duplicate detection

## Goal

Safely detect identical or probably identical files.

### Stage 1 — Candidate detection

- [ ] Same file size
- [ ] Same filename
- [ ] Similar filename
- [ ] Metadata comparison

### Stage 2 — Hash verification

- [ ] Hash candidates
- [ ] Compare hashes
- [ ] Confirm duplicates
- [ ] Cache hashes
- [ ] Avoid unnecessary re-hashing

### Review UI

- [ ] Show duplicate groups
- [ ] Show file paths
- [ ] Show size
- [ ] Show modified dates
- [ ] Show hash result
- [ ] Allow manual decisions
- [ ] Never automatically delete by default

### Example

```text
POSSIBLE DUPLICATE

File A:
Pictures/2024/IMG_1042.JPG

File B:
Backup/iPhone/IMG_1042.JPG

Size:
5.84 MB

Hash:
IDENTICAL

[ KEEP BOTH ]
[ REVIEW ]
```

---

# v0.4 — Organize

## Goal

Organize files using context rather than file extensions alone.

### Planned categories

- [ ] Photos
- [ ] Video
- [ ] Movies
- [ ] Series
- [ ] Music
- [ ] Documents
- [ ] Archives
- [ ] Backups
- [ ] Application bundles
- [ ] Unknown

### Context awareness

FileM4ster should understand relationships such as:

```text
Movie/
├── movie.mkv
├── movie.srt
├── poster.jpg
└── movie.nfo
```

and:

```text
Album/
├── track01.flac
├── track02.flac
├── cover.jpg
└── album.cue
```

### Planned functionality

- [ ] Analyze directory context
- [ ] Detect media sidecars
- [ ] Detect application bundles
- [ ] Detect `.sparsebundle`
- [ ] Detect `.photoslibrary`
- [ ] Detect `.app`
- [ ] Build organization plan
- [ ] Preview all moves
- [ ] Require approval
- [ ] Log all operations
- [ ] Support undo where possible

### Core rule

```text
ANALYZE
   │
   ▼
PLAN
   │
   ▼
PREVIEW
   │
   ▼
APPROVAL
   │
   ▼
MOVE
```

Never:

```text
SCAN
  │
  ▼
MOVE EVERYTHING IMMEDIATELY
```

---

# v0.5 — Cloud

## Goal

Add cloud and remote storage through `rclone`.

### Planned connections

- [ ] Nextcloud
- [ ] Google Drive
- [ ] OneDrive
- [ ] WebDAV
- [ ] S3-compatible storage

### Planned features

- [ ] rclone detection
- [ ] Connection setup
- [ ] Authentication
- [ ] Cloud source
- [ ] Cloud destination
- [ ] Progress
- [ ] Resume
- [ ] Verification
- [ ] Job history
- [ ] Scheduled backup jobs

### Architecture

```text
TransferJob
     │
     ▼
Transfer Engine
     │
     ├────► rsync
     │
     └────► rclone
```

The rest of FileM4ster should not need to care which engine performs the operation.

---

# v0.6 — FileM4ster Agent

## Goal

Introduce a dedicated remote worker service.

### Component

```text
filem4ster-agent
```

### Potential platforms

- [ ] Linux
- [ ] macOS
- [ ] Windows

### Agent responsibilities

- [ ] Register worker
- [ ] Report system information
- [ ] Report storage
- [ ] Report available engines
- [ ] Receive jobs
- [ ] Validate jobs
- [ ] Execute jobs
- [ ] Stream progress
- [ ] Maintain jobs after GUI disconnect
- [ ] Resume interrupted jobs
- [ ] Verify transfers
- [ ] Report logs

### Architecture

```text
                 FileM4ster.app
                       │
           ┌───────────┼───────────┐
           ▼           ▼           ▼
     TinyMonkey     Worker B    Worker C
       Agent          Agent       Agent
           │           │           │
           ▼           ▼           ▼
        Storage      Storage      Storage
```

### Main benefit

A FileM4ster job should continue even when:

```text
FileM4ster.app
      │
      X
   closed
```

The worker remains active:

```text
filem4ster-agent
      │
      ▼
running job
```

---

# v0.7 — Backup workflows

## Goal

Build repeatable backup workflows on top of the transfer system.

### Planned features

- [ ] Saved jobs
- [ ] Reusable transfer profiles
- [ ] Scheduled execution
- [ ] Backup destinations
- [ ] Backup verification
- [ ] Backup history
- [ ] Notifications
- [ ] Failure reporting
- [ ] Retention planning

This phase should reuse the Safe Copy and verification architecture rather than create a separate backup engine.

---

# v0.8 — Storage management

## Goal

Provide a unified view of connected storage.

### Planned features

- [ ] Local disks
- [ ] NAS shares
- [ ] Remote workers
- [ ] Cloud storage
- [ ] Capacity
- [ ] Used space
- [ ] Free space
- [ ] Filesystem
- [ ] Health metadata where available
- [ ] Inventory age
- [ ] Connection status

### Example

```text
STORAGE

TinyMonkey
4 TB ext4
Used: 1.9 TB
Free: 1.7 TB
Online ✓

Buffalo
2 TB NAS
Used: 1.9 TB
Free: 1.8 GB
Legacy SMB ⚠

Google Drive
Cloud
Connected ✓
```

---

# v0.9 — Release preparation

## Goal

Prepare FileM4ster for a stable public or private v1.0 release.

### Application quality

- [ ] Major workflows tested
- [ ] Error handling reviewed
- [ ] Logs reviewed
- [ ] Performance reviewed
- [ ] UI consistency reviewed
- [ ] Accessibility reviewed
- [ ] Security review
- [ ] Data-loss scenarios tested

### macOS

- [ ] Production build
- [ ] Application icon
- [ ] Bundle metadata
- [ ] Code signing
- [ ] Notarization
- [ ] `.app` packaging
- [ ] Installation testing

### Development process

- [ ] Versioning policy
- [ ] Release notes
- [ ] Git tags
- [ ] GitHub Releases
- [ ] CI
- [ ] Automated builds
- [ ] Automated tests

---

# v1.0 — Stable release

## Goal

A version of FileM4ster suitable for normal everyday use.

The final v1.0 feature set will be determined by practical testing during the earlier releases.

At minimum, the stable release should have a reliable foundation for:

- Safe transfers
- Preflight
- Preview
- Progress
- Resume
- Verification
- Job history
- Local execution
- Remote execution
- Clear error handling
- Stable application builds

---

# Future possibilities

The following ideas are intentionally not assigned to a specific version yet.

## Transfer features

- Bandwidth limiting
- Parallel transfers
- Transfer priorities
- Transfer queues
- Scheduled transfer windows
- Incremental backup policies
- Snapshots

## Storage features

- SMART information
- Filesystem health
- Disk notifications
- Capacity forecasting

## Organization

- EXIF analysis
- Photo timelines
- Media libraries
- Advanced metadata
- AI-assisted classification
- User-defined organization rules

## Networking

- Worker discovery
- VPN-aware workers
- Remote workers over the internet
- Encrypted agent communication

## User experience

- Notifications
- Menu bar status
- Transfer widgets
- Dark and light themes
- Advanced filtering
- Search everywhere

## Automation

- Scheduled jobs
- Conditional jobs
- Storage-triggered jobs
- Automatic off-site backup
- Backup rotation

---

# Features intentionally avoided early

The following functionality should not be implemented until the safety model is mature:

```text
Automatic deletion
Automatic duplicate cleanup
Destructive mirror
Automatic source cleanup
Unreviewed mass move operations
Arbitrary shell command execution
Permanent root execution
```

These operations can cause data loss and therefore require stronger safeguards than Safe Copy.

---

# Current status

## Project foundation

- [x] Project created
- [x] Tauri 2 configured
- [x] React + TypeScript configured
- [x] Rust configured
- [x] macOS development build working
- [x] Git repository created
- [x] GitHub repository created
- [x] Project README created
- [x] Architecture documentation created
- [x] Roadmap created

## Current development focus

```text
v0.1
  │
  ▼
Application shell
  │
  ▼
Transfer Job
  │
  ▼
Source + Destination
  │
  ▼
Preflight
  │
  ▼
Safe Copy
```

---

# Immediate next steps

The next development tasks are:

1. Build the FileM4ster application shell
2. Create the first `TransferJob` model
3. Add source and destination selection
4. Implement preflight
5. Implement preview
6. Implement local Safe Copy
7. Add live progress
8. Add interruption handling
9. Add resume
10. Add verification
11. Add job history
12. Add TinyMonkey remote execution

---

# GitHub workflow

The roadmap should gradually be converted into GitHub Issues.

Example:

```text
#1 Build FileM4ster application shell

#2 Add TransferJob model

#3 Add source and destination selection

#4 Implement transfer preflight

#5 Add transfer preview

#6 Implement rsync Safe Copy engine

#7 Add live transfer progress

#8 Implement safe stop

#9 Implement resume

#10 Add transfer verification

#11 Add SQLite job history

#12 Add TinyMonkey remote execution
```

Each issue can then follow a development workflow such as:

```text
Issue
  │
  ▼
Branch
  │
  ▼
Implementation
  │
  ▼
Test
  │
  ▼
Commit
  │
  ▼
Push
  │
  ▼
Pull Request
  │
  ▼
Review
  │
  ▼
Merge
```

This workflow will become increasingly useful as FileM4ster grows.

---

# Roadmap maintenance

This roadmap is a living document.

It should change when:

- Practical implementation reveals better solutions
- Priorities change
- Features become unnecessary
- New safety requirements are discovered
- User experience testing changes the design
- Architecture evolves

Completed roadmap items should be checked off.

Major roadmap changes should be committed to Git so that the evolution of the project remains visible.

---

# Related documentation

Project overview:

[`../README.md`](../README.md)

Architecture:

[`architecture.md`](architecture.md)

---

# Current roadmap target

The immediate goal remains:

> **FileM4ster v0.1 should perform the Buffalo-to-TinyMonkey Safe Copy workflow from the GUI without requiring the user to manually manage the transfer from a terminal.**