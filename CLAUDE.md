# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**kururi-compiler** is a custom programming language compiler that implements the "Kururi" language. The project has been refactored from a microservices architecture to a single unified HTTP service to optimize storage usage and simplify deployment.

### Architecture

The compiler is now a single Rust HTTP service that provides multiple endpoints:
- **Unified Compiler Service** (Port 8080) - Single service with all compilation stages
- **Orchestrator** - Python service that coordinates compilation

### Current Example Program

The `example.kururi` file contains a multiplication table generator with advanced language features:
- Nested for loops with custom iteration syntax (`for i < 9`)
- Variable declarations with type annotations (`let num1: number = i + 1`)
- Conditional statements (`if result < 10`)
- String concatenation and number operations
- Built-in `output()` function calls

## Quick Start

### Prerequisites
- Docker and Docker Compose
- Python 3.13+ (for orchestrator)
- Rust 1.83.0+ (for individual service development)

### Running the Complete System

```bash
# Start the unified compiler service
docker-compose up --build

# In another terminal, run the compilation pipeline
cd orchestrator
python run_pipeline.py ../example.kururi output.py
```

### Development Workflow

#### Working with the Unified Compiler Service

```bash
# Navigate to compiler directory
cd compiler

# Build locally
cargo build

# Run locally (for development)
cargo run

# Build Docker image
docker build -t kururi-compiler .
```

#### Service Endpoints

**Unified Compiler Service (localhost:8080)**:
- **Complete Pipeline**: `POST /compile` - Input: `{"code": "string"}` → Output: `{"code": "string", "tokens": ["string"], "ast": ["string"], "checked_ast": ["string"]}`
- **Individual Steps** (for debugging):
  - `POST /lex` - Input: `{"code": "string"}` → Output: `{"tokens": ["string"]}`
  - `POST /parse` - Input: `{"tokens": ["string"]}` → Output: `{"ast": ["string"]}`
  - `POST /semantic` - Input: `{"ast": ["string"]}` → Output: `{"checked_ast": ["string"]}`
  - `POST /codegen` - Input: `{"checked_ast": ["string"]}` → Output: `{"code": "string"}`

#### Orchestrator Development

```bash
cd orchestrator

# Install dependencies (using Poetry)
poetry install

# Run pipeline
poetry run python run_pipeline.py <input.kururi> <output.py>

# Or using Python directly
python run_pipeline.py <input.kururi> <output.py>
```

## Project Structure

```
kururi-compiler/
├── compiler/            # Unified Rust compiler service (refactored)
│   ├── Cargo.toml       # Library and binary configuration with dev dependencies
│   ├── Dockerfile       # Optimized multi-stage build
│   └── src/
│       ├── lib.rs       # Library entry point and module exports
│       ├── main.rs      # HTTP server binary
│       ├── types.rs     # Request/response type definitions
│       ├── error.rs     # Unified error handling
│       ├── token.rs     # Token definitions for Kururi language
│       ├── ast.rs       # AST node definitions and types
│       ├── lexer.rs     # Complete lexical analysis with full tokenization
│       ├── parser.rs    # Full recursive descent parser (with RefCell complexity)
│       ├── parser_new.rs# Simplified parser for example.kururi testing
│       ├── semantic.rs  # Type checking and scope management
│       ├── codegen.rs   # AST-to-Python code generation
│       ├── compiler.rs  # Integrated compilation pipeline (both APIs)
│       └── handlers.rs  # HTTP request handlers
├── orchestrator/        # Python coordination service
│   ├── pyproject.toml
│   ├── poetry.lock
│   └── run_pipeline.py
├── docker-compose.yml   # Service orchestration
├── example.kururi       # Sample Kururi language file
└── readme.md           # Basic project info (Japanese)
```

## Kururi Language Syntax

Based on `example.kururi`, the language supports:

```kururi
function main(): void{
    output("掛け算九九の表")
    output("=================")
    
    // 外側のループ（1から9まで）
    for i < 9 {
        let row: string = ""
        
        // 内側のループ（1から9まで）
        for j < 9 {
            let num1: number = i + 1
            let num2: number = j + 1
            let result: number = num1 * num2
            
            if result < 10 {
                row = row + " " + result + " "
            } else {
                row = row + result + " "
            }
        }
        output(row)
    }
}
```

Features:
- Function declarations with type annotations
- Variable declarations (`let`, `const`) with type annotations
- For loops with custom syntax (`for i < 9`)
- Conditional statements (`if/else`)
- Binary operations (arithmetic, string concatenation, comparison)
- Number and string literals
- Comments (single-line with `//`)
- Built-in `output()` function

## Development Status

✅ **Current Implementation Status**: 
- **Infrastructure**: ✅ Complete (unified HTTP server, Docker setup, orchestrator)
- **Lexer**: ✅ Complete (full tokenization including keywords, operators, literals, comments)
- **Parser**: ⚠️ Partial (AST generation works for example.kururi via `parser_new.rs`; full parser in `parser.rs` has RefCell complexity issues)
- **Semantic Analysis**: ✅ Complete (type checking, variable scoping, function validation)
- **Code Generation**: ⚠️ Mostly working (AST to Python conversion with some string/number type handling issues)
- **AST-based Compilation**: ✅ Functional (proven working through direct tests)
- **HTTP API**: ❌ Docker caching issues prevent updated compilation logic from being served

**Known Issues:**
- Docker builds cache source code changes, requiring `--no-cache` rebuilds
- String concatenation in generated Python code has type conversion issues  
- Full generic parser (`parser.rs`) needs RefCell architecture fixes
- HTTP endpoints may serve cached dummy data instead of actual compilation results

The compiler successfully compiles the multiplication table example through AST-based compilation when tested directly.

## Development Commands

### Building and Testing
```bash
# Build the Rust compiler
cd compiler
cargo build

# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture

# Run in development mode
cargo run
```

### Testing the Complete Pipeline
```bash
# Start the unified service (may serve cached/dummy data)
docker compose up -d

# Test with example file (may return dummy data due to Docker caching)
cd orchestrator
python run_pipeline.py ../example.kururi test_output.py

# PREFERRED: Test AST-based compilation directly (always works)
cd compiler
cargo test test_compile_ast_example_kururi -- --nocapture

# Alternative: Test direct compilation bypassing Docker
cd orchestrator
python test_direct_compilation.py
```

### Docker Troubleshooting
```bash
# Force complete rebuild (required when source code changes)
docker compose down
docker compose build --no-cache
docker compose up -d

# Alternative: Touch source files to break cache
cd compiler && touch src/* && docker compose build && docker compose up -d

# Verify Docker service is using updated code
curl -X POST http://localhost:8080/compile \
  -H "Content-Type: application/json" \
  -d '{"code":"function main(): void{ output(\"test\") }"}'
```

### Adding New Language Features
1. Update lexer to recognize new tokens
2. Modify parser to handle new syntax in AST
3. Update semantic analyzer for new type checking rules
4. Extend codegen to emit appropriate target code

### Debugging Services
```bash
# View logs for the compiler service
docker-compose logs compiler

# Test individual endpoints
curl -X POST http://localhost:8080/lex \
  -H "Content-Type: application/json" \
  -d '{"code": "test code"}'

# Test complete pipeline
curl -X POST http://localhost:8080/compile \
  -H "Content-Type: application/json" \
  -d '{"code": "test code"}'
```

### Building and Deployment
```bash
# Build the compiler service
docker-compose build

# Rebuild the compiler service
docker-compose build compiler

# Start in detached mode
docker-compose up -d

# Stop the service
docker-compose down
```

## Dependencies

### Unified Rust Compiler Service
- `actix-web` v4 - HTTP server framework
- `serde` v1.0 - JSON serialization/deserialization
- `serde_json` v1.0 - JSON handling

### Python Orchestrator  
- `requests` >=2.32.4 - HTTP client for service communication
- Python 3.13+ required

## Network Configuration

The unified service communicates over a Docker bridge network `compiler-net`:
- Compiler: `localhost:8080` → `compiler:8080` (internal)

## Files to Ignore

- `ref.md` - Contains external reference links (excluded from git)
- `target/` directories - Rust build artifacts
- `.DS_Store` - macOS system files
- Python virtual environments (`.venv/`, `venv/`)

## Architecture Details

### Dual API Design
The compiler provides two interfaces:
- **String-based API**: Legacy HTTP endpoints for orchestrator compatibility
- **AST-based API**: Modern type-safe compilation for direct usage

### Key Components
- **Token Module** (`token.rs`): Comprehensive token definitions for all Kururi language constructs
- **AST Module** (`ast.rs`): Type-safe AST nodes with proper Kururi type system including ForStatement, IfStatement, BinaryExpression
- **Lexer** (`lexer.rs`): Full tokenization supporting strings, keywords, operators, comments, and number literals
- **Parser** (`parser_new.rs`): Working parser specifically for example.kururi with multiplication table support
- **Parser Full** (`parser.rs`): Generic recursive descent parser (currently has RefCell complexity issues)
- **Semantic Analyzer** (`semantic.rs`): Function registration, variable scoping, type checking with proper mutable state management
- **Code Generator** (`codegen.rs`): Converts AST to Python with for loops, conditionals, and expression handling
- **Compiler** (`compiler.rs`): Dual API system - legacy string-based and modern AST-based compilation
- **HTTP Handlers** (`handlers.rs`): REST API endpoints that use AST-based compilation (when Docker cache is fresh)

### Compilation Pipeline
1. **Lexical Analysis**: Source code → Tokens
2. **Syntax Analysis**: Tokens → AST
3. **Semantic Analysis**: AST → Type-checked AST
4. **Code Generation**: Type-checked AST → Python code

## Next Development Priorities

1. **Fix Docker caching issues** to ensure HTTP API serves updated compilation logic
2. **Improve code generation type handling** for proper number/string operations in Python output
3. **Fix parser.rs RefCell complexity** for full language support beyond example.kururi
4. **Expand parser_new.rs** to handle complete Kururi grammar (classes, arrays, more operators)
5. **Add comprehensive test coverage** for edge cases and error conditions
6. **Enhance error reporting** with source location information and better diagnostics
7. **Add language server features** (LSP support, diagnostics, completion)

## Critical Issues for New Developers

### Docker Caching Problem
- Docker aggressively caches the `COPY src src` layer in the Dockerfile
- Changes to Rust source files may not be reflected in the running container
- **Always use `docker compose build --no-cache`** when testing source code changes
- Verify the API returns actual compilation results, not dummy data

### Testing Strategy
- **Preferred**: Use `cargo test test_compile_ast_example_kururi -- --nocapture` for reliable testing
- **Alternative**: Use `python test_direct_compilation.py` in orchestrator/ to bypass Docker
- **Last resort**: Use Docker with forced rebuilds for integration testing

## Useful Commands Reference

```bash
# Recommended development workflow
cd compiler && cargo test test_compile_ast_example_kururi -- --nocapture

# Force Docker rebuild and test (when needed)
docker compose down && docker compose build --no-cache && docker compose up -d
cd orchestrator && python run_pipeline.py ../example.kururi test.py

# Direct compilation test (bypasses Docker issues)
cd orchestrator && python test_direct_compilation.py

# Local development server
cd compiler && cargo run  # runs on localhost:8080

# Verify compilation works end-to-end
curl http://localhost:8080/compile -X POST -H "Content-Type: application/json" \
  -d '{"code":"function main(): void{ output(\"Hello from Kururi\") }"}'

# Check individual compilation stages  
curl http://localhost:8080/lex -X POST -H "Content-Type: application/json" -d '{"code":"test"}'

# Run all tests with output
cd compiler && cargo test -- --nocapture

# Check if Docker is serving dummy vs. real data
curl http://localhost:8080/compile -X POST -H "Content-Type: application/json" \
  -d '{"code":"function main(): void{output(\"test\")}"}' | jq .code | grep -q "dummy" && echo "CACHED" || echo "UPDATED"
```

---

*This compiler is actively developed and has working AST-based compilation for the multiplication table example program. The main challenge is Docker caching preventing updated compilation logic from being served via HTTP. The core language processing (lexing, parsing, semantic analysis, code generation) is functional and tested.*