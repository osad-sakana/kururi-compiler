#!/usr/bin/env python3
import requests
import sys


LEXER_URL = "http://localhost:60000/lex"
PARSER_URL = "http://localhost:60001/parse"
SEMANTIC_URL = "http://localhost:60002/semantic"
CODEGEN_URL = "http://localhost:60003/codegen"


def run_pipeline(source_code: str) -> str:
    # 1. lexer 字句解析
    payload = {"code": source_code}
    print(f"\n[LEXER] POST {LEXER_URL}")
    print("  request payload:", payload)
    r = requests.post(LEXER_URL, json=payload)
    print("  response status:", r.status_code)
    print("  response body  :", r.text)
    r.raise_for_status()
    tokens = r.json().get("tokens")
    print("  parsed tokens  :", tokens)

    # 2. parser 構文解析
    payload = {"tokens": tokens}
    print(f"\n[PARSER] POST {PARSER_URL}")
    print("  request payload:", payload)
    r = requests.post(PARSER_URL, json=payload)
    print("  response status:", r.status_code)
    print("  response body  :", r.text)
    r.raise_for_status()
    ast = r.json().get("ast")
    print("  parsed AST     :", ast)

    # 3. semantic セマンティック解析
    payload = {"ast": ast}
    print(f"\n[SEMANTIC] POST {SEMANTIC_URL}")
    print("  request payload:", payload)
    r = requests.post(SEMANTIC_URL, json=payload)
    print("  response status:", r.status_code)
    print("  response body  :", r.text)
    r.raise_for_status()
    checked_ast = r.json().get("checked_ast")
    print("  checked AST    :", checked_ast)

    # 4. codegen コード生成
    payload = {"checked_ast": checked_ast}
    print(f"\n[CODEGEN] POST {CODEGEN_URL}")
    print("  request payload:", payload)
    r = requests.post(CODEGEN_URL, json=payload)
    print("  response status:", r.status_code)
    print("  response body  :", r.text)
    r.raise_for_status()
    code = r.json().get("code")
    print("\n[CODEGEN] generated code:\n", code)

    return code


def main():
    if len(sys.argv) != 3:
        print("使い方: run_pipeline.py <入力.kururi> <出力.py>")
        sys.exit(1)

    src_path, out_path = sys.argv[1], sys.argv[2]
    source = open(src_path, encoding="utf-8").read()
    code = run_pipeline(source)
    with open(out_path, "w", encoding="utf-8") as f:
        f.write(code)
    print(f"\n生成完了: {out_path}")


if __name__ == "__main__":
    main()
