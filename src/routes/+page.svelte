<script lang="ts">
  import { save } from "@tauri-apps/api/dialog";
  import { writeTextFile } from "@tauri-apps/api/fs";
  import Icon from "@iconify/svelte";

  const sampleCSVContent = `Day,Student 1,Student 2,Student 3,Student 4,Student 5,Student 6,Student 7,Student 8,Student 9,Student 10,Student 11,Student 12,Student 13,Student 14,Student 15
1,佐藤 翔太,鈴木 花子,高橋 健太,田中 美咲,伊藤 優斗,渡辺 明美,山本 大輔,中村 恵,小林 誠,加藤 結衣,吉田 拓也,山田 千尋,佐々木 涼太,山口 奈々,松本 大地
2,佐藤 翔太,鈴木 花子,高橋 健太,田中 美咲,井上 真央,木村 翔,林 奈々美`;

  async function downloadSampleCSV() {
    const defaultPath = "sample_user_data.csv";

    try {
      const path = await save({ defaultPath });
      if (path) {
        await writeTextFile(path, sampleCSVContent);
        console.log(`Sample file saved to ${path}`);
      } else {
        console.log("File save was canceled");
      }
    } catch (error) {
      console.error("Error writing sample file:", error);
    }
  }
</script>

<div class="ml-4 mt-4">
  <h4 class="h4 mb-2">使い方</h4>
  <ol class="list-inside list-decimal space-y-1">
    <li>ユーザーの日付ごとのデータを用意する</li>
    <ul class="ml-10 list-disc space-y-1">
      <li>
        <div class="flex items-center">
          <p class="mr-2">入力サンプル:</p>
          <button class="btn h-8 border border-gray-500 bg-stone-50 focus:!outline-none" on:click={downloadSampleCSV}>
            ダウンロード
          </button>
        </div>
        <div>※日数は必ず合うように調整して利用してください</div>
      </li>
    </ul>
    <li>
      <div class="flex items-center">
        サイドバーより
        <Icon icon="mdi:user" height="20" />
        ボタンをクリックし、用意したデータを登録する
      </div>
    </li>
    <ul class="ml-10 list-disc space-y-1">
      <li>ST割当の不要なユーザーがいればST対象から外しておく</li>
    </ul>
    <li>
      <div class="flex items-center">
        サイドバーより
        <Icon icon="mdi:calendar" height="20" />
        ボタンをクリックしデータを確認する
      </div>
    </li>
    <ul class="ml-10 list-disc space-y-1">
      <li>ST割当が不要な日付があればチェックを外しておく</li>
    </ul>
    <li>ページ右下の解析ボタンを押し結果を確認する</li>
    <ul class="ml-10 list-disc space-y-1">
      <li>完全ランダムのため再度解析ボタンをクリックすることで結果は変わる</li>
    </ul>
    <li>必要に応じてダウンロードボタンをクリックし結果を活用する</li>
  </ol>
</div>
