<script lang="ts">
  import Icon from "@iconify/svelte";
</script>

<div class="ml-4 mt-4">
  <h4 class="h4 mb-2">使い方</h4>
  <ol class="list-inside list-decimal space-y-1">
    <li>ユーザーの日付ごとのデータを用意する</li>
    <ul class="ml-10 list-disc space-y-1">
      <li>
        <div class="flex">入力サンプル</div>
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
