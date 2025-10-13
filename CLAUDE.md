# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is Fulcitt POS - a Point of Sale application built with Tauri (Rust backend) + Vue 3 (TypeScript frontend). It's designed as a minimal POS system for Gruppo Fulcitt with features like product management, sales processing, receipt printing, and sales reporting.

## Development Commands

### Main Development
- `bun run dev` - Start development server with hot reload
- `bun run build` - Build for production (runs TypeScript check + Vite build)
- `bun run preview` - Preview production build

### Code Quality
- `bun run lint` - Run ESLint on Vue/TypeScript files
- `bun run lint:fix` - Auto-fix ESLint issues

### Database Operations
- `bun run sqlx` - Run SQLx CLI commands in Rust backend
- `bun run sqlx:make_migration` - Create new database migration

### Tauri/Rust Operations
- `bun run tauri` - Run Tauri CLI commands
- `bun run cargo` - Run Cargo commands in src-tauri directory

## Architecture

### Frontend (Vue 3 + TypeScript)
- **Framework**: Vue 3 with Composition API
- **State Management**: Pinia stores (`cartStore.ts`, `messagesStore.ts`)
- **Routing**: Vue Router in `routes.ts`
- **Styling**: TailwindCSS + DaisyUI components
- **API Layer**: Tauri invoke functions in `repositories.ts`

### Backend (Rust + Tauri)
- **Database**: SQLite with SQLx for migrations and queries
- **Core Logic**: Located in `src-tauri/src/lib.rs` with Tauri commands
- **Models**: Data structures in `src-tauri/src/models.rs`
- **Printing**: ESC/POS thermal printer support via USB
- **Exports**: Excel export functionality for sales data
- **Internationalization**: Fluent localization (Italian support)

### Key Components
- **POS Interface**: Product selection, cart management, checkout
- **Product Management**: CRUD operations for inventory
- **Sales Processing**: Transaction handling with receipt printing
- **Reports**: Sales analytics and data export
- **Settings**: Printer configuration and layout customization

### Database Schema
Uses SQLite with migrations in `src-tauri/migrations/`:
- `products` - Product catalog with UUID-based IDs
- `sales` - Transaction records
- `sale_items` - Line items for each sale

### Known Build Issues
- Linux builds require `NO_STRIP=true` environment variable due to Tauri issue #8929
- Printer configuration persists in Tauri's store.json

## Development Notes

- Uses Bun as package manager (not npm/yarn)
- Hot reload runs on port 1420 in development
- Database is automatically created in app data directory
- Printer functionality uses different drivers in debug vs release builds (Console vs USB)
- Italian localization files are in `locales/it/app.ftl`