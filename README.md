# st-scheduler

<img src=".github/st-scheduler.gif" alt="Sample" />

放課後デイサービスなどの専門的支援加算に該当する専門職(ST, OT等)を自動的に割り当てるスケジューラー。

日付ごとに出席者を羅列したCSVデータを入力とし、以下の条件に合うように自動的にSTを割り当てます。  
※現在(20240712)、国が定めている算定回数として定義。

- 12日以上出席で6日
- 6日以上出席で4日
- 1日以上出席で2日

入力データのサンプルはアプリ内よりダウンロードできます。  
また、以下はアプリ内でカスタマイズ可能です。

- 1日の合計の割当人数
- 固定で割り当てる日付
- 未割り当てとする日

## Getting Started

[リリースページ](https://github.com/ban367/st-scheduler/releases)より該当のOSのアプリをダウンロードできます。

## Developing

### Start

```bash
npm run tauri dev
```

### Build

Apple Silicon および x86 双方用のビルドコマンド

```bash
npm run tauri build -- --target universal-apple-darwin
```
