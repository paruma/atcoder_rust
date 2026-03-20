# Notion 編集ガイド（AI向け）

## 重要なポイント

### `page_id` は UUID 形式で渡す

`notion-fetch` は URL を受け付けるが、`notion-update-page` の `page_id` は UUID 形式でなければならない。UUID は URL 末尾の英数字部分から直接取得できる（fetch 不要）。

- ❌ `https://www.notion.so/ABC443-2f997aaeaceb8070a960dcf274afa1c7`
- ✅ `2f997aaeaceb8070a960dcf274afa1c7`（URL末尾の英数字部分）

### `content_updates` のパラメータ名

`notion-update-page` の `content_updates` パラメータの正しい形式:

```json
{
  "old_str": "置換前のテキスト",
  "new_str": "置換後のテキスト"
}
```

⚠️ **注意:** `old_string` / `new_string` は間違い。必ず `old_str` / `new_str` を使うこと。

## 基本的な流れ

1. `notion-fetch` でページ内容を確認する
2. `notion-update-page` で編集する
3. エラーが出たら、MCPツールのエラーメッセージに従う

## コマンド例

**ページ内容を部分置換:**
```
notion-update-page:
  page_id: "..."
  command: "update_content"
  content_updates:
    - old_str: "..."
      new_str: "..."
```

**全体を置き換え:**
```
notion-update-page:
  page_id: "..."
  command: "replace_content"
  new_str: "..."
```

**プロパティを更新:**
```
notion-update-page:
  page_id: "..."
  command: "update_properties"
  properties:
    "title": "..."
```

その他の詳細はエラーメッセージから確認してください。
