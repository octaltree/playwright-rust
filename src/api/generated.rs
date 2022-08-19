mod accessibility {
    impl Accessibility {
        fn snapshot_builder() -> SnapshotBuilder { todo!() }
    }
}
mod android {
    impl Android {
        fn devices_builder() -> DevicesBuilder { todo!() }
        fn set_default_timeout(timeout: ()) -> () { todo!() }
    }
}
mod android_device {
    impl AndroidDevice {
        async fn close() -> () { todo!() }
        fn drag_builder(selector: (), dest: ()) -> DragBuilder { todo!() }
        fn fill_builder(selector: (), text: ()) -> FillBuilder { todo!() }
        fn fling_builder(selector: (), direction: ()) -> FlingBuilder { todo!() }
        async fn info(selector: ()) -> () { todo!() }
        fn input() -> () { todo!() }
        fn install_apk_builder(file: ()) -> InstallApkBuilder { todo!() }
        fn launch_browser_builder() -> LaunchBrowserBuilder { todo!() }
        fn long_tap_builder(selector: ()) -> LongTapBuilder { todo!() }
        fn model() -> () { todo!() }
        async fn open(command: ()) -> () { todo!() }
        fn pinch_close_builder(selector: (), percent: ()) -> PinchCloseBuilder { todo!() }
        fn pinch_open_builder(selector: (), percent: ()) -> PinchOpenBuilder { todo!() }
        fn press_builder(selector: (), key: ()) -> PressBuilder { todo!() }
        fn push_builder(file: (), path: ()) -> PushBuilder { todo!() }
        fn screenshot_builder() -> ScreenshotBuilder { todo!() }
        fn scroll_builder(selector: (), direction: (), percent: ()) -> ScrollBuilder { todo!() }
        fn serial() -> () { todo!() }
        fn set_default_timeout(timeout: ()) -> () { todo!() }
        async fn shell(command: ()) -> () { todo!() }
        fn swipe_builder(selector: (), direction: (), percent: ()) -> SwipeBuilder { todo!() }
        fn tap_builder(selector: ()) -> TapBuilder { todo!() }
        fn wait_builder(selector: ()) -> WaitBuilder { todo!() }
        async fn wait_for_event(event: ()) -> () { todo!() }
        fn web_view_builder(selector: ()) -> WebViewBuilder { todo!() }
        fn web_views() -> () { todo!() }
    }
}
mod android_input {
    impl AndroidInput {
        async fn drag(from: (), to: (), steps: ()) -> () { todo!() }
        async fn press(key: ()) -> () { todo!() }
        async fn swipe(from: (), segments: (), steps: ()) -> () { todo!() }
        async fn tap(point: ()) -> () { todo!() }
        async fn r#type(text: ()) -> () { todo!() }
    }
}
mod android_socket {
    impl AndroidSocket {
        async fn close() -> () { todo!() }
        async fn write(data: ()) -> () { todo!() }
    }
}
mod android_web_view {
    impl AndroidWebView {
        async fn page() -> () { todo!() }
        fn pid() -> () { todo!() }
        fn pkg() -> () { todo!() }
    }
}
mod api_request {
    impl ApiRequest {
        fn new_context_builder() -> NewContextBuilder { todo!() }
    }
}
mod api_request_context {
    impl ApiRequestContext {
        fn create_form_data() -> () { todo!() }
        fn delete_builder(url: ()) -> DeleteBuilder { todo!() }
        async fn dispose() -> () { todo!() }
        fn fetch_builder(url_or_request: ()) -> FetchBuilder { todo!() }
        fn get_builder(url: ()) -> GetBuilder { todo!() }
        fn head_builder(url: ()) -> HeadBuilder { todo!() }
        fn patch_builder(url: ()) -> PatchBuilder { todo!() }
        fn post_builder(url: ()) -> PostBuilder { todo!() }
        fn put_builder(url: ()) -> PutBuilder { todo!() }
        fn storage_state_builder() -> StorageStateBuilder { todo!() }
    }
}
mod api_response {
    impl ApiResponse {
        async fn body() -> () { todo!() }
        async fn dispose() -> () { todo!() }
        fn headers() -> () { todo!() }
        fn headers_array() -> () { todo!() }
        async fn json() -> () { todo!() }
        async fn json() -> () { todo!() }
        fn ok() -> () { todo!() }
        fn status() -> () { todo!() }
        fn status_text() -> () { todo!() }
        async fn text() -> () { todo!() }
        fn url() -> () { todo!() }
    }
}
mod api_response_assertions {
    impl ApiResponseAssertions {
        fn not() -> () { todo!() }
        async fn not_to_be_ok() -> () { todo!() }
        async fn to_be_ok() -> () { todo!() }
    }
}
mod browser {
    /// Extends EventEmitter
    impl Browser {
        fn browser_type() -> () { todo!() }
        async fn close() -> () { todo!() }
        fn contexts() -> () { todo!() }
        fn is_connected() -> () { todo!() }
        async fn new_browser_cdp_session() -> () { todo!() }
        fn new_context_builder() -> NewContextBuilder { todo!() }
        fn new_page_builder() -> NewPageBuilder { todo!() }
        fn start_tracing_builder() -> StartTracingBuilder { todo!() }
        async fn stop_tracing() -> () { todo!() }
        fn version() -> () { todo!() }
    }
}
mod browser_context {
    /// Extends EventEmitter
    impl BrowserContext {
        async fn add_cookies(cookies: ()) -> () { todo!() }
        fn add_init_script_builder(script: (), script: ()) -> AddInitScriptBuilder { todo!() }
        fn background_pages() -> () { todo!() }
        fn browser() -> () { todo!() }
        async fn clear_cookies() -> () { todo!() }
        async fn clear_permissions() -> () { todo!() }
        async fn close() -> () { todo!() }
        async fn cookies() -> () { todo!() }
        fn expose_binding_builder(name: (), callback: ()) -> ExposeBindingBuilder { todo!() }
        async fn expose_function(name: (), callback: ()) -> () { todo!() }
        fn grant_permissions_builder(permissions: ()) -> GrantPermissionsBuilder { todo!() }
        async fn new_cdp_session(page: ()) -> () { todo!() }
        async fn new_page() -> () { todo!() }
        fn pages() -> () { todo!() }
        fn request() -> () { todo!() }
        fn route_builder(url: (), handler: (), handler: ()) -> RouteBuilder { todo!() }
        fn route_from_har_builder(har: ()) -> RouteFromHarBuilder { todo!() }
        fn service_workers() -> () { todo!() }
        fn set_default_navigation_timeout(timeout: ()) -> () { todo!() }
        fn set_default_timeout(timeout: ()) -> () { todo!() }
        async fn set_extra_http_headers(headers: ()) -> () { todo!() }
        async fn set_geolocation(geolocation: ()) -> () { todo!() }
        #[deprecated]
        async fn set_http_credentials(http_credentials: ()) -> () { todo!() }
        async fn set_offline(offline: ()) -> () { todo!() }
        fn storage_state_builder() -> StorageStateBuilder { todo!() }
        fn tracing() -> () { todo!() }
        fn unroute_builder(url: ()) -> UnrouteBuilder { todo!() }
        fn wait_for_event_builder(event: ()) -> WaitForEventBuilder { todo!() }
        fn wait_for_page_builder(action: (), callback: ()) -> WaitForPageBuilder { todo!() }
        fn wait_for_event2_builder(event: ()) -> WaitForEvent2Builder { todo!() }
    }
}
mod browser_server {
    impl BrowserServer {
        async fn close() -> () { todo!() }
        async fn kill() -> () { todo!() }
        fn process() -> () { todo!() }
        fn ws_endpoint() -> () { todo!() }
    }
}
mod browser_type {
    impl BrowserType {
        fn connect_builder(ws_endpoint: ()) -> ConnectBuilder { todo!() }
        fn connect_over_cdp_builder(endpoint_url: ()) -> ConnectOverCdpBuilder { todo!() }
        fn executable_path() -> () { todo!() }
        fn launch_builder() -> LaunchBuilder { todo!() }
        fn launch_persistent_context_builder(user_data_dir: ()) -> LaunchPersistentContextBuilder {
            todo!()
        }
        fn launch_server_builder() -> LaunchServerBuilder { todo!() }
        fn name() -> () { todo!() }
    }
}
mod cdp_session {
    /// Extends EventEmitter
    impl CdpSession {
        async fn detach() -> () { todo!() }
        async fn send(method: ()) -> () { todo!() }
    }
}
mod console_message {
    impl ConsoleMessage {
        fn args() -> () { todo!() }
        fn location() -> () { todo!() }
        fn location() -> () { todo!() }
        fn text() -> () { todo!() }
        fn r#type() -> () { todo!() }
    }
}
mod coverage {
    impl Coverage {
        fn start_css_coverage_builder() -> StartCssCoverageBuilder { todo!() }
        fn start_js_coverage_builder() -> StartJsCoverageBuilder { todo!() }
        async fn stop_css_coverage() -> () { todo!() }
        async fn stop_js_coverage() -> () { todo!() }
    }
}
mod dialog {
    impl Dialog {
        async fn accept() -> () { todo!() }
        fn default_value() -> () { todo!() }
        async fn dismiss() -> () { todo!() }
        fn message() -> () { todo!() }
        fn r#type() -> () { todo!() }
    }
}
mod download {
    impl Download {
        async fn cancel() -> () { todo!() }
        async fn create_read_stream() -> () { todo!() }
        async fn delete() -> () { todo!() }
        async fn failure() -> () { todo!() }
        fn page() -> () { todo!() }
        async fn path() -> () { todo!() }
        async fn save_as(path: ()) -> () { todo!() }
        fn suggested_filename() -> () { todo!() }
        fn url() -> () { todo!() }
    }
}
mod electron {
    impl Electron {
        fn launch_builder() -> LaunchBuilder { todo!() }
    }
}
mod electron_application {
    impl ElectronApplication {
        async fn browser_window(page: ()) -> () { todo!() }
        async fn close() -> () { todo!() }
        fn context() -> () { todo!() }
        async fn evaluate(expression: ()) -> () { todo!() }
        async fn evaluate_handle(expression: ()) -> () { todo!() }
        async fn first_window() -> () { todo!() }
        fn process() -> () { todo!() }
        async fn wait_for_event(event: ()) -> () { todo!() }
        fn windows() -> () { todo!() }
    }
}
mod element_handle {
    /// Extends JSHandle
    impl ElementHandle {
        async fn bounding_box() -> () { todo!() }
        fn check_builder() -> CheckBuilder { todo!() }
        fn click_builder() -> ClickBuilder { todo!() }
        async fn content_frame() -> () { todo!() }
        fn dblclick_builder() -> DblclickBuilder { todo!() }
        async fn dispatch_event(r#type: ()) -> () { todo!() }
        async fn eval_on_selector(selector: (), expression: ()) -> () { todo!() }
        async fn eval_on_selector_all(selector: (), expression: ()) -> () { todo!() }
        fn fill_builder(value: ()) -> FillBuilder { todo!() }
        async fn focus() -> () { todo!() }
        async fn get_attribute(name: ()) -> () { todo!() }
        fn hover_builder() -> HoverBuilder { todo!() }
        async fn inner_html() -> () { todo!() }
        async fn inner_text() -> () { todo!() }
        fn input_value_builder() -> InputValueBuilder { todo!() }
        async fn is_checked() -> () { todo!() }
        async fn is_disabled() -> () { todo!() }
        async fn is_editable() -> () { todo!() }
        async fn is_enabled() -> () { todo!() }
        async fn is_hidden() -> () { todo!() }
        async fn is_visible() -> () { todo!() }
        async fn owner_frame() -> () { todo!() }
        fn press_builder(key: ()) -> PressBuilder { todo!() }
        async fn query_selector(selector: ()) -> () { todo!() }
        async fn query_selector_all(selector: ()) -> () { todo!() }
        fn screenshot_builder() -> ScreenshotBuilder { todo!() }
        fn scroll_into_view_if_needed_builder() -> ScrollIntoViewIfNeededBuilder { todo!() }
        fn select_option_builder(values: ()) -> SelectOptionBuilder { todo!() }
        fn select_text_builder() -> SelectTextBuilder { todo!() }
        fn set_checked_builder(checked: ()) -> SetCheckedBuilder { todo!() }
        fn set_input_files_builder(files: ()) -> SetInputFilesBuilder { todo!() }
        fn tap_builder() -> TapBuilder { todo!() }
        async fn text_content() -> () { todo!() }
        fn type_builder(text: ()) -> TypeBuilder { todo!() }
        fn uncheck_builder() -> UncheckBuilder { todo!() }
        fn wait_for_element_state_builder(state: ()) -> WaitForElementStateBuilder { todo!() }
        fn wait_for_selector_builder(selector: ()) -> WaitForSelectorBuilder { todo!() }
    }
}
mod error {
    /// Extends Exception
    impl Error {
        fn message() -> () { todo!() }
        /// unnecessary
        fn name() -> () { todo!() }
        /// unnecessary
        fn stack() -> () { todo!() }
    }
}
mod file_chooser {
    impl FileChooser {
        fn element() -> () { todo!() }
        fn is_multiple() -> () { todo!() }
        fn page() -> () { todo!() }
        fn set_files_builder(files: ()) -> SetFilesBuilder { todo!() }
    }
}
mod form_data {
    impl FormData {
        fn create() -> () { todo!() }
        fn set(name: (), value: ()) -> () { todo!() }
    }
}
mod frame {
    impl Frame {
        fn add_script_tag_builder() -> AddScriptTagBuilder { todo!() }
        fn add_style_tag_builder() -> AddStyleTagBuilder { todo!() }
        fn check_builder(selector: ()) -> CheckBuilder { todo!() }
        fn child_frames() -> () { todo!() }
        fn click_builder(selector: ()) -> ClickBuilder { todo!() }
        async fn content() -> () { todo!() }
        fn dblclick_builder(selector: ()) -> DblclickBuilder { todo!() }
        fn dispatch_event_builder(selector: (), r#type: ()) -> DispatchEventBuilder { todo!() }
        fn drag_and_drop_builder(source: (), target: ()) -> DragAndDropBuilder { todo!() }
        fn eval_on_selector_builder(selector: (), expression: ()) -> EvalOnSelectorBuilder {
            todo!()
        }
        async fn eval_on_selector_all(selector: (), expression: ()) -> () { todo!() }
        async fn evaluate(expression: ()) -> () { todo!() }
        async fn evaluate_handle(expression: ()) -> () { todo!() }
        fn fill_builder(selector: (), value: ()) -> FillBuilder { todo!() }
        fn focus_builder(selector: ()) -> FocusBuilder { todo!() }
        async fn frame_element() -> () { todo!() }
        fn frame_locator(selector: ()) -> () { todo!() }
        fn get_attribute_builder(selector: (), name: ()) -> GetAttributeBuilder { todo!() }
        fn goto_builder(url: ()) -> GotoBuilder { todo!() }
        fn hover_builder(selector: ()) -> HoverBuilder { todo!() }
        fn inner_html_builder(selector: ()) -> InnerHtmlBuilder { todo!() }
        fn inner_text_builder(selector: ()) -> InnerTextBuilder { todo!() }
        fn input_value_builder(selector: ()) -> InputValueBuilder { todo!() }
        fn is_checked_builder(selector: ()) -> IsCheckedBuilder { todo!() }
        fn is_detached() -> () { todo!() }
        fn is_disabled_builder(selector: ()) -> IsDisabledBuilder { todo!() }
        fn is_editable_builder(selector: ()) -> IsEditableBuilder { todo!() }
        fn is_enabled_builder(selector: ()) -> IsEnabledBuilder { todo!() }
        fn is_hidden_builder(selector: ()) -> IsHiddenBuilder { todo!() }
        fn is_visible_builder(selector: ()) -> IsVisibleBuilder { todo!() }
        fn locator_builder(selector: ()) -> LocatorBuilder { todo!() }
        fn name() -> () { todo!() }
        fn page() -> () { todo!() }
        fn parent_frame() -> () { todo!() }
        fn press_builder(selector: (), key: ()) -> PressBuilder { todo!() }
        fn query_selector_builder(selector: ()) -> QuerySelectorBuilder { todo!() }
        async fn query_selector_all(selector: ()) -> () { todo!() }
        fn select_option_builder(selector: (), values: ()) -> SelectOptionBuilder { todo!() }
        fn set_checked_builder(selector: (), checked: ()) -> SetCheckedBuilder { todo!() }
        fn set_content_builder(html: ()) -> SetContentBuilder { todo!() }
        fn set_input_files_builder(selector: (), files: ()) -> SetInputFilesBuilder { todo!() }
        fn tap_builder(selector: ()) -> TapBuilder { todo!() }
        fn text_content_builder(selector: ()) -> TextContentBuilder { todo!() }
        async fn title() -> () { todo!() }
        fn type_builder(selector: (), text: ()) -> TypeBuilder { todo!() }
        fn uncheck_builder(selector: ()) -> UncheckBuilder { todo!() }
        fn url() -> () { todo!() }
        fn wait_for_function_builder(expression: ()) -> WaitForFunctionBuilder { todo!() }
        fn wait_for_load_state_builder() -> WaitForLoadStateBuilder { todo!() }
        fn wait_for_navigation_builder(action: (), callback: ()) -> WaitForNavigationBuilder {
            todo!()
        }
        fn wait_for_selector_builder(selector: ()) -> WaitForSelectorBuilder { todo!() }
        async fn wait_for_timeout(timeout: ()) -> () { todo!() }
        fn wait_for_url_builder(url: ()) -> WaitForUrlBuilder { todo!() }
    }
}
mod frame_locator {
    impl FrameLocator {
        fn first() -> () { todo!() }
        fn frame_locator(selector: ()) -> () { todo!() }
        fn last() -> () { todo!() }
        fn locator_builder(selector: ()) -> LocatorBuilder { todo!() }
        fn nth(index: ()) -> () { todo!() }
    }
}
mod js_handle {
    impl JsHandle {
        fn as_element() -> () { todo!() }
        async fn dispose() -> () { todo!() }
        async fn evaluate(expression: ()) -> () { todo!() }
        async fn evaluate_handle(expression: ()) -> () { todo!() }
        async fn get_properties() -> () { todo!() }
        async fn get_property(property_name: ()) -> () { todo!() }
        async fn json_value() -> () { todo!() }
    }
}
mod keyboard {
    impl Keyboard {
        async fn down(key: ()) -> () { todo!() }
        async fn insert_text(text: ()) -> () { todo!() }
        fn press_builder(key: ()) -> PressBuilder { todo!() }
        fn type_builder(text: ()) -> TypeBuilder { todo!() }
        async fn up(key: ()) -> () { todo!() }
    }
}
mod locator {
    impl Locator {
        async fn all_inner_texts() -> () { todo!() }
        async fn all_text_contents() -> () { todo!() }
        fn bounding_box_builder() -> BoundingBoxBuilder { todo!() }
        fn check_builder() -> CheckBuilder { todo!() }
        fn click_builder() -> ClickBuilder { todo!() }
        async fn count() -> () { todo!() }
        fn dblclick_builder() -> DblclickBuilder { todo!() }
        fn dispatch_event_builder(r#type: ()) -> DispatchEventBuilder { todo!() }
        fn drag_to_builder(target: ()) -> DragToBuilder { todo!() }
        fn element_handle_builder() -> ElementHandleBuilder { todo!() }
        async fn element_handles() -> () { todo!() }
        fn evaluate_builder(expression: ()) -> EvaluateBuilder { todo!() }
        async fn evaluate_all(expression: ()) -> () { todo!() }
        fn evaluate_handle_builder(expression: ()) -> EvaluateHandleBuilder { todo!() }
        fn fill_builder(value: ()) -> FillBuilder { todo!() }
        fn filter_builder() -> FilterBuilder { todo!() }
        fn first() -> () { todo!() }
        fn focus_builder() -> FocusBuilder { todo!() }
        fn frame_locator(selector: ()) -> () { todo!() }
        fn get_attribute_builder(name: ()) -> GetAttributeBuilder { todo!() }
        async fn highlight() -> () { todo!() }
        fn hover_builder() -> HoverBuilder { todo!() }
        fn inner_html_builder() -> InnerHtmlBuilder { todo!() }
        fn inner_text_builder() -> InnerTextBuilder { todo!() }
        fn input_value_builder() -> InputValueBuilder { todo!() }
        fn is_checked_builder() -> IsCheckedBuilder { todo!() }
        fn is_disabled_builder() -> IsDisabledBuilder { todo!() }
        fn is_editable_builder() -> IsEditableBuilder { todo!() }
        fn is_enabled_builder() -> IsEnabledBuilder { todo!() }
        fn is_hidden_builder() -> IsHiddenBuilder { todo!() }
        fn is_visible_builder() -> IsVisibleBuilder { todo!() }
        fn last() -> () { todo!() }
        fn locator_builder(selector: ()) -> LocatorBuilder { todo!() }
        fn nth(index: ()) -> () { todo!() }
        fn page() -> () { todo!() }
        fn press_builder(key: ()) -> PressBuilder { todo!() }
        fn screenshot_builder() -> ScreenshotBuilder { todo!() }
        fn scroll_into_view_if_needed_builder() -> ScrollIntoViewIfNeededBuilder { todo!() }
        fn select_option_builder(values: ()) -> SelectOptionBuilder { todo!() }
        fn select_text_builder() -> SelectTextBuilder { todo!() }
        fn set_checked_builder(checked: ()) -> SetCheckedBuilder { todo!() }
        fn set_input_files_builder(files: ()) -> SetInputFilesBuilder { todo!() }
        fn tap_builder() -> TapBuilder { todo!() }
        fn text_content_builder() -> TextContentBuilder { todo!() }
        fn type_builder(text: ()) -> TypeBuilder { todo!() }
        fn uncheck_builder() -> UncheckBuilder { todo!() }
        fn wait_for_builder() -> WaitForBuilder { todo!() }
    }
}
mod locator_assertions {
    impl LocatorAssertions {
        fn not() -> () { todo!() }
        fn not_to_be_checked_builder() -> NotToBeCheckedBuilder { todo!() }
        fn not_to_be_disabled_builder() -> NotToBeDisabledBuilder { todo!() }
        fn not_to_be_editable_builder() -> NotToBeEditableBuilder { todo!() }
        fn not_to_be_empty_builder() -> NotToBeEmptyBuilder { todo!() }
        fn not_to_be_enabled_builder() -> NotToBeEnabledBuilder { todo!() }
        fn not_to_be_focused_builder() -> NotToBeFocusedBuilder { todo!() }
        fn not_to_be_hidden_builder() -> NotToBeHiddenBuilder { todo!() }
        fn not_to_be_visible_builder() -> NotToBeVisibleBuilder { todo!() }
        fn not_to_contain_text_builder(expected: ()) -> NotToContainTextBuilder { todo!() }
        fn not_to_have_attribute_builder(name: (), value: ()) -> NotToHaveAttributeBuilder {
            todo!()
        }
        fn not_to_have_class_builder(expected: ()) -> NotToHaveClassBuilder { todo!() }
        fn not_to_have_count_builder(count: ()) -> NotToHaveCountBuilder { todo!() }
        fn not_to_have_css_builder(name: (), value: ()) -> NotToHaveCssBuilder { todo!() }
        fn not_to_have_id_builder(id: ()) -> NotToHaveIdBuilder { todo!() }
        fn not_to_have_js_property_builder(name: (), value: ()) -> NotToHaveJsPropertyBuilder {
            todo!()
        }
        fn not_to_have_text_builder(expected: ()) -> NotToHaveTextBuilder { todo!() }
        fn not_to_have_value_builder(value: ()) -> NotToHaveValueBuilder { todo!() }
        fn not_to_have_values_builder(values: ()) -> NotToHaveValuesBuilder { todo!() }
        fn to_be_checked_builder() -> ToBeCheckedBuilder { todo!() }
        fn to_be_disabled_builder() -> ToBeDisabledBuilder { todo!() }
        fn to_be_editable_builder() -> ToBeEditableBuilder { todo!() }
        fn to_be_empty_builder() -> ToBeEmptyBuilder { todo!() }
        fn to_be_enabled_builder() -> ToBeEnabledBuilder { todo!() }
        fn to_be_focused_builder() -> ToBeFocusedBuilder { todo!() }
        fn to_be_hidden_builder() -> ToBeHiddenBuilder { todo!() }
        fn to_be_visible_builder() -> ToBeVisibleBuilder { todo!() }
        fn to_contain_text_builder(expected: (), expected: ()) -> ToContainTextBuilder { todo!() }
        fn to_have_attribute_builder(name: (), value: ()) -> ToHaveAttributeBuilder { todo!() }
        fn to_have_class_builder(expected: (), expected: ()) -> ToHaveClassBuilder { todo!() }
        fn to_have_count_builder(count: ()) -> ToHaveCountBuilder { todo!() }
        fn to_have_css_builder(name: (), value: ()) -> ToHaveCssBuilder { todo!() }
        fn to_have_id_builder(id: ()) -> ToHaveIdBuilder { todo!() }
        fn to_have_js_property_builder(name: (), value: ()) -> ToHaveJsPropertyBuilder { todo!() }
        fn to_have_screenshot1_builder(name: ()) -> ToHaveScreenshot1Builder { todo!() }
        fn to_have_screenshot2_builder() -> ToHaveScreenshot2Builder { todo!() }
        fn to_have_text_builder(expected: (), expected: ()) -> ToHaveTextBuilder { todo!() }
        fn to_have_value_builder(value: ()) -> ToHaveValueBuilder { todo!() }
        fn to_have_values_builder(values: (), values: ()) -> ToHaveValuesBuilder { todo!() }
    }
}
mod logger {
    impl Logger {
        fn is_enabled(name: (), severity: ()) -> () { todo!() }
        fn log(name: (), severity: (), message: (), args: (), hints: ()) -> () { todo!() }
    }
}
mod mouse {
    impl Mouse {
        fn click_builder(x: (), y: ()) -> ClickBuilder { todo!() }
        fn dblclick_builder(x: (), y: ()) -> DblclickBuilder { todo!() }
        fn down_builder() -> DownBuilder { todo!() }
        fn move_builder(x: (), y: ()) -> MoveBuilder { todo!() }
        fn up_builder() -> UpBuilder { todo!() }
        async fn wheel(delta_x: (), delta_y: ()) -> () { todo!() }
    }
}
mod page {
    /// Extends EventEmitter
    impl Page {
        fn accessibility() -> () { todo!() }
        fn add_init_script_builder(script: (), script: ()) -> AddInitScriptBuilder { todo!() }
        fn add_script_tag_builder() -> AddScriptTagBuilder { todo!() }
        fn add_style_tag_builder() -> AddStyleTagBuilder { todo!() }
        async fn bring_to_front() -> () { todo!() }
        fn check_builder(selector: ()) -> CheckBuilder { todo!() }
        fn click_builder(selector: ()) -> ClickBuilder { todo!() }
        fn close_builder() -> CloseBuilder { todo!() }
        async fn content() -> () { todo!() }
        fn context() -> () { todo!() }
        fn coverage() -> () { todo!() }
        fn dblclick_builder(selector: ()) -> DblclickBuilder { todo!() }
        fn dispatch_event_builder(selector: (), r#type: ()) -> DispatchEventBuilder { todo!() }
        fn drag_and_drop_builder(source: (), target: ()) -> DragAndDropBuilder { todo!() }
        fn emulate_media_builder() -> EmulateMediaBuilder { todo!() }
        fn eval_on_selector_builder(selector: (), expression: ()) -> EvalOnSelectorBuilder {
            todo!()
        }
        async fn eval_on_selector_all(selector: (), expression: ()) -> () { todo!() }
        async fn evaluate(expression: ()) -> () { todo!() }
        async fn evaluate_handle(expression: ()) -> () { todo!() }
        fn expose_binding_builder(name: (), callback: ()) -> ExposeBindingBuilder { todo!() }
        async fn expose_function(name: (), callback: ()) -> () { todo!() }
        fn fill_builder(selector: (), value: ()) -> FillBuilder { todo!() }
        fn focus_builder(selector: ()) -> FocusBuilder { todo!() }
        fn frame_builder(frame_selector: (), name: ()) -> FrameBuilder { todo!() }
        fn frame_by_url(url: ()) -> () { todo!() }
        fn frame_locator(selector: ()) -> () { todo!() }
        fn frames() -> () { todo!() }
        fn get_attribute_builder(selector: (), name: ()) -> GetAttributeBuilder { todo!() }
        fn go_back_builder() -> GoBackBuilder { todo!() }
        fn go_forward_builder() -> GoForwardBuilder { todo!() }
        fn goto_builder(url: ()) -> GotoBuilder { todo!() }
        fn hover_builder(selector: ()) -> HoverBuilder { todo!() }
        fn inner_html_builder(selector: ()) -> InnerHtmlBuilder { todo!() }
        fn inner_text_builder(selector: ()) -> InnerTextBuilder { todo!() }
        fn input_value_builder(selector: ()) -> InputValueBuilder { todo!() }
        fn is_checked_builder(selector: ()) -> IsCheckedBuilder { todo!() }
        fn is_closed() -> () { todo!() }
        fn is_disabled_builder(selector: ()) -> IsDisabledBuilder { todo!() }
        fn is_editable_builder(selector: ()) -> IsEditableBuilder { todo!() }
        fn is_enabled_builder(selector: ()) -> IsEnabledBuilder { todo!() }
        fn is_hidden_builder(selector: ()) -> IsHiddenBuilder { todo!() }
        fn is_visible_builder(selector: ()) -> IsVisibleBuilder { todo!() }
        fn keyboard() -> () { todo!() }
        fn locator_builder(selector: ()) -> LocatorBuilder { todo!() }
        fn main_frame() -> () { todo!() }
        fn mouse() -> () { todo!() }
        async fn opener() -> () { todo!() }
        async fn pause() -> () { todo!() }
        fn pdf_builder() -> PdfBuilder { todo!() }
        fn press_builder(selector: (), key: ()) -> PressBuilder { todo!() }
        fn query_selector_builder(selector: ()) -> QuerySelectorBuilder { todo!() }
        async fn query_selector_all(selector: ()) -> () { todo!() }
        fn reload_builder() -> ReloadBuilder { todo!() }
        fn request() -> () { todo!() }
        fn route_builder(url: (), handler: (), handler: ()) -> RouteBuilder { todo!() }
        fn route_from_har_builder(har: ()) -> RouteFromHarBuilder { todo!() }
        fn screenshot_builder() -> ScreenshotBuilder { todo!() }
        fn select_option_builder(selector: (), values: ()) -> SelectOptionBuilder { todo!() }
        fn set_checked_builder(selector: (), checked: ()) -> SetCheckedBuilder { todo!() }
        fn set_content_builder(html: ()) -> SetContentBuilder { todo!() }
        fn set_default_navigation_timeout(timeout: ()) -> () { todo!() }
        fn set_default_timeout(timeout: ()) -> () { todo!() }
        async fn set_extra_http_headers(headers: ()) -> () { todo!() }
        fn set_input_files_builder(selector: (), files: ()) -> SetInputFilesBuilder { todo!() }
        async fn set_viewport_size(viewport_size: (), width: (), height: ()) -> () { todo!() }
        fn tap_builder(selector: ()) -> TapBuilder { todo!() }
        fn text_content_builder(selector: ()) -> TextContentBuilder { todo!() }
        async fn title() -> () { todo!() }
        fn touchscreen() -> () { todo!() }
        fn type_builder(selector: (), text: ()) -> TypeBuilder { todo!() }
        fn uncheck_builder(selector: ()) -> UncheckBuilder { todo!() }
        fn unroute_builder(url: ()) -> UnrouteBuilder { todo!() }
        fn url() -> () { todo!() }
        fn video() -> () { todo!() }
        fn viewport_size() -> () { todo!() }
        fn wait_for_close_builder(callback: ()) -> WaitForCloseBuilder { todo!() }
        fn wait_for_console_message_builder(
            action: (),
            callback: ()
        ) -> WaitForConsoleMessageBuilder {
            todo!()
        }
        fn wait_for_download_builder(action: (), callback: ()) -> WaitForDownloadBuilder { todo!() }
        fn wait_for_event_builder(event: ()) -> WaitForEventBuilder { todo!() }
        fn wait_for_file_chooser_builder(action: (), callback: ()) -> WaitForFileChooserBuilder {
            todo!()
        }
        fn wait_for_function_builder(expression: ()) -> WaitForFunctionBuilder { todo!() }
        fn wait_for_load_state_builder() -> WaitForLoadStateBuilder { todo!() }
        fn wait_for_navigation_builder(action: (), callback: ()) -> WaitForNavigationBuilder {
            todo!()
        }
        fn wait_for_popup_builder(action: (), callback: ()) -> WaitForPopupBuilder { todo!() }
        fn wait_for_request_builder(
            action: (),
            url_or_predicate: (),
            callback: ()
        ) -> WaitForRequestBuilder {
            todo!()
        }
        fn wait_for_request_finished_builder(
            action: (),
            callback: ()
        ) -> WaitForRequestFinishedBuilder {
            todo!()
        }
        fn wait_for_response_builder(
            action: (),
            url_or_predicate: (),
            callback: ()
        ) -> WaitForResponseBuilder {
            todo!()
        }
        fn wait_for_selector_builder(selector: ()) -> WaitForSelectorBuilder { todo!() }
        async fn wait_for_timeout(timeout: ()) -> () { todo!() }
        fn wait_for_url_builder(url: ()) -> WaitForUrlBuilder { todo!() }
        fn wait_for_web_socket_builder(action: (), callback: ()) -> WaitForWebSocketBuilder {
            todo!()
        }
        fn wait_for_worker_builder(action: (), callback: ()) -> WaitForWorkerBuilder { todo!() }
        fn workers() -> () { todo!() }
        fn wait_for_event2_builder(event: ()) -> WaitForEvent2Builder { todo!() }
        fn once_dialog(handler: ()) -> () { todo!() }
    }
}
mod page_assertions {
    impl PageAssertions {
        fn not() -> () { todo!() }
        fn not_to_have_title_builder(title_or_reg_exp: ()) -> NotToHaveTitleBuilder { todo!() }
        fn not_to_have_url_builder(url_or_reg_exp: ()) -> NotToHaveUrlBuilder { todo!() }
        fn to_have_screenshot1_builder(name: ()) -> ToHaveScreenshot1Builder { todo!() }
        fn to_have_screenshot2_builder() -> ToHaveScreenshot2Builder { todo!() }
        fn to_have_title_builder(title_or_reg_exp: ()) -> ToHaveTitleBuilder { todo!() }
        fn to_have_url_builder(url_or_reg_exp: ()) -> ToHaveUrlBuilder { todo!() }
    }
}
mod playwright {
    impl Playwright {
        fn chromium() -> () { todo!() }
        fn devices() -> () { todo!() }
        fn devices() -> () { todo!() }
        fn errors() -> () { todo!() }
        fn firefox() -> () { todo!() }
        fn request() -> () { todo!() }
        fn selectors() -> () { todo!() }
        fn webkit() -> () { todo!() }
        fn close() -> () { todo!() }
        fn create_builder() -> CreateBuilder { todo!() }
        async fn stop() -> () { todo!() }
    }
}
mod playwright_assertions {
    impl PlaywrightAssertions {
        fn expect_api_response(response: ()) -> () { todo!() }
        fn expect_locator(locator: ()) -> () { todo!() }
        fn expect_page(page: ()) -> () { todo!() }
    }
}
mod playwright_exception {
    /// Extends RuntimeException
    impl PlaywrightException {}
}
mod request {
    impl Request {
        async fn all_headers() -> () { todo!() }
        fn failure() -> () { todo!() }
        fn frame() -> () { todo!() }
        fn headers() -> () { todo!() }
        async fn headers_array() -> () { todo!() }
        async fn header_value(name: ()) -> () { todo!() }
        fn is_navigation_request() -> () { todo!() }
        fn method() -> () { todo!() }
        fn post_data() -> () { todo!() }
        fn post_data_buffer() -> () { todo!() }
        fn post_data_json() -> () { todo!() }
        fn redirected_from() -> () { todo!() }
        fn redirected_to() -> () { todo!() }
        fn resource_type() -> () { todo!() }
        async fn response() -> () { todo!() }
        fn service_worker() -> () { todo!() }
        async fn sizes() -> () { todo!() }
        fn timing() -> () { todo!() }
        fn url() -> () { todo!() }
        fn post_data_json() -> () { todo!() }
    }
}
mod request_options {
    impl RequestOptions {
        fn create() -> () { todo!() }
        fn set_data(data: ()) -> () { todo!() }
        fn set_fail_on_status_code(fail_on_status_code: ()) -> () { todo!() }
        fn set_form(form: ()) -> () { todo!() }
        fn set_header(name: (), value: ()) -> () { todo!() }
        fn set_ignore_https_errors(ignore_https_errors: ()) -> () { todo!() }
        fn set_method(method: ()) -> () { todo!() }
        fn set_multipart(form: ()) -> () { todo!() }
        fn set_query_param(name: (), value: ()) -> () { todo!() }
        fn set_timeout(timeout: ()) -> () { todo!() }
    }
}
mod response {
    impl Response {
        async fn all_headers() -> () { todo!() }
        async fn body() -> () { todo!() }
        async fn finished() -> () { todo!() }
        fn frame() -> () { todo!() }
        fn from_service_worker() -> () { todo!() }
        fn headers() -> () { todo!() }
        async fn headers_array() -> () { todo!() }
        async fn header_value(name: ()) -> () { todo!() }
        async fn header_values(name: ()) -> () { todo!() }
        async fn json() -> () { todo!() }
        async fn json() -> () { todo!() }
        fn ok() -> () { todo!() }
        fn request() -> () { todo!() }
        async fn security_details() -> () { todo!() }
        async fn server_addr() -> () { todo!() }
        fn status() -> () { todo!() }
        fn status_text() -> () { todo!() }
        async fn text() -> () { todo!() }
        fn url() -> () { todo!() }
    }
}
mod route {
    impl Route {
        async fn abort() -> () { todo!() }
        fn continue_builder() -> ContinueBuilder { todo!() }
        fn fallback_builder() -> FallbackBuilder { todo!() }
        fn fulfill_builder() -> FulfillBuilder { todo!() }
        fn request() -> () { todo!() }
    }
}
mod screenshot_assertions {
    impl ScreenshotAssertions {
        fn to_match_snapshot1_builder(name: ()) -> ToMatchSnapshot1Builder { todo!() }
        fn to_match_snapshot2_builder() -> ToMatchSnapshot2Builder { todo!() }
    }
}
mod selectors {
    impl Selectors {
        fn register_builder(name: (), script: (), script: ()) -> RegisterBuilder { todo!() }
    }
}
mod timeout_error {
    /// Extends Error
    impl TimeoutError {}
}
mod touchscreen {
    impl Touchscreen {
        async fn tap(x: (), y: ()) -> () { todo!() }
    }
}
mod tracing {
    impl Tracing {
        fn start_builder() -> StartBuilder { todo!() }
        fn start_chunk_builder() -> StartChunkBuilder { todo!() }
        fn stop_builder() -> StopBuilder { todo!() }
        fn stop_chunk_builder() -> StopChunkBuilder { todo!() }
    }
}
mod video {
    impl Video {
        async fn delete() -> () { todo!() }
        async fn path() -> () { todo!() }
        async fn save_as(path: ()) -> () { todo!() }
    }
}
mod web_socket {
    impl WebSocket {
        fn is_closed() -> () { todo!() }
        fn url() -> () { todo!() }
        fn wait_for_event_builder(event: ()) -> WaitForEventBuilder { todo!() }
        fn wait_for_frame_received_builder(callback: ()) -> WaitForFrameReceivedBuilder { todo!() }
        fn wait_for_frame_sent_builder(callback: ()) -> WaitForFrameSentBuilder { todo!() }
        fn wait_for_event2_builder(event: ()) -> WaitForEvent2Builder { todo!() }
    }
}
mod web_socket_frame {
    impl WebSocketFrame {
        fn binary() -> () { todo!() }
        fn text() -> () { todo!() }
    }
}
mod worker {
    impl Worker {
        async fn evaluate(expression: ()) -> () { todo!() }
        async fn evaluate_handle(expression: ()) -> () { todo!() }
        fn url() -> () { todo!() }
        fn wait_for_close_builder(callback: ()) -> WaitForCloseBuilder { todo!() }
    }
}
// vim: foldnestmax=0 ft=rust
