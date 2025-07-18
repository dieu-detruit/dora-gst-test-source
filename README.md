# dora-gst-test-source

videotestsrcを使用してダミー映像を生成するdora-rsノードです。GStreamerのvideotestsrc要素を使い、様々なテストパターンの映像を生成できます。

## 機能

- GStreamerのvideotestsrcを使用した動的なテスト映像の生成
- 様々なパターン（SMPTE、ノイズ、単色など）の生成
- サイズ、フレームレート、色などの詳細な設定
- kornia-ioを使用した効率的な映像処理
- dora-rsとの統合によるリアルタイム映像配信

## 環境変数とデフォルト値

すべての環境変数にはデフォルト値が設定されているため、設定せずに実行可能です。

### 映像設定

| 環境変数 | デフォルト値 | 説明 |
|---------|-------------|------|
| `IMAGE_COLS` | `640` | 映像の幅（ピクセル） |
| `IMAGE_ROWS` | `480` | 映像の高さ（ピクセル） |
| `SOURCE_FPS` | `30` | フレームレート |
| `SOURCE_FORMAT` | `RGB` | 出力フォーマット（RGB, BGR, GRAY8など） |

### videotestsrcプロパティ

| 環境変数 | デフォルト値 | 説明 |
|---------|-------------|------|
| `PATTERN` | `0` | テストパターン（0=smpte, 1=snow, 2=black, 3=white, 4=red, 5=green, 6=blue） |
| `ANIMATION_MODE` | `0` | アニメーションモード（0=frames, 1=wall-time, 2=running-time） |
| `MOTION` | `0` | モーション設定 |
| `BACKGROUND_COLOR` | `0xff000000` | 背景色（ARGB形式） |
| `FOREGROUND_COLOR` | `0xffffffff` | 前景色（ARGB形式） |
| `FLIP` | `false` | 映像の反転 |
| `IS_LIVE` | `true` | ライブストリーミングモード |

## 使用例

### デフォルト設定で実行
```bash
cargo run
```

### カスタム設定で実行
```bash
# HD解像度、60FPS、雪ノイズパターン
IMAGE_COLS=1920 IMAGE_ROWS=1080 SOURCE_FPS=60 PATTERN=1 cargo run

# 単色（赤）パターン
PATTERN=4 cargo run

# BGRフォーマットで出力
SOURCE_FORMAT=BGR cargo run
```

## ビルド

```bash
cargo build --release
```

## 依存関係

- `dora-node-api`: dora-rsとの統合
- `kornia-io`: GStreamer統合と映像処理
- GStreamer: システムにインストールされている必要があります

### GStreamerのインストール

#### Ubuntu/Debian
```bash
sudo apt install gstreamer1.0-tools gstreamer1.0-plugins-base gstreamer1.0-plugins-good gstreamer1.0-plugins-bad
```

#### macOS
```bash
brew install gstreamer gst-plugins-base gst-plugins-good gst-plugins-bad
```

## テストパターンの種類

- `0`: SMPTE色バーテスト
- `1`: ランダムノイズ（雪）
- `2`: 黒画面
- `3`: 白画面  
- `4`: 赤画面
- `5`: 緑画面
- `6`: 青画面
- `7`: チェッカーボード
- `8`: 水平グラデーション
- その他多数のパターンが利用可能

## 出力

ノードは`frame`という名前で映像フレームを出力します。各フレームには以下のメタデータが付与されます：

- `encoding`: フォーマット（RGB、BGRなど）
- `width`: 映像の幅
- `height`: 映像の高さ

## トラブルシューティング

### GStreamerエラー
- GStreamerが正しくインストールされているか確認
- 必要なプラグインがインストールされているか確認

### フォーマットエラー
- `SOURCE_FORMAT`が正しい値（RGB、BGR、GRAY8など）に設定されているか確認

### パフォーマンス問題
- フレームレートやサイズを下げて試行
- システムのリソース使用量を確認 
