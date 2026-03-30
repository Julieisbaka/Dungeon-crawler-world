# Grid Visualization Quick Reference

## Quick Start

1. **Launch the game** in debug mode (dev-mode enabled by default)
2. **Enable Developer Mode** in Settings
3. **Open the Console** (enable "Show Console" in Settings)
4. **Type command**: `invoke grid`

## Commands

| Command | Description |
|---------|-------------|
| `invoke grid` | Open the grid visualization window |
| `help` | Show all available console commands |

## UI Controls

### Buttons

- **Regenerate**: Generate a new random grid layout
- **Reset View**: Reset zoom to 1.0x and center the view
- **+ / -**: Zoom in/out (range: 0.1x - 5.0x)
- **Maximize**: Toggle full-screen window mode

### Visual Elements

#### Cell Structure

- **Thick dark borders**: Cell boundaries (3x3 grid)
- **Thin gray lines**: Neighborhood divisions (4 per cell)

#### Node Colors

- **Gray**: Normal room
- **Blue**: Bathroom/Restroom
- **Green**: Safe Room
- **Orange**: Stairwell

#### Connections

- **Light gray lines**: MST connections between nodes

## Understanding the Layout

### Grid Hierarchy

```
Floor
 └─ Grid (3x3 cells)
     └─ Cell (4 neighborhoods)
         └─ Neighborhood (5-15 nodes)
             └─ Node (room with type)
```

### What is MST?

**Minimum Spanning Tree** - An algorithm that connects all nodes in a neighborhood using the shortest total distance without creating loops. This ensures every room is reachable while keeping the layout efficient.

## Keyboard Shortcuts

Currently the grid visualization uses button controls only. Future versions may include:

- Mouse wheel zoom
- Click-and-drag pan
- Arrow key navigation

## Troubleshooting

### "Unknown command: invoke"

- Make sure Developer Mode is enabled in Settings
- Restart the application if you just enabled it

### Window not appearing

- Check that dev-mode is enabled (default in debug builds)
- Try closing and reopening with `invoke grid` again

### Grid looks empty

- Click "Regenerate" to generate a new grid
- Try zooming out with the "-" button

### Can't see all cells

- Use the scroll bars to navigate
- Click "Reset View" to center the grid
- Try zooming out to see more of the grid

## For Developers

### Building without grid feature

```bash
cargo build --no-default-features
```

### Running tests

```bash
cargo check --lib  # Validates code without graphics
```

See GRID_TESTING.md for comprehensive testing procedures.

## Technical Details

- **Default grid size**: 3x3 cells
- **Cell size**: 200 units
- **Nodes per neighborhood**: 5-15 (random)
- **MST algorithm**: Kruskal's algorithm
- **Special room spawn rates**:
  - Bathroom: ~40%
  - Safe Room: ~10%
  - Stairwell: ~5%

## Related Documentation

- `GRID_TESTING.md` - Comprehensive testing guide
- `GRID_IMPLEMENTATION.md` - Technical implementation details
- `tests/grid_tests.rs` - Unit test suite
