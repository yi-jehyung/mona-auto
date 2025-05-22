## core

## ----- Action (action.rs) -----
action-display-leftclick = 좌클릭: ({$x}, {$y})
action-display-leftclick-image = 좌클릭: 이미지 위치
action-display-rightclick = 우클릭: ({$x}, {$y})
action-display-rightclick-image = 우클릭: 이미지 위치
action-display-drag = 드래그: ({$start_x}, {$start_y}) → ({$end_x}, {$end_y})
action-display-scroll-up = 위로 스크롤: ({$x}, {$y})
action-display-scroll-down = 아래로 스크롤: ({$x}, {$y})
action-image-position = 이미지 위치
action-display-scroll-up-image = 위로 스크롤: {action-image-position}
action-display-scroll-down-image = 아래로 스크롤: {action-image-position}
action-display-keyinput = 키 입력
action-display-textinput = 텍스트 입력
action-display-delay = 대기 {$millis}ms
action-display-send-discord = 디스코드 전송
action-display-enable = {$name} 활성화
action-display-disable = {$name} 비활성화

## gui

## ----- action_panel.rs -----
action-panel-header = 일치 행동
action-panel-button-add-action = 행동 추가
# Left Click
action-panel-left-click = 좌클릭
# Right Click
action-panel-right-click = 우클릭
# Drag
action-panel-drag = 드래그
# Scroll
action-panel-scroll = 스크롤
action-panel-scroll-direction-option-up = 위로
action-panel-scroll-direction-option-down = 아래로
# Key Input
action-panel-key-input = 키 입력
action-panel-key-add = 키 추가
action-panel-key-type-down = 누르기
action-panel-key-type-up = 때기
action-panel-key-type-down-and-up = 누르고 때기
action-panel-key-type-delay = 딜레이
action-panel-key-custom-vk = 커스텀 VK:
# Text Input
action-panel-text-input = 텍스트 입력
# Delay
action-panel-delay = 대기
# Send Discord
action-panel-send-discord = 디스코드 전송
action-panel-label-webhook-url = 웹훅 URL
action-panel-label-message = 메시지
action-panel-send-screenshot = 스크린샷 전송
# Toggle Image Enable
action-panel-toggle-image-enable = 이미지 활성/비활성화
action-panel-label-target = 대상
action-panel-enable-enabled = 활성화
action-panel-enable-disabled = 비활성화

action-panel-checkbox-use-matched-position = 일치한 위치 사용
action-panel-duration-ms = 지속 시간(ms):
action-panel-button-open-coordinate-picker = 좌표 선택기
action-panel-label-millis-with-seconds = 밀리세컨드 ({ $seconds }초)
# Context
action-panel-context-edit = 편집
action-panel-context-delete = 삭제
action-panel-context-move-up = 위로 이동
action-panel-context-move-down = 아래로 이동

## ----- add_action_modal.rs -----
# Heading
add-action-modal-heading-edit = 행동 편집
add-action-modal-heading-add = 행동 추가
# Button
add-action-modal-button-confirm = 확인
add-action-modal-button-cancel = 취소

## ----- control_panel.rs -----
# Label
control-panel-label-target-window = 대상 윈도우
# Button
control-panel-button-find = 대상 윈도우 찾기
control-panel-button-start = 시작
control-panel-button-stop = 중지(F8)

## ----- coordinate_picker_viewport.rs -----
# Heading
coordinate-picker-heading = 좌표 선택
# Button
coordinate-picker-button-close = 닫기

## ----- error_modal.rs -----
# Heading
error-modal-heading = 에러 발생
# Button
error-modal-button-ok = 확인

## -----image_edit_viewport.rs -----
# Heading
image-edit-viewport-heading = 이미지 편집
# Button
image-edit-viewport-button-retake = 다시 캡처
image-edit-viewport-button-crop = 자르기 범위 지정
image-edit-viewport-button-roi = ROI 범위 지정
image-edit-viewport-button-ok = 확인
image-edit-viewport-button-cancel = 취소
# Image Range
image-edit-viewport-label-image-range = 이미지 범위
# ROI Range
image-edit-viewport-label-roi-range = ROI 범위
image-edit-viewport-checkbox-use-crop = 이미지 범위 사용
image-edit-viewport-warning-roi-size = ⚠ ROI 범위가 이미지 범위보다 작음

## -----image_list_panel.rs -----
# Label
image-list-panel-label = 이미지 목록
# Button
image-list-panel-button-add-folder = 🗁 추가
image-list-panel-button-add-image = 🗋 추가
# Context
image-list-panel-context-rename = 이름 변경
image-list-panel-context-delete = 삭제
image-list-panel-context-move-up = 위로 이동
image-list-panel-context-move-down = 아래로 이동

## ----- image_preview_panel.rs -----
# Label
image-preview-panel-no-selection = ⚠ 선택된 이미지 없음
image-preview-panel-no-path = ⚠ 프로젝트 경로 없음
image-preview-panel-load-failed = ⚠ 이미지 없음 또는 로드 실패
image-preview-panel-always-run = 항상 실행
# Button
image-preview-panel-button-retake = 다시 캡처
# Modal
image-preview-panel-modal-heading = 이미지 편집
image-preview-panel-note = 새로 열린 창에서 완료 해주세요

## ----- menu_bar.rs -----
# File
menu-file = 파일
menu-file-new = 새로 만들기
menu-file-open = 열기
menu-file-save = 저장
# Menu
menu-menu = 메뉴
menu-lang = 🌏 언어
menu-check-updates = 업데이트 확인
menu-help = 사용법
menu-quit = 나가기

## ----- project_panel.rs -----
project-panel-name = 프로젝트 이름:
project-panel-description = 프로젝트 설명:

## ----- setting_panel.rs -----
# Input Type
setting-panel-label-input-type = 입력 방식
# Capture Type
setting-panel-label-capture-type = 캡처 방식
# Loop Frequency
setting-panel-label-loop-per-second = 반복 빈도
setting-panel-loop-per-second-very-low = 매우 낮음
setting-panel-loop-per-second-low = 낮음
setting-panel-loop-per-second-medium = 보통
setting-panel-loop-per-second-high = 높음
setting-panel-loop-per-second-very-high = 매우 높음
# Sensitivity
setting-panel-label-threshold = 민감도
setting-panel-threshold-low = 낮음
setting-panel-threshold-medium = 보통
setting-panel-threshold-sensitive = 민감
setting-panel-threshold-very-sensitive = 매우 민감

## ----- window_resize_modal.rs -----
# Heading
window-resize-modal-heading = 창 크기가 이전과 일치하지 않습니다
# Option
window-resize-modal-option-restore = 이전 크기로 복원
window-resize-modal-option-update = ⚠ 현재 크기로 업데이트
# Button
window-resize-modal-button-confirm = 확인
window-resize-modal-button-cancel = 취소

## message

## message action.rs
message-action-error-cant-find-matched-position = 일치하는 위치를 찾을 수 없어 원래 위치를 사용합니다: { $x } { $y }
message-action-left-click = 좌클릭: {$x}, {$y}
message-action-right-click = 우클릭: {$x}, {$y}
message-action-drag = 드래그: ({$x1}, {$y1}) -> ({$x2}, {$y2})
message-action-scroll = 스크롤: {$x}, {$y}
message-action-key-input = 키 입력
message-action-text-input = 텍스트 입력: "{$text}"
message-action-delay = 대기: {$millis}ms
message-action-send-discord = 디스코드 전송: "{$message}"
message-action-send-discord-with-screenshot = 디스코드 전송: "{$message}" - 스크린샷 포함
message-action-toogle-enable = 이미지 활성화 상태 설정: "{$name}" 활성화
message-action-toogle-disable = 이미지 활성화 상태 설정: "{$name}" 비활성화

## message automation_loop.rs
message-automation-loop-error-cant-find-window = 자동화 대상 윈도우를 찾을 수 없습니다. 자동화 루프를 시작할 수 없습니다.
message-automation-loop-error-fail-capture = 캡처 실패: {$error}
message-automation-loop-error-fail-load-template = 템플릿 이미지 로드 실패: {$error}
message-automation-loop-template-found = {$name} 발견: ({$x}, {$y}), 일치도: {$similarity}
message-automation-loop-found = {$name}
message-automation-loop-error-match-failed = 일치도 {$similarity} - 매칭 실패

## message capture.rs
# error-capture-failed-create-image = Failed to create image

## message project.rs
message-project-invalid-index = 존재하지 않는 인덱스: {$index}
message-project-failed-create-dir = 디렉토리 생성 실패: {$error}
message-project-failed-create-json = JSON 파일 생성 실패: {$error}
message-project-failed-saved-json = JSON 파일 저장 실패: {$error}
message-project-successfully-saved-json = JSON 파일 저장 완료: {$path}
message-project-failed-create-file = 파일 생성 실패: {$path}
error-project-failed-open-file = 파일 열기 실패: {$error}
error-project-failed-parse-json = JSON 파일 읽기 실패: {$error}
error-project-file-selection-canceled = 파일 선택 취소
error-project-file-create-project = 새 프로젝트 만들기 실패
error-project-no-first-window = 첫번째 창이 없습니다
error-project-last-first-window = 마지막 창이 없습니다

## message setting.rs
message-setting-failed-create-ron = RON 파일 생성 실패: {$path}
message-setting-successfully-saved-ron = RON 파일 저장 성공: {$path}
message-setting-failed-saved-ron = RON 파일 저장 실패: {$path}
message-setting-failed-create-ron = RON 파일 생성 실패: {$path}
error-setting-failed-open-file = 파일 열기 실패: {$error}
error-setting-failed-parse-ron = RON 파일 읽기 실패: {$error}

## message window.rs
error-window-saved-windows-empty = 저장된 창 목록이 비어 있습니다.
error-window-failed-find-top-hwnd = 최상위 창 '{$title}' (class: '{class}')의 HWND를 찾을 수 없습니다.
error-window-cant-find-parent-hwnd = '{$title}'창의 부모 HWND를 찾을 수 없어 탐색을 중단합니다.
error-window-cant-find-low-hwnd = '{$title1}' (class: '{$class}') 창을 부모 '{$title2}'아래에서 찾을 수 없습니다.
message-window-save-information = 창을 좌클릭해 창 정보 저장...