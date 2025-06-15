## core

## ----- Action (action.rs) -----
action-display-leftclick                 = Left click at ({$x}, {$y})
action-display-leftclick-image           = Left click at image position
action-display-rightclick                = Right click at ({$x}, {$y})
action-display-rightclick-image          = Right click at image position
action-display-drag                      = Drag from ({$start_x}, {$start_y}) to ({$end_x}, {$end_y})
action-display-scroll-up                 = Scroll up at ({$x}, {$y})
action-display-scroll-down               = Scroll down at ({$x}, {$y})
action-image-position                    = image position
action-display-scroll-up-image           = Scroll up at {action-image-position}
action-display-scroll-down-image         = Scroll down at {action-image-position}
action-display-keyinput                  = Key input
action-display-textinput                 = Text input
action-display-delay                     = Wait {$millis}ms
action-display-send-discord              = Send message to Discord
action-display-enable                    = Enable {$name}
action-display-disable                   = Disable {$name}

## gui

## ----- action_panel.rs -----
action-panel-header = Match Action
action-panel-button-add-action = Add Action
# Left Click
action-panel-left-click = Left Click
# Right Click
action-panel-right-click = Right Click
# Drag
action-panel-drag = Drag
# Scroll
action-panel-scroll = Scroll
action-panel-scroll-direction-option-up = Up
action-panel-scroll-direction-option-down = Down
# Key Input
action-panel-key-input = Key Input
action-panel-key-add = Add Key
action-panel-key-type-down = Key Down
action-panel-key-type-up = Key Up
action-panel-key-type-down-and-up = Key Down and Up
action-panel-key-type-delay = Delay
action-panel-key-custom-vk = Custom VK:
# Text Input
action-panel-text-input = Text Input
# Delay
action-panel-delay = Delay
# Send Discord
action-panel-send-discord = Send Discord Message
action-panel-label-webhook-url = Webhook URL
action-panel-label-message = Message
action-panel-send-screenshot = Send screenshot
# Toggle Image Enable
action-panel-toggle-image-enable = Toggle Image Enable
action-panel-label-target = Target
action-panel-enable-enabled = Enabled
action-panel-enable-disabled = Disabled

action-panel-checkbox-use-matched-position = Use Matched Position
action-panel-duration-ms = Duration (ms):
action-panel-button-open-coordinate-picker = Open Coordinate Picker
action-panel-label-millis-with-seconds = Milliseconds ({ $seconds } seconds)
# Context
action-panel-context-edit = Edit
action-panel-context-delete = Delete
action-panel-context-move-up = Move Up
action-panel-context-move-down = Move Down

## ----- add_action_modal.rs -----
# Heading
add-action-modal-heading-edit = Edit Action
add-action-modal-heading-add = Add Action
# Button
add-action-modal-button-confirm = Confirm
add-action-modal-button-cancel = Cancel

## ----- control_panel.rs -----
# Label
control-panel-label-target-window = Target Window
# Button
control-panel-button-find = Find Target Window
control-panel-button-start = Start
control-panel-button-stop = Stop(F8)
control-panel-button-add = Add
control-panel-button-delete = Delete

## ----- coordinate_picker_viewport.rs -----
# Heading
coordinate-picker-heading = Select Coordinate
# Button
coordinate-picker-button-close = Close

## ----- error_modal.rs -----
# Heading
error-modal-heading = Error Occurred
# Button
error-modal-button-ok = OK

## -----image_edit_viewport.rs -----
# Heading
image-edit-viewport-heading = Image Edit
# Button
image-edit-viewport-button-retake = Retake
image-edit-viewport-button-crop = Crop Image
image-edit-viewport-button-roi = Define ROI
image-edit-viewport-button-ok = OK
image-edit-viewport-button-cancel = Cancel
# Image Range
image-edit-viewport-label-image-range = Image Range
# ROI Range
image-edit-viewport-label-roi-range = ROI Range
image-edit-viewport-checkbox-use-crop = Use Crop Selection
image-edit-viewport-warning-roi-size = ⚠ ROI size is smaller than the image

## -----image_list_panel.rs -----
# Label
image-list-panel-label = Image List
# Button
image-list-panel-button-add-folder = 🗁 Add
image-list-panel-button-add-image = 🗋 Add
# Context
image-list-panel-context-rename = Rename
image-list-panel-context-delete = Delete
image-list-panel-context-move-up = Move Up
image-list-panel-context-move-down = Move down

## ----- image_preview_panel.rs -----
# Label
image-preview-panel-no-selection = ⚠ No image selected
image-preview-panel-no-path = ⚠ Project path not set
image-preview-panel-load-failed = ⚠ No image found or failed to load
image-preview-panel-always-run = Always run
# Button
image-preview-panel-button-retake = Re-Capture
# Modal
image-preview-panel-modal-heading = Image Edit
image-preview-panel-note = Please complete in the newly opened window.

## ----- menu_bar.rs -----
# File
menu-file = File
menu-file-new = New
menu-file-open = Open
menu-file-save = Save
# Menu
menu-menu = Menu
menu-lang = 🌏 Language
menu-check-updates = Check for updates
menu-help = Help
menu-quit = Quit

## ----- project_panel.rs -----
project-panel-description = Project Description:

## ----- setting_panel.rs -----
# Input Type
setting-panel-label-input-type = Input Type
# Capture Type
setting-panel-label-capture-type = Capture Type
# Loop Frequency
setting-panel-label-loop-per-second = Loop Frequency
setting-panel-loop-per-second-very-low = Very Low
setting-panel-loop-per-second-low = Low
setting-panel-loop-per-second-medium = Medium
setting-panel-loop-per-second-high = High
setting-panel-loop-per-second-very-high = Very High
# Sensitivity
setting-panel-label-threshold = Sensitivity
setting-panel-threshold-low = Low
setting-panel-threshold-medium = Medium
setting-panel-threshold-sensitive = Sensitive
setting-panel-threshold-very-sensitive = Very Sensitive

## ----- window_resize_modal.rs -----
# Heading
window-resize-modal-heading = Window size mismatch
# Option
window-resize-modal-option-restore = Restore Previous Size
window-resize-modal-option-update = ⚠ Update to Current Size
# Button
window-resize-modal-button-confirm = Confirm
window-resize-modal-button-cancel = Cancel

## message

## message action.rs
message-action-error-cant-find-matched-position = Can't find matched position, falling back to original: { $x } { $y }
message-action-left-click = Left Click: {$x}, {$y}
message-action-right-click = Right Click: {$x}, {$y}
message-action-drag = Drag: ({$x1}, {$y1}) -> ({$x2}, {$y2})
message-action-scroll = Scroll: {$x}, {$y}
message-action-key-input = Key Input
message-action-text-input = Text Input: "{ $text }"
message-action-delay = Wait: {$millis}ms
message-action-send-discord = Send to Discord: "{$message}"
message-action-send-discord-with-screenshot = Send to Discord: "{$message}" - with Screenshot
message-action-toogle-enable = Set Image Status: "{$name}" Enabled
message-action-toogle-disable = Set Image Status: "{$name}" Disabled

## message automation_loop.rs
message-automation-loop-error-cant-find-window = Unable to find the target window for automation. Cannot start the automation loop.
message-automation-loop-error-fail-capture = Capture failed: {$error}
message-automation-loop-error-fail-load-template = Error loading template image: {$error}
message-automation-loop-template-found = {$name} found at: ({$x}, {$y}), similarity: {$similarity}
message-automation-loop-found = {$name}
message-automation-loop-error-match-failed = Similarity {$similarity} - Match failed

## message capture.rs
# error-capture-failed-create-image = Failed to create image

## message project.rs
message-project-invalid-index = Invalid index: {$index}
message-project-failed-create-dir = Failed to create directory: {$error}
message-project-failed-create-json = Failed to create JSON file: {$error}
message-project-failed-saved-json = Failed saved JSON file: {$error}
message-project-successfully-saved-json = Successfully saved JSON file: {$path}
message-project-failed-create-file = Failed to create file: {$path}
error-project-failed-open-file = Failed to open file: {$error}
error-project-failed-parse-json = Failed to parse JSON: {$error}
error-project-file-selection-canceled = File selection canceled
error-project-file-create-project = Failed to create new project
error-project-no-first-window = No first window available
error-project-last-first-window = No last window available

## message setting.rs
message-setting-failed-create-ron = Failed to create RON file: {$path}
message-setting-successfully-saved-ron = Successfully saved RON file: {$path}
message-setting-failed-saved-ron = Failed saved RON file: {$path}
message-setting-failed-create-ron = Failed to create file: {$path}
error-setting-failed-open-file = Failed to open file: {$error}
error-setting-failed-parse-ron = Failed to parse RON: {$error}

## message window.rs
error-window-saved-windows-empty = The list of saved windows is empty.
error-window-failed-find-top-hwnd = Failed to find the HWND of the top-level window '{$title}' (class: '{class}').
error-window-cant-find-parent-hwnd = Could not find the parent HWND of the '{$title}' window, stopping traversal."
error-window-cant-find-low-hwnd = Could not find the '{$title1}' (class: '{$class}') window under parent '{$title2}'.
message-window-save-information = Left-click the window to save its information...