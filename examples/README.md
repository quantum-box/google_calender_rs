# Google Calendar SDK使用例

このディレクトリには、Google Calendar SDKの使用例が含まれています。

## タイムゾーンサポートの使用例

`timezone_examples.rs` には、異なるタイムゾーン形式でのイベント作成方法が示されています：

1. UTCでのイベント作成
   - デフォルトのタイムゾーン（UTC）を使用
   - タイムゾーンパラメータを省略可能

2. Region/City形式でのイベント作成
   - `Asia/Tokyo` などの地域/都市形式でタイムゾーンを指定
   - 例：`Some("Asia/Tokyo".to_string())`

3. GMTオフセット形式でのイベント作成
   - `GMT+09:00` 形式でタイムゾーンを指定
   - 例：`Some("GMT+09:00".to_string())`

## 実行方法

```bash
# 環境変数の設定
export GOOGLE_SA_SHEET_CRED=/path/to/your/credentials.json

# 例の実行
cargo run --example timezone_examples
```

## 注意事項

- 実行前に、Google Calendar APIの認証情報を環境変数 `GOOGLE_SA_SHEET_CRED` に設定してください
- カレンダーIDを適切な値に変更してください
- タイムゾーン文字列は厳密に検証されます：
  - Region/City形式：`Asia/Tokyo`, `America/New_York` など
  - GMTオフセット形式：`GMT+09:00`, `GMT-05:00` など
  - UTC：タイムゾーン未指定または明示的に `"UTC"` を指定
