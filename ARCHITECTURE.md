# Bevy Roguelike Architecture

## Overview

This is a turn-based roguelike game built with Bevy 0.15, following the Model-View-Controller (MVC) pattern. The game is structured into distinct plugins that handle different aspects of the game logic.

## Core Architecture

### 1. Plugin Structure

The game is divided into four main plugins:

- ModelPlugin: Game state and logic
- ViewPlugin: Rendering and display
- ControllerPlugin: Input handling
- UiPlugin: User interface elements

### 2. Model (Game State)

Located in `src/model/`

- Components:
  - Player
  - Position
  - Description
  - Renderable
  - TerrainType
  - TurnQueue
- Resources:
  - Map (with size and tiles)
- Systems:
  - spawn_map
  - spawn_player
  - process_turns
  - execute_actions

### 3. View (Rendering)

Located in `src/view/`

- Systems:
  - ascii_renderer: Handles the ASCII rendering of game elements
- Utils:
  - spawner: Helper functions for spawning visual elements

### 4. Controller (Input)

Located in `src/controller/`

- Systems:
  - keyboard_input: Processes player input
- Commands:
  - try_move: Handles movement attempts

### 5. UI (Interface)

Located in `src/ui/`

- Components:
  - game_camera
- Systems:
  - spawn_camera
- Constants:
  - ui_constants

## Game Flow

1. The game runs in distinct states (Load, Paused, Running)
2. Update cycle follows the AppSet order:
   - RecordInput: Capture player actions
   - Visibility: Update what can be seen
   - Update: Process game logic
   - Render: Display the game state

## Technical Details

- Window Configuration:
  - Default size: 800x600
  - Supports fullscreen mode
- Asset Management:
  - Uses nearest neighbor filtering for pixel-perfect rendering
  - Custom asset path configuration

## Design Patterns

1. Entity Component System (ECS)

   - Leverages Bevy's ECS for efficient game state management
   - Clear separation of data (Components) and behavior (Systems)

2. Plugin Architecture

   - Modular design with clear boundaries
   - Each plugin handles a specific aspect of the game

3. Resource Management
   - Centralized game state through Bevy resources
   - Map state managed through HashMap for efficient lookups

## Future Considerations

- Expandable component system for new entities
- Modular combat system
- Flexible map generation system
