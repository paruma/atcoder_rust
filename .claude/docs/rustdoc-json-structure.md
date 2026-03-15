# rustdoc JSON の構造

`cargo +nightly rustdoc -p <crate> -- -Z unstable-options --output-format json` で生成される JSON の構造メモ。

出力先: `target/doc/<crate_name>.json`

---

## トップレベル構造

```json
{
  "root": "<root_module_id>",
  "crate_version": null,
  "includes_private": false,
  "index": { "<id>": <Item>, ... },
  "paths": { "<id>": <ItemSummary>, ... },
  "external_crates": { ... },
  "target": { ... },
  "format_version": 37
}
```

- `index`: クレート内の全アイテム（関数・struct・enum・impl 等）を ID でインデックスしたマップ
- `paths`: アイテムの簡易情報（外部クレートの型の解決に使う）
- `root`: ルートモジュールの ID

---

## Item の構造

```json
{
  "id": 12345,
  "crate_id": 0,
  "name": "MyStruct",
  "span": {
    "filename": "src/mylib/foo/bar.rs",
    "begin": [10, 1],
    "end":   [30, 2]
  },
  "visibility": "public",
  "docs": "ドキュメントコメント文字列",
  "links": {},
  "attrs": [],
  "deprecation": null,
  "inner": { "<kind>": { ... } }
}
```

### 重要フィールド

| フィールド | 型 | 説明 |
|---|---|---|
| `id` | int | アイテムの一意 ID |
| `crate_id` | int | `0` = 自クレート、それ以外 = 外部クレート |
| `name` | str \| null | アイテム名（impl 等は null） |
| `span.filename` | str | ソースファイルのパス（`src/mylib/...` 形式） |
| `inner` | object | アイテムの種別と詳細（下記参照） |

---

## `inner` の種別

`inner` は **単一キー** を持つオブジェクトで、そのキーがアイテムの種別を表す。

```json
// struct の場合
"inner": { "struct": { "kind": ..., "generics": ..., "impls": [101, 102, ...] } }

// enum の場合
"inner": { "enum": { "variants": [...], "generics": ..., "impls": [201, 202, ...] } }

// impl の場合
"inner": { "impl": { "is_unsafe": false, "trait": ..., "for": ..., ... } }

// function の場合
"inner": { "function": { ... } }

// module の場合
"inner": { "module": { "items": [...] } }
```

主要な kind 一覧（mylib での出現数順）:
`impl`, `function`, `module`, `struct`, `assoc_type`, `struct_field`,
`type_alias`, `trait`, `enum`, `assoc_const`, `variant`, `constant`, `macro`, `use`

---

## struct / enum の `impls`

```json
"inner": {
  "struct": {
    "kind": { "plain": { "fields": [...], "has_stripped_fields": false } },
    "generics": { "params": [...], "where_predicates": [] },
    "impls": [101, 102, 103]   // impl アイテムの ID リスト（int）
  }
}
```

- `impls` は **int のリスト**（文字列ではない）
- index のキーは文字列なので `str(impl_id)` で lookup する必要がある

---

## impl アイテムの構造

```json
{
  "id": 101,
  "crate_id": 0,
  "name": null,
  "inner": {
    "impl": {
      "is_unsafe": false,
      "generics": { "params": [...], "where_predicates": [] },
      "provided_trait_methods": [],
      "trait": {
        "path": "Default",
        "id": 42,
        "args": { "angle_bracketed": { "args": [], "constraints": [] } }
      },
      "for": {
        "resolved_path": { "path": "MyStruct", "id": 12345, ... }
      },
      "items": [...]
    }
  }
}
```

- `trait` が `null` の場合は inherent impl（固有実装）
- `trait.path` はトレイト名（短縮名または完全修飾名）
  - 例: `"Default"`, `"Hash"`, `"std::fmt::Debug"`
  - **末尾のセグメントだけ取り出す**のが安全: `path.split("::")[-1]`

---

## トレイト実装の調べ方（Python）

```python
def collect_trait_names_for_item(item_id: str, data: dict) -> set[str]:
    index = data["index"]
    item = index.get(item_id)
    if item is None:
        return set()

    inner = item.get("inner", {})
    # struct と enum で inner のキーが異なる
    kind_inner = inner.get("struct") or inner.get("enum") or {}
    impl_ids: list[int] = kind_inner.get("impls", [])

    trait_names: set[str] = set()
    for impl_id in impl_ids:
        impl_item = index.get(str(impl_id))   # int -> str 変換が必要
        if impl_item is None:
            continue
        impl_inner = impl_item.get("inner", {}).get("impl", {})
        trait_info = impl_inner.get("trait")
        if trait_info is not None:
            path = trait_info.get("path")
            if path:
                trait_names.add(path.split("::")[-1])

    return trait_names
```

---

## struct / enum の一覧取得（自クレートのみ）

```python
# crate_id == 0 かつ inner のキーが "struct" または "enum" のもの
structs_and_enums = [
    (item_id, item)
    for item_id, item in data["index"].items()
    if item.get("crate_id") == 0
    and any(k in item.get("inner", {}) for k in ("struct", "enum"))
    and item.get("name")
]
```

---

## 注意点

- `index` のキーは **文字列**、`impls` リストの要素は **int** → 必ず `str()` 変換してから lookup
- `name` が `null` のアイテムが多数ある（impl, struct_field 等）
- `span` が `null` のアイテムもある（外部クレートからの再エクスポート等）
- `format_version` は nightly のバージョンによって変わる可能性がある

---

## 関連ファイル

- 生成コマンド: `cargo +nightly rustdoc -p mylib -- -Z unstable-options --output-format json`
- 出力先: `target/doc/mylib.json`
