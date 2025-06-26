#!/usr/bin/env python3
import requests
import sys


COMPILER_URL = "http://localhost:8080"

def run_pipeline(source_code: str) -> str:
    # 統合されたコンパイラサービスで全パイプラインを実行
    payload = {"code": source_code}
    print(f"\n[COMPILER] POST {COMPILER_URL}/compile")
    print("  request payload:", payload)
    r = requests.post(f"{COMPILER_URL}/compile", json=payload)
    print("  response status:", r.status_code)
    print("  response body  :", r.text)
    r.raise_for_status()
    
    result = r.json()
    tokens = result.get("tokens")
    ast = result.get("ast")
    checked_ast = result.get("checked_ast")
    code = result.get("code")
    
    print("\n[COMPILER] Pipeline Results:")
    print("  tokens      :", tokens)
    print("  ast         :", ast)
    print("  checked_ast :", checked_ast)
    print("  final code  :", code)

    return code

def run_individual_step(step: str, source_code: str = None, tokens: list = None, ast: list = None, checked_ast: list = None):
    """個別ステップの実行（デバッグ用）"""
    if step == "lex" and source_code:
        payload = {"code": source_code}
        r = requests.post(f"{COMPILER_URL}/lex", json=payload)
        return r.json()
    elif step == "parse" and tokens:
        payload = {"tokens": tokens}
        r = requests.post(f"{COMPILER_URL}/parse", json=payload)
        return r.json()
    elif step == "semantic" and ast:
        payload = {"ast": ast}
        r = requests.post(f"{COMPILER_URL}/semantic", json=payload)
        return r.json()
    elif step == "codegen" and checked_ast:
        payload = {"checked_ast": checked_ast}
        r = requests.post(f"{COMPILER_URL}/codegen", json=payload)
        return r.json()
    else:
        raise ValueError(f"Invalid step: {step} or missing required parameters")


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
