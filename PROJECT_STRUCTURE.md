# Meme Koin - Project Structure

This document outlines the modular architecture of the Meme Koin application built with Leptos and Cloudflare Workers.

## 📁 Directory Structure

```
src/
├── lib.rs                  # Main library entry point
├── app/                    # Application core
│   ├── mod.rs             # App module exports
│   ├── router.rs          # Main app component and routing
│   └── shell.rs           # HTML shell for SSR
├── pages/                  # Page components
│   ├── mod.rs             # Page module exports
│   └── home.rs            # Home page component
├── components/            # Reusable UI components
│   ├── mod.rs             # Component module exports
│   └── ui/                # UI component library
│       ├── mod.rs         # UI module exports
│       ├── button.rs      # Button component
│       ├── card.rs        # Card components
│       └── layout.rs      # Layout components
├── server/                # Server-side functionality
│   ├── mod.rs             # Server module exports
│   └── functions.rs       # Server functions
└── utils/                 # Utility functions
    ├── mod.rs             # Utils module exports
    ├── format.rs          # Formatting utilities
    └── validation.rs      # Validation utilities
```

## 🏗️ Architecture Overview

### Core Modules

#### `app/` - Application Core
- **`router.rs`**: Contains the main `App` component with routing setup
- **`shell.rs`**: HTML shell template for server-side rendering
- **`mod.rs`**: Exports the main app components

#### `pages/` - Page Components
- **`home.rs`**: Home page component with hero section and features
- **`mod.rs`**: Exports all page components

#### `components/` - Reusable Components
- **`ui/button.rs`**: Configurable button component with variants and sizes
- **`ui/card.rs`**: Card components (Card, CardHeader, CardContent, CardFooter)
- **`ui/layout.rs`**: Layout components (Container, Grid, Flex)
- **`ui/mod.rs`**: Exports all UI components

#### `server/` - Server Functions
- **`functions.rs`**: Server functions for API endpoints
- **`mod.rs`**: Exports server functions

#### `utils/` - Utilities
- **`format.rs`**: Formatting functions (currency, numbers, percentages)
- **`validation.rs`**: Validation functions (email, wallet, password)
- **`mod.rs`**: Exports utility functions

## 🔧 Key Features

### Component System
- **Modular Design**: Each component is self-contained and reusable
- **Type Safety**: Full TypeScript-like safety with Rust
- **Props System**: Flexible prop system with optional parameters

### Server Functions
- **API Endpoints**: Server functions automatically create API endpoints
- **Type Safety**: Shared types between client and server
- **Worker Integration**: Seamless integration with Cloudflare Workers

### Utilities
- **Formatting**: Currency, number, and percentage formatting
- **Validation**: Input validation for forms and user data
- **Reusable**: Shared utilities across the application

## 🚀 Usage Examples

### Adding a New Page
1. Create a new file in `src/pages/`
2. Add the component to `src/pages/mod.rs`
3. Add a route in `src/app/router.rs`

### Adding a New Component
1. Create a new file in `src/components/ui/`
2. Add the component to `src/components/ui/mod.rs`
3. Use the component in your pages

### Adding a Server Function
1. Add the function to `src/server/functions.rs`
2. Register it in `src/lib.rs` in `register_server_functions()`
3. Use it in your components with `use_server_fn()`

## 📦 Module Exports

Each module has a `mod.rs` file that re-exports its public components, making imports clean and organized:

```rust
// Instead of:
use crate::components::ui::button::Button;
use crate::components::ui::card::Card;

// You can use:
use crate::components::ui::{Button, Card};
```

This structure provides a clean, maintainable, and scalable foundation for the Meme Koin application.
