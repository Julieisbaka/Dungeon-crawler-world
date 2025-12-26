# Grid Visualization Feature - Testing Guide

## Overview
This document explains how to test the new grid visualization feature, which displays the procedural generation of the first floor dungeon with cells, neighborhoods, MST connections, and special rooms.

## Prerequisites
- The application must be built with the `dev-mode` feature enabled (default in debug builds)
- Developer mode must be enabled in settings

## Testing Instructions

### 1. Launch the Application
```bash
cargo run
```

### 2. Enable Developer Mode
- Go to Settings
- Enable "Developer Mode"
- Enable "Show Console"

### 3. Open the Grid Visualization
In the console window, type:
```
invoke grid
```

This will open the Grid Visualization preview window.

### 4. Verify Grid Visualization Features

#### Cell Structure
- **Expected**: A 3x3 grid of cells should be visible
- Each cell should have clear borders in dark gray
- Each cell should be divided into 4 quadrants (neighborhoods)

#### Neighborhoods
- **Expected**: Within each cell, 4 neighborhoods are visible
- Neighborhood boundaries shown with lighter gray dividing lines
- Each neighborhood should contain randomly placed nodes (5-15 per neighborhood)

#### MST (Minimum Spanning Tree)
- **Expected**: Within each neighborhood, nodes are connected by light gray lines
- Lines form a tree structure (no cycles)
- All nodes within a neighborhood should be connected

#### Special Rooms (Color-Coded)
- **Gray nodes**: Normal rooms
- **Blue nodes**: Bathrooms
- **Green nodes**: Safe Rooms
- **Orange/Yellow nodes**: Stairwells
- Special rooms should have a white outline

#### Legend
- A color legend should appear at the top of the window
- Shows the meaning of each node color

#### Controls
- **Regenerate button**: Clicking should generate a new random grid
- **Reset View button**: Resets zoom and pan to default
- **Zoom controls**: + and - buttons to zoom in/out
- **Maximize button**: Expands window to full screen

### 5. Test Regeneration
- Click the "Regenerate" button multiple times
- **Expected**: Grid should regenerate with different node positions and MST edges each time
- Structure (3x3 cells, 4 neighborhoods per cell) should remain constant

### 6. Test Zoom
- Click the + button to zoom in
- Click the - button to zoom out
- **Expected**: 
  - Grid elements scale appropriately
  - Zoom level displayed accurately (e.g., "1.5x")
  - Zoom range limited (0.1x to 5.0x)

### 7. Command Integration
Test the console command integration:
```
help
```
- **Expected**: Help text should include the "invoke grid" command

### 8. Dev-Mode Gating
To verify dev-mode gating works:
```bash
cargo build --no-default-features
```
- The grid feature should be excluded from the build when dev-mode is disabled

## Known Limitations in CI
- Tests requiring graphics libraries (Vulkan, X11) cannot run in CI environment
- `cargo check --lib` validates code correctness without requiring graphics libraries
- Manual testing on a local machine with graphics support is required for full UI validation

## Screenshots
When testing, take screenshots showing:
1. The full grid visualization window with all elements visible
2. The legend and control buttons
3. Zoom in/out states
4. Different regenerated grids
