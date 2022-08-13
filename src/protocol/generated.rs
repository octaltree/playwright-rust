#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Guid(String);
pub type Channel = Guid;
pub mod api_request_context {
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Initializer {
        #[serde(rename = "tracing")]
        tracing: crate::protocol::generated::Tracing
    }
    pub mod commands {
        pub type Dispose = ();
        pub type DisposeArgs = ();
        pub type DisposeApiResponse = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct DisposeApiResponseArgs {
            #[serde(rename = "fetchUid")]
            fetch_uid: String
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Fetch {
            #[serde(rename = "response")]
            response: crate::protocol::generated::ApiResponse
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct FetchArgs {
            #[serde(rename = "failOnStatusCode")]
            fail_on_status_code: Option<bool>,
            #[serde(rename = "formData")]
            form_data: Option<Vec<crate::protocol::generated::NameValue>>,
            #[serde(rename = "headers")]
            headers: Option<Vec<crate::protocol::generated::NameValue>>,
            #[serde(rename = "ignoreHTTPSErrors")]
            ignore_https_errors: Option<bool>,
            #[serde(rename = "jsonData")]
            json_data: Option<String>,
            #[serde(rename = "method")]
            method: Option<String>,
            #[serde(rename = "multipartData")]
            multipart_data: Option<Vec<crate::protocol::generated::FormField>>,
            #[serde(rename = "params")]
            params: Option<Vec<crate::protocol::generated::NameValue>>,
            #[serde(rename = "postData")]
            post_data: Option<Vec<u8>>,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>,
            #[serde(rename = "url")]
            url: String
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct FetchLog {
            #[serde(rename = "log")]
            log: Vec<String>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct FetchLogArgs {
            #[serde(rename = "fetchUid")]
            fetch_uid: String
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct FetchResponseBody {
            #[serde(rename = "binary")]
            binary: Option<Vec<u8>>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct FetchResponseBodyArgs {
            #[serde(rename = "fetchUid")]
            fetch_uid: String
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct StorageState {
            #[serde(rename = "cookies")]
            cookies: Vec<crate::protocol::generated::NetworkCookie>,
            #[serde(rename = "origins")]
            origins: Vec<crate::protocol::generated::OriginStorage>
        }
        pub type StorageStateArgs = ();
    }
}
pub type ApiRequestContext = Guid;
pub mod android {
    pub mod commands {
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Devices {
            #[serde(rename = "devices")]
            devices: Vec<crate::protocol::generated::AndroidDevice>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct DevicesArgs {
            #[serde(rename = "host")]
            host: Option<String>,
            #[serde(rename = "omitDriverInstall")]
            omit_driver_install: Option<bool>,
            #[serde(rename = "port")]
            port: Option<serde_json::Number>
        }
        pub type SetDefaultTimeoutNoReply = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetDefaultTimeoutNoReplyArgs {
            #[serde(rename = "timeout")]
            timeout: serde_json::Number
        }
    }
}
pub type Android = Guid;
pub mod android_device {
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Initializer {
        #[serde(rename = "model")]
        model: String,
        #[serde(rename = "serial")]
        serial: String
    }
    pub mod events {
        #[derive(Debug, Deserialize, Serialize)]
        pub enum Events {
            #[serde(rename = "webViewAdded")]
            WebViewAdded(WebViewAdded),
            #[serde(rename = "webViewRemoved")]
            WebViewRemoved(WebViewRemoved)
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct WebViewAdded {
            #[serde(rename = "webView")]
            web_view: crate::protocol::generated::AndroidWebView
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct WebViewRemoved {
            #[serde(rename = "socketName")]
            socket_name: String
        }
    }
    pub mod commands {
        pub type Close = ();
        pub type CloseArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ConnectToWebView {
            #[serde(rename = "context")]
            context: crate::protocol::generated::BrowserContext
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ConnectToWebViewArgs {
            #[serde(rename = "socketName")]
            socket_name: String
        }
        pub type Drag = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct DragArgs {
            #[serde(rename = "dest")]
            dest: crate::protocol::generated::Point,
            #[serde(rename = "selector")]
            selector: crate::protocol::generated::AndroidSelector,
            #[serde(rename = "speed")]
            speed: Option<serde_json::Number>,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>
        }
        pub type Fill = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct FillArgs {
            #[serde(rename = "selector")]
            selector: crate::protocol::generated::AndroidSelector,
            #[serde(rename = "text")]
            text: String,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>
        }
        pub type Fling = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct FlingArgs {
            #[serde(rename = "direction")]
            direction: FlingArgsDirection,
            #[serde(rename = "selector")]
            selector: crate::protocol::generated::AndroidSelector,
            #[serde(rename = "speed")]
            speed: Option<serde_json::Number>,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub enum FlingArgsDirection {
            #[serde(rename = "up")]
            Up,
            #[serde(rename = "down")]
            Down,
            #[serde(rename = "left")]
            Left,
            #[serde(rename = "right")]
            Right
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Info {
            #[serde(rename = "info")]
            info: crate::protocol::generated::AndroidElementInfo
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct InfoArgs {
            #[serde(rename = "selector")]
            selector: crate::protocol::generated::AndroidSelector
        }
        pub type InputDrag = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct InputDragArgs {
            #[serde(rename = "from")]
            from: crate::protocol::generated::Point,
            #[serde(rename = "steps")]
            steps: serde_json::Number,
            #[serde(rename = "to")]
            to: crate::protocol::generated::Point
        }
        pub type InputPress = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct InputPressArgs {
            #[serde(rename = "key")]
            key: String
        }
        pub type InputSwipe = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct InputSwipeArgs {
            #[serde(rename = "segments")]
            segments: Vec<crate::protocol::generated::Point>,
            #[serde(rename = "steps")]
            steps: serde_json::Number
        }
        pub type InputTap = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct InputTapArgs {
            #[serde(rename = "point")]
            point: crate::protocol::generated::Point
        }
        pub type InputType = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct InputTypeArgs {
            #[serde(rename = "text")]
            text: String
        }
        pub type InstallApk = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct InstallApkArgs {
            #[serde(rename = "args")]
            args: Option<Vec<String>>,
            #[serde(rename = "file")]
            file: Vec<u8>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct LaunchBrowser {
            #[serde(rename = "context")]
            context: crate::protocol::generated::BrowserContext
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct LaunchBrowserArgs {
            #[serde(flatten)]
            #[serde(rename = "$mixin")]
            mixin: crate::protocol::generated::ContextOptions,
            #[serde(rename = "pkg")]
            pkg: Option<String>,
            #[serde(rename = "proxy")]
            proxy: Option<LaunchBrowserArgsProxy>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct LaunchBrowserArgsProxy {
            #[serde(rename = "bypass")]
            bypass: Option<String>,
            #[serde(rename = "password")]
            password: Option<String>,
            #[serde(rename = "server")]
            server: String,
            #[serde(rename = "username")]
            username: Option<String>
        }
        pub type LongTap = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct LongTapArgs {
            #[serde(rename = "selector")]
            selector: crate::protocol::generated::AndroidSelector,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Open {
            #[serde(rename = "socket")]
            socket: crate::protocol::generated::AndroidSocket
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct OpenArgs {
            #[serde(rename = "command")]
            command: String
        }
        pub type PinchClose = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct PinchCloseArgs {
            #[serde(rename = "percent")]
            percent: serde_json::Number,
            #[serde(rename = "selector")]
            selector: crate::protocol::generated::AndroidSelector,
            #[serde(rename = "speed")]
            speed: Option<serde_json::Number>,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>
        }
        pub type PinchOpen = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct PinchOpenArgs {
            #[serde(rename = "percent")]
            percent: serde_json::Number,
            #[serde(rename = "selector")]
            selector: crate::protocol::generated::AndroidSelector,
            #[serde(rename = "speed")]
            speed: Option<serde_json::Number>,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>
        }
        pub type Push = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct PushArgs {
            #[serde(rename = "file")]
            file: Vec<u8>,
            #[serde(rename = "mode")]
            mode: Option<serde_json::Number>,
            #[serde(rename = "path")]
            path: String
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Screenshot {
            #[serde(rename = "binary")]
            binary: Vec<u8>
        }
        pub type ScreenshotArgs = ();
        pub type Scroll = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ScrollArgs {
            #[serde(rename = "direction")]
            direction: ScrollArgsDirection,
            #[serde(rename = "percent")]
            percent: serde_json::Number,
            #[serde(rename = "selector")]
            selector: crate::protocol::generated::AndroidSelector,
            #[serde(rename = "speed")]
            speed: Option<serde_json::Number>,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub enum ScrollArgsDirection {
            #[serde(rename = "up")]
            Up,
            #[serde(rename = "down")]
            Down,
            #[serde(rename = "left")]
            Left,
            #[serde(rename = "right")]
            Right
        }
        pub type SetDefaultTimeoutNoReply = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetDefaultTimeoutNoReplyArgs {
            #[serde(rename = "timeout")]
            timeout: serde_json::Number
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Shell {
            #[serde(rename = "result")]
            result: Vec<u8>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ShellArgs {
            #[serde(rename = "command")]
            command: String
        }
        pub type Swipe = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SwipeArgs {
            #[serde(rename = "direction")]
            direction: SwipeArgsDirection,
            #[serde(rename = "percent")]
            percent: serde_json::Number,
            #[serde(rename = "selector")]
            selector: crate::protocol::generated::AndroidSelector,
            #[serde(rename = "speed")]
            speed: Option<serde_json::Number>,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub enum SwipeArgsDirection {
            #[serde(rename = "up")]
            Up,
            #[serde(rename = "down")]
            Down,
            #[serde(rename = "left")]
            Left,
            #[serde(rename = "right")]
            Right
        }
        pub type Tap = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct TapArgs {
            #[serde(rename = "duration")]
            duration: Option<serde_json::Number>,
            #[serde(rename = "selector")]
            selector: crate::protocol::generated::AndroidSelector,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>
        }
        pub type Wait = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct WaitArgs {
            #[serde(rename = "selector")]
            selector: crate::protocol::generated::AndroidSelector,
            #[serde(rename = "state")]
            state: Option<WaitArgsState>,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub enum WaitArgsState {
            #[serde(rename = "gone")]
            Gone
        }
    }
}
pub type AndroidDevice = Guid;
pub mod android_socket {
    pub mod events {
        #[derive(Debug, Deserialize, Serialize)]
        pub enum Events {
            #[serde(rename = "close")]
            Close,
            #[serde(rename = "data")]
            Data(Data)
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Data {
            #[serde(rename = "data")]
            data: Vec<u8>
        }
    }
    pub mod commands {
        pub type Close = ();
        pub type CloseArgs = ();
        pub type Write = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct WriteArgs {
            #[serde(rename = "data")]
            data: Vec<u8>
        }
    }
}
pub type AndroidSocket = Guid;
pub mod artifact {
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Initializer {
        #[serde(rename = "absolutePath")]
        absolute_path: String
    }
    pub mod commands {
        pub type Cancel = ();
        pub type CancelArgs = ();
        pub type Delete = ();
        pub type DeleteArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Failure {
            #[serde(rename = "error")]
            error: Option<String>
        }
        pub type FailureArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct PathAfterFinished {
            #[serde(rename = "value")]
            value: Option<String>
        }
        pub type PathAfterFinishedArgs = ();
        pub type SaveAs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SaveAsArgs {
            #[serde(rename = "path")]
            path: String
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SaveAsStream {
            #[serde(rename = "stream")]
            stream: crate::protocol::generated::Stream
        }
        pub type SaveAsStreamArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Stream {
            #[serde(rename = "stream")]
            stream: Option<crate::protocol::generated::Stream>
        }
        pub type StreamArgs = ();
    }
}
pub type Artifact = Guid;
pub mod binding_call {
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Initializer {
        #[serde(rename = "args")]
        args: Option<Vec<crate::protocol::generated::SerializedValue>>,
        #[serde(rename = "frame")]
        frame: crate::protocol::generated::Frame,
        #[serde(rename = "handle")]
        handle: Option<crate::protocol::generated::JsHandle>,
        #[serde(rename = "name")]
        name: String
    }
    pub mod commands {
        pub type Reject = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct RejectArgs {
            #[serde(rename = "error")]
            error: crate::protocol::generated::SerializedError
        }
        pub type Resolve = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ResolveArgs {
            #[serde(rename = "result")]
            result: crate::protocol::generated::SerializedArgument
        }
    }
}
pub type BindingCall = Guid;
pub mod browser {
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Initializer {
        #[serde(rename = "name")]
        name: String,
        #[serde(rename = "version")]
        version: String
    }
    pub mod events {
        #[derive(Debug, Deserialize, Serialize)]
        pub enum Events {
            #[serde(rename = "close")]
            Close
        }
    }
    pub mod commands {
        pub type Close = ();
        pub type CloseArgs = ();
        pub type KillForTests = ();
        pub type KillForTestsArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct NewBrowserCdpSession {
            #[serde(rename = "session")]
            session: crate::protocol::generated::CdpSession
        }
        pub type NewBrowserCdpSessionArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct NewContext {
            #[serde(rename = "context")]
            context: crate::protocol::generated::BrowserContext
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct NewContextArgs {
            #[serde(flatten)]
            #[serde(rename = "$mixin")]
            mixin: crate::protocol::generated::ContextOptions,
            #[serde(rename = "proxy")]
            proxy: Option<NewContextArgsProxy>,
            #[serde(rename = "storageState")]
            storage_state: Option<NewContextArgsStorageState>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct NewContextArgsProxy {
            #[serde(rename = "bypass")]
            bypass: Option<String>,
            #[serde(rename = "password")]
            password: Option<String>,
            #[serde(rename = "server")]
            server: String,
            #[serde(rename = "username")]
            username: Option<String>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct NewContextArgsStorageState {
            #[serde(rename = "cookies")]
            cookies: Option<Vec<crate::protocol::generated::SetNetworkCookie>>,
            #[serde(rename = "origins")]
            origins: Option<Vec<crate::protocol::generated::OriginStorage>>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct NewContextForReuse {
            #[serde(rename = "context")]
            context: crate::protocol::generated::BrowserContext
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct NewContextForReuseArgs {
            #[serde(flatten)]
            #[serde(rename = "$mixin")]
            mixin: crate::protocol::generated::ContextOptions,
            #[serde(rename = "proxy")]
            proxy: Option<NewContextForReuseArgsProxy>,
            #[serde(rename = "storageState")]
            storage_state: Option<NewContextForReuseArgsStorageState>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct NewContextForReuseArgsProxy {
            #[serde(rename = "bypass")]
            bypass: Option<String>,
            #[serde(rename = "password")]
            password: Option<String>,
            #[serde(rename = "server")]
            server: String,
            #[serde(rename = "username")]
            username: Option<String>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct NewContextForReuseArgsStorageState {
            #[serde(rename = "cookies")]
            cookies: Option<Vec<crate::protocol::generated::SetNetworkCookie>>,
            #[serde(rename = "origins")]
            origins: Option<Vec<crate::protocol::generated::OriginStorage>>
        }
        pub type StartTracing = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct StartTracingArgs {
            #[serde(rename = "categories")]
            categories: Option<Vec<String>>,
            #[serde(rename = "page")]
            page: Option<crate::protocol::generated::Page>,
            #[serde(rename = "path")]
            path: Option<String>,
            #[serde(rename = "screenshots")]
            screenshots: Option<bool>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct StopTracing {
            #[serde(rename = "binary")]
            binary: Vec<u8>
        }
        pub type StopTracingArgs = ();
    }
}
pub type Browser = Guid;
pub mod browser_context {
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Initializer {
        #[serde(rename = "isChromium")]
        is_chromium: bool,
        #[serde(rename = "requestContext")]
        request_context: crate::protocol::generated::ApiRequestContext,
        #[serde(rename = "tracing")]
        tracing: crate::protocol::generated::Tracing
    }
    pub mod events {
        #[derive(Debug, Deserialize, Serialize)]
        pub enum Events {
            #[serde(rename = "backgroundPage")]
            BackgroundPage(BackgroundPage),
            #[serde(rename = "bindingCall")]
            BindingCall(BindingCall),
            #[serde(rename = "close")]
            Close,
            #[serde(rename = "page")]
            Page(Page),
            #[serde(rename = "request")]
            Request(Request),
            #[serde(rename = "requestFailed")]
            RequestFailed(RequestFailed),
            #[serde(rename = "requestFinished")]
            RequestFinished(RequestFinished),
            #[serde(rename = "response")]
            Response(Response),
            #[serde(rename = "route")]
            Route(Route),
            #[serde(rename = "serviceWorker")]
            ServiceWorker(ServiceWorker),
            #[serde(rename = "video")]
            Video(Video)
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct BackgroundPage {
            #[serde(rename = "page")]
            page: crate::protocol::generated::Page
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct BindingCall {
            #[serde(rename = "binding")]
            binding: crate::protocol::generated::BindingCall
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Page {
            #[serde(rename = "page")]
            page: crate::protocol::generated::Page
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Request {
            #[serde(rename = "page")]
            page: Option<crate::protocol::generated::Page>,
            #[serde(rename = "request")]
            request: crate::protocol::generated::Request
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct RequestFailed {
            #[serde(rename = "failureText")]
            failure_text: Option<String>,
            #[serde(rename = "page")]
            page: Option<crate::protocol::generated::Page>,
            #[serde(rename = "request")]
            request: crate::protocol::generated::Request,
            #[serde(rename = "responseEndTiming")]
            response_end_timing: serde_json::Number
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct RequestFinished {
            #[serde(rename = "page")]
            page: Option<crate::protocol::generated::Page>,
            #[serde(rename = "request")]
            request: crate::protocol::generated::Request,
            #[serde(rename = "response")]
            response: Option<crate::protocol::generated::Response>,
            #[serde(rename = "responseEndTiming")]
            response_end_timing: serde_json::Number
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Response {
            #[serde(rename = "page")]
            page: Option<crate::protocol::generated::Page>,
            #[serde(rename = "response")]
            response: crate::protocol::generated::Response
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Route {
            #[serde(rename = "request")]
            request: crate::protocol::generated::Request,
            #[serde(rename = "route")]
            route: crate::protocol::generated::Route
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ServiceWorker {
            #[serde(rename = "worker")]
            worker: crate::protocol::generated::Worker
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Video {
            #[serde(rename = "artifact")]
            artifact: crate::protocol::generated::Artifact
        }
    }
    pub mod commands {
        pub type AddCookies = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct AddCookiesArgs {
            #[serde(rename = "cookies")]
            cookies: Vec<crate::protocol::generated::SetNetworkCookie>
        }
        pub type AddInitScript = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct AddInitScriptArgs {
            #[serde(rename = "source")]
            source: String
        }
        pub type ClearCookies = ();
        pub type ClearCookiesArgs = ();
        pub type ClearPermissions = ();
        pub type ClearPermissionsArgs = ();
        pub type Close = ();
        pub type CloseArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Cookies {
            #[serde(rename = "cookies")]
            cookies: Vec<crate::protocol::generated::NetworkCookie>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct CookiesArgs {
            #[serde(rename = "urls")]
            urls: Vec<String>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct CreateTempFile {
            #[serde(rename = "writableStream")]
            writable_stream: crate::protocol::generated::WritableStream
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct CreateTempFileArgs {
            #[serde(rename = "name")]
            name: String
        }
        pub type ExposeBinding = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ExposeBindingArgs {
            #[serde(rename = "name")]
            name: String,
            #[serde(rename = "needsHandle")]
            needs_handle: Option<bool>
        }
        pub type GrantPermissions = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct GrantPermissionsArgs {
            #[serde(rename = "origin")]
            origin: Option<String>,
            #[serde(rename = "permissions")]
            permissions: Vec<String>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct HarExport {
            #[serde(rename = "artifact")]
            artifact: crate::protocol::generated::Artifact
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct HarExportArgs {
            #[serde(rename = "harId")]
            har_id: Option<String>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct HarStart {
            #[serde(rename = "harId")]
            har_id: String
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct HarStartArgs {
            #[serde(rename = "options")]
            options: crate::protocol::generated::RecordHarOptions,
            #[serde(rename = "page")]
            page: Option<crate::protocol::generated::Page>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct NewCdpSession {
            #[serde(rename = "session")]
            session: crate::protocol::generated::CdpSession
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct NewCdpSessionArgs {
            #[serde(rename = "frame")]
            frame: Option<crate::protocol::generated::Frame>,
            #[serde(rename = "page")]
            page: Option<crate::protocol::generated::Page>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct NewPage {
            #[serde(rename = "page")]
            page: crate::protocol::generated::Page
        }
        pub type NewPageArgs = ();
        pub type Pause = ();
        pub type PauseArgs = ();
        pub type RecorderSupplementEnable = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct RecorderSupplementEnableArgs {
            #[serde(rename = "contextOptions")]
            context_options: Option<String>,
            #[serde(rename = "device")]
            device: Option<String>,
            #[serde(rename = "language")]
            language: Option<String>,
            #[serde(rename = "launchOptions")]
            launch_options: Option<String>,
            #[serde(rename = "outputFile")]
            output_file: Option<String>,
            #[serde(rename = "pauseOnNextStatement")]
            pause_on_next_statement: Option<bool>,
            #[serde(rename = "saveStorage")]
            save_storage: Option<String>,
            #[serde(rename = "startRecording")]
            start_recording: Option<bool>
        }
        pub type SetDefaultNavigationTimeoutNoReply = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetDefaultNavigationTimeoutNoReplyArgs {
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>
        }
        pub type SetDefaultTimeoutNoReply = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetDefaultTimeoutNoReplyArgs {
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>
        }
        pub type SetExtraHttpHeaders = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetExtraHttpHeadersArgs {
            #[serde(rename = "headers")]
            headers: Vec<crate::protocol::generated::NameValue>
        }
        pub type SetGeolocation = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetGeolocationArgs {
            #[serde(rename = "geolocation")]
            geolocation: Option<SetGeolocationArgsGeolocation>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetGeolocationArgsGeolocation {
            #[serde(rename = "accuracy")]
            accuracy: Option<serde_json::Number>,
            #[serde(rename = "latitude")]
            latitude: serde_json::Number,
            #[serde(rename = "longitude")]
            longitude: serde_json::Number
        }
        pub type SetHttpCredentials = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetHttpCredentialsArgs {
            #[serde(rename = "httpCredentials")]
            http_credentials: Option<SetHttpCredentialsArgsHttpCredentials>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetHttpCredentialsArgsHttpCredentials {
            #[serde(rename = "password")]
            password: String,
            #[serde(rename = "username")]
            username: String
        }
        pub type SetNetworkInterceptionEnabled = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetNetworkInterceptionEnabledArgs {
            #[serde(rename = "enabled")]
            enabled: bool
        }
        pub type SetOffline = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetOfflineArgs {
            #[serde(rename = "offline")]
            offline: bool
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct StorageState {
            #[serde(rename = "cookies")]
            cookies: Vec<crate::protocol::generated::NetworkCookie>,
            #[serde(rename = "origins")]
            origins: Vec<crate::protocol::generated::OriginStorage>
        }
        pub type StorageStateArgs = ();
    }
}
pub type BrowserContext = Guid;
pub mod browser_type {
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Initializer {
        #[serde(rename = "executablePath")]
        executable_path: String,
        #[serde(rename = "name")]
        name: String
    }
    pub mod commands {
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Connect {
            #[serde(rename = "pipe")]
            pipe: crate::protocol::generated::JsonPipe
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ConnectArgs {
            #[serde(rename = "headers")]
            headers: Option<String>,
            #[serde(rename = "slowMo")]
            slow_mo: Option<serde_json::Number>,
            #[serde(rename = "socksProxyRedirectPortForTest")]
            socks_proxy_redirect_port_for_test: Option<serde_json::Number>,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>,
            #[serde(rename = "wsEndpoint")]
            ws_endpoint: String
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ConnectOverCdp {
            #[serde(rename = "browser")]
            browser: crate::protocol::generated::Browser,
            #[serde(rename = "defaultContext")]
            default_context: Option<crate::protocol::generated::BrowserContext>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ConnectOverCdpArgs {
            #[serde(rename = "endpointURL")]
            endpoint_url: String,
            #[serde(rename = "headers")]
            headers: Option<Vec<crate::protocol::generated::NameValue>>,
            #[serde(rename = "slowMo")]
            slow_mo: Option<serde_json::Number>,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Launch {
            #[serde(rename = "browser")]
            browser: crate::protocol::generated::Browser
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct LaunchArgs {
            #[serde(flatten)]
            #[serde(rename = "$mixin")]
            mixin: crate::protocol::generated::LaunchOptions,
            #[serde(rename = "firefoxUserPrefs")]
            firefox_user_prefs: Option<String>,
            #[serde(rename = "slowMo")]
            slow_mo: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct LaunchPersistentContext {
            #[serde(rename = "context")]
            context: crate::protocol::generated::BrowserContext
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct LaunchPersistentContextArgs {
            #[serde(flatten)]
            #[serde(rename = "$mixin1")]
            mixin1: crate::protocol::generated::LaunchOptions,
            #[serde(flatten)]
            #[serde(rename = "$mixin2")]
            mixin2: crate::protocol::generated::ContextOptions,
            #[serde(rename = "slowMo")]
            slow_mo: Option<serde_json::Number>,
            #[serde(rename = "userDataDir")]
            user_data_dir: String
        }
    }
}
pub type BrowserType = Guid;
pub mod cdp_session {
    pub mod events {
        #[derive(Debug, Deserialize, Serialize)]
        pub enum Events {
            #[serde(rename = "event")]
            Event(Event)
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Event {
            #[serde(rename = "method")]
            method: String,
            #[serde(rename = "params")]
            params: Option<String>
        }
    }
    pub mod commands {
        pub type Detach = ();
        pub type DetachArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Send {
            #[serde(rename = "result")]
            result: String
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SendArgs {
            #[serde(rename = "method")]
            method: String,
            #[serde(rename = "params")]
            params: Option<String>
        }
    }
}
pub type CdpSession = Guid;
pub mod console_message {
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Initializer {
        #[serde(rename = "args")]
        args: Vec<crate::protocol::generated::JsHandle>,
        #[serde(rename = "location")]
        location: InitializerLocation,
        #[serde(rename = "text")]
        text: String,
        #[serde(rename = "type")]
        r#type: String
    }
    #[derive(Debug, Serialize, Deserialize)]
    pub struct InitializerLocation {
        #[serde(rename = "columnNumber")]
        column_number: serde_json::Number,
        #[serde(rename = "lineNumber")]
        line_number: serde_json::Number,
        #[serde(rename = "url")]
        url: String
    }
}
pub type ConsoleMessage = Guid;
pub mod dialog {
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Initializer {
        #[serde(rename = "defaultValue")]
        default_value: String,
        #[serde(rename = "message")]
        message: String,
        #[serde(rename = "type")]
        r#type: String
    }
    pub mod commands {
        pub type Accept = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct AcceptArgs {
            #[serde(rename = "promptText")]
            prompt_text: Option<String>
        }
        pub type Dismiss = ();
        pub type DismissArgs = ();
    }
}
pub type Dialog = Guid;
pub mod electron {
    pub mod commands {
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Launch {
            #[serde(rename = "electronApplication")]
            electron_application: crate::protocol::generated::ElectronApplication
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct LaunchArgs {
            #[serde(rename = "acceptDownloads")]
            accept_downloads: Option<bool>,
            #[serde(rename = "args")]
            args: Option<Vec<String>>,
            #[serde(rename = "bypassCSP")]
            bypass_csp: Option<bool>,
            #[serde(rename = "colorScheme")]
            color_scheme: Option<LaunchArgsColorScheme>,
            #[serde(rename = "cwd")]
            cwd: Option<String>,
            #[serde(rename = "env")]
            env: Option<Vec<crate::protocol::generated::NameValue>>,
            #[serde(rename = "executablePath")]
            executable_path: Option<String>,
            #[serde(rename = "extraHTTPHeaders")]
            extra_http_headers: Option<Vec<crate::protocol::generated::NameValue>>,
            #[serde(rename = "geolocation")]
            geolocation: Option<LaunchArgsGeolocation>,
            #[serde(rename = "httpCredentials")]
            http_credentials: Option<LaunchArgsHttpCredentials>,
            #[serde(rename = "ignoreHTTPSErrors")]
            ignore_https_errors: Option<bool>,
            #[serde(rename = "locale")]
            locale: Option<String>,
            #[serde(rename = "offline")]
            offline: Option<bool>,
            #[serde(rename = "recordHar")]
            record_har: Option<crate::protocol::generated::RecordHarOptions>,
            #[serde(rename = "recordVideo")]
            record_video: Option<LaunchArgsRecordVideo>,
            #[serde(rename = "strictSelectors")]
            strict_selectors: Option<bool>,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>,
            #[serde(rename = "timezoneId")]
            timezone_id: Option<String>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub enum LaunchArgsColorScheme {
            #[serde(rename = "dark")]
            Dark,
            #[serde(rename = "light")]
            Light,
            #[serde(rename = "no-preference")]
            NoPreference
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct LaunchArgsGeolocation {
            #[serde(rename = "accuracy")]
            accuracy: Option<serde_json::Number>,
            #[serde(rename = "latitude")]
            latitude: serde_json::Number,
            #[serde(rename = "longitude")]
            longitude: serde_json::Number
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct LaunchArgsHttpCredentials {
            #[serde(rename = "password")]
            password: String,
            #[serde(rename = "username")]
            username: String
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct LaunchArgsRecordVideo {
            #[serde(rename = "dir")]
            dir: String,
            #[serde(rename = "size")]
            size: Option<LaunchArgsRecordVideoSize>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct LaunchArgsRecordVideoSize {
            #[serde(rename = "height")]
            height: serde_json::Number,
            #[serde(rename = "width")]
            width: serde_json::Number
        }
    }
}
pub type Electron = Guid;
pub mod electron_application {
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Initializer {
        #[serde(rename = "context")]
        context: crate::protocol::generated::BrowserContext
    }
    pub mod events {
        #[derive(Debug, Deserialize, Serialize)]
        pub enum Events {
            #[serde(rename = "close")]
            Close
        }
    }
    pub mod commands {
        #[derive(Debug, Serialize, Deserialize)]
        pub struct BrowserWindow {
            #[serde(rename = "handle")]
            handle: crate::protocol::generated::JsHandle
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct BrowserWindowArgs {
            #[serde(rename = "page")]
            page: crate::protocol::generated::Page
        }
        pub type Close = ();
        pub type CloseArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct EvaluateExpression {
            #[serde(rename = "value")]
            value: crate::protocol::generated::SerializedValue
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct EvaluateExpressionArgs {
            #[serde(rename = "arg")]
            arg: crate::protocol::generated::SerializedArgument,
            #[serde(rename = "expression")]
            expression: String,
            #[serde(rename = "isFunction")]
            is_function: Option<bool>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct EvaluateExpressionHandle {
            #[serde(rename = "handle")]
            handle: crate::protocol::generated::JsHandle
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct EvaluateExpressionHandleArgs {
            #[serde(rename = "arg")]
            arg: crate::protocol::generated::SerializedArgument,
            #[serde(rename = "expression")]
            expression: String,
            #[serde(rename = "isFunction")]
            is_function: Option<bool>
        }
    }
}
pub type ElectronApplication = Guid;
/// Extends JSHandle
pub mod element_handle {
    pub mod commands {
        #[derive(Debug, Serialize, Deserialize)]
        pub struct BoundingBox {
            #[serde(rename = "value")]
            value: Option<crate::protocol::generated::Rect>
        }
        pub type BoundingBoxArgs = ();
        pub type Check = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct CheckArgs {
            #[serde(rename = "force")]
            force: Option<bool>,
            #[serde(rename = "noWaitAfter")]
            no_wait_after: Option<bool>,
            #[serde(rename = "position")]
            position: Option<crate::protocol::generated::Point>,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>,
            #[serde(rename = "trial")]
            trial: Option<bool>
        }
        pub type Click = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ClickArgs {
            #[serde(rename = "button")]
            button: Option<ClickArgsButton>,
            #[serde(rename = "clickCount")]
            click_count: Option<serde_json::Number>,
            #[serde(rename = "delay")]
            delay: Option<serde_json::Number>,
            #[serde(rename = "force")]
            force: Option<bool>,
            #[serde(rename = "modifiers")]
            modifiers: Option<Vec<ClickArgsModifiers>>,
            #[serde(rename = "noWaitAfter")]
            no_wait_after: Option<bool>,
            #[serde(rename = "position")]
            position: Option<crate::protocol::generated::Point>,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>,
            #[serde(rename = "trial")]
            trial: Option<bool>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub enum ClickArgsButton {
            #[serde(rename = "left")]
            Left,
            #[serde(rename = "right")]
            Right,
            #[serde(rename = "middle")]
            Middle
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub enum ClickArgsModifiers {
            Alt,
            Control,
            Meta,
            Shift
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ContentFrame {
            #[serde(rename = "frame")]
            frame: Option<crate::protocol::generated::Frame>
        }
        pub type ContentFrameArgs = ();
        pub type Dblclick = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct DblclickArgs {
            #[serde(rename = "button")]
            button: Option<DblclickArgsButton>,
            #[serde(rename = "delay")]
            delay: Option<serde_json::Number>,
            #[serde(rename = "force")]
            force: Option<bool>,
            #[serde(rename = "modifiers")]
            modifiers: Option<Vec<DblclickArgsModifiers>>,
            #[serde(rename = "noWaitAfter")]
            no_wait_after: Option<bool>,
            #[serde(rename = "position")]
            position: Option<crate::protocol::generated::Point>,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>,
            #[serde(rename = "trial")]
            trial: Option<bool>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub enum DblclickArgsButton {
            #[serde(rename = "left")]
            Left,
            #[serde(rename = "right")]
            Right,
            #[serde(rename = "middle")]
            Middle
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub enum DblclickArgsModifiers {
            Alt,
            Control,
            Meta,
            Shift
        }
        pub type DispatchEvent = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct DispatchEventArgs {
            #[serde(rename = "eventInit")]
            event_init: crate::protocol::generated::SerializedArgument,
            #[serde(rename = "type")]
            r#type: String
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct EvalOnSelector {
            #[serde(rename = "value")]
            value: crate::protocol::generated::SerializedValue
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct EvalOnSelectorArgs {
            #[serde(rename = "arg")]
            arg: crate::protocol::generated::SerializedArgument,
            #[serde(rename = "expression")]
            expression: String,
            #[serde(rename = "isFunction")]
            is_function: Option<bool>,
            #[serde(rename = "selector")]
            selector: String,
            #[serde(rename = "strict")]
            strict: Option<bool>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct EvalOnSelectorAll {
            #[serde(rename = "value")]
            value: crate::protocol::generated::SerializedValue
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct EvalOnSelectorAllArgs {
            #[serde(rename = "arg")]
            arg: crate::protocol::generated::SerializedArgument,
            #[serde(rename = "expression")]
            expression: String,
            #[serde(rename = "isFunction")]
            is_function: Option<bool>,
            #[serde(rename = "selector")]
            selector: String
        }
        pub type Fill = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct FillArgs {
            #[serde(rename = "force")]
            force: Option<bool>,
            #[serde(rename = "noWaitAfter")]
            no_wait_after: Option<bool>,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>,
            #[serde(rename = "value")]
            value: String
        }
        pub type Focus = ();
        pub type FocusArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetAttribute {
            #[serde(rename = "value")]
            value: Option<String>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetAttributeArgs {
            #[serde(rename = "name")]
            name: String
        }
        pub type Hover = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct HoverArgs {
            #[serde(rename = "force")]
            force: Option<bool>,
            #[serde(rename = "modifiers")]
            modifiers: Option<Vec<HoverArgsModifiers>>,
            #[serde(rename = "position")]
            position: Option<crate::protocol::generated::Point>,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>,
            #[serde(rename = "trial")]
            trial: Option<bool>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub enum HoverArgsModifiers {
            Alt,
            Control,
            Meta,
            Shift
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct InnerHtml {
            #[serde(rename = "value")]
            value: String
        }
        pub type InnerHtmlArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct InnerText {
            #[serde(rename = "value")]
            value: String
        }
        pub type InnerTextArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct InputValue {
            #[serde(rename = "value")]
            value: String
        }
        pub type InputValueArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct IsChecked {
            #[serde(rename = "value")]
            value: bool
        }
        pub type IsCheckedArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct IsDisabled {
            #[serde(rename = "value")]
            value: bool
        }
        pub type IsDisabledArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct IsEditable {
            #[serde(rename = "value")]
            value: bool
        }
        pub type IsEditableArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct IsEnabled {
            #[serde(rename = "value")]
            value: bool
        }
        pub type IsEnabledArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct IsHidden {
            #[serde(rename = "value")]
            value: bool
        }
        pub type IsHiddenArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct IsVisible {
            #[serde(rename = "value")]
            value: bool
        }
        pub type IsVisibleArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct OwnerFrame {
            #[serde(rename = "frame")]
            frame: Option<crate::protocol::generated::Frame>
        }
        pub type OwnerFrameArgs = ();
        pub type Press = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct PressArgs {
            #[serde(rename = "delay")]
            delay: Option<serde_json::Number>,
            #[serde(rename = "key")]
            key: String,
            #[serde(rename = "noWaitAfter")]
            no_wait_after: Option<bool>,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct QuerySelector {
            #[serde(rename = "element")]
            element: Option<crate::protocol::generated::ElementHandle>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct QuerySelectorArgs {
            #[serde(rename = "selector")]
            selector: String,
            #[serde(rename = "strict")]
            strict: Option<bool>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct QuerySelectorAll {
            #[serde(rename = "elements")]
            elements: Vec<crate::protocol::generated::ElementHandle>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct QuerySelectorAllArgs {
            #[serde(rename = "selector")]
            selector: String
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Screenshot {
            #[serde(rename = "binary")]
            binary: Vec<u8>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ScreenshotArgs {
            #[serde(flatten)]
            #[serde(rename = "$mixin")]
            mixin: crate::protocol::generated::CommonScreenshotOptions,
            #[serde(rename = "quality")]
            quality: Option<serde_json::Number>,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>,
            #[serde(rename = "type")]
            r#type: Option<ScreenshotArgsType>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub enum ScreenshotArgsType {
            #[serde(rename = "png")]
            Png,
            #[serde(rename = "jpeg")]
            Jpeg
        }
        pub type ScrollIntoViewIfNeeded = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ScrollIntoViewIfNeededArgs {
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SelectOption {
            #[serde(rename = "values")]
            values: Vec<String>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SelectOptionArgs {
            #[serde(rename = "elements")]
            elements: Option<Vec<crate::protocol::generated::ElementHandle>>,
            #[serde(rename = "force")]
            force: Option<bool>,
            #[serde(rename = "noWaitAfter")]
            no_wait_after: Option<bool>,
            #[serde(rename = "options")]
            options: Option<Vec<SelectOptionArgsOptions>>,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SelectOptionArgsOptions {
            #[serde(rename = "index")]
            index: Option<serde_json::Number>,
            #[serde(rename = "label")]
            label: Option<String>,
            #[serde(rename = "value")]
            value: Option<String>
        }
        pub type SelectText = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SelectTextArgs {
            #[serde(rename = "force")]
            force: Option<bool>,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>
        }
        pub type SetInputFilePaths = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetInputFilePathsArgs {
            #[serde(rename = "localPaths")]
            local_paths: Option<Vec<String>>,
            #[serde(rename = "noWaitAfter")]
            no_wait_after: Option<bool>,
            #[serde(rename = "streams")]
            streams: Option<Vec<crate::protocol::generated::WritableStream>>,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>
        }
        pub type SetInputFiles = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetInputFilesArgs {
            #[serde(rename = "files")]
            files: Vec<SetInputFilesArgsFiles>,
            #[serde(rename = "noWaitAfter")]
            no_wait_after: Option<bool>,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetInputFilesArgsFiles {
            #[serde(rename = "buffer")]
            buffer: Vec<u8>,
            #[serde(rename = "mimeType")]
            mime_type: Option<String>,
            #[serde(rename = "name")]
            name: String
        }
        pub type Tap = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct TapArgs {
            #[serde(rename = "force")]
            force: Option<bool>,
            #[serde(rename = "modifiers")]
            modifiers: Option<Vec<TapArgsModifiers>>,
            #[serde(rename = "noWaitAfter")]
            no_wait_after: Option<bool>,
            #[serde(rename = "position")]
            position: Option<crate::protocol::generated::Point>,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>,
            #[serde(rename = "trial")]
            trial: Option<bool>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub enum TapArgsModifiers {
            Alt,
            Control,
            Meta,
            Shift
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct TextContent {
            #[serde(rename = "value")]
            value: Option<String>
        }
        pub type TextContentArgs = ();
        pub type Type = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct TypeArgs {
            #[serde(rename = "delay")]
            delay: Option<serde_json::Number>,
            #[serde(rename = "noWaitAfter")]
            no_wait_after: Option<bool>,
            #[serde(rename = "text")]
            text: String,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>
        }
        pub type Uncheck = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct UncheckArgs {
            #[serde(rename = "force")]
            force: Option<bool>,
            #[serde(rename = "noWaitAfter")]
            no_wait_after: Option<bool>,
            #[serde(rename = "position")]
            position: Option<crate::protocol::generated::Point>,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>,
            #[serde(rename = "trial")]
            trial: Option<bool>
        }
        pub type WaitForElementState = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct WaitForElementStateArgs {
            #[serde(rename = "state")]
            state: WaitForElementStateArgsState,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub enum WaitForElementStateArgsState {
            #[serde(rename = "visible")]
            Visible,
            #[serde(rename = "hidden")]
            Hidden,
            #[serde(rename = "stable")]
            Stable,
            #[serde(rename = "enabled")]
            Enabled,
            #[serde(rename = "disabled")]
            Disabled,
            #[serde(rename = "editable")]
            Editable
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct WaitForSelector {
            #[serde(rename = "element")]
            element: Option<crate::protocol::generated::ElementHandle>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct WaitForSelectorArgs {
            #[serde(rename = "selector")]
            selector: String,
            #[serde(rename = "state")]
            state: Option<WaitForSelectorArgsState>,
            #[serde(rename = "strict")]
            strict: Option<bool>,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub enum WaitForSelectorArgsState {
            #[serde(rename = "attached")]
            Attached,
            #[serde(rename = "detached")]
            Detached,
            #[serde(rename = "visible")]
            Visible,
            #[serde(rename = "hidden")]
            Hidden
        }
    }
}
pub type ElementHandle = Guid;
pub mod event_target {
    pub mod commands {
        pub type WaitForEventInfo = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct WaitForEventInfoArgs {
            #[serde(rename = "info")]
            info: WaitForEventInfoArgsInfo
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct WaitForEventInfoArgsInfo {
            #[serde(rename = "error")]
            error: Option<String>,
            #[serde(rename = "event")]
            event: Option<String>,
            #[serde(rename = "message")]
            message: Option<String>,
            #[serde(rename = "phase")]
            phase: WaitForEventInfoArgsInfoPhase,
            #[serde(rename = "waitId")]
            wait_id: String
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub enum WaitForEventInfoArgsInfoPhase {
            #[serde(rename = "before")]
            Before,
            #[serde(rename = "after")]
            After,
            #[serde(rename = "log")]
            Log
        }
    }
}
pub type EventTarget = Guid;
pub mod frame {
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Initializer {
        #[serde(rename = "loadStates")]
        load_states: Vec<crate::protocol::generated::LifecycleEvent>,
        #[serde(rename = "name")]
        name: String,
        #[serde(rename = "parentFrame")]
        parent_frame: Option<crate::protocol::generated::Frame>,
        #[serde(rename = "url")]
        url: String
    }
    pub mod events {
        #[derive(Debug, Deserialize, Serialize)]
        pub enum Events {
            #[serde(rename = "loadstate")]
            Loadstate(Loadstate),
            #[serde(rename = "navigated")]
            Navigated(Navigated)
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Loadstate {
            #[serde(rename = "add")]
            add: Option<crate::protocol::generated::LifecycleEvent>,
            #[serde(rename = "remove")]
            remove: Option<crate::protocol::generated::LifecycleEvent>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Navigated {
            #[serde(rename = "error")]
            error: Option<String>,
            #[serde(rename = "name")]
            name: String,
            #[serde(rename = "newDocument")]
            new_document: Option<NavigatedNewDocument>,
            #[serde(rename = "url")]
            url: String
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct NavigatedNewDocument {
            #[serde(rename = "request")]
            request: Option<crate::protocol::generated::Request>
        }
    }
    pub mod commands {
        #[derive(Debug, Serialize, Deserialize)]
        pub struct AddScriptTag {
            #[serde(rename = "element")]
            element: crate::protocol::generated::ElementHandle
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct AddScriptTagArgs {
            #[serde(rename = "content")]
            content: Option<String>,
            #[serde(rename = "type")]
            r#type: Option<String>,
            #[serde(rename = "url")]
            url: Option<String>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct AddStyleTag {
            #[serde(rename = "element")]
            element: crate::protocol::generated::ElementHandle
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct AddStyleTagArgs {
            #[serde(rename = "content")]
            content: Option<String>,
            #[serde(rename = "url")]
            url: Option<String>
        }
        pub type Check = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct CheckArgs {
            #[serde(rename = "force")]
            force: Option<bool>,
            #[serde(rename = "noWaitAfter")]
            no_wait_after: Option<bool>,
            #[serde(rename = "position")]
            position: Option<crate::protocol::generated::Point>,
            #[serde(rename = "selector")]
            selector: String,
            #[serde(rename = "strict")]
            strict: Option<bool>,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>,
            #[serde(rename = "trial")]
            trial: Option<bool>
        }
        pub type Click = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ClickArgs {
            #[serde(rename = "button")]
            button: Option<ClickArgsButton>,
            #[serde(rename = "clickCount")]
            click_count: Option<serde_json::Number>,
            #[serde(rename = "delay")]
            delay: Option<serde_json::Number>,
            #[serde(rename = "force")]
            force: Option<bool>,
            #[serde(rename = "modifiers")]
            modifiers: Option<Vec<ClickArgsModifiers>>,
            #[serde(rename = "noWaitAfter")]
            no_wait_after: Option<bool>,
            #[serde(rename = "position")]
            position: Option<crate::protocol::generated::Point>,
            #[serde(rename = "selector")]
            selector: String,
            #[serde(rename = "strict")]
            strict: Option<bool>,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>,
            #[serde(rename = "trial")]
            trial: Option<bool>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub enum ClickArgsButton {
            #[serde(rename = "left")]
            Left,
            #[serde(rename = "right")]
            Right,
            #[serde(rename = "middle")]
            Middle
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub enum ClickArgsModifiers {
            Alt,
            Control,
            Meta,
            Shift
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Content {
            #[serde(rename = "value")]
            value: String
        }
        pub type ContentArgs = ();
        pub type Dblclick = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct DblclickArgs {
            #[serde(rename = "button")]
            button: Option<DblclickArgsButton>,
            #[serde(rename = "delay")]
            delay: Option<serde_json::Number>,
            #[serde(rename = "force")]
            force: Option<bool>,
            #[serde(rename = "modifiers")]
            modifiers: Option<Vec<DblclickArgsModifiers>>,
            #[serde(rename = "noWaitAfter")]
            no_wait_after: Option<bool>,
            #[serde(rename = "position")]
            position: Option<crate::protocol::generated::Point>,
            #[serde(rename = "selector")]
            selector: String,
            #[serde(rename = "strict")]
            strict: Option<bool>,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>,
            #[serde(rename = "trial")]
            trial: Option<bool>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub enum DblclickArgsButton {
            #[serde(rename = "left")]
            Left,
            #[serde(rename = "right")]
            Right,
            #[serde(rename = "middle")]
            Middle
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub enum DblclickArgsModifiers {
            Alt,
            Control,
            Meta,
            Shift
        }
        pub type DispatchEvent = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct DispatchEventArgs {
            #[serde(rename = "eventInit")]
            event_init: crate::protocol::generated::SerializedArgument,
            #[serde(rename = "selector")]
            selector: String,
            #[serde(rename = "strict")]
            strict: Option<bool>,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>,
            #[serde(rename = "type")]
            r#type: String
        }
        pub type DragAndDrop = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct DragAndDropArgs {
            #[serde(rename = "force")]
            force: Option<bool>,
            #[serde(rename = "noWaitAfter")]
            no_wait_after: Option<bool>,
            #[serde(rename = "source")]
            source: String,
            #[serde(rename = "sourcePosition")]
            source_position: Option<crate::protocol::generated::Point>,
            #[serde(rename = "strict")]
            strict: Option<bool>,
            #[serde(rename = "target")]
            target: String,
            #[serde(rename = "targetPosition")]
            target_position: Option<crate::protocol::generated::Point>,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>,
            #[serde(rename = "trial")]
            trial: Option<bool>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct EvalOnSelector {
            #[serde(rename = "value")]
            value: crate::protocol::generated::SerializedValue
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct EvalOnSelectorArgs {
            #[serde(rename = "arg")]
            arg: crate::protocol::generated::SerializedArgument,
            #[serde(rename = "expression")]
            expression: String,
            #[serde(rename = "isFunction")]
            is_function: Option<bool>,
            #[serde(rename = "selector")]
            selector: String,
            #[serde(rename = "strict")]
            strict: Option<bool>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct EvalOnSelectorAll {
            #[serde(rename = "value")]
            value: crate::protocol::generated::SerializedValue
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct EvalOnSelectorAllArgs {
            #[serde(rename = "arg")]
            arg: crate::protocol::generated::SerializedArgument,
            #[serde(rename = "expression")]
            expression: String,
            #[serde(rename = "isFunction")]
            is_function: Option<bool>,
            #[serde(rename = "selector")]
            selector: String
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct EvaluateExpression {
            #[serde(rename = "value")]
            value: crate::protocol::generated::SerializedValue
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct EvaluateExpressionArgs {
            #[serde(rename = "arg")]
            arg: crate::protocol::generated::SerializedArgument,
            #[serde(rename = "expression")]
            expression: String,
            #[serde(rename = "isFunction")]
            is_function: Option<bool>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct EvaluateExpressionHandle {
            #[serde(rename = "handle")]
            handle: crate::protocol::generated::JsHandle
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct EvaluateExpressionHandleArgs {
            #[serde(rename = "arg")]
            arg: crate::protocol::generated::SerializedArgument,
            #[serde(rename = "expression")]
            expression: String,
            #[serde(rename = "isFunction")]
            is_function: Option<bool>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Expect {
            #[serde(rename = "log")]
            log: Option<Vec<String>>,
            #[serde(rename = "matches")]
            matches: bool,
            #[serde(rename = "received")]
            received: Option<crate::protocol::generated::SerializedValue>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ExpectArgs {
            #[serde(rename = "expectedNumber")]
            expected_number: Option<serde_json::Number>,
            #[serde(rename = "expectedText")]
            expected_text: Option<Vec<crate::protocol::generated::ExpectedTextValue>>,
            #[serde(rename = "expectedValue")]
            expected_value: Option<crate::protocol::generated::SerializedArgument>,
            #[serde(rename = "expression")]
            expression: String,
            #[serde(rename = "expressionArg")]
            expression_arg: Option<String>,
            #[serde(rename = "isNot")]
            is_not: bool,
            #[serde(rename = "selector")]
            selector: String,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>,
            #[serde(rename = "useInnerText")]
            use_inner_text: Option<bool>
        }
        pub type Fill = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct FillArgs {
            #[serde(rename = "force")]
            force: Option<bool>,
            #[serde(rename = "noWaitAfter")]
            no_wait_after: Option<bool>,
            #[serde(rename = "selector")]
            selector: String,
            #[serde(rename = "strict")]
            strict: Option<bool>,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>,
            #[serde(rename = "value")]
            value: String
        }
        pub type Focus = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct FocusArgs {
            #[serde(rename = "selector")]
            selector: String,
            #[serde(rename = "strict")]
            strict: Option<bool>,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct FrameElement {
            #[serde(rename = "element")]
            element: crate::protocol::generated::ElementHandle
        }
        pub type FrameElementArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetAttribute {
            #[serde(rename = "value")]
            value: Option<String>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetAttributeArgs {
            #[serde(rename = "name")]
            name: String,
            #[serde(rename = "selector")]
            selector: String,
            #[serde(rename = "strict")]
            strict: Option<bool>,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Goto {
            #[serde(rename = "response")]
            response: Option<crate::protocol::generated::Response>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct GotoArgs {
            #[serde(rename = "referer")]
            referer: Option<String>,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>,
            #[serde(rename = "url")]
            url: String,
            #[serde(rename = "waitUntil")]
            wait_until: Option<crate::protocol::generated::LifecycleEvent>
        }
        pub type Highlight = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct HighlightArgs {
            #[serde(rename = "selector")]
            selector: String
        }
        pub type Hover = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct HoverArgs {
            #[serde(rename = "force")]
            force: Option<bool>,
            #[serde(rename = "modifiers")]
            modifiers: Option<Vec<HoverArgsModifiers>>,
            #[serde(rename = "position")]
            position: Option<crate::protocol::generated::Point>,
            #[serde(rename = "selector")]
            selector: String,
            #[serde(rename = "strict")]
            strict: Option<bool>,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>,
            #[serde(rename = "trial")]
            trial: Option<bool>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub enum HoverArgsModifiers {
            Alt,
            Control,
            Meta,
            Shift
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct InnerHtml {
            #[serde(rename = "value")]
            value: String
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct InnerHtmlArgs {
            #[serde(rename = "selector")]
            selector: String,
            #[serde(rename = "strict")]
            strict: Option<bool>,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct InnerText {
            #[serde(rename = "value")]
            value: String
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct InnerTextArgs {
            #[serde(rename = "selector")]
            selector: String,
            #[serde(rename = "strict")]
            strict: Option<bool>,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct InputValue {
            #[serde(rename = "value")]
            value: String
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct InputValueArgs {
            #[serde(rename = "selector")]
            selector: String,
            #[serde(rename = "strict")]
            strict: Option<bool>,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct IsChecked {
            #[serde(rename = "value")]
            value: bool
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct IsCheckedArgs {
            #[serde(rename = "selector")]
            selector: String,
            #[serde(rename = "strict")]
            strict: Option<bool>,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct IsDisabled {
            #[serde(rename = "value")]
            value: bool
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct IsDisabledArgs {
            #[serde(rename = "selector")]
            selector: String,
            #[serde(rename = "strict")]
            strict: Option<bool>,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct IsEditable {
            #[serde(rename = "value")]
            value: bool
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct IsEditableArgs {
            #[serde(rename = "selector")]
            selector: String,
            #[serde(rename = "strict")]
            strict: Option<bool>,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct IsEnabled {
            #[serde(rename = "value")]
            value: bool
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct IsEnabledArgs {
            #[serde(rename = "selector")]
            selector: String,
            #[serde(rename = "strict")]
            strict: Option<bool>,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct IsHidden {
            #[serde(rename = "value")]
            value: bool
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct IsHiddenArgs {
            #[serde(rename = "selector")]
            selector: String,
            #[serde(rename = "strict")]
            strict: Option<bool>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct IsVisible {
            #[serde(rename = "value")]
            value: bool
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct IsVisibleArgs {
            #[serde(rename = "selector")]
            selector: String,
            #[serde(rename = "strict")]
            strict: Option<bool>
        }
        pub type Press = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct PressArgs {
            #[serde(rename = "delay")]
            delay: Option<serde_json::Number>,
            #[serde(rename = "key")]
            key: String,
            #[serde(rename = "noWaitAfter")]
            no_wait_after: Option<bool>,
            #[serde(rename = "selector")]
            selector: String,
            #[serde(rename = "strict")]
            strict: Option<bool>,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct QueryCount {
            #[serde(rename = "value")]
            value: serde_json::Number
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct QueryCountArgs {
            #[serde(rename = "selector")]
            selector: String
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct QuerySelector {
            #[serde(rename = "element")]
            element: Option<crate::protocol::generated::ElementHandle>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct QuerySelectorArgs {
            #[serde(rename = "selector")]
            selector: String,
            #[serde(rename = "strict")]
            strict: Option<bool>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct QuerySelectorAll {
            #[serde(rename = "elements")]
            elements: Vec<crate::protocol::generated::ElementHandle>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct QuerySelectorAllArgs {
            #[serde(rename = "selector")]
            selector: String
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SelectOption {
            #[serde(rename = "values")]
            values: Vec<String>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SelectOptionArgs {
            #[serde(rename = "elements")]
            elements: Option<Vec<crate::protocol::generated::ElementHandle>>,
            #[serde(rename = "force")]
            force: Option<bool>,
            #[serde(rename = "noWaitAfter")]
            no_wait_after: Option<bool>,
            #[serde(rename = "options")]
            options: Option<Vec<SelectOptionArgsOptions>>,
            #[serde(rename = "selector")]
            selector: String,
            #[serde(rename = "strict")]
            strict: Option<bool>,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SelectOptionArgsOptions {
            #[serde(rename = "index")]
            index: Option<serde_json::Number>,
            #[serde(rename = "label")]
            label: Option<String>,
            #[serde(rename = "value")]
            value: Option<String>
        }
        pub type SetContent = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetContentArgs {
            #[serde(rename = "html")]
            html: String,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>,
            #[serde(rename = "waitUntil")]
            wait_until: Option<crate::protocol::generated::LifecycleEvent>
        }
        pub type SetInputFilePaths = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetInputFilePathsArgs {
            #[serde(rename = "localPaths")]
            local_paths: Option<Vec<String>>,
            #[serde(rename = "noWaitAfter")]
            no_wait_after: Option<bool>,
            #[serde(rename = "selector")]
            selector: String,
            #[serde(rename = "streams")]
            streams: Option<Vec<crate::protocol::generated::WritableStream>>,
            #[serde(rename = "strict")]
            strict: Option<bool>,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>
        }
        pub type SetInputFiles = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetInputFilesArgs {
            #[serde(rename = "files")]
            files: Vec<SetInputFilesArgsFiles>,
            #[serde(rename = "noWaitAfter")]
            no_wait_after: Option<bool>,
            #[serde(rename = "selector")]
            selector: String,
            #[serde(rename = "strict")]
            strict: Option<bool>,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetInputFilesArgsFiles {
            #[serde(rename = "buffer")]
            buffer: Vec<u8>,
            #[serde(rename = "mimeType")]
            mime_type: Option<String>,
            #[serde(rename = "name")]
            name: String
        }
        pub type Tap = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct TapArgs {
            #[serde(rename = "force")]
            force: Option<bool>,
            #[serde(rename = "modifiers")]
            modifiers: Option<Vec<TapArgsModifiers>>,
            #[serde(rename = "noWaitAfter")]
            no_wait_after: Option<bool>,
            #[serde(rename = "position")]
            position: Option<crate::protocol::generated::Point>,
            #[serde(rename = "selector")]
            selector: String,
            #[serde(rename = "strict")]
            strict: Option<bool>,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>,
            #[serde(rename = "trial")]
            trial: Option<bool>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub enum TapArgsModifiers {
            Alt,
            Control,
            Meta,
            Shift
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct TextContent {
            #[serde(rename = "value")]
            value: Option<String>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct TextContentArgs {
            #[serde(rename = "selector")]
            selector: String,
            #[serde(rename = "strict")]
            strict: Option<bool>,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Title {
            #[serde(rename = "value")]
            value: String
        }
        pub type TitleArgs = ();
        pub type Type = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct TypeArgs {
            #[serde(rename = "delay")]
            delay: Option<serde_json::Number>,
            #[serde(rename = "noWaitAfter")]
            no_wait_after: Option<bool>,
            #[serde(rename = "selector")]
            selector: String,
            #[serde(rename = "strict")]
            strict: Option<bool>,
            #[serde(rename = "text")]
            text: String,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>
        }
        pub type Uncheck = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct UncheckArgs {
            #[serde(rename = "force")]
            force: Option<bool>,
            #[serde(rename = "noWaitAfter")]
            no_wait_after: Option<bool>,
            #[serde(rename = "position")]
            position: Option<crate::protocol::generated::Point>,
            #[serde(rename = "selector")]
            selector: String,
            #[serde(rename = "strict")]
            strict: Option<bool>,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>,
            #[serde(rename = "trial")]
            trial: Option<bool>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct WaitForFunction {
            #[serde(rename = "handle")]
            handle: crate::protocol::generated::JsHandle
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct WaitForFunctionArgs {
            #[serde(rename = "arg")]
            arg: crate::protocol::generated::SerializedArgument,
            #[serde(rename = "expression")]
            expression: String,
            #[serde(rename = "isFunction")]
            is_function: Option<bool>,
            #[serde(rename = "pollingInterval")]
            polling_interval: Option<serde_json::Number>,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct WaitForSelector {
            #[serde(rename = "element")]
            element: Option<crate::protocol::generated::ElementHandle>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct WaitForSelectorArgs {
            #[serde(rename = "omitReturnValue")]
            omit_return_value: Option<bool>,
            #[serde(rename = "selector")]
            selector: String,
            #[serde(rename = "state")]
            state: Option<WaitForSelectorArgsState>,
            #[serde(rename = "strict")]
            strict: Option<bool>,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub enum WaitForSelectorArgsState {
            #[serde(rename = "attached")]
            Attached,
            #[serde(rename = "detached")]
            Detached,
            #[serde(rename = "visible")]
            Visible,
            #[serde(rename = "hidden")]
            Hidden
        }
        pub type WaitForTimeout = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct WaitForTimeoutArgs {
            #[serde(rename = "timeout")]
            timeout: serde_json::Number
        }
    }
}
pub type Frame = Guid;
pub mod js_handle {
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Initializer {
        #[serde(rename = "preview")]
        preview: String
    }
    pub mod events {
        #[derive(Debug, Deserialize, Serialize)]
        pub enum Events {
            #[serde(rename = "previewUpdated")]
            PreviewUpdated(PreviewUpdated)
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct PreviewUpdated {
            #[serde(rename = "preview")]
            preview: String
        }
    }
    pub mod commands {
        pub type Dispose = ();
        pub type DisposeArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct EvaluateExpression {
            #[serde(rename = "value")]
            value: crate::protocol::generated::SerializedValue
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct EvaluateExpressionArgs {
            #[serde(rename = "arg")]
            arg: crate::protocol::generated::SerializedArgument,
            #[serde(rename = "expression")]
            expression: String,
            #[serde(rename = "isFunction")]
            is_function: Option<bool>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct EvaluateExpressionHandle {
            #[serde(rename = "handle")]
            handle: crate::protocol::generated::JsHandle
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct EvaluateExpressionHandleArgs {
            #[serde(rename = "arg")]
            arg: crate::protocol::generated::SerializedArgument,
            #[serde(rename = "expression")]
            expression: String,
            #[serde(rename = "isFunction")]
            is_function: Option<bool>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetProperty {
            #[serde(rename = "handle")]
            handle: crate::protocol::generated::JsHandle
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetPropertyArgs {
            #[serde(rename = "name")]
            name: String
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetPropertyList {
            #[serde(rename = "properties")]
            properties: Vec<GetPropertyListProperties>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetPropertyListProperties {
            #[serde(rename = "name")]
            name: String,
            #[serde(rename = "value")]
            value: crate::protocol::generated::JsHandle
        }
        pub type GetPropertyListArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct JsonValue {
            #[serde(rename = "value")]
            value: crate::protocol::generated::SerializedValue
        }
        pub type JsonValueArgs = ();
    }
}
pub type JsHandle = Guid;
pub mod json_pipe {
    pub mod events {
        #[derive(Debug, Deserialize, Serialize)]
        pub enum Events {
            #[serde(rename = "closed")]
            Closed(Closed),
            #[serde(rename = "message")]
            Message(Message)
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Closed {
            #[serde(rename = "error")]
            error: Option<crate::protocol::generated::SerializedError>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Message {
            #[serde(rename = "message")]
            message: String
        }
    }
    pub mod commands {
        pub type Close = ();
        pub type CloseArgs = ();
        pub type Send = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SendArgs {
            #[serde(rename = "message")]
            message: String
        }
    }
}
pub type JsonPipe = Guid;
pub mod local_utils {
    pub mod commands {
        pub type HarClose = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct HarCloseArgs {
            #[serde(rename = "harId")]
            har_id: String
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct HarLookup {
            #[serde(rename = "action")]
            action: HarLookupAction,
            #[serde(rename = "body")]
            body: Option<Vec<u8>>,
            #[serde(rename = "headers")]
            headers: Option<Vec<crate::protocol::generated::NameValue>>,
            #[serde(rename = "message")]
            message: Option<String>,
            #[serde(rename = "redirectURL")]
            redirect_url: Option<String>,
            #[serde(rename = "status")]
            status: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub enum HarLookupAction {
            #[serde(rename = "error")]
            Error,
            #[serde(rename = "redirect")]
            Redirect,
            #[serde(rename = "fulfill")]
            Fulfill,
            #[serde(rename = "noentry")]
            Noentry
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct HarLookupArgs {
            #[serde(rename = "harId")]
            har_id: String,
            #[serde(rename = "headers")]
            headers: Vec<crate::protocol::generated::NameValue>,
            #[serde(rename = "isNavigationRequest")]
            is_navigation_request: bool,
            #[serde(rename = "method")]
            method: String,
            #[serde(rename = "postData")]
            post_data: Option<Vec<u8>>,
            #[serde(rename = "url")]
            url: String
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct HarOpen {
            #[serde(rename = "error")]
            error: Option<String>,
            #[serde(rename = "harId")]
            har_id: Option<String>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct HarOpenArgs {
            #[serde(rename = "file")]
            file: String
        }
        pub type HarUnzip = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct HarUnzipArgs {
            #[serde(rename = "harFile")]
            har_file: String,
            #[serde(rename = "zipFile")]
            zip_file: String
        }
        pub type Zip = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ZipArgs {
            #[serde(rename = "entries")]
            entries: Vec<crate::protocol::generated::NameValue>,
            #[serde(rename = "zipFile")]
            zip_file: String
        }
    }
}
pub type LocalUtils = Guid;
pub mod page {
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Initializer {
        #[serde(rename = "isClosed")]
        is_closed: bool,
        #[serde(rename = "mainFrame")]
        main_frame: crate::protocol::generated::Frame,
        #[serde(rename = "opener")]
        opener: Option<crate::protocol::generated::Page>,
        #[serde(rename = "viewportSize")]
        viewport_size: Option<InitializerViewportSize>
    }
    #[derive(Debug, Serialize, Deserialize)]
    pub struct InitializerViewportSize {
        #[serde(rename = "height")]
        height: serde_json::Number,
        #[serde(rename = "width")]
        width: serde_json::Number
    }
    pub mod events {
        #[derive(Debug, Deserialize, Serialize)]
        pub enum Events {
            #[serde(rename = "bindingCall")]
            BindingCall(BindingCall),
            #[serde(rename = "close")]
            Close,
            #[serde(rename = "console")]
            Console(Console),
            #[serde(rename = "crash")]
            Crash,
            #[serde(rename = "dialog")]
            Dialog(Dialog),
            #[serde(rename = "download")]
            Download(Download),
            #[serde(rename = "fileChooser")]
            FileChooser(FileChooser),
            #[serde(rename = "frameAttached")]
            FrameAttached(FrameAttached),
            #[serde(rename = "frameDetached")]
            FrameDetached(FrameDetached),
            #[serde(rename = "pageError")]
            PageError(PageError),
            #[serde(rename = "route")]
            Route(Route),
            #[serde(rename = "video")]
            Video(Video),
            #[serde(rename = "webSocket")]
            WebSocket(WebSocket),
            #[serde(rename = "worker")]
            Worker(Worker)
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct BindingCall {
            #[serde(rename = "binding")]
            binding: crate::protocol::generated::BindingCall
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Console {
            #[serde(rename = "message")]
            message: crate::protocol::generated::ConsoleMessage
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Dialog {
            #[serde(rename = "dialog")]
            dialog: crate::protocol::generated::Dialog
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Download {
            #[serde(rename = "artifact")]
            artifact: crate::protocol::generated::Artifact,
            #[serde(rename = "suggestedFilename")]
            suggested_filename: String,
            #[serde(rename = "url")]
            url: String
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct FileChooser {
            #[serde(rename = "element")]
            element: crate::protocol::generated::ElementHandle,
            #[serde(rename = "isMultiple")]
            is_multiple: bool
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct FrameAttached {
            #[serde(rename = "frame")]
            frame: crate::protocol::generated::Frame
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct FrameDetached {
            #[serde(rename = "frame")]
            frame: crate::protocol::generated::Frame
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct PageError {
            #[serde(rename = "error")]
            error: crate::protocol::generated::SerializedError
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Route {
            #[serde(rename = "request")]
            request: crate::protocol::generated::Request,
            #[serde(rename = "route")]
            route: crate::protocol::generated::Route
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Video {
            #[serde(rename = "artifact")]
            artifact: crate::protocol::generated::Artifact
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct WebSocket {
            #[serde(rename = "webSocket")]
            web_socket: crate::protocol::generated::WebSocket
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Worker {
            #[serde(rename = "worker")]
            worker: crate::protocol::generated::Worker
        }
    }
    pub mod commands {
        #[derive(Debug, Serialize, Deserialize)]
        pub struct AccessibilitySnapshot {
            #[serde(rename = "rootAXNode")]
            root_ax_node: Option<crate::protocol::generated::AxNode>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct AccessibilitySnapshotArgs {
            #[serde(rename = "interestingOnly")]
            interesting_only: Option<bool>,
            #[serde(rename = "root")]
            root: Option<crate::protocol::generated::ElementHandle>
        }
        pub type AddInitScript = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct AddInitScriptArgs {
            #[serde(rename = "source")]
            source: String
        }
        pub type BringToFront = ();
        pub type BringToFrontArgs = ();
        pub type Close = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct CloseArgs {
            #[serde(rename = "runBeforeUnload")]
            run_before_unload: Option<bool>
        }
        pub type EmulateMedia = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct EmulateMediaArgs {
            #[serde(rename = "colorScheme")]
            color_scheme: Option<EmulateMediaArgsColorScheme>,
            #[serde(rename = "forcedColors")]
            forced_colors: Option<EmulateMediaArgsForcedColors>,
            #[serde(rename = "media")]
            media: Option<EmulateMediaArgsMedia>,
            #[serde(rename = "reducedMotion")]
            reduced_motion: Option<EmulateMediaArgsReducedMotion>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub enum EmulateMediaArgsColorScheme {
            #[serde(rename = "dark")]
            Dark,
            #[serde(rename = "light")]
            Light,
            #[serde(rename = "no-preference")]
            NoPreference,
            #[serde(rename = "null")]
            Null
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub enum EmulateMediaArgsForcedColors {
            #[serde(rename = "active")]
            Active,
            #[serde(rename = "none")]
            None,
            #[serde(rename = "null")]
            Null
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub enum EmulateMediaArgsMedia {
            #[serde(rename = "screen")]
            Screen,
            #[serde(rename = "print")]
            Print,
            #[serde(rename = "null")]
            Null
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub enum EmulateMediaArgsReducedMotion {
            #[serde(rename = "reduce")]
            Reduce,
            #[serde(rename = "no-preference")]
            NoPreference,
            #[serde(rename = "null")]
            Null
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ExpectScreenshot {
            #[serde(rename = "actual")]
            actual: Option<Vec<u8>>,
            #[serde(rename = "diff")]
            diff: Option<Vec<u8>>,
            #[serde(rename = "errorMessage")]
            error_message: Option<String>,
            #[serde(rename = "log")]
            log: Option<Vec<String>>,
            #[serde(rename = "previous")]
            previous: Option<Vec<u8>>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ExpectScreenshotArgs {
            #[serde(rename = "comparatorOptions")]
            comparator_options: Option<ExpectScreenshotArgsComparatorOptions>,
            #[serde(rename = "expected")]
            expected: Option<Vec<u8>>,
            #[serde(rename = "isNot")]
            is_not: bool,
            #[serde(rename = "locator")]
            locator: Option<ExpectScreenshotArgsLocator>,
            #[serde(rename = "screenshotOptions")]
            screenshot_options: Option<ExpectScreenshotArgsScreenshotOptions>,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ExpectScreenshotArgsComparatorOptions {
            #[serde(rename = "maxDiffPixelRatio")]
            max_diff_pixel_ratio: Option<serde_json::Number>,
            #[serde(rename = "maxDiffPixels")]
            max_diff_pixels: Option<serde_json::Number>,
            #[serde(rename = "threshold")]
            threshold: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ExpectScreenshotArgsLocator {
            #[serde(rename = "frame")]
            frame: crate::protocol::generated::Frame,
            #[serde(rename = "selector")]
            selector: String
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ExpectScreenshotArgsScreenshotOptions {
            #[serde(flatten)]
            #[serde(rename = "$mixin")]
            mixin: crate::protocol::generated::CommonScreenshotOptions,
            #[serde(rename = "clip")]
            clip: Option<crate::protocol::generated::Rect>,
            #[serde(rename = "fullPage")]
            full_page: Option<bool>
        }
        pub type ExposeBinding = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ExposeBindingArgs {
            #[serde(rename = "name")]
            name: String,
            #[serde(rename = "needsHandle")]
            needs_handle: Option<bool>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct GoBack {
            #[serde(rename = "response")]
            response: Option<crate::protocol::generated::Response>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct GoBackArgs {
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>,
            #[serde(rename = "waitUntil")]
            wait_until: Option<crate::protocol::generated::LifecycleEvent>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct GoForward {
            #[serde(rename = "response")]
            response: Option<crate::protocol::generated::Response>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct GoForwardArgs {
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>,
            #[serde(rename = "waitUntil")]
            wait_until: Option<crate::protocol::generated::LifecycleEvent>
        }
        pub type KeyboardDown = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct KeyboardDownArgs {
            #[serde(rename = "key")]
            key: String
        }
        pub type KeyboardInsertText = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct KeyboardInsertTextArgs {
            #[serde(rename = "text")]
            text: String
        }
        pub type KeyboardPress = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct KeyboardPressArgs {
            #[serde(rename = "delay")]
            delay: Option<serde_json::Number>,
            #[serde(rename = "key")]
            key: String
        }
        pub type KeyboardType = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct KeyboardTypeArgs {
            #[serde(rename = "delay")]
            delay: Option<serde_json::Number>,
            #[serde(rename = "text")]
            text: String
        }
        pub type KeyboardUp = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct KeyboardUpArgs {
            #[serde(rename = "key")]
            key: String
        }
        pub type MouseClick = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct MouseClickArgs {
            #[serde(rename = "button")]
            button: Option<MouseClickArgsButton>,
            #[serde(rename = "clickCount")]
            click_count: Option<serde_json::Number>,
            #[serde(rename = "delay")]
            delay: Option<serde_json::Number>,
            #[serde(rename = "x")]
            x: serde_json::Number,
            #[serde(rename = "y")]
            y: serde_json::Number
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub enum MouseClickArgsButton {
            #[serde(rename = "left")]
            Left,
            #[serde(rename = "right")]
            Right,
            #[serde(rename = "middle")]
            Middle
        }
        pub type MouseDown = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct MouseDownArgs {
            #[serde(rename = "button")]
            button: Option<MouseDownArgsButton>,
            #[serde(rename = "clickCount")]
            click_count: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub enum MouseDownArgsButton {
            #[serde(rename = "left")]
            Left,
            #[serde(rename = "right")]
            Right,
            #[serde(rename = "middle")]
            Middle
        }
        pub type MouseMove = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct MouseMoveArgs {
            #[serde(rename = "steps")]
            steps: Option<serde_json::Number>,
            #[serde(rename = "x")]
            x: serde_json::Number,
            #[serde(rename = "y")]
            y: serde_json::Number
        }
        pub type MouseUp = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct MouseUpArgs {
            #[serde(rename = "button")]
            button: Option<MouseUpArgsButton>,
            #[serde(rename = "clickCount")]
            click_count: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub enum MouseUpArgsButton {
            #[serde(rename = "left")]
            Left,
            #[serde(rename = "right")]
            Right,
            #[serde(rename = "middle")]
            Middle
        }
        pub type MouseWheel = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct MouseWheelArgs {
            #[serde(rename = "deltaX")]
            delta_x: serde_json::Number,
            #[serde(rename = "deltaY")]
            delta_y: serde_json::Number
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Pdf {
            #[serde(rename = "pdf")]
            pdf: Vec<u8>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct PdfArgs {
            #[serde(rename = "displayHeaderFooter")]
            display_header_footer: Option<bool>,
            #[serde(rename = "footerTemplate")]
            footer_template: Option<String>,
            #[serde(rename = "format")]
            format: Option<String>,
            #[serde(rename = "headerTemplate")]
            header_template: Option<String>,
            #[serde(rename = "height")]
            height: Option<String>,
            #[serde(rename = "landscape")]
            landscape: Option<bool>,
            #[serde(rename = "margin")]
            margin: Option<PdfArgsMargin>,
            #[serde(rename = "pageRanges")]
            page_ranges: Option<String>,
            #[serde(rename = "preferCSSPageSize")]
            prefer_css_page_size: Option<bool>,
            #[serde(rename = "printBackground")]
            print_background: Option<bool>,
            #[serde(rename = "scale")]
            scale: Option<serde_json::Number>,
            #[serde(rename = "width")]
            width: Option<String>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct PdfArgsMargin {
            #[serde(rename = "bottom")]
            bottom: Option<String>,
            #[serde(rename = "left")]
            left: Option<String>,
            #[serde(rename = "right")]
            right: Option<String>,
            #[serde(rename = "top")]
            top: Option<String>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Reload {
            #[serde(rename = "response")]
            response: Option<crate::protocol::generated::Response>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ReloadArgs {
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>,
            #[serde(rename = "waitUntil")]
            wait_until: Option<crate::protocol::generated::LifecycleEvent>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Screenshot {
            #[serde(rename = "binary")]
            binary: Vec<u8>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ScreenshotArgs {
            #[serde(flatten)]
            #[serde(rename = "$mixin")]
            mixin: crate::protocol::generated::CommonScreenshotOptions,
            #[serde(rename = "clip")]
            clip: Option<crate::protocol::generated::Rect>,
            #[serde(rename = "fullPage")]
            full_page: Option<bool>,
            #[serde(rename = "quality")]
            quality: Option<serde_json::Number>,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>,
            #[serde(rename = "type")]
            r#type: Option<ScreenshotArgsType>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub enum ScreenshotArgsType {
            #[serde(rename = "png")]
            Png,
            #[serde(rename = "jpeg")]
            Jpeg
        }
        pub type SetDefaultNavigationTimeoutNoReply = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetDefaultNavigationTimeoutNoReplyArgs {
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>
        }
        pub type SetDefaultTimeoutNoReply = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetDefaultTimeoutNoReplyArgs {
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>
        }
        pub type SetExtraHttpHeaders = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetExtraHttpHeadersArgs {
            #[serde(rename = "headers")]
            headers: Vec<crate::protocol::generated::NameValue>
        }
        pub type SetFileChooserInterceptedNoReply = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetFileChooserInterceptedNoReplyArgs {
            #[serde(rename = "intercepted")]
            intercepted: bool
        }
        pub type SetNetworkInterceptionEnabled = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetNetworkInterceptionEnabledArgs {
            #[serde(rename = "enabled")]
            enabled: bool
        }
        pub type SetViewportSize = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetViewportSizeArgs {
            #[serde(rename = "viewportSize")]
            viewport_size: SetViewportSizeArgsViewportSize
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetViewportSizeArgsViewportSize {
            #[serde(rename = "height")]
            height: serde_json::Number,
            #[serde(rename = "width")]
            width: serde_json::Number
        }
        pub type StartCssCoverage = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct StartCssCoverageArgs {
            #[serde(rename = "resetOnNavigation")]
            reset_on_navigation: Option<bool>
        }
        pub type StartJsCoverage = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct StartJsCoverageArgs {
            #[serde(rename = "reportAnonymousScripts")]
            report_anonymous_scripts: Option<bool>,
            #[serde(rename = "resetOnNavigation")]
            reset_on_navigation: Option<bool>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct StopCssCoverage {
            #[serde(rename = "entries")]
            entries: Vec<StopCssCoverageEntries>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct StopCssCoverageEntries {
            #[serde(rename = "ranges")]
            ranges: Vec<StopCssCoverageEntriesRanges>,
            #[serde(rename = "text")]
            text: Option<String>,
            #[serde(rename = "url")]
            url: String
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct StopCssCoverageEntriesRanges {
            #[serde(rename = "end")]
            end: serde_json::Number,
            #[serde(rename = "start")]
            start: serde_json::Number
        }
        pub type StopCssCoverageArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct StopJsCoverage {
            #[serde(rename = "entries")]
            entries: Vec<StopJsCoverageEntries>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct StopJsCoverageEntries {
            #[serde(rename = "functions")]
            functions: Vec<StopJsCoverageEntriesFunctions>,
            #[serde(rename = "scriptId")]
            script_id: String,
            #[serde(rename = "source")]
            source: Option<String>,
            #[serde(rename = "url")]
            url: String
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct StopJsCoverageEntriesFunctions {
            #[serde(rename = "functionName")]
            function_name: String,
            #[serde(rename = "isBlockCoverage")]
            is_block_coverage: bool,
            #[serde(rename = "ranges")]
            ranges: Vec<StopJsCoverageEntriesFunctionsRanges>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct StopJsCoverageEntriesFunctionsRanges {
            #[serde(rename = "count")]
            count: serde_json::Number,
            #[serde(rename = "endOffset")]
            end_offset: serde_json::Number,
            #[serde(rename = "startOffset")]
            start_offset: serde_json::Number
        }
        pub type StopJsCoverageArgs = ();
        pub type TouchscreenTap = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct TouchscreenTapArgs {
            #[serde(rename = "x")]
            x: serde_json::Number,
            #[serde(rename = "y")]
            y: serde_json::Number
        }
    }
}
pub type Page = Guid;
pub mod playwright {
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Initializer {
        #[serde(rename = "android")]
        android: crate::protocol::generated::Android,
        #[serde(rename = "chromium")]
        chromium: crate::protocol::generated::BrowserType,
        #[serde(rename = "deviceDescriptors")]
        device_descriptors: Vec<InitializerDeviceDescriptors>,
        #[serde(rename = "electron")]
        electron: crate::protocol::generated::Electron,
        #[serde(rename = "firefox")]
        firefox: crate::protocol::generated::BrowserType,
        #[serde(rename = "preLaunchedBrowser")]
        pre_launched_browser: Option<crate::protocol::generated::Browser>,
        #[serde(rename = "selectors")]
        selectors: crate::protocol::generated::Selectors,
        #[serde(rename = "socksSupport")]
        socks_support: Option<crate::protocol::generated::SocksSupport>,
        #[serde(rename = "utils")]
        utils: crate::protocol::generated::LocalUtils,
        #[serde(rename = "webkit")]
        webkit: crate::protocol::generated::BrowserType
    }
    #[derive(Debug, Serialize, Deserialize)]
    pub struct InitializerDeviceDescriptors {
        #[serde(rename = "descriptor")]
        descriptor: InitializerDeviceDescriptorsDescriptor,
        #[serde(rename = "name")]
        name: String
    }
    #[derive(Debug, Serialize, Deserialize)]
    pub struct InitializerDeviceDescriptorsDescriptor {
        #[serde(rename = "defaultBrowserType")]
        default_browser_type: InitializerDeviceDescriptorsDescriptorDefaultBrowserType,
        #[serde(rename = "deviceScaleFactor")]
        device_scale_factor: serde_json::Number,
        #[serde(rename = "hasTouch")]
        has_touch: bool,
        #[serde(rename = "isMobile")]
        is_mobile: bool,
        #[serde(rename = "screen")]
        screen: Option<InitializerDeviceDescriptorsDescriptorScreen>,
        #[serde(rename = "userAgent")]
        user_agent: String,
        #[serde(rename = "viewport")]
        viewport: InitializerDeviceDescriptorsDescriptorViewport
    }
    #[derive(Debug, Serialize, Deserialize)]
    pub enum InitializerDeviceDescriptorsDescriptorDefaultBrowserType {
        #[serde(rename = "chromium")]
        Chromium,
        #[serde(rename = "firefox")]
        Firefox,
        #[serde(rename = "webkit")]
        Webkit
    }
    #[derive(Debug, Serialize, Deserialize)]
    pub struct InitializerDeviceDescriptorsDescriptorScreen {
        #[serde(rename = "height")]
        height: serde_json::Number,
        #[serde(rename = "width")]
        width: serde_json::Number
    }
    #[derive(Debug, Serialize, Deserialize)]
    pub struct InitializerDeviceDescriptorsDescriptorViewport {
        #[serde(rename = "height")]
        height: serde_json::Number,
        #[serde(rename = "width")]
        width: serde_json::Number
    }
    pub mod commands {
        pub type HideHighlight = ();
        pub type HideHighlightArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct NewRequest {
            #[serde(rename = "request")]
            request: crate::protocol::generated::ApiRequestContext
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct NewRequestArgs {
            #[serde(rename = "baseURL")]
            base_url: Option<String>,
            #[serde(rename = "extraHTTPHeaders")]
            extra_http_headers: Option<Vec<crate::protocol::generated::NameValue>>,
            #[serde(rename = "httpCredentials")]
            http_credentials: Option<NewRequestArgsHttpCredentials>,
            #[serde(rename = "ignoreHTTPSErrors")]
            ignore_https_errors: Option<bool>,
            #[serde(rename = "proxy")]
            proxy: Option<NewRequestArgsProxy>,
            #[serde(rename = "storageState")]
            storage_state: Option<NewRequestArgsStorageState>,
            #[serde(rename = "timeout")]
            timeout: Option<serde_json::Number>,
            #[serde(rename = "tracesDir")]
            traces_dir: Option<String>,
            #[serde(rename = "userAgent")]
            user_agent: Option<String>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct NewRequestArgsHttpCredentials {
            #[serde(rename = "password")]
            password: String,
            #[serde(rename = "username")]
            username: String
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct NewRequestArgsProxy {
            #[serde(rename = "bypass")]
            bypass: Option<String>,
            #[serde(rename = "password")]
            password: Option<String>,
            #[serde(rename = "server")]
            server: String,
            #[serde(rename = "username")]
            username: Option<String>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct NewRequestArgsStorageState {
            #[serde(rename = "cookies")]
            cookies: Vec<crate::protocol::generated::NetworkCookie>,
            #[serde(rename = "origins")]
            origins: Vec<crate::protocol::generated::OriginStorage>
        }
    }
}
pub type Playwright = Guid;
pub mod request {
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Initializer {
        #[serde(rename = "frame")]
        frame: Option<crate::protocol::generated::Frame>,
        #[serde(rename = "headers")]
        headers: Vec<crate::protocol::generated::NameValue>,
        #[serde(rename = "isNavigationRequest")]
        is_navigation_request: bool,
        #[serde(rename = "method")]
        method: String,
        #[serde(rename = "postData")]
        post_data: Option<Vec<u8>>,
        #[serde(rename = "redirectedFrom")]
        redirected_from: Option<crate::protocol::generated::Request>,
        #[serde(rename = "resourceType")]
        resource_type: String,
        #[serde(rename = "serviceWorker")]
        service_worker: Option<crate::protocol::generated::Worker>,
        #[serde(rename = "url")]
        url: String
    }
    pub mod commands {
        #[derive(Debug, Serialize, Deserialize)]
        pub struct RawRequestHeaders {
            #[serde(rename = "headers")]
            headers: Vec<crate::protocol::generated::NameValue>
        }
        pub type RawRequestHeadersArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Response {
            #[serde(rename = "response")]
            response: Option<crate::protocol::generated::Response>
        }
        pub type ResponseArgs = ();
    }
}
pub type Request = Guid;
pub mod response {
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Initializer {
        #[serde(rename = "fromServiceWorker")]
        from_service_worker: bool,
        #[serde(rename = "headers")]
        headers: Vec<crate::protocol::generated::NameValue>,
        #[serde(rename = "request")]
        request: crate::protocol::generated::Request,
        #[serde(rename = "status")]
        status: serde_json::Number,
        #[serde(rename = "statusText")]
        status_text: String,
        #[serde(rename = "timing")]
        timing: crate::protocol::generated::ResourceTiming,
        #[serde(rename = "url")]
        url: String
    }
    pub mod commands {
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Body {
            #[serde(rename = "binary")]
            binary: Vec<u8>
        }
        pub type BodyArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct RawResponseHeaders {
            #[serde(rename = "headers")]
            headers: Vec<crate::protocol::generated::NameValue>
        }
        pub type RawResponseHeadersArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SecurityDetails {
            #[serde(rename = "value")]
            value: Option<crate::protocol::generated::SecurityDetails>
        }
        pub type SecurityDetailsArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ServerAddr {
            #[serde(rename = "value")]
            value: Option<crate::protocol::generated::RemoteAddr>
        }
        pub type ServerAddrArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Sizes {
            #[serde(rename = "sizes")]
            sizes: crate::protocol::generated::RequestSizes
        }
        pub type SizesArgs = ();
    }
}
pub type Response = Guid;
pub mod root {
    pub mod commands {
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Initialize {
            #[serde(rename = "playwright")]
            playwright: crate::protocol::generated::Playwright
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct InitializeArgs {
            #[serde(rename = "sdkLanguage")]
            sdk_language: String
        }
    }
}
pub type Root = Guid;
pub mod route {
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Initializer {
        #[serde(rename = "request")]
        request: crate::protocol::generated::Request
    }
    pub mod commands {
        pub type Abort = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct AbortArgs {
            #[serde(rename = "errorCode")]
            error_code: Option<String>
        }
        pub type Continue = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ContinueArgs {
            #[serde(rename = "headers")]
            headers: Option<Vec<crate::protocol::generated::NameValue>>,
            #[serde(rename = "method")]
            method: Option<String>,
            #[serde(rename = "postData")]
            post_data: Option<Vec<u8>>,
            #[serde(rename = "url")]
            url: Option<String>
        }
        pub type Fulfill = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct FulfillArgs {
            #[serde(rename = "body")]
            body: Option<String>,
            #[serde(rename = "fetchResponseUid")]
            fetch_response_uid: Option<String>,
            #[serde(rename = "headers")]
            headers: Option<Vec<crate::protocol::generated::NameValue>>,
            #[serde(rename = "isBase64")]
            is_base64: Option<bool>,
            #[serde(rename = "status")]
            status: Option<serde_json::Number>
        }
        pub type RedirectNavigationRequest = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct RedirectNavigationRequestArgs {
            #[serde(rename = "url")]
            url: String
        }
    }
}
pub type Route = Guid;
pub mod selectors {
    pub mod commands {
        pub type Register = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct RegisterArgs {
            #[serde(rename = "contentScript")]
            content_script: Option<bool>,
            #[serde(rename = "name")]
            name: String,
            #[serde(rename = "source")]
            source: String
        }
    }
}
pub type Selectors = Guid;
pub mod socks_support {
    pub mod events {
        #[derive(Debug, Deserialize, Serialize)]
        pub enum Events {
            #[serde(rename = "socksClosed")]
            SocksClosed(SocksClosed),
            #[serde(rename = "socksData")]
            SocksData(SocksData),
            #[serde(rename = "socksRequested")]
            SocksRequested(SocksRequested)
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SocksClosed {
            #[serde(rename = "uid")]
            uid: String
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SocksData {
            #[serde(rename = "data")]
            data: Vec<u8>,
            #[serde(rename = "uid")]
            uid: String
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SocksRequested {
            #[serde(rename = "host")]
            host: String,
            #[serde(rename = "port")]
            port: serde_json::Number,
            #[serde(rename = "uid")]
            uid: String
        }
    }
    pub mod commands {
        pub type SocksConnected = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SocksConnectedArgs {
            #[serde(rename = "host")]
            host: String,
            #[serde(rename = "port")]
            port: serde_json::Number,
            #[serde(rename = "uid")]
            uid: String
        }
        pub type SocksData = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SocksDataArgs {
            #[serde(rename = "data")]
            data: Vec<u8>,
            #[serde(rename = "uid")]
            uid: String
        }
        pub type SocksEnd = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SocksEndArgs {
            #[serde(rename = "uid")]
            uid: String
        }
        pub type SocksError = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SocksErrorArgs {
            #[serde(rename = "error")]
            error: String,
            #[serde(rename = "uid")]
            uid: String
        }
        pub type SocksFailed = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SocksFailedArgs {
            #[serde(rename = "errorCode")]
            error_code: String,
            #[serde(rename = "uid")]
            uid: String
        }
    }
}
pub type SocksSupport = Guid;
pub mod stream {
    pub mod commands {
        pub type Close = ();
        pub type CloseArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Read {
            #[serde(rename = "binary")]
            binary: Vec<u8>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ReadArgs {
            #[serde(rename = "size")]
            size: Option<serde_json::Number>
        }
    }
}
pub type Stream = Guid;
pub mod tracing {
    pub mod commands {
        pub type TracingStart = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct TracingStartArgs {
            #[serde(rename = "name")]
            name: Option<String>,
            #[serde(rename = "screenshots")]
            screenshots: Option<bool>,
            #[serde(rename = "snapshots")]
            snapshots: Option<bool>,
            #[serde(rename = "sources")]
            sources: Option<bool>
        }
        pub type TracingStartChunk = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct TracingStartChunkArgs {
            #[serde(rename = "title")]
            title: Option<String>
        }
        pub type TracingStop = ();
        pub type TracingStopArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct TracingStopChunk {
            #[serde(rename = "artifact")]
            artifact: Option<crate::protocol::generated::Artifact>,
            #[serde(rename = "sourceEntries")]
            source_entries: Option<Vec<crate::protocol::generated::NameValue>>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct TracingStopChunkArgs {
            #[serde(rename = "mode")]
            mode: TracingStopChunkArgsMode
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub enum TracingStopChunkArgsMode {
            #[serde(rename = "doNotSave")]
            DoNotSave,
            #[serde(rename = "compressTrace")]
            CompressTrace,
            #[serde(rename = "compressTraceAndSources")]
            CompressTraceAndSources
        }
    }
}
pub type Tracing = Guid;
pub mod web_socket {
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Initializer {
        #[serde(rename = "url")]
        url: String
    }
    pub mod events {
        #[derive(Debug, Deserialize, Serialize)]
        pub enum Events {
            #[serde(rename = "close")]
            Close,
            #[serde(rename = "frameReceived")]
            FrameReceived(FrameReceived),
            #[serde(rename = "frameSent")]
            FrameSent(FrameSent),
            #[serde(rename = "open")]
            Open,
            #[serde(rename = "socketError")]
            SocketError(SocketError)
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct FrameReceived {
            #[serde(rename = "data")]
            data: String,
            #[serde(rename = "opcode")]
            opcode: serde_json::Number
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct FrameSent {
            #[serde(rename = "data")]
            data: String,
            #[serde(rename = "opcode")]
            opcode: serde_json::Number
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SocketError {
            #[serde(rename = "error")]
            error: String
        }
    }
}
pub type WebSocket = Guid;
pub mod worker {
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Initializer {
        #[serde(rename = "url")]
        url: String
    }
    pub mod events {
        #[derive(Debug, Deserialize, Serialize)]
        pub enum Events {
            #[serde(rename = "close")]
            Close
        }
    }
    pub mod commands {
        #[derive(Debug, Serialize, Deserialize)]
        pub struct EvaluateExpression {
            #[serde(rename = "value")]
            value: crate::protocol::generated::SerializedValue
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct EvaluateExpressionArgs {
            #[serde(rename = "arg")]
            arg: crate::protocol::generated::SerializedArgument,
            #[serde(rename = "expression")]
            expression: String,
            #[serde(rename = "isFunction")]
            is_function: Option<bool>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct EvaluateExpressionHandle {
            #[serde(rename = "handle")]
            handle: crate::protocol::generated::JsHandle
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct EvaluateExpressionHandleArgs {
            #[serde(rename = "arg")]
            arg: crate::protocol::generated::SerializedArgument,
            #[serde(rename = "expression")]
            expression: String,
            #[serde(rename = "isFunction")]
            is_function: Option<bool>
        }
    }
}
pub type Worker = Guid;
pub mod writable_stream {
    pub mod commands {
        pub type Close = ();
        pub type CloseArgs = ();
        pub type Write = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct WriteArgs {
            #[serde(rename = "binary")]
            binary: Vec<u8>
        }
    }
}
pub type WritableStream = Guid;
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    #[serde(rename = "fetchUid")]
    fetch_uid: String,
    #[serde(rename = "headers")]
    headers: Vec<crate::protocol::generated::NameValue>,
    #[serde(rename = "status")]
    status: serde_json::Number,
    #[serde(rename = "statusText")]
    status_text: String,
    #[serde(rename = "url")]
    url: String
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AxNode {
    #[serde(rename = "autocomplete")]
    autocomplete: Option<String>,
    #[serde(rename = "checked")]
    checked: Option<AxNodeChecked>,
    #[serde(rename = "children")]
    children: Option<Vec<crate::protocol::generated::AxNode>>,
    #[serde(rename = "description")]
    description: Option<String>,
    #[serde(rename = "disabled")]
    disabled: Option<bool>,
    #[serde(rename = "expanded")]
    expanded: Option<bool>,
    #[serde(rename = "focused")]
    focused: Option<bool>,
    #[serde(rename = "haspopup")]
    haspopup: Option<String>,
    #[serde(rename = "invalid")]
    invalid: Option<String>,
    #[serde(rename = "keyshortcuts")]
    keyshortcuts: Option<String>,
    #[serde(rename = "level")]
    level: Option<serde_json::Number>,
    #[serde(rename = "modal")]
    modal: Option<bool>,
    #[serde(rename = "multiline")]
    multiline: Option<bool>,
    #[serde(rename = "multiselectable")]
    multiselectable: Option<bool>,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "orientation")]
    orientation: Option<String>,
    #[serde(rename = "pressed")]
    pressed: Option<AxNodePressed>,
    #[serde(rename = "readonly")]
    readonly: Option<bool>,
    #[serde(rename = "required")]
    required: Option<bool>,
    #[serde(rename = "role")]
    role: String,
    #[serde(rename = "roledescription")]
    roledescription: Option<String>,
    #[serde(rename = "selected")]
    selected: Option<bool>,
    #[serde(rename = "valueNumber")]
    value_number: Option<serde_json::Number>,
    #[serde(rename = "valueString")]
    value_string: Option<String>,
    #[serde(rename = "valuemax")]
    valuemax: Option<serde_json::Number>,
    #[serde(rename = "valuemin")]
    valuemin: Option<serde_json::Number>,
    #[serde(rename = "valuetext")]
    valuetext: Option<String>
}
#[derive(Debug, Serialize, Deserialize)]
pub enum AxNodeChecked {
    #[serde(rename = "checked")]
    Checked,
    #[serde(rename = "unchecked")]
    Unchecked,
    #[serde(rename = "mixed")]
    Mixed
}
#[derive(Debug, Serialize, Deserialize)]
pub enum AxNodePressed {
    #[serde(rename = "pressed")]
    Pressed,
    #[serde(rename = "released")]
    Released,
    #[serde(rename = "mixed")]
    Mixed
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AndroidElementInfo {
    #[serde(rename = "bounds")]
    bounds: crate::protocol::generated::Rect,
    #[serde(rename = "checkable")]
    checkable: bool,
    #[serde(rename = "checked")]
    checked: bool,
    #[serde(rename = "children")]
    children: Option<Vec<crate::protocol::generated::AndroidElementInfo>>,
    #[serde(rename = "clazz")]
    clazz: String,
    #[serde(rename = "clickable")]
    clickable: bool,
    #[serde(rename = "desc")]
    desc: String,
    #[serde(rename = "enabled")]
    enabled: bool,
    #[serde(rename = "focusable")]
    focusable: bool,
    #[serde(rename = "focused")]
    focused: bool,
    #[serde(rename = "longClickable")]
    long_clickable: bool,
    #[serde(rename = "pkg")]
    pkg: String,
    #[serde(rename = "res")]
    res: String,
    #[serde(rename = "scrollable")]
    scrollable: bool,
    #[serde(rename = "selected")]
    selected: bool,
    #[serde(rename = "text")]
    text: String
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AndroidSelector {
    #[serde(rename = "checkable")]
    checkable: Option<bool>,
    #[serde(rename = "checked")]
    checked: Option<bool>,
    #[serde(rename = "clazz")]
    clazz: Option<String>,
    #[serde(rename = "clickable")]
    clickable: Option<bool>,
    #[serde(rename = "depth")]
    depth: Option<serde_json::Number>,
    #[serde(rename = "desc")]
    desc: Option<String>,
    #[serde(rename = "enabled")]
    enabled: Option<bool>,
    #[serde(rename = "focusable")]
    focusable: Option<bool>,
    #[serde(rename = "focused")]
    focused: Option<bool>,
    #[serde(rename = "hasChild")]
    has_child: Option<Box<AndroidSelectorHasChild>>,
    #[serde(rename = "hasDescendant")]
    has_descendant: Option<Box<AndroidSelectorHasDescendant>>,
    #[serde(rename = "longClickable")]
    long_clickable: Option<bool>,
    #[serde(rename = "pkg")]
    pkg: Option<String>,
    #[serde(rename = "res")]
    res: Option<String>,
    #[serde(rename = "scrollable")]
    scrollable: Option<bool>,
    #[serde(rename = "selected")]
    selected: Option<bool>,
    #[serde(rename = "text")]
    text: Option<String>
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AndroidSelectorHasChild {
    #[serde(rename = "selector")]
    selector: crate::protocol::generated::AndroidSelector
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AndroidSelectorHasDescendant {
    #[serde(rename = "maxDepth")]
    max_depth: Option<serde_json::Number>,
    #[serde(rename = "selector")]
    selector: crate::protocol::generated::AndroidSelector
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AndroidWebView {
    #[serde(rename = "pid")]
    pid: serde_json::Number,
    #[serde(rename = "pkg")]
    pkg: String,
    #[serde(rename = "socketName")]
    socket_name: String
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ExpectedTextValue {
    #[serde(rename = "ignoreCase")]
    ignore_case: Option<bool>,
    #[serde(rename = "matchSubstring")]
    match_substring: Option<bool>,
    #[serde(rename = "normalizeWhiteSpace")]
    normalize_white_space: Option<bool>,
    #[serde(rename = "regexFlags")]
    regex_flags: Option<String>,
    #[serde(rename = "regexSource")]
    regex_source: Option<String>,
    #[serde(rename = "string")]
    string: Option<String>
}
#[derive(Debug, Serialize, Deserialize)]
pub struct FormField {
    #[serde(rename = "file")]
    file: Option<FormFieldFile>,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "value")]
    value: Option<String>
}
#[derive(Debug, Serialize, Deserialize)]
pub struct FormFieldFile {
    #[serde(rename = "buffer")]
    buffer: Vec<u8>,
    #[serde(rename = "mimeType")]
    mime_type: Option<String>,
    #[serde(rename = "name")]
    name: String
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(rename = "apiName")]
    api_name: Option<String>,
    #[serde(rename = "internal")]
    internal: Option<bool>,
    #[serde(rename = "stack")]
    stack: Option<Vec<crate::protocol::generated::StackFrame>>
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NameValue {
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "value")]
    value: String
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkCookie {
    #[serde(rename = "domain")]
    domain: String,
    #[serde(rename = "expires")]
    expires: serde_json::Number,
    #[serde(rename = "httpOnly")]
    http_only: bool,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "path")]
    path: String,
    #[serde(rename = "sameSite")]
    same_site: NetworkCookieSameSite,
    #[serde(rename = "secure")]
    secure: bool,
    #[serde(rename = "value")]
    value: String
}
#[derive(Debug, Serialize, Deserialize)]
pub enum NetworkCookieSameSite {
    Strict,
    Lax,
    None
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OriginStorage {
    #[serde(rename = "localStorage")]
    local_storage: Vec<crate::protocol::generated::NameValue>,
    #[serde(rename = "origin")]
    origin: String
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Point {
    #[serde(rename = "x")]
    x: serde_json::Number,
    #[serde(rename = "y")]
    y: serde_json::Number
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RecordHarOptions {
    #[serde(rename = "content")]
    content: Option<RecordHarOptionsContent>,
    #[serde(rename = "mode")]
    mode: Option<RecordHarOptionsMode>,
    #[serde(rename = "path")]
    path: String,
    #[serde(rename = "urlGlob")]
    url_glob: Option<String>,
    #[serde(rename = "urlRegexFlags")]
    url_regex_flags: Option<String>,
    #[serde(rename = "urlRegexSource")]
    url_regex_source: Option<String>
}
#[derive(Debug, Serialize, Deserialize)]
pub enum RecordHarOptionsContent {
    #[serde(rename = "embed")]
    Embed,
    #[serde(rename = "attach")]
    Attach,
    #[serde(rename = "omit")]
    Omit
}
#[derive(Debug, Serialize, Deserialize)]
pub enum RecordHarOptionsMode {
    #[serde(rename = "full")]
    Full,
    #[serde(rename = "minimal")]
    Minimal
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Rect {
    #[serde(rename = "height")]
    height: serde_json::Number,
    #[serde(rename = "width")]
    width: serde_json::Number,
    #[serde(rename = "x")]
    x: serde_json::Number,
    #[serde(rename = "y")]
    y: serde_json::Number
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RemoteAddr {
    #[serde(rename = "ipAddress")]
    ip_address: String,
    #[serde(rename = "port")]
    port: serde_json::Number
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestSizes {
    #[serde(rename = "requestBodySize")]
    request_body_size: serde_json::Number,
    #[serde(rename = "requestHeadersSize")]
    request_headers_size: serde_json::Number,
    #[serde(rename = "responseBodySize")]
    response_body_size: serde_json::Number,
    #[serde(rename = "responseHeadersSize")]
    response_headers_size: serde_json::Number
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceTiming {
    #[serde(rename = "connectEnd")]
    connect_end: serde_json::Number,
    #[serde(rename = "connectStart")]
    connect_start: serde_json::Number,
    #[serde(rename = "domainLookupEnd")]
    domain_lookup_end: serde_json::Number,
    #[serde(rename = "domainLookupStart")]
    domain_lookup_start: serde_json::Number,
    #[serde(rename = "requestStart")]
    request_start: serde_json::Number,
    #[serde(rename = "responseStart")]
    response_start: serde_json::Number,
    #[serde(rename = "secureConnectionStart")]
    secure_connection_start: serde_json::Number,
    #[serde(rename = "startTime")]
    start_time: serde_json::Number
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityDetails {
    #[serde(rename = "issuer")]
    issuer: Option<String>,
    #[serde(rename = "protocol")]
    protocol: Option<String>,
    #[serde(rename = "subjectName")]
    subject_name: Option<String>,
    #[serde(rename = "validFrom")]
    valid_from: Option<serde_json::Number>,
    #[serde(rename = "validTo")]
    valid_to: Option<serde_json::Number>
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedArgument {
    #[serde(rename = "handles")]
    handles: Vec<crate::protocol::generated::Channel>,
    #[serde(rename = "value")]
    value: crate::protocol::generated::SerializedValue
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedError {
    #[serde(rename = "error")]
    error: Option<SerializedErrorError>,
    #[serde(rename = "value")]
    value: Option<crate::protocol::generated::SerializedValue>
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedErrorError {
    #[serde(rename = "message")]
    message: String,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "stack")]
    stack: Option<String>
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedValue {
    #[serde(rename = "a")]
    a: Option<Vec<crate::protocol::generated::SerializedValue>>,
    #[serde(rename = "b")]
    b: Option<bool>,
    #[serde(rename = "d")]
    d: Option<String>,
    #[serde(rename = "h")]
    h: Option<serde_json::Number>,
    #[serde(rename = "id")]
    id: Option<serde_json::Number>,
    #[serde(rename = "n")]
    n: Option<serde_json::Number>,
    #[serde(rename = "o")]
    o: Option<Vec<SerializedValueO>>,
    #[serde(rename = "r")]
    r: Option<SerializedValueR>,
    #[serde(rename = "ref")]
    r#ref: Option<serde_json::Number>,
    #[serde(rename = "s")]
    s: Option<String>,
    #[serde(rename = "u")]
    u: Option<String>,
    #[serde(rename = "v")]
    v: Option<SerializedValueV>
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedValueO {
    #[serde(rename = "k")]
    k: String,
    #[serde(rename = "v")]
    v: crate::protocol::generated::SerializedValue
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedValueR {
    #[serde(rename = "f")]
    f: String,
    #[serde(rename = "p")]
    p: String
}
#[derive(Debug, Serialize, Deserialize)]
pub enum SerializedValueV {
    #[serde(rename = "null")]
    Null,
    #[serde(rename = "undefined")]
    Undefined,
    NaN,
    Infinity,
    #[serde(rename = "-Infinity")]
    NegInfinity,
    #[serde(rename = "-0")]
    Neg0
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SetNetworkCookie {
    #[serde(rename = "domain")]
    domain: Option<String>,
    #[serde(rename = "expires")]
    expires: Option<serde_json::Number>,
    #[serde(rename = "httpOnly")]
    http_only: Option<bool>,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "path")]
    path: Option<String>,
    #[serde(rename = "sameSite")]
    same_site: Option<SetNetworkCookieSameSite>,
    #[serde(rename = "secure")]
    secure: Option<bool>,
    #[serde(rename = "url")]
    url: Option<String>,
    #[serde(rename = "value")]
    value: String
}
#[derive(Debug, Serialize, Deserialize)]
pub enum SetNetworkCookieSameSite {
    Strict,
    Lax,
    None
}
#[derive(Debug, Serialize, Deserialize)]
pub struct StackFrame {
    #[serde(rename = "column")]
    column: Option<serde_json::Number>,
    #[serde(rename = "file")]
    file: String,
    #[serde(rename = "function")]
    function: Option<String>,
    #[serde(rename = "line")]
    line: Option<serde_json::Number>
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CommonScreenshotOptions {
    #[serde(rename = "animations")]
    animations: Option<CommonScreenshotOptionsAnimations>,
    #[serde(rename = "caret")]
    caret: Option<CommonScreenshotOptionsCaret>,
    #[serde(rename = "mask")]
    mask: Option<Vec<CommonScreenshotOptionsMask>>,
    #[serde(rename = "omitBackground")]
    omit_background: Option<bool>,
    #[serde(rename = "scale")]
    scale: Option<CommonScreenshotOptionsScale>
}
#[derive(Debug, Serialize, Deserialize)]
pub enum CommonScreenshotOptionsAnimations {
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "allow")]
    Allow
}
#[derive(Debug, Serialize, Deserialize)]
pub enum CommonScreenshotOptionsCaret {
    #[serde(rename = "hide")]
    Hide,
    #[serde(rename = "initial")]
    Initial
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CommonScreenshotOptionsMask {
    #[serde(rename = "frame")]
    frame: crate::protocol::generated::Frame,
    #[serde(rename = "selector")]
    selector: String
}
#[derive(Debug, Serialize, Deserialize)]
pub enum CommonScreenshotOptionsScale {
    #[serde(rename = "css")]
    Css,
    #[serde(rename = "device")]
    Device
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ContextOptions {
    #[serde(rename = "acceptDownloads")]
    accept_downloads: Option<bool>,
    #[serde(rename = "baseURL")]
    base_url: Option<String>,
    #[serde(rename = "bypassCSP")]
    bypass_csp: Option<bool>,
    #[serde(rename = "colorScheme")]
    color_scheme: Option<ContextOptionsColorScheme>,
    #[serde(rename = "deviceScaleFactor")]
    device_scale_factor: Option<serde_json::Number>,
    #[serde(rename = "extraHTTPHeaders")]
    extra_http_headers: Option<Vec<crate::protocol::generated::NameValue>>,
    #[serde(rename = "forcedColors")]
    forced_colors: Option<ContextOptionsForcedColors>,
    #[serde(rename = "geolocation")]
    geolocation: Option<ContextOptionsGeolocation>,
    #[serde(rename = "hasTouch")]
    has_touch: Option<bool>,
    #[serde(rename = "httpCredentials")]
    http_credentials: Option<ContextOptionsHttpCredentials>,
    #[serde(rename = "ignoreHTTPSErrors")]
    ignore_https_errors: Option<bool>,
    #[serde(rename = "isMobile")]
    is_mobile: Option<bool>,
    #[serde(rename = "javaScriptEnabled")]
    java_script_enabled: Option<bool>,
    #[serde(rename = "locale")]
    locale: Option<String>,
    #[serde(rename = "noDefaultViewport")]
    no_default_viewport: Option<bool>,
    #[serde(rename = "offline")]
    offline: Option<bool>,
    #[serde(rename = "permissions")]
    permissions: Option<Vec<String>>,
    #[serde(rename = "recordHar")]
    record_har: Option<crate::protocol::generated::RecordHarOptions>,
    #[serde(rename = "recordVideo")]
    record_video: Option<ContextOptionsRecordVideo>,
    #[serde(rename = "reducedMotion")]
    reduced_motion: Option<ContextOptionsReducedMotion>,
    #[serde(rename = "screen")]
    screen: Option<ContextOptionsScreen>,
    #[serde(rename = "serviceWorkers")]
    service_workers: Option<ContextOptionsServiceWorkers>,
    #[serde(rename = "strictSelectors")]
    strict_selectors: Option<bool>,
    #[serde(rename = "timezoneId")]
    timezone_id: Option<String>,
    #[serde(rename = "userAgent")]
    user_agent: Option<String>,
    #[serde(rename = "viewport")]
    viewport: Option<ContextOptionsViewport>
}
#[derive(Debug, Serialize, Deserialize)]
pub enum ContextOptionsColorScheme {
    #[serde(rename = "dark")]
    Dark,
    #[serde(rename = "light")]
    Light,
    #[serde(rename = "no-preference")]
    NoPreference
}
#[derive(Debug, Serialize, Deserialize)]
pub enum ContextOptionsForcedColors {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "none")]
    None
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ContextOptionsGeolocation {
    #[serde(rename = "accuracy")]
    accuracy: Option<serde_json::Number>,
    #[serde(rename = "latitude")]
    latitude: serde_json::Number,
    #[serde(rename = "longitude")]
    longitude: serde_json::Number
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ContextOptionsHttpCredentials {
    #[serde(rename = "password")]
    password: String,
    #[serde(rename = "username")]
    username: String
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ContextOptionsRecordVideo {
    #[serde(rename = "dir")]
    dir: String,
    #[serde(rename = "size")]
    size: Option<ContextOptionsRecordVideoSize>
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ContextOptionsRecordVideoSize {
    #[serde(rename = "height")]
    height: serde_json::Number,
    #[serde(rename = "width")]
    width: serde_json::Number
}
#[derive(Debug, Serialize, Deserialize)]
pub enum ContextOptionsReducedMotion {
    #[serde(rename = "reduce")]
    Reduce,
    #[serde(rename = "no-preference")]
    NoPreference
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ContextOptionsScreen {
    #[serde(rename = "height")]
    height: serde_json::Number,
    #[serde(rename = "width")]
    width: serde_json::Number
}
#[derive(Debug, Serialize, Deserialize)]
pub enum ContextOptionsServiceWorkers {
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "block")]
    Block
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ContextOptionsViewport {
    #[serde(rename = "height")]
    height: serde_json::Number,
    #[serde(rename = "width")]
    width: serde_json::Number
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LaunchOptions {
    #[serde(rename = "args")]
    args: Option<Vec<String>>,
    #[serde(rename = "channel")]
    channel: Option<String>,
    #[serde(rename = "chromiumSandbox")]
    chromium_sandbox: Option<bool>,
    #[serde(rename = "devtools")]
    devtools: Option<bool>,
    #[serde(rename = "downloadsPath")]
    downloads_path: Option<String>,
    #[serde(rename = "env")]
    env: Option<Vec<crate::protocol::generated::NameValue>>,
    #[serde(rename = "executablePath")]
    executable_path: Option<String>,
    #[serde(rename = "handleSIGHUP")]
    handle_sighup: Option<bool>,
    #[serde(rename = "handleSIGINT")]
    handle_sigint: Option<bool>,
    #[serde(rename = "handleSIGTERM")]
    handle_sigterm: Option<bool>,
    #[serde(rename = "headless")]
    headless: Option<bool>,
    #[serde(rename = "ignoreAllDefaultArgs")]
    ignore_all_default_args: Option<bool>,
    #[serde(rename = "ignoreDefaultArgs")]
    ignore_default_args: Option<Vec<String>>,
    #[serde(rename = "proxy")]
    proxy: Option<LaunchOptionsProxy>,
    #[serde(rename = "timeout")]
    timeout: Option<serde_json::Number>,
    #[serde(rename = "tracesDir")]
    traces_dir: Option<String>
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LaunchOptionsProxy {
    #[serde(rename = "bypass")]
    bypass: Option<String>,
    #[serde(rename = "password")]
    password: Option<String>,
    #[serde(rename = "server")]
    server: String,
    #[serde(rename = "username")]
    username: Option<String>
}
#[derive(Debug, Serialize, Deserialize)]
pub enum LifecycleEvent {
    #[serde(rename = "load")]
    Load,
    #[serde(rename = "domcontentloaded")]
    Domcontentloaded,
    #[serde(rename = "networkidle")]
    Networkidle,
    #[serde(rename = "commit")]
    Commit
}
// vim: foldnestmax=0 ft=rust
