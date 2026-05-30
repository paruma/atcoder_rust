# fetch_problems

AtCoder Problems API から全問題の情報を取得し、推定難易度（Difficulty）を含む TSV ファイルを生成するツールです。

## 使い方

`uv` を使用して、依存関係のインストールなしに直接実行できます。

```bash
# デフォルト出力 (atcoder_problems.tsv)
uv run tools/fetch_problems/fetch_problems.py

# 出力先を指定する場合
uv run tools/fetch_problems/fetch_problems.py --output all_problems.tsv

# コンテストIDの接頭辞でフィルタリングする場合 (例: ABC400番台のみ)
uv run tools/fetch_problems/fetch_problems.py --prefix abc4
```

## オプション

- `--output`, `-o`: 出力先 TSV ファイルのパス (デフォルト: `atcoder_problems.tsv`)
- `--prefix`, `-p`: コンテスト ID の接頭辞によるフィルタリング (大文字小文字を区別しません)

## 出力項目 (TSV)

- **Contest**: コンテスト名 (例: ABC459)
- **Index**: 問題番号 (例: C)
- **Title**: 問題名 (例: Drop Blocks)
- **Difficulty**: 推定難易度 (convert_difficulty 適用後の数値。データがない場合は `-`)
- **URL**: 問題の URL

## 依存関係

- Python >= 3.12
- requests (uv によって自動的に管理されます)
