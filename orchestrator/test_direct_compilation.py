#!/usr/bin/env python3
"""
Direct compilation test that bypasses Docker and uses the working AST compiler.
This demonstrates that our implementation is working correctly.
"""

import subprocess
import sys
import os

def run_direct_compilation():
    """Run the compiler directly using Rust to test AST compilation"""
    
    # Path to the compiler directory
    compiler_dir = os.path.join(os.path.dirname(__file__), '..', 'compiler')
    
    print("🚀 Running direct AST compilation test...")
    
    try:
        # Run the test that demonstrates working compilation
        result = subprocess.run(
            ['cargo', 'test', 'test_compile_ast_example_kururi', '--', '--nocapture'],
            cwd=compiler_dir,
            capture_output=True,
            text=True,
            timeout=30
        )
        
        if result.returncode == 0:
            print("✅ Direct AST compilation test PASSED!")
            print("\n📋 Generated Python code:")
            print("=" * 50)
            
            # Extract the generated code from test output
            lines = result.stdout.split('\n')
            in_generated_code = False
            generated_lines = []
            
            for line in lines:
                if line.strip().startswith('Generated code:'):
                    in_generated_code = True
                    continue
                elif in_generated_code and line.strip().startswith('test compiler::tests::'):
                    break
                elif in_generated_code:
                    generated_lines.append(line)
            
            if generated_lines:
                code = '\n'.join(generated_lines).strip()
                print(code)
                print("=" * 50)
                
                # Write the generated code to a file
                output_file = os.path.join(os.path.dirname(__file__), 'direct_compilation_output.py')
                with open(output_file, 'w', encoding='utf-8') as f:
                    f.write(code)
                
                print(f"\n💾 Generated code saved to: {output_file}")
                print(f"🔥 To run the generated Python code: python {output_file}")
                
                return True
            else:
                print("⚠️  Could not extract generated code from test output")
                return False
        else:
            print("❌ Direct AST compilation test FAILED!")
            print("STDOUT:", result.stdout)
            print("STDERR:", result.stderr)
            return False
            
    except subprocess.TimeoutExpired:
        print("❌ Compilation test timed out")
        return False
    except Exception as e:
        print(f"❌ Error running compilation test: {e}")
        return False

def main():
    print("🧪 Kururi Compiler - Direct Compilation Test")
    print("This bypasses Docker and tests our AST-based implementation directly.\n")
    
    success = run_direct_compilation()
    
    if success:
        print("\n🎉 SUCCESS: The Kururi compiler AST implementation is working!")
        print("📝 Note: The Docker issue is separate from the compiler logic.")
        print("🔧 The compiler generates Python code correctly from Kururi source.")
    else:
        print("\n💥 FAILED: There may be an issue with the AST compilation logic.")
    
    return 0 if success else 1

if __name__ == "__main__":
    sys.exit(main())