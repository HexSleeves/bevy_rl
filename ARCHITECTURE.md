# Bevy Roguelike Architecture

## Overview

This is a turn-based roguelike game built with Bevy 0.15, following the Model-View-Controller (MVC) pattern. The game is structured into distinct plugins that handle different aspects of the game logic, promoting modularity and separation of concerns.

## Core Architecture

### 1. Plugin Structure

The game is divided into four main plugins, each with a specific responsibility:

- **ModelPlugin**: Manages game state, entities, and core game logic
- **ViewPlugin**: Handles rendering and visual representation
- **ControllerPlugin**: Processes input and converts it to game actions
- **UiPlugin**: Manages user interface elements and camera

This separation allows for independent development and testing of each component, making the codebase more maintainable and extensible.

### 2. Model (Game State)

Located in `src/model/`

#### Components

- **PlayerTag**: Marks an entity as player-controlled
- **Position**: Stores entity location with comprehensive vector operations
- **Description**: Contains entity description text
- **Renderable**: Visual representation (glyph and color)
- **TerrainType**: Defines terrain properties (Wall, Floor)
- **TurnActor**: Manages turn-based timing and scheduling
- **Actor**: Identifies entities that can take actions

#### Resources

- **CurrentMap**: Stores terrain layout using a HashMap for efficient position lookups
- **TurnQueue**: Manages turn order and timing for all actors

#### Actions

- **GameAction**: Trait for defining executable game actions
- **BuildableGameAction**: Trait for actions with builder pattern support
- **Walk**: Concrete implementation for movement actions using builder pattern

#### Commands

- **TryMove**: EntityCommand for attempting movement with collision detection

#### Types

- **MoveDirection**: Enum for cardinal directions with vector operations
- **ActionType**: Enum for different action types (Move, Attack, Wait)
- **GameError**: Error types for game logic failures

#### Systems

- **spawn_map**: Procedurally generates the game world
- **spawn_player**: Creates the player entity with required components
- **process_turns**: Core turn management system that schedules actor turns

#### Constants

- **ModelConstants**: Defines map dimensions and other model-specific constants

### 3. View (Rendering)

Located in `src/view/`

#### Systems

- **position_to_transform**: Converts logical positions to screen coordinates
- **add_sprite_to_tile**: Adds visual representation to map tiles

#### Utils

- **spawner**: Helper functions for creating visual entities

#### Constants

- **ViewConstants**: Defines tile size and other rendering parameters

### 4. Controller (Input)

Located in `src/controller/`

#### Systems

- **keyboard_input**: Processes keyboard input and converts it to game actions

#### Types

- **Direction**: Enum for movement directions with vector operations
- **InputAction**: Player-specific actions that can be mapped to keys

### 5. UI (Interface)

Located in `src/ui/`

#### Components

- **GameCamera**: Marks the main game camera entity

#### Systems

- **spawn_camera**: Creates and configures the game camera

### 6. Macros and Utilities

Located in `src/macros/`

#### Debug Macros

- **impl_debug_with_field**: Macro for implementing Debug trait for structs
- **impl_game_action**: Macro for implementing the GameAction builder pattern

## Game Flow

1. The game operates in three distinct states:

   - **Load**: Initial loading of assets and game setup
   - **Paused**: Game logic suspended but rendering continues
   - **Running**: Normal gameplay with all systems active

2. Update cycle follows the AppSet order:

   - **RecordInput**: Capture and process player actions
   - **Visibility**: Update field of view and visibility calculations
   - **Update**: Process game logic, physics, and state changes
   - **Render**: Update visual representations and camera

3. Turn-based flow:
   - Turn queue tracks all actors and their next action time
   - Player input is awaited when it's the player's turn
   - Actions are executed through the GameAction trait
   - Turn timing is managed by the TurnActor component

## Technical Implementation

### ECS Architecture

The game leverages Bevy's Entity Component System (ECS) for efficient state management:

- **Entities**: Represent game objects (player, enemies, items)
- **Components**: Store data for entities (Position, Renderable)
- **Systems**: Process entities with specific components
- **Resources**: Store global state (Map, TurnQueue)
- **Events**: Handle communication between systems
- **Commands**: Define entity-specific operations (TryMove)

### Window Configuration

- Default size: 800x600 pixels
- Configurable fullscreen mode through AppSettings
- Custom window title from AppConstants

### Asset Management

- Uses nearest neighbor filtering for pixel-perfect rendering
- Custom asset path configuration
- Text-based rendering for ASCII display

## Design Patterns

### 1. Entity Component System (ECS)

- Composition over inheritance for game objects
- Data-oriented design for performance
- Systems operate on component queries for efficient processing
- Clear separation of data (Components) and behavior (Systems)

### 2. Plugin Architecture

- Modular design with clear boundaries
- Each plugin registers its components, resources, and systems
- Startup and update systems properly organized
- System sets ensure correct execution order

### 3. Turn-Based Management

- Priority queue approach for actor scheduling
- Action component pattern for command processing
- Clear separation between input handling and action execution
- Builder pattern for constructing game actions

### 4. Vector Operations

- Comprehensive Position implementation with operator overloading
- Direction enums with vector operations
- Efficient position calculations and transformations

### 5. Error Handling

- GameError enum for type-safe error handling
- Proper error propagation in game actions
- Result return types for fallible operations

## Code Organization

```bash
src/
├── app_constants.rs    # Global constants
├── app_settings.rs     # Configuration settings
├── controller/         # Input handling
├── macros/             # Custom macros for the project
│   ├── debug.rs        # Debug utilities and macros
├── main.rs             # Application entry point
├── model/              # Game state and logic
│   ├── actions/        # Game action implementations
│   ├── commands/       # Entity command implementations
│   ├── components/     # Entity components
│   ├── resources/      # Global resources
│   ├── systems/        # Game logic systems
│   ├── types/          # Type definitions and traits
│   └── utils/          # Helper functions
├── ui/                 # User interface
└── view/               # Rendering and display
```

## Future Enhancements

### Immediate Improvements

- Enhanced map generation with rooms and corridors
- Field of view and lighting system
- Enemy AI with pathfinding
- Combat system with stats and damage calculation
- Item and inventory system

### Long-term Considerations

- Save/load game functionality
- Procedural content generation
- Multiple levels and progression
- Status effects and buff system
- Sound and music integration
- Particle effects for visual feedback

### Technical Debt

- Add comprehensive unit tests
- Implement proper error handling
- Add documentation comments for public APIs
- Consider performance optimizations for larger maps
- Implement proper logging system

## Development Workflow

- Use Bevy's hot reloading for faster iteration
- Leverage Rust's type system for compile-time safety
- Follow Rust idioms and best practices
- Maintain separation of concerns through the MVC pattern
- Utilize builder pattern for complex object construction
