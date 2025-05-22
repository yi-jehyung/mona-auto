## core

## ----- Action (action.rs) -----
action-display-leftclick = 左クリック：({$x}, {$y})
action-display-leftclick-image = 左クリック：画像位置
action-display-rightclick = 右クリック：({$x}, {$y})
action-display-rightclick-image = 右クリック：画像位置
action-display-drag = ドラッグ：({$start_x}, {$start_y}) → ({$end_x}, {$end_y})
action-display-scroll-up = スクロールアップ：({$x}, {$y})
action-display-scroll-down = スクロールダウン：({$x}, {$y})
action-image-position = 画像位置
action-display-scroll-up-image = スクロールアップ：{action-image-position}
action-display-scroll-down-image = スクロールダウン：{action-image-position}
action-display-keyinput = キー入力
action-display-textinput = テキスト入力
action-display-delay = 待機 {$millis}ms
action-display-send-discord = Discordに送信
action-display-enable = {$name} を有効化
action-display-disable = {$name} を無効化

## gui

## ----- action_panel.rs -----
action-panel-header = マッチアクション
action-panel-button-add-action = アクションを追加

action-panel-left-click = 左クリック

action-panel-right-click = 右クリック

action-panel-drag = ドラッグ

action-panel-scroll = スクロール
action-panel-scroll-direction-option-up = 上へ
action-panel-scroll-direction-option-down = 下へ

action-panel-key-input = キー入力
action-panel-key-add = キーを追加
action-panel-key-type-down = キーダウン
action-panel-key-type-up = キーアップ
action-panel-key-type-down-and-up = キーダウン＆アップ
action-panel-key-type-delay = 遅延
action-panel-key-custom-vk = カスタム VK:

action-panel-text-input = テキスト入力

action-panel-delay = 待機

action-panel-send-discord = Discord メッセージを送信
action-panel-label-webhook-url = Webhook URL
action-panel-label-message = メッセージ
action-panel-send-screenshot = スクリーンショットを送信

action-panel-toggle-image-enable = 画像の有効化／無効化
action-panel-label-target = 対象
action-panel-enable-enabled = 有効化
action-panel-enable-disabled = 無効化

action-panel-checkbox-use-matched-position = マッチした位置を使用
action-panel-duration-ms = 継続時間（ms）:
action-panel-button-open-coordinate-picker = 座標ピッカー
action-panel-label-millis-with-seconds = ミリ秒（{ $seconds }秒）

action-panel-context-edit = 編集
action-panel-context-delete = 削除
action-panel-context-move-up = 上へ移動
action-panel-context-move-down = 下へ移動

## ----- add_action_modal.rs -----
add-action-modal-heading-edit = アクションを編集
add-action-modal-heading-add = アクションを追加
add-action-modal-button-confirm = 確認
add-action-modal-button-cancel = キャンセル

## ----- control_panel.rs -----
control-panel-label-target-window = 対象ウィンドウ
control-panel-button-find = 対象ウィンドウを検索
control-panel-button-start = 開始
control-panel-button-stop = 停止(F8)

## ----- coordinate_picker_viewport.rs -----
coordinate-picker-heading = 座標を選択
coordinate-picker-button-close = 閉じる

## ----- error_modal.rs -----
error-modal-heading = エラーが発生しました
error-modal-button-ok = 確認

## -----image_edit_viewport.rs -----
image-edit-viewport-heading = 画像編集
image-edit-viewport-button-retake = 再キャプチャ
image-edit-viewport-button-crop = 切り抜き
image-edit-viewport-button-roi = ROIを定義
image-edit-viewport-button-ok = OK
image-edit-viewport-button-cancel = キャンセル
image-edit-viewport-label-image-range = 画像範囲
image-edit-viewport-label-roi-range = ROI範囲
image-edit-viewport-checkbox-use-crop = 切り抜き選択を使用
image-edit-viewport-warning-roi-size = ⚠ ROIサイズが画像より小さい

## -----image_list_panel.rs -----
image-list-panel-label = 画像リスト
image-list-panel-button-add-folder = 🗁 追加
image-list-panel-button-add-image = 🗋 追加

image-list-panel-context-rename = 名前を変更
image-list-panel-context-delete = 削除
image-list-panel-context-move-up = 上へ移動
image-list-panel-context-move-down = 下へ移動

## ----- image_preview_panel.rs -----
image-preview-panel-no-selection = ⚠ 選択された画像がありません
image-preview-panel-no-path = ⚠ プロジェクトパスが設定されていません
image-preview-panel-load-failed = ⚠ 画像が見つからないか、読み込みに失敗しました
image-preview-panel-always-run = 常に実行
image-preview-panel-button-retake = 再キャプチャ
image-preview-panel-modal-heading = 画像編集
image-preview-panel-note = 新しく開いたウィンドウで完了してください

## ----- menu_bar.rs -----
menu-file = ファイル
menu-file-new = 新規作成
menu-file-open = 開く
menu-file-save = 保存

menu-menu = メニュー
menu-lang = 🌏 言語
menu-check-updates = 更新を確認
menu-help = ヘルプ
menu-quit = 終了

## ----- project_panel.rs -----
project-panel-name = プロジェクト名:
project-panel-description = プロジェクトの説明:

## ----- setting_panel.rs -----
setting-panel-label-input-type = 入力タイプ
setting-panel-label-capture-type = キャプチャタイプ

setting-panel-label-loop-per-second = ループ頻度
setting-panel-loop-per-second-very-low = 非常に低い
setting-panel-loop-per-second-low = 低い
setting-panel-loop-per-second-medium = 普通
setting-panel-loop-per-second-high = 高い
setting-panel-loop-per-second-very-high = 非常に高い

setting-panel-label-threshold = 感度
setting-panel-threshold-low = 低い
setting-panel-threshold-medium = 普通
setting-panel-threshold-sensitive = 敏感
setting-panel-threshold-very-sensitive = 非常に敏感

## ----- window_resize_modal.rs -----
window-resize-modal-heading = ウィンドウサイズが以前と一致しません
window-resize-modal-option-restore = 前のサイズに復元
window-resize-modal-option-update = ⚠ 現在のサイズに更新
window-resize-modal-button-confirm = 確認
window-resize-modal-button-cancel = キャンセル

## message

## message action.rs
message-action-error-cant-find-matched-position = 一致する位置が見つかりません。元の位置にフォールバックします: { $x } { $y }
message-action-left-click = 左クリック: {$x}, {$y}
message-action-right-click = 右クリック: {$x}, {$y}
message-action-drag = ドラッグ: ({$x1}, {$y1}) → ({$x2}, {$y2})
message-action-scroll = スクロール: {$x}, {$y}
message-action-key-input = キー入力
message-action-text-input = テキスト入力: "{$text}"
message-action-delay = 待機: {$millis}ミリ秒
message-action-send-discord = Discordに送信: "{$message}"
message-action-send-discord-with-screenshot = Discordに送信: "{$message}" - スクリーンショット付き
message-action-toogle-enable = 画像状態設定: "{$name}" を有効化
message-action-toogle-disable = 画像状態設定: "{$name}" を無効化

## message automation_loop.rs
message-automation-loop-error-cant-find-window = 自動化対象のウィンドウが見つかりません。自動化ループを開始できません。
message-automation-loop-error-fail-capture = キャプチャに失敗しました: {$error}
message-automation-loop-error-fail-load-template = テンプレート画像の読み込みに失敗しました: {$error}
message-automation-loop-template-found = {$name} を発見: ({$x}, {$y}), 類似度: {$similarity}
message-automation-loop-found = {$name} を検出しました
message-automation-loop-error-match-failed = 類似度 {$similarity} - マッチ失敗

## message capture.rs
# error-capture-failed-create-image = 画像の作成に失敗しました

## message project.rs
message-project-invalid-index = 無効なインデックス: {$index}
message-project-failed-create-dir = ディレクトリの作成に失敗しました: {$error}
message-project-failed-create-json = JSONファイルの作成に失敗しました: {$error}
message-project-failed-saved-json = JSONファイルの保存に失敗しました: {$error}
message-project-successfully-saved-json = JSONファイルを正常に保存しました: {$path}
message-project-failed-create-file = ファイルの作成に失敗しました: {$path}
error-project-failed-open-file = ファイルの読み込みに失敗しました: {$error}
error-project-failed-parse-json = JSONの解析に失敗しました: {$error}
error-project-file-selection-canceled = ファイル選択がキャンセルされました
error-project-file-create-project = 新しいプロジェクトの作成に失敗しました
error-project-no-first-window = 最初のウィンドウがありません
error-project-last-first-window = 最後のウィンドウがありません

## message setting.rs
message-setting-failed-create-ron = RONファイルの作成に失敗しました: {$path}
message-setting-successfully-saved-ron = RONファイルを正常に保存しました: {$path}
message-setting-failed-saved-ron = RONファイルの保存に失敗しました: {$path}
message-setting-failed-create-ron = ファイルの作成に失敗しました: {$path}
error-setting-failed-open-file = ファイルの読み込みに失敗しました: {$error}
error-setting-failed-parse-ron = RONの解析に失敗しました: {$error}

## message window.rs
error-window-saved-windows-empty = 保存されたウィンドウのリストが空です。
error-window-failed-find-top-hwnd = トップレベルウィンドウ '{$title}' (クラス: '{class}') のHWNDが見つかりませんでした。
error-window-cant-find-parent-hwnd = '{$title}' ウィンドウの親HWNDが見つかりません。探索を中止します。
error-window-cant-find-low-hwnd = 親ウィンドウ '{$title2}' の下に '{$title1}' (クラス: '{$class}') ウィンドウが見つかりません。
message-window-save-information = 情報を保存するには、対象ウィンドウを左クリックしてください...