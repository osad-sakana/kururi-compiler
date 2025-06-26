# Kururiの文法

```kururi
// 変数宣言（型必須・セミコロン不要）
const test: string = "Hello World"
let   price: number = 123

// 配列（T[] 形式・リテラルは []）
let fruits: string[] = ["apple", "banana", "cherry"]
let matrix: number[][] = [
  [1, 2, 3],
  [4, 5, 6],
]

// 配列操作
output(fruits[0])      // 要素アクセス
fruits[1] = "melon"    // 要素更新
let len: number = fruits.length  // 長さ取得
foreach fruit in fruits {
  output(fruit)
}

// 条件分岐
if price < 300 {
  output("hoge")
} elseif price < 600 {
  output("fuga")
} else {
  output("hohe")
}

// while ループ
while true {
  // …
}

// for ループ（仮変数 counter：0 から自動インクリメント）
for counter < 10 {
  output(counter)
}

// 関数宣言（TypeScript 風の戻り値型注釈）
function greet(name: string): string {
  return "こんにちは、" + name
}

// クラス宣言（デフォルト private・呼び出す要素に public）
class Player {
  // フィールドは常に private
  name: string = ""
  age: number  = 20

  // 呼び出したいメソッドだけ public
  public function run(): void {
    draw()
    update()
  }

  function draw(): void  { /* private */ }
  function update(): void { /* private */ }
}

// エントリポイントは必ず main()
function main(): void {
  let pl: Player = new Player
  pl.run()
}
```
