# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**kururi-compiler** is a custom programming language compiler that implements the "Kururi" language. The project has been refactored from a microservices architecture to a single unified HTTP service to optimize storage usage and simplify deployment.

### Architecture

The compiler is now a single Rust HTTP service that provides multiple endpoints:
- **Unified Compiler Service** (Port 8080) - Single service with all compilation stages
- **Orchestrator** - Python service that coordinates compilation

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
│   ├── Cargo.toml       # Library and binary configuration
│   ├── Dockerfile       # Optimized multi-stage build
│   └── src/
│       ├── lib.rs       # Library entry point and module exports
│       ├── main.rs      # HTTP server binary
│       ├── types.rs     # Request/response type definitions
│       ├── error.rs     # Unified error handling
│       ├── lexer.rs     # Lexical analysis logic
│       ├── parser.rs    # Syntax analysis logic
│       ├── semantic.rs  # Semantic analysis logic
│       ├── codegen.rs   # Code generation logic
│       ├── compiler.rs  # Integrated compilation pipeline
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
function main(): void {
    const moji: string = "Hello World by Kururi!"
    output(moji)
}
```

Features:
- Function declarations with type annotations
- Constant declarations with type annotations  
- String literals
- Built-in `output()` function

## Development Status

⚠️ **Current Implementation Status**: 
- **Infrastructure**: ✅ Complete (unified HTTP server, Docker setup, orchestrator)
- **Unified Service**: ✅ Complete (all endpoints functional)
- **Lexer**: ⚠️ Dummy implementation (returns input as single token)
- **Parser**: ⚠️ Dummy implementation (passes tokens through as AST)
- **Semantic**: ⚠️ Dummy implementation (passes AST through unchanged)
- **Codegen**: ⚠️ Basic implementation (generates Python print statements)
- **Storage Optimization**: ✅ Complete (reduced from 4 containers to 1)

## Common Development Tasks

### Testing the Pipeline
```bash
# Start the unified service
docker-compose up -d

# Test with example file
cd orchestrator
python run_pipeline.py ../example.kururi test_output.py

# Check generated output
cat test_output.py
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

## Next Development Priorities

1. **Implement proper lexical analysis** in lexer service
2. **Build recursive descent parser** in parser service  
3. **Add type checking and symbol table** in semantic service
4. **Enhance code generation** for complete language features
5. **Add comprehensive test suite** for each service
6. **Add error handling and reporting** across pipeline
7. **Implement language specification** documentation

## Useful Commands Reference

```bash
# Complete rebuild and test
docker-compose down && docker-compose up --build -d
cd orchestrator && python run_pipeline.py ../example.kururi test.py

# Local development
cd compiler && cargo run  # runs on localhost:8080

# Check service health
curl http://localhost:8080/lex -X POST -H "Content-Type: application/json" -d '{"code":"test"}'
curl http://localhost:8080/parse -X POST -H "Content-Type: application/json" -d '{"tokens":["test"]}'
curl http://localhost:8080/semantic -X POST -H "Content-Type: application/json" -d '{"ast":["test"]}'
curl http://localhost:8080/codegen -X POST -H "Content-Type: application/json" -d '{"checked_ast":["test"]}'
curl http://localhost:8080/compile -X POST -H "Content-Type: application/json" -d '{"code":"test"}'
```

---

*This compiler is in active development. The current implementation provides a working unified pipeline infrastructure but requires actual language processing logic to be implemented. The architecture has been optimized for reduced storage usage by consolidating microservices into a single container.*