# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

### Building
- **Native build**: `cargo build --release`
- **Web build**: `trunk build --release`
- **Development build**: `cargo build` (faster compilation)

### Running
- **Native app**: `cargo run --release`
- **Web app**: `trunk serve --release` (then open http://localhost:8080)

### Testing
- **Run all tests**: `cargo test`
- **Run specific test**: `cargo test [TESTNAME]`

## Architecture Overview

AgIsoTerminalDesigner is a graphical editor for designing ISOBUS Virtual Terminal object pools. The application is built with Rust using the egui framework and can run both as a native desktop application and as a web application.

### Key Components

1. **Main Application** (`src/main.rs`): Entry point containing the `DesignerApp` struct that manages the entire application state, UI layout, and file operations.

2. **Core Library** (`src/lib.rs`): Exports the main traits and types used throughout the application.

3. **EditorProject** (`src/editor_project.rs`): Manages the project state including:
   - Object pool management with undo/redo functionality
   - Selected object tracking with navigation history
   - Object renaming and custom naming support
   - Uses RefCell pattern for mutable state management within the UI framework

4. **Object Configuration** (`src/object_configuring.rs`): Implements the `ConfigurableObject` trait for rendering parameter UI for each object type.

5. **Object Rendering** (`src/object_rendering.rs`): Implements the `RenderableObject` trait for visual rendering of objects in the preview.

6. **Object Relationships** (`src/allowed_object_relationships.rs`): Defines which object types can reference other types according to ISO 11783 specifications.

7. **Object Defaults** (`src/object_defaults.rs`): Provides default instances for each object type.

### Key Patterns

- **Trait-based Design**: Uses `ConfigurableObject` and `RenderableObject` traits to provide consistent interfaces for all object types.
- **RefCell Pattern**: Enables interior mutability for UI state management while maintaining Rust's borrowing rules.
- **Undo/Redo System**: Maintains separate histories for object pool changes and selection changes.
- **ID Management**: Objects are identified by `ObjectId` with careful tracking when IDs change.

### Dependencies

- **ag-iso-stack**: Core ISO 11783 object pool implementation
- **egui/eframe**: Cross-platform GUI framework
- **rfd**: File dialog support
- **trunk**: Web application bundler (for web builds)

### UI Structure

The application uses egui panels:
- Top panel: Menu bar with file operations and object creation
- Left panel: Object hierarchy tree and object list with search
- Central panel: Visual preview of the active mask
- Right panel: Object parameter configuration and preview

### Important Notes

- The application supports both native and web platforms with platform-specific code paths
- Object pool files use the `.iop` extension
- The project is under active development with many objects partially implemented