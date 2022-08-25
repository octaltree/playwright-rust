use crate::imp::core::OnlyGuid;
pub(crate) type Channel = OnlyGuid;
fn is_default<T>(v: &T) -> bool
where
    T: PartialEq + Default
{
    T::default().eq(v)
}
pub(crate) type ApiRequestContext = OnlyGuid;
pub mod api_request_context {
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Initializer {
        #[serde(rename = "tracing")]
        pub(crate) tracing: crate::protocol::generated::Tracing
    }
    pub mod commands {
        pub type Dispose = ();
        pub type DisposeArgs = ();
        pub type DisposeApiResponse = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct DisposeApiResponseArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "fetchUid")]
            pub(crate) fetch_uid: &'a str
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Fetch {
            #[serde(rename = "response")]
            pub(crate) response: crate::protocol::generated::ApiResponse
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct FetchArgs<'a> {
            #[serde(rename = "failOnStatusCode")]
            pub(crate) fail_on_status_code: Option<bool>,
            #[serde(rename = "formData")]
            pub(crate) form_data: Option<Vec<crate::protocol::generated::NameValue>>,
            #[serde(rename = "headers")]
            pub(crate) headers: Option<Vec<crate::protocol::generated::NameValue>>,
            #[serde(rename = "ignoreHTTPSErrors")]
            pub(crate) ignore_https_errors: Option<bool>,
            #[serde(borrow)]
            #[serde(rename = "jsonData")]
            pub(crate) json_data: Option<&'a str>,
            #[serde(borrow)]
            #[serde(rename = "method")]
            pub(crate) method: Option<&'a str>,
            #[serde(rename = "multipartData")]
            pub(crate) multipart_data: Option<Vec<crate::protocol::generated::FormField>>,
            #[serde(rename = "params")]
            pub(crate) params: Option<Vec<crate::protocol::generated::NameValue>>,
            #[serde(borrow)]
            #[serde(rename = "postData")]
            pub(crate) post_data: Option<&'a [u8]>,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>,
            #[serde(borrow)]
            #[serde(rename = "url")]
            pub(crate) url: &'a str
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct FetchLog {
            #[serde(rename = "log")]
            pub(crate) log: Vec<String>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct FetchLogArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "fetchUid")]
            pub(crate) fetch_uid: &'a str
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct FetchResponseBody {
            #[serde(rename = "binary")]
            pub(crate) binary: Option<Vec<u8>>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct FetchResponseBodyArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "fetchUid")]
            pub(crate) fetch_uid: &'a str
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct StorageState {
            #[serde(rename = "cookies")]
            pub(crate) cookies: Vec<crate::protocol::generated::NetworkCookie>,
            #[serde(rename = "origins")]
            pub(crate) origins: Vec<crate::protocol::generated::OriginStorage>
        }
        pub type StorageStateArgs = ();
    }
}
pub(crate) type Android = OnlyGuid;
pub mod android {
    pub mod commands {
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Devices {
            #[serde(rename = "devices")]
            pub(crate) devices: Vec<crate::protocol::generated::AndroidDevice>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct DevicesArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "host")]
            pub(crate) host: Option<&'a str>,
            #[serde(rename = "omitDriverInstall")]
            pub(crate) omit_driver_install: Option<bool>,
            #[serde(rename = "port")]
            pub(crate) port: Option<serde_json::Number>
        }
        pub type SetDefaultTimeoutNoReply = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetDefaultTimeoutNoReplyArgs {
            #[serde(rename = "timeout")]
            pub(crate) timeout: serde_json::Number
        }
    }
}
pub(crate) type AndroidDevice = OnlyGuid;
pub mod android_device {
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Initializer {
        #[serde(rename = "model")]
        pub(crate) model: String,
        #[serde(rename = "serial")]
        pub(crate) serial: String
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
            pub(crate) web_view: crate::protocol::generated::AndroidWebView
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct WebViewRemoved {
            #[serde(rename = "socketName")]
            pub(crate) socket_name: String
        }
    }
    pub mod commands {
        pub type Close = ();
        pub type CloseArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ConnectToWebView {
            #[serde(rename = "context")]
            pub(crate) context: crate::protocol::generated::BrowserContext
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ConnectToWebViewArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "socketName")]
            pub(crate) socket_name: &'a str
        }
        pub type Drag = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct DragArgs {
            #[serde(rename = "dest")]
            pub(crate) dest: crate::protocol::generated::Point,
            #[serde(rename = "selector")]
            pub(crate) selector: crate::protocol::generated::AndroidSelector,
            #[serde(rename = "speed")]
            pub(crate) speed: Option<serde_json::Number>,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>
        }
        pub type Fill = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct FillArgs<'a> {
            #[serde(rename = "selector")]
            pub(crate) selector: crate::protocol::generated::AndroidSelector,
            #[serde(borrow)]
            #[serde(rename = "text")]
            pub(crate) text: &'a str,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>
        }
        pub type Fling = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct FlingArgs {
            #[serde(rename = "direction")]
            pub(crate) direction: FlingArgsDirection,
            #[serde(rename = "selector")]
            pub(crate) selector: crate::protocol::generated::AndroidSelector,
            #[serde(rename = "speed")]
            pub(crate) speed: Option<serde_json::Number>,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
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
            pub(crate) info: crate::protocol::generated::AndroidElementInfo
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct InfoArgs {
            #[serde(rename = "selector")]
            pub(crate) selector: crate::protocol::generated::AndroidSelector
        }
        pub type InputDrag = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct InputDragArgs {
            #[serde(rename = "from")]
            pub(crate) from: crate::protocol::generated::Point,
            #[serde(rename = "steps")]
            pub(crate) steps: serde_json::Number,
            #[serde(rename = "to")]
            pub(crate) to: crate::protocol::generated::Point
        }
        pub type InputPress = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct InputPressArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "key")]
            pub(crate) key: &'a str
        }
        pub type InputSwipe = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct InputSwipeArgs {
            #[serde(rename = "segments")]
            pub(crate) segments: Vec<crate::protocol::generated::Point>,
            #[serde(rename = "steps")]
            pub(crate) steps: serde_json::Number
        }
        pub type InputTap = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct InputTapArgs {
            #[serde(rename = "point")]
            pub(crate) point: crate::protocol::generated::Point
        }
        pub type InputType = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct InputTypeArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "text")]
            pub(crate) text: &'a str
        }
        pub type InstallApk = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct InstallApkArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "args")]
            pub(crate) args: Option<Vec<&'a str>>,
            #[serde(borrow)]
            #[serde(rename = "file")]
            pub(crate) file: &'a [u8]
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct LaunchBrowser {
            #[serde(rename = "context")]
            pub(crate) context: crate::protocol::generated::BrowserContext
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct LaunchBrowserArgs<'a> {
            #[serde(flatten)]
            #[serde(rename = "$mixin")]
            pub(crate) mixin: crate::protocol::generated::ContextOptions,
            #[serde(borrow)]
            #[serde(rename = "pkg")]
            pub(crate) pkg: Option<&'a str>,
            #[serde(borrow)]
            #[serde(rename = "proxy")]
            pub(crate) proxy: Option<LaunchBrowserArgsProxy<'a>>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct LaunchBrowserArgsProxy<'a> {
            #[serde(borrow)]
            #[serde(rename = "bypass")]
            pub(crate) bypass: Option<&'a str>,
            #[serde(borrow)]
            #[serde(rename = "password")]
            pub(crate) password: Option<&'a str>,
            #[serde(borrow)]
            #[serde(rename = "server")]
            pub(crate) server: &'a str,
            #[serde(borrow)]
            #[serde(rename = "username")]
            pub(crate) username: Option<&'a str>
        }
        pub type LongTap = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct LongTapArgs {
            #[serde(rename = "selector")]
            pub(crate) selector: crate::protocol::generated::AndroidSelector,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Open {
            #[serde(rename = "socket")]
            pub(crate) socket: crate::protocol::generated::AndroidSocket
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct OpenArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "command")]
            pub(crate) command: &'a str
        }
        pub type PinchClose = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct PinchCloseArgs {
            #[serde(rename = "percent")]
            pub(crate) percent: serde_json::Number,
            #[serde(rename = "selector")]
            pub(crate) selector: crate::protocol::generated::AndroidSelector,
            #[serde(rename = "speed")]
            pub(crate) speed: Option<serde_json::Number>,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>
        }
        pub type PinchOpen = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct PinchOpenArgs {
            #[serde(rename = "percent")]
            pub(crate) percent: serde_json::Number,
            #[serde(rename = "selector")]
            pub(crate) selector: crate::protocol::generated::AndroidSelector,
            #[serde(rename = "speed")]
            pub(crate) speed: Option<serde_json::Number>,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>
        }
        pub type Push = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct PushArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "file")]
            pub(crate) file: &'a [u8],
            #[serde(rename = "mode")]
            pub(crate) mode: Option<serde_json::Number>,
            #[serde(borrow)]
            #[serde(rename = "path")]
            pub(crate) path: &'a str
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Screenshot {
            #[serde(rename = "binary")]
            pub(crate) binary: Vec<u8>
        }
        pub type ScreenshotArgs = ();
        pub type Scroll = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ScrollArgs {
            #[serde(rename = "direction")]
            pub(crate) direction: ScrollArgsDirection,
            #[serde(rename = "percent")]
            pub(crate) percent: serde_json::Number,
            #[serde(rename = "selector")]
            pub(crate) selector: crate::protocol::generated::AndroidSelector,
            #[serde(rename = "speed")]
            pub(crate) speed: Option<serde_json::Number>,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
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
            pub(crate) timeout: serde_json::Number
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Shell {
            #[serde(rename = "result")]
            pub(crate) result: Vec<u8>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ShellArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "command")]
            pub(crate) command: &'a str
        }
        pub type Swipe = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SwipeArgs {
            #[serde(rename = "direction")]
            pub(crate) direction: SwipeArgsDirection,
            #[serde(rename = "percent")]
            pub(crate) percent: serde_json::Number,
            #[serde(rename = "selector")]
            pub(crate) selector: crate::protocol::generated::AndroidSelector,
            #[serde(rename = "speed")]
            pub(crate) speed: Option<serde_json::Number>,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
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
            pub(crate) duration: Option<serde_json::Number>,
            #[serde(rename = "selector")]
            pub(crate) selector: crate::protocol::generated::AndroidSelector,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>
        }
        pub type Wait = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct WaitArgs {
            #[serde(rename = "selector")]
            pub(crate) selector: crate::protocol::generated::AndroidSelector,
            #[serde(rename = "state")]
            pub(crate) state: Option<WaitArgsState>,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
        pub enum WaitArgsState {
            #[serde(rename = "gone")]
            Gone
        }
    }
}
pub(crate) type AndroidSocket = OnlyGuid;
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
            pub(crate) data: Vec<u8>
        }
    }
    pub mod commands {
        pub type Close = ();
        pub type CloseArgs = ();
        pub type Write = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct WriteArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "data")]
            pub(crate) data: &'a [u8]
        }
    }
}
pub(crate) type Artifact = OnlyGuid;
pub mod artifact {
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Initializer {
        #[serde(rename = "absolutePath")]
        pub(crate) absolute_path: String
    }
    pub mod commands {
        pub type Cancel = ();
        pub type CancelArgs = ();
        pub type Delete = ();
        pub type DeleteArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Failure {
            #[serde(rename = "error")]
            pub(crate) error: Option<String>
        }
        pub type FailureArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct PathAfterFinished {
            #[serde(rename = "value")]
            pub(crate) value: Option<String>
        }
        pub type PathAfterFinishedArgs = ();
        pub type SaveAs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SaveAsArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "path")]
            pub(crate) path: &'a str
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SaveAsStream {
            #[serde(rename = "stream")]
            pub(crate) stream: crate::protocol::generated::Stream
        }
        pub type SaveAsStreamArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Stream {
            #[serde(rename = "stream")]
            pub(crate) stream: Option<crate::protocol::generated::Stream>
        }
        pub type StreamArgs = ();
    }
}
pub(crate) type BindingCall = OnlyGuid;
pub mod binding_call {
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Initializer {
        #[serde(rename = "args")]
        pub(crate) args: Option<Vec<crate::protocol::generated::SerializedValue>>,
        #[serde(rename = "frame")]
        pub(crate) frame: crate::protocol::generated::Frame,
        #[serde(rename = "handle")]
        pub(crate) handle: Option<crate::protocol::generated::JsHandle>,
        #[serde(rename = "name")]
        pub(crate) name: String
    }
    pub mod commands {
        pub type Reject = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct RejectArgs {
            #[serde(rename = "error")]
            pub(crate) error: crate::protocol::generated::SerializedError
        }
        pub type Resolve = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ResolveArgs {
            #[serde(rename = "result")]
            pub(crate) result: crate::protocol::generated::SerializedArgument
        }
    }
}
pub(crate) type Browser = OnlyGuid;
pub mod browser {
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Initializer {
        #[serde(rename = "name")]
        pub(crate) name: String,
        #[serde(rename = "version")]
        pub(crate) version: String
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
            pub(crate) session: crate::protocol::generated::CdpSession
        }
        pub type NewBrowserCdpSessionArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct NewContext {
            #[serde(rename = "context")]
            pub(crate) context: crate::protocol::generated::BrowserContext
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct NewContextArgs<'a> {
            #[serde(flatten)]
            #[serde(rename = "$mixin")]
            pub(crate) mixin: crate::protocol::generated::ContextOptions,
            #[serde(borrow)]
            #[serde(rename = "proxy")]
            pub(crate) proxy: Option<NewContextArgsProxy<'a>>,
            #[serde(rename = "storageState")]
            pub(crate) storage_state: Option<NewContextArgsStorageState>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct NewContextArgsProxy<'a> {
            #[serde(borrow)]
            #[serde(rename = "bypass")]
            pub(crate) bypass: Option<&'a str>,
            #[serde(borrow)]
            #[serde(rename = "password")]
            pub(crate) password: Option<&'a str>,
            #[serde(borrow)]
            #[serde(rename = "server")]
            pub(crate) server: &'a str,
            #[serde(borrow)]
            #[serde(rename = "username")]
            pub(crate) username: Option<&'a str>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct NewContextArgsStorageState {
            #[serde(rename = "cookies")]
            pub(crate) cookies: Option<Vec<crate::protocol::generated::SetNetworkCookie>>,
            #[serde(rename = "origins")]
            pub(crate) origins: Option<Vec<crate::protocol::generated::OriginStorage>>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct NewContextForReuse {
            #[serde(rename = "context")]
            pub(crate) context: crate::protocol::generated::BrowserContext
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct NewContextForReuseArgs<'a> {
            #[serde(flatten)]
            #[serde(rename = "$mixin")]
            pub(crate) mixin: crate::protocol::generated::ContextOptions,
            #[serde(borrow)]
            #[serde(rename = "proxy")]
            pub(crate) proxy: Option<NewContextForReuseArgsProxy<'a>>,
            #[serde(rename = "storageState")]
            pub(crate) storage_state: Option<NewContextForReuseArgsStorageState>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct NewContextForReuseArgsProxy<'a> {
            #[serde(borrow)]
            #[serde(rename = "bypass")]
            pub(crate) bypass: Option<&'a str>,
            #[serde(borrow)]
            #[serde(rename = "password")]
            pub(crate) password: Option<&'a str>,
            #[serde(borrow)]
            #[serde(rename = "server")]
            pub(crate) server: &'a str,
            #[serde(borrow)]
            #[serde(rename = "username")]
            pub(crate) username: Option<&'a str>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct NewContextForReuseArgsStorageState {
            #[serde(rename = "cookies")]
            pub(crate) cookies: Option<Vec<crate::protocol::generated::SetNetworkCookie>>,
            #[serde(rename = "origins")]
            pub(crate) origins: Option<Vec<crate::protocol::generated::OriginStorage>>
        }
        pub type StartTracing = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct StartTracingArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "categories")]
            pub(crate) categories: Option<Vec<&'a str>>,
            #[serde(rename = "page")]
            pub(crate) page: Option<crate::protocol::generated::Page>,
            #[serde(borrow)]
            #[serde(rename = "path")]
            pub(crate) path: Option<&'a str>,
            #[serde(rename = "screenshots")]
            pub(crate) screenshots: Option<bool>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct StopTracing {
            #[serde(rename = "binary")]
            pub(crate) binary: Vec<u8>
        }
        pub type StopTracingArgs = ();
    }
}
pub(crate) type BrowserContext = OnlyGuid;
pub mod browser_context {
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Initializer {
        #[serde(rename = "isChromium")]
        pub(crate) is_chromium: bool,
        #[serde(rename = "requestContext")]
        pub(crate) request_context: crate::protocol::generated::ApiRequestContext,
        #[serde(rename = "tracing")]
        pub(crate) tracing: crate::protocol::generated::Tracing
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
            pub(crate) page: crate::protocol::generated::Page
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct BindingCall {
            #[serde(rename = "binding")]
            pub(crate) binding: crate::protocol::generated::BindingCall
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Page {
            #[serde(rename = "page")]
            pub(crate) page: crate::protocol::generated::Page
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Request {
            #[serde(rename = "page")]
            pub(crate) page: Option<crate::protocol::generated::Page>,
            #[serde(rename = "request")]
            pub(crate) request: crate::protocol::generated::Request
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct RequestFailed {
            #[serde(rename = "failureText")]
            pub(crate) failure_text: Option<String>,
            #[serde(rename = "page")]
            pub(crate) page: Option<crate::protocol::generated::Page>,
            #[serde(rename = "request")]
            pub(crate) request: crate::protocol::generated::Request,
            #[serde(rename = "responseEndTiming")]
            pub(crate) response_end_timing: serde_json::Number
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct RequestFinished {
            #[serde(rename = "page")]
            pub(crate) page: Option<crate::protocol::generated::Page>,
            #[serde(rename = "request")]
            pub(crate) request: crate::protocol::generated::Request,
            #[serde(rename = "response")]
            pub(crate) response: Option<crate::protocol::generated::Response>,
            #[serde(rename = "responseEndTiming")]
            pub(crate) response_end_timing: serde_json::Number
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Response {
            #[serde(rename = "page")]
            pub(crate) page: Option<crate::protocol::generated::Page>,
            #[serde(rename = "response")]
            pub(crate) response: crate::protocol::generated::Response
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Route {
            #[serde(rename = "request")]
            pub(crate) request: crate::protocol::generated::Request,
            #[serde(rename = "route")]
            pub(crate) route: crate::protocol::generated::Route
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ServiceWorker {
            #[serde(rename = "worker")]
            pub(crate) worker: crate::protocol::generated::Worker
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Video {
            #[serde(rename = "artifact")]
            pub(crate) artifact: crate::protocol::generated::Artifact
        }
    }
    pub mod commands {
        pub type AddCookies = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct AddCookiesArgs {
            #[serde(rename = "cookies")]
            pub(crate) cookies: Vec<crate::protocol::generated::SetNetworkCookie>
        }
        pub type AddInitScript = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct AddInitScriptArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "source")]
            pub(crate) source: &'a str
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
            pub(crate) cookies: Vec<crate::protocol::generated::NetworkCookie>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct CookiesArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "urls")]
            pub(crate) urls: Vec<&'a str>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct CreateTempFile {
            #[serde(rename = "writableStream")]
            pub(crate) writable_stream: crate::protocol::generated::WritableStream
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct CreateTempFileArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "name")]
            pub(crate) name: &'a str
        }
        pub type ExposeBinding = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ExposeBindingArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "name")]
            pub(crate) name: &'a str,
            #[serde(rename = "needsHandle")]
            pub(crate) needs_handle: Option<bool>
        }
        pub type GrantPermissions = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct GrantPermissionsArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "origin")]
            pub(crate) origin: Option<&'a str>,
            #[serde(borrow)]
            #[serde(rename = "permissions")]
            pub(crate) permissions: Vec<&'a str>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct HarExport {
            #[serde(rename = "artifact")]
            pub(crate) artifact: crate::protocol::generated::Artifact
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct HarExportArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "harId")]
            pub(crate) har_id: Option<&'a str>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct HarStart {
            #[serde(rename = "harId")]
            pub(crate) har_id: String
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct HarStartArgs {
            #[serde(rename = "options")]
            pub(crate) options: crate::protocol::generated::RecordHarOptions,
            #[serde(rename = "page")]
            pub(crate) page: Option<crate::protocol::generated::Page>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct NewCdpSession {
            #[serde(rename = "session")]
            pub(crate) session: crate::protocol::generated::CdpSession
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct NewCdpSessionArgs {
            #[serde(rename = "frame")]
            pub(crate) frame: Option<crate::protocol::generated::Frame>,
            #[serde(rename = "page")]
            pub(crate) page: Option<crate::protocol::generated::Page>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct NewPage {
            #[serde(rename = "page")]
            pub(crate) page: crate::protocol::generated::Page
        }
        pub type NewPageArgs = ();
        pub type Pause = ();
        pub type PauseArgs = ();
        pub type RecorderSupplementEnable = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct RecorderSupplementEnableArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "contextOptions")]
            pub(crate) context_options: Option<&'a str>,
            #[serde(borrow)]
            #[serde(rename = "device")]
            pub(crate) device: Option<&'a str>,
            #[serde(borrow)]
            #[serde(rename = "language")]
            pub(crate) language: Option<&'a str>,
            #[serde(borrow)]
            #[serde(rename = "launchOptions")]
            pub(crate) launch_options: Option<&'a str>,
            #[serde(borrow)]
            #[serde(rename = "outputFile")]
            pub(crate) output_file: Option<&'a str>,
            #[serde(rename = "pauseOnNextStatement")]
            pub(crate) pause_on_next_statement: Option<bool>,
            #[serde(borrow)]
            #[serde(rename = "saveStorage")]
            pub(crate) save_storage: Option<&'a str>,
            #[serde(rename = "startRecording")]
            pub(crate) start_recording: Option<bool>
        }
        pub type SetDefaultNavigationTimeoutNoReply = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetDefaultNavigationTimeoutNoReplyArgs {
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>
        }
        pub type SetDefaultTimeoutNoReply = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetDefaultTimeoutNoReplyArgs {
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>
        }
        pub type SetExtraHttpHeaders = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetExtraHttpHeadersArgs {
            #[serde(rename = "headers")]
            pub(crate) headers: Vec<crate::protocol::generated::NameValue>
        }
        pub type SetGeolocation = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetGeolocationArgs {
            #[serde(rename = "geolocation")]
            pub(crate) geolocation: Option<SetGeolocationArgsGeolocation>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetGeolocationArgsGeolocation {
            #[serde(rename = "accuracy")]
            pub(crate) accuracy: Option<serde_json::Number>,
            #[serde(rename = "latitude")]
            pub(crate) latitude: serde_json::Number,
            #[serde(rename = "longitude")]
            pub(crate) longitude: serde_json::Number
        }
        pub type SetHttpCredentials = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetHttpCredentialsArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "httpCredentials")]
            pub(crate) http_credentials: Option<SetHttpCredentialsArgsHttpCredentials<'a>>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetHttpCredentialsArgsHttpCredentials<'a> {
            #[serde(borrow)]
            #[serde(rename = "password")]
            pub(crate) password: &'a str,
            #[serde(borrow)]
            #[serde(rename = "username")]
            pub(crate) username: &'a str
        }
        pub type SetNetworkInterceptionEnabled = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetNetworkInterceptionEnabledArgs {
            #[serde(rename = "enabled")]
            pub(crate) enabled: bool
        }
        pub type SetOffline = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetOfflineArgs {
            #[serde(rename = "offline")]
            pub(crate) offline: bool
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct StorageState {
            #[serde(rename = "cookies")]
            pub(crate) cookies: Vec<crate::protocol::generated::NetworkCookie>,
            #[serde(rename = "origins")]
            pub(crate) origins: Vec<crate::protocol::generated::OriginStorage>
        }
        pub type StorageStateArgs = ();
    }
}
pub(crate) type BrowserType = OnlyGuid;
pub mod browser_type {
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Initializer {
        #[serde(rename = "executablePath")]
        pub(crate) executable_path: String,
        #[serde(rename = "name")]
        pub(crate) name: String
    }
    pub mod commands {
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Connect {
            #[serde(rename = "pipe")]
            pub(crate) pipe: crate::protocol::generated::JsonPipe
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ConnectArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "headers")]
            pub(crate) headers: Option<&'a str>,
            #[serde(rename = "slowMo")]
            pub(crate) slow_mo: Option<serde_json::Number>,
            #[serde(rename = "socksProxyRedirectPortForTest")]
            pub(crate) socks_proxy_redirect_port_for_test: Option<serde_json::Number>,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>,
            #[serde(borrow)]
            #[serde(rename = "wsEndpoint")]
            pub(crate) ws_endpoint: &'a str
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ConnectOverCdp {
            #[serde(rename = "browser")]
            pub(crate) browser: crate::protocol::generated::Browser,
            #[serde(rename = "defaultContext")]
            pub(crate) default_context: Option<crate::protocol::generated::BrowserContext>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ConnectOverCdpArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "endpointURL")]
            pub(crate) endpoint_url: &'a str,
            #[serde(rename = "headers")]
            pub(crate) headers: Option<Vec<crate::protocol::generated::NameValue>>,
            #[serde(rename = "slowMo")]
            pub(crate) slow_mo: Option<serde_json::Number>,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Launch {
            #[serde(rename = "browser")]
            pub(crate) browser: crate::protocol::generated::Browser
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct LaunchArgs<'a> {
            #[serde(flatten)]
            #[serde(rename = "$mixin")]
            pub(crate) mixin: crate::protocol::generated::LaunchOptions,
            #[serde(borrow)]
            #[serde(rename = "firefoxUserPrefs")]
            pub(crate) firefox_user_prefs: Option<&'a str>,
            #[serde(rename = "slowMo")]
            pub(crate) slow_mo: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct LaunchPersistentContext {
            #[serde(rename = "context")]
            pub(crate) context: crate::protocol::generated::BrowserContext
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct LaunchPersistentContextArgs<'a> {
            #[serde(flatten)]
            #[serde(rename = "$mixin1")]
            pub(crate) mixin1: crate::protocol::generated::LaunchOptions,
            #[serde(flatten)]
            #[serde(rename = "$mixin2")]
            pub(crate) mixin2: crate::protocol::generated::ContextOptions,
            #[serde(rename = "slowMo")]
            pub(crate) slow_mo: Option<serde_json::Number>,
            #[serde(borrow)]
            #[serde(rename = "userDataDir")]
            pub(crate) user_data_dir: &'a str
        }
    }
}
pub(crate) type CdpSession = OnlyGuid;
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
            pub(crate) method: String,
            #[serde(rename = "params")]
            pub(crate) params: Option<String>
        }
    }
    pub mod commands {
        pub type Detach = ();
        pub type DetachArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Send {
            #[serde(rename = "result")]
            pub(crate) result: String
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SendArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "method")]
            pub(crate) method: &'a str,
            #[serde(borrow)]
            #[serde(rename = "params")]
            pub(crate) params: Option<&'a str>
        }
    }
}
pub(crate) type ConsoleMessage = OnlyGuid;
pub mod console_message {
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Initializer {
        #[serde(rename = "args")]
        pub(crate) args: Vec<crate::protocol::generated::JsHandle>,
        #[serde(rename = "location")]
        pub(crate) location: InitializerLocation,
        #[serde(rename = "text")]
        pub(crate) text: String,
        #[serde(rename = "type")]
        pub(crate) r#type: String
    }
    #[derive(Debug, Serialize, Deserialize)]
    pub struct InitializerLocation {
        #[serde(rename = "columnNumber")]
        pub(crate) column_number: serde_json::Number,
        #[serde(rename = "lineNumber")]
        pub(crate) line_number: serde_json::Number,
        #[serde(rename = "url")]
        pub(crate) url: String
    }
}
pub(crate) type Dialog = OnlyGuid;
pub mod dialog {
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Initializer {
        #[serde(rename = "defaultValue")]
        pub(crate) default_value: String,
        #[serde(rename = "message")]
        pub(crate) message: String,
        #[serde(rename = "type")]
        pub(crate) r#type: String
    }
    pub mod commands {
        pub type Accept = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct AcceptArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "promptText")]
            pub(crate) prompt_text: Option<&'a str>
        }
        pub type Dismiss = ();
        pub type DismissArgs = ();
    }
}
pub(crate) type Electron = OnlyGuid;
pub mod electron {
    pub mod commands {
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Launch {
            #[serde(rename = "electronApplication")]
            pub(crate) electron_application: crate::protocol::generated::ElectronApplication
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct LaunchArgs<'a> {
            #[serde(rename = "acceptDownloads")]
            pub(crate) accept_downloads: Option<bool>,
            #[serde(borrow)]
            #[serde(rename = "args")]
            pub(crate) args: Option<Vec<&'a str>>,
            #[serde(rename = "bypassCSP")]
            pub(crate) bypass_csp: Option<bool>,
            #[serde(rename = "colorScheme")]
            pub(crate) color_scheme: Option<LaunchArgsColorScheme>,
            #[serde(borrow)]
            #[serde(rename = "cwd")]
            pub(crate) cwd: Option<&'a str>,
            #[serde(rename = "env")]
            pub(crate) env: Option<Vec<crate::protocol::generated::NameValue>>,
            #[serde(borrow)]
            #[serde(rename = "executablePath")]
            pub(crate) executable_path: Option<&'a str>,
            #[serde(rename = "extraHTTPHeaders")]
            pub(crate) extra_http_headers: Option<Vec<crate::protocol::generated::NameValue>>,
            #[serde(rename = "geolocation")]
            pub(crate) geolocation: Option<LaunchArgsGeolocation>,
            #[serde(borrow)]
            #[serde(rename = "httpCredentials")]
            pub(crate) http_credentials: Option<LaunchArgsHttpCredentials<'a>>,
            #[serde(rename = "ignoreHTTPSErrors")]
            pub(crate) ignore_https_errors: Option<bool>,
            #[serde(borrow)]
            #[serde(rename = "locale")]
            pub(crate) locale: Option<&'a str>,
            #[serde(rename = "offline")]
            pub(crate) offline: Option<bool>,
            #[serde(rename = "recordHar")]
            pub(crate) record_har: Option<crate::protocol::generated::RecordHarOptions>,
            #[serde(borrow)]
            #[serde(rename = "recordVideo")]
            pub(crate) record_video: Option<LaunchArgsRecordVideo<'a>>,
            #[serde(rename = "strictSelectors")]
            pub(crate) strict_selectors: Option<bool>,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>,
            #[serde(borrow)]
            #[serde(rename = "timezoneId")]
            pub(crate) timezone_id: Option<&'a str>
        }
        #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
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
            pub(crate) accuracy: Option<serde_json::Number>,
            #[serde(rename = "latitude")]
            pub(crate) latitude: serde_json::Number,
            #[serde(rename = "longitude")]
            pub(crate) longitude: serde_json::Number
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct LaunchArgsHttpCredentials<'a> {
            #[serde(borrow)]
            #[serde(rename = "password")]
            pub(crate) password: &'a str,
            #[serde(borrow)]
            #[serde(rename = "username")]
            pub(crate) username: &'a str
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct LaunchArgsRecordVideo<'a> {
            #[serde(borrow)]
            #[serde(rename = "dir")]
            pub(crate) dir: &'a str,
            #[serde(rename = "size")]
            pub(crate) size: Option<LaunchArgsRecordVideoSize>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct LaunchArgsRecordVideoSize {
            #[serde(rename = "height")]
            pub(crate) height: serde_json::Number,
            #[serde(rename = "width")]
            pub(crate) width: serde_json::Number
        }
    }
}
pub(crate) type ElectronApplication = OnlyGuid;
pub mod electron_application {
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Initializer {
        #[serde(rename = "context")]
        pub(crate) context: crate::protocol::generated::BrowserContext
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
            pub(crate) handle: crate::protocol::generated::JsHandle
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct BrowserWindowArgs {
            #[serde(rename = "page")]
            pub(crate) page: crate::protocol::generated::Page
        }
        pub type Close = ();
        pub type CloseArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct EvaluateExpression {
            #[serde(rename = "value")]
            pub(crate) value: crate::protocol::generated::SerializedValue
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct EvaluateExpressionArgs<'a> {
            #[serde(rename = "arg")]
            pub(crate) arg: crate::protocol::generated::SerializedArgument,
            #[serde(borrow)]
            #[serde(rename = "expression")]
            pub(crate) expression: &'a str,
            #[serde(rename = "isFunction")]
            pub(crate) is_function: Option<bool>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct EvaluateExpressionHandle {
            #[serde(rename = "handle")]
            pub(crate) handle: crate::protocol::generated::JsHandle
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct EvaluateExpressionHandleArgs<'a> {
            #[serde(rename = "arg")]
            pub(crate) arg: crate::protocol::generated::SerializedArgument,
            #[serde(borrow)]
            #[serde(rename = "expression")]
            pub(crate) expression: &'a str,
            #[serde(rename = "isFunction")]
            pub(crate) is_function: Option<bool>
        }
    }
}
pub(crate) type ElementHandle = OnlyGuid;
/// Extends JSHandle
pub mod element_handle {
    pub mod commands {
        #[derive(Debug, Serialize, Deserialize)]
        pub struct BoundingBox {
            #[serde(rename = "value")]
            pub(crate) value: Option<crate::protocol::generated::Rect>
        }
        pub type BoundingBoxArgs = ();
        pub type Check = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct CheckArgs {
            #[serde(rename = "force")]
            pub(crate) force: Option<bool>,
            #[serde(rename = "noWaitAfter")]
            pub(crate) no_wait_after: Option<bool>,
            #[serde(rename = "position")]
            pub(crate) position: Option<crate::protocol::generated::Point>,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>,
            #[serde(rename = "trial")]
            pub(crate) trial: Option<bool>
        }
        pub type Click = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ClickArgs {
            #[serde(rename = "button")]
            pub(crate) button: Option<ClickArgsButton>,
            #[serde(rename = "clickCount")]
            pub(crate) click_count: Option<serde_json::Number>,
            #[serde(rename = "delay")]
            pub(crate) delay: Option<serde_json::Number>,
            #[serde(rename = "force")]
            pub(crate) force: Option<bool>,
            #[serde(rename = "modifiers")]
            pub(crate) modifiers: Option<Vec<ClickArgsModifiers>>,
            #[serde(rename = "noWaitAfter")]
            pub(crate) no_wait_after: Option<bool>,
            #[serde(rename = "position")]
            pub(crate) position: Option<crate::protocol::generated::Point>,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>,
            #[serde(rename = "trial")]
            pub(crate) trial: Option<bool>
        }
        #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
        pub enum ClickArgsButton {
            #[serde(rename = "left")]
            Left,
            #[serde(rename = "right")]
            Right,
            #[serde(rename = "middle")]
            Middle
        }
        #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
        pub enum ClickArgsModifiers {
            Alt,
            Control,
            Meta,
            Shift
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ContentFrame {
            #[serde(rename = "frame")]
            pub(crate) frame: Option<crate::protocol::generated::Frame>
        }
        pub type ContentFrameArgs = ();
        pub type Dblclick = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct DblclickArgs {
            #[serde(rename = "button")]
            pub(crate) button: Option<DblclickArgsButton>,
            #[serde(rename = "delay")]
            pub(crate) delay: Option<serde_json::Number>,
            #[serde(rename = "force")]
            pub(crate) force: Option<bool>,
            #[serde(rename = "modifiers")]
            pub(crate) modifiers: Option<Vec<DblclickArgsModifiers>>,
            #[serde(rename = "noWaitAfter")]
            pub(crate) no_wait_after: Option<bool>,
            #[serde(rename = "position")]
            pub(crate) position: Option<crate::protocol::generated::Point>,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>,
            #[serde(rename = "trial")]
            pub(crate) trial: Option<bool>
        }
        #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
        pub enum DblclickArgsButton {
            #[serde(rename = "left")]
            Left,
            #[serde(rename = "right")]
            Right,
            #[serde(rename = "middle")]
            Middle
        }
        #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
        pub enum DblclickArgsModifiers {
            Alt,
            Control,
            Meta,
            Shift
        }
        pub type DispatchEvent = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct DispatchEventArgs<'a> {
            #[serde(rename = "eventInit")]
            pub(crate) event_init: crate::protocol::generated::SerializedArgument,
            #[serde(borrow)]
            #[serde(rename = "type")]
            pub(crate) r#type: &'a str
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct EvalOnSelector {
            #[serde(rename = "value")]
            pub(crate) value: crate::protocol::generated::SerializedValue
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct EvalOnSelectorArgs<'a> {
            #[serde(rename = "arg")]
            pub(crate) arg: crate::protocol::generated::SerializedArgument,
            #[serde(borrow)]
            #[serde(rename = "expression")]
            pub(crate) expression: &'a str,
            #[serde(rename = "isFunction")]
            pub(crate) is_function: Option<bool>,
            #[serde(borrow)]
            #[serde(rename = "selector")]
            pub(crate) selector: &'a str,
            #[serde(rename = "strict")]
            pub(crate) strict: Option<bool>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct EvalOnSelectorAll {
            #[serde(rename = "value")]
            pub(crate) value: crate::protocol::generated::SerializedValue
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct EvalOnSelectorAllArgs<'a> {
            #[serde(rename = "arg")]
            pub(crate) arg: crate::protocol::generated::SerializedArgument,
            #[serde(borrow)]
            #[serde(rename = "expression")]
            pub(crate) expression: &'a str,
            #[serde(rename = "isFunction")]
            pub(crate) is_function: Option<bool>,
            #[serde(borrow)]
            #[serde(rename = "selector")]
            pub(crate) selector: &'a str
        }
        pub type Fill = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct FillArgs<'a> {
            #[serde(rename = "force")]
            pub(crate) force: Option<bool>,
            #[serde(rename = "noWaitAfter")]
            pub(crate) no_wait_after: Option<bool>,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>,
            #[serde(borrow)]
            #[serde(rename = "value")]
            pub(crate) value: &'a str
        }
        pub type Focus = ();
        pub type FocusArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetAttribute {
            #[serde(rename = "value")]
            pub(crate) value: Option<String>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetAttributeArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "name")]
            pub(crate) name: &'a str
        }
        pub type Hover = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct HoverArgs {
            #[serde(rename = "force")]
            pub(crate) force: Option<bool>,
            #[serde(rename = "modifiers")]
            pub(crate) modifiers: Option<Vec<HoverArgsModifiers>>,
            #[serde(rename = "position")]
            pub(crate) position: Option<crate::protocol::generated::Point>,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>,
            #[serde(rename = "trial")]
            pub(crate) trial: Option<bool>
        }
        #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
        pub enum HoverArgsModifiers {
            Alt,
            Control,
            Meta,
            Shift
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct InnerHtml {
            #[serde(rename = "value")]
            pub(crate) value: String
        }
        pub type InnerHtmlArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct InnerText {
            #[serde(rename = "value")]
            pub(crate) value: String
        }
        pub type InnerTextArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct InputValue {
            #[serde(rename = "value")]
            pub(crate) value: String
        }
        pub type InputValueArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct IsChecked {
            #[serde(rename = "value")]
            pub(crate) value: bool
        }
        pub type IsCheckedArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct IsDisabled {
            #[serde(rename = "value")]
            pub(crate) value: bool
        }
        pub type IsDisabledArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct IsEditable {
            #[serde(rename = "value")]
            pub(crate) value: bool
        }
        pub type IsEditableArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct IsEnabled {
            #[serde(rename = "value")]
            pub(crate) value: bool
        }
        pub type IsEnabledArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct IsHidden {
            #[serde(rename = "value")]
            pub(crate) value: bool
        }
        pub type IsHiddenArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct IsVisible {
            #[serde(rename = "value")]
            pub(crate) value: bool
        }
        pub type IsVisibleArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct OwnerFrame {
            #[serde(rename = "frame")]
            pub(crate) frame: Option<crate::protocol::generated::Frame>
        }
        pub type OwnerFrameArgs = ();
        pub type Press = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct PressArgs<'a> {
            #[serde(rename = "delay")]
            pub(crate) delay: Option<serde_json::Number>,
            #[serde(borrow)]
            #[serde(rename = "key")]
            pub(crate) key: &'a str,
            #[serde(rename = "noWaitAfter")]
            pub(crate) no_wait_after: Option<bool>,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct QuerySelector {
            #[serde(rename = "element")]
            pub(crate) element: Option<crate::protocol::generated::ElementHandle>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct QuerySelectorArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "selector")]
            pub(crate) selector: &'a str,
            #[serde(rename = "strict")]
            pub(crate) strict: Option<bool>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct QuerySelectorAll {
            #[serde(rename = "elements")]
            pub(crate) elements: Vec<crate::protocol::generated::ElementHandle>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct QuerySelectorAllArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "selector")]
            pub(crate) selector: &'a str
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Screenshot {
            #[serde(rename = "binary")]
            pub(crate) binary: Vec<u8>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ScreenshotArgs {
            #[serde(flatten)]
            #[serde(rename = "$mixin")]
            pub(crate) mixin: crate::protocol::generated::CommonScreenshotOptions,
            #[serde(rename = "quality")]
            pub(crate) quality: Option<serde_json::Number>,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>,
            #[serde(rename = "type")]
            pub(crate) r#type: Option<ScreenshotArgsType>
        }
        #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
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
            pub(crate) timeout: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SelectOption {
            #[serde(rename = "values")]
            pub(crate) values: Vec<String>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SelectOptionArgs<'a> {
            #[serde(rename = "elements")]
            pub(crate) elements: Option<Vec<crate::protocol::generated::ElementHandle>>,
            #[serde(rename = "force")]
            pub(crate) force: Option<bool>,
            #[serde(rename = "noWaitAfter")]
            pub(crate) no_wait_after: Option<bool>,
            #[serde(borrow)]
            #[serde(rename = "options")]
            pub(crate) options: Option<Vec<SelectOptionArgsOptions<'a>>>,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SelectOptionArgsOptions<'a> {
            #[serde(rename = "index")]
            pub(crate) index: Option<serde_json::Number>,
            #[serde(borrow)]
            #[serde(rename = "label")]
            pub(crate) label: Option<&'a str>,
            #[serde(borrow)]
            #[serde(rename = "value")]
            pub(crate) value: Option<&'a str>
        }
        pub type SelectText = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SelectTextArgs {
            #[serde(rename = "force")]
            pub(crate) force: Option<bool>,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>
        }
        pub type SetInputFilePaths = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetInputFilePathsArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "localPaths")]
            pub(crate) local_paths: Option<Vec<&'a str>>,
            #[serde(rename = "noWaitAfter")]
            pub(crate) no_wait_after: Option<bool>,
            #[serde(rename = "streams")]
            pub(crate) streams: Option<Vec<crate::protocol::generated::WritableStream>>,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>
        }
        pub type SetInputFiles = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetInputFilesArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "files")]
            pub(crate) files: Vec<SetInputFilesArgsFiles<'a>>,
            #[serde(rename = "noWaitAfter")]
            pub(crate) no_wait_after: Option<bool>,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetInputFilesArgsFiles<'a> {
            #[serde(borrow)]
            #[serde(rename = "buffer")]
            pub(crate) buffer: &'a [u8],
            #[serde(borrow)]
            #[serde(rename = "mimeType")]
            pub(crate) mime_type: Option<&'a str>,
            #[serde(borrow)]
            #[serde(rename = "name")]
            pub(crate) name: &'a str
        }
        pub type Tap = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct TapArgs {
            #[serde(rename = "force")]
            pub(crate) force: Option<bool>,
            #[serde(rename = "modifiers")]
            pub(crate) modifiers: Option<Vec<TapArgsModifiers>>,
            #[serde(rename = "noWaitAfter")]
            pub(crate) no_wait_after: Option<bool>,
            #[serde(rename = "position")]
            pub(crate) position: Option<crate::protocol::generated::Point>,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>,
            #[serde(rename = "trial")]
            pub(crate) trial: Option<bool>
        }
        #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
        pub enum TapArgsModifiers {
            Alt,
            Control,
            Meta,
            Shift
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct TextContent {
            #[serde(rename = "value")]
            pub(crate) value: Option<String>
        }
        pub type TextContentArgs = ();
        pub type Type = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct TypeArgs<'a> {
            #[serde(rename = "delay")]
            pub(crate) delay: Option<serde_json::Number>,
            #[serde(rename = "noWaitAfter")]
            pub(crate) no_wait_after: Option<bool>,
            #[serde(borrow)]
            #[serde(rename = "text")]
            pub(crate) text: &'a str,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>
        }
        pub type Uncheck = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct UncheckArgs {
            #[serde(rename = "force")]
            pub(crate) force: Option<bool>,
            #[serde(rename = "noWaitAfter")]
            pub(crate) no_wait_after: Option<bool>,
            #[serde(rename = "position")]
            pub(crate) position: Option<crate::protocol::generated::Point>,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>,
            #[serde(rename = "trial")]
            pub(crate) trial: Option<bool>
        }
        pub type WaitForElementState = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct WaitForElementStateArgs {
            #[serde(rename = "state")]
            pub(crate) state: WaitForElementStateArgsState,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
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
            pub(crate) element: Option<crate::protocol::generated::ElementHandle>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct WaitForSelectorArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "selector")]
            pub(crate) selector: &'a str,
            #[serde(rename = "state")]
            pub(crate) state: Option<WaitForSelectorArgsState>,
            #[serde(rename = "strict")]
            pub(crate) strict: Option<bool>,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
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
pub(crate) type EventTarget = OnlyGuid;
pub mod event_target {
    pub mod commands {
        pub type WaitForEventInfo = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct WaitForEventInfoArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "info")]
            pub(crate) info: WaitForEventInfoArgsInfo<'a>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct WaitForEventInfoArgsInfo<'a> {
            #[serde(borrow)]
            #[serde(rename = "error")]
            pub(crate) error: Option<&'a str>,
            #[serde(borrow)]
            #[serde(rename = "event")]
            pub(crate) event: Option<&'a str>,
            #[serde(borrow)]
            #[serde(rename = "message")]
            pub(crate) message: Option<&'a str>,
            #[serde(rename = "phase")]
            pub(crate) phase: WaitForEventInfoArgsInfoPhase,
            #[serde(borrow)]
            #[serde(rename = "waitId")]
            pub(crate) wait_id: &'a str
        }
        #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
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
pub(crate) type Frame = OnlyGuid;
pub mod frame {
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Initializer {
        #[serde(rename = "loadStates")]
        pub(crate) load_states: Vec<crate::protocol::generated::LifecycleEvent>,
        #[serde(rename = "name")]
        pub(crate) name: String,
        #[serde(rename = "parentFrame")]
        pub(crate) parent_frame: Option<crate::protocol::generated::Frame>,
        #[serde(rename = "url")]
        pub(crate) url: String
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
            pub(crate) add: Option<crate::protocol::generated::LifecycleEvent>,
            #[serde(rename = "remove")]
            pub(crate) remove: Option<crate::protocol::generated::LifecycleEvent>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Navigated {
            #[serde(rename = "error")]
            pub(crate) error: Option<String>,
            #[serde(rename = "name")]
            pub(crate) name: String,
            #[serde(rename = "newDocument")]
            pub(crate) new_document: Option<NavigatedNewDocument>,
            #[serde(rename = "url")]
            pub(crate) url: String
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct NavigatedNewDocument {
            #[serde(rename = "request")]
            pub(crate) request: Option<crate::protocol::generated::Request>
        }
    }
    pub mod commands {
        #[derive(Debug, Serialize, Deserialize)]
        pub struct AddScriptTag {
            #[serde(rename = "element")]
            pub(crate) element: crate::protocol::generated::ElementHandle
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct AddScriptTagArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "content")]
            pub(crate) content: Option<&'a str>,
            #[serde(borrow)]
            #[serde(rename = "type")]
            pub(crate) r#type: Option<&'a str>,
            #[serde(borrow)]
            #[serde(rename = "url")]
            pub(crate) url: Option<&'a str>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct AddStyleTag {
            #[serde(rename = "element")]
            pub(crate) element: crate::protocol::generated::ElementHandle
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct AddStyleTagArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "content")]
            pub(crate) content: Option<&'a str>,
            #[serde(borrow)]
            #[serde(rename = "url")]
            pub(crate) url: Option<&'a str>
        }
        pub type Check = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct CheckArgs<'a> {
            #[serde(rename = "force")]
            pub(crate) force: Option<bool>,
            #[serde(rename = "noWaitAfter")]
            pub(crate) no_wait_after: Option<bool>,
            #[serde(rename = "position")]
            pub(crate) position: Option<crate::protocol::generated::Point>,
            #[serde(borrow)]
            #[serde(rename = "selector")]
            pub(crate) selector: &'a str,
            #[serde(rename = "strict")]
            pub(crate) strict: Option<bool>,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>,
            #[serde(rename = "trial")]
            pub(crate) trial: Option<bool>
        }
        pub type Click = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ClickArgs<'a> {
            #[serde(rename = "button")]
            pub(crate) button: Option<ClickArgsButton>,
            #[serde(rename = "clickCount")]
            pub(crate) click_count: Option<serde_json::Number>,
            #[serde(rename = "delay")]
            pub(crate) delay: Option<serde_json::Number>,
            #[serde(rename = "force")]
            pub(crate) force: Option<bool>,
            #[serde(rename = "modifiers")]
            pub(crate) modifiers: Option<Vec<ClickArgsModifiers>>,
            #[serde(rename = "noWaitAfter")]
            pub(crate) no_wait_after: Option<bool>,
            #[serde(rename = "position")]
            pub(crate) position: Option<crate::protocol::generated::Point>,
            #[serde(borrow)]
            #[serde(rename = "selector")]
            pub(crate) selector: &'a str,
            #[serde(rename = "strict")]
            pub(crate) strict: Option<bool>,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>,
            #[serde(rename = "trial")]
            pub(crate) trial: Option<bool>
        }
        #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
        pub enum ClickArgsButton {
            #[serde(rename = "left")]
            Left,
            #[serde(rename = "right")]
            Right,
            #[serde(rename = "middle")]
            Middle
        }
        #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
        pub enum ClickArgsModifiers {
            Alt,
            Control,
            Meta,
            Shift
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Content {
            #[serde(rename = "value")]
            pub(crate) value: String
        }
        pub type ContentArgs = ();
        pub type Dblclick = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct DblclickArgs<'a> {
            #[serde(rename = "button")]
            pub(crate) button: Option<DblclickArgsButton>,
            #[serde(rename = "delay")]
            pub(crate) delay: Option<serde_json::Number>,
            #[serde(rename = "force")]
            pub(crate) force: Option<bool>,
            #[serde(rename = "modifiers")]
            pub(crate) modifiers: Option<Vec<DblclickArgsModifiers>>,
            #[serde(rename = "noWaitAfter")]
            pub(crate) no_wait_after: Option<bool>,
            #[serde(rename = "position")]
            pub(crate) position: Option<crate::protocol::generated::Point>,
            #[serde(borrow)]
            #[serde(rename = "selector")]
            pub(crate) selector: &'a str,
            #[serde(rename = "strict")]
            pub(crate) strict: Option<bool>,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>,
            #[serde(rename = "trial")]
            pub(crate) trial: Option<bool>
        }
        #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
        pub enum DblclickArgsButton {
            #[serde(rename = "left")]
            Left,
            #[serde(rename = "right")]
            Right,
            #[serde(rename = "middle")]
            Middle
        }
        #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
        pub enum DblclickArgsModifiers {
            Alt,
            Control,
            Meta,
            Shift
        }
        pub type DispatchEvent = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct DispatchEventArgs<'a> {
            #[serde(rename = "eventInit")]
            pub(crate) event_init: crate::protocol::generated::SerializedArgument,
            #[serde(borrow)]
            #[serde(rename = "selector")]
            pub(crate) selector: &'a str,
            #[serde(rename = "strict")]
            pub(crate) strict: Option<bool>,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>,
            #[serde(borrow)]
            #[serde(rename = "type")]
            pub(crate) r#type: &'a str
        }
        pub type DragAndDrop = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct DragAndDropArgs<'a> {
            #[serde(rename = "force")]
            pub(crate) force: Option<bool>,
            #[serde(rename = "noWaitAfter")]
            pub(crate) no_wait_after: Option<bool>,
            #[serde(borrow)]
            #[serde(rename = "source")]
            pub(crate) source: &'a str,
            #[serde(rename = "sourcePosition")]
            pub(crate) source_position: Option<crate::protocol::generated::Point>,
            #[serde(rename = "strict")]
            pub(crate) strict: Option<bool>,
            #[serde(borrow)]
            #[serde(rename = "target")]
            pub(crate) target: &'a str,
            #[serde(rename = "targetPosition")]
            pub(crate) target_position: Option<crate::protocol::generated::Point>,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>,
            #[serde(rename = "trial")]
            pub(crate) trial: Option<bool>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct EvalOnSelector {
            #[serde(rename = "value")]
            pub(crate) value: crate::protocol::generated::SerializedValue
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct EvalOnSelectorArgs<'a> {
            #[serde(rename = "arg")]
            pub(crate) arg: crate::protocol::generated::SerializedArgument,
            #[serde(borrow)]
            #[serde(rename = "expression")]
            pub(crate) expression: &'a str,
            #[serde(rename = "isFunction")]
            pub(crate) is_function: Option<bool>,
            #[serde(borrow)]
            #[serde(rename = "selector")]
            pub(crate) selector: &'a str,
            #[serde(rename = "strict")]
            pub(crate) strict: Option<bool>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct EvalOnSelectorAll {
            #[serde(rename = "value")]
            pub(crate) value: crate::protocol::generated::SerializedValue
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct EvalOnSelectorAllArgs<'a> {
            #[serde(rename = "arg")]
            pub(crate) arg: crate::protocol::generated::SerializedArgument,
            #[serde(borrow)]
            #[serde(rename = "expression")]
            pub(crate) expression: &'a str,
            #[serde(rename = "isFunction")]
            pub(crate) is_function: Option<bool>,
            #[serde(borrow)]
            #[serde(rename = "selector")]
            pub(crate) selector: &'a str
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct EvaluateExpression {
            #[serde(rename = "value")]
            pub(crate) value: crate::protocol::generated::SerializedValue
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct EvaluateExpressionArgs<'a> {
            #[serde(rename = "arg")]
            pub(crate) arg: crate::protocol::generated::SerializedArgument,
            #[serde(borrow)]
            #[serde(rename = "expression")]
            pub(crate) expression: &'a str,
            #[serde(rename = "isFunction")]
            pub(crate) is_function: Option<bool>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct EvaluateExpressionHandle {
            #[serde(rename = "handle")]
            pub(crate) handle: crate::protocol::generated::JsHandle
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct EvaluateExpressionHandleArgs<'a> {
            #[serde(rename = "arg")]
            pub(crate) arg: crate::protocol::generated::SerializedArgument,
            #[serde(borrow)]
            #[serde(rename = "expression")]
            pub(crate) expression: &'a str,
            #[serde(rename = "isFunction")]
            pub(crate) is_function: Option<bool>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Expect {
            #[serde(rename = "log")]
            pub(crate) log: Option<Vec<String>>,
            #[serde(rename = "matches")]
            pub(crate) matches: bool,
            #[serde(rename = "received")]
            pub(crate) received: Option<crate::protocol::generated::SerializedValue>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ExpectArgs<'a> {
            #[serde(rename = "expectedNumber")]
            pub(crate) expected_number: Option<serde_json::Number>,
            #[serde(rename = "expectedText")]
            pub(crate) expected_text: Option<Vec<crate::protocol::generated::ExpectedTextValue>>,
            #[serde(rename = "expectedValue")]
            pub(crate) expected_value: Option<crate::protocol::generated::SerializedArgument>,
            #[serde(borrow)]
            #[serde(rename = "expression")]
            pub(crate) expression: &'a str,
            #[serde(borrow)]
            #[serde(rename = "expressionArg")]
            pub(crate) expression_arg: Option<&'a str>,
            #[serde(rename = "isNot")]
            pub(crate) is_not: bool,
            #[serde(borrow)]
            #[serde(rename = "selector")]
            pub(crate) selector: &'a str,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>,
            #[serde(rename = "useInnerText")]
            pub(crate) use_inner_text: Option<bool>
        }
        pub type Fill = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct FillArgs<'a> {
            #[serde(rename = "force")]
            pub(crate) force: Option<bool>,
            #[serde(rename = "noWaitAfter")]
            pub(crate) no_wait_after: Option<bool>,
            #[serde(borrow)]
            #[serde(rename = "selector")]
            pub(crate) selector: &'a str,
            #[serde(rename = "strict")]
            pub(crate) strict: Option<bool>,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>,
            #[serde(borrow)]
            #[serde(rename = "value")]
            pub(crate) value: &'a str
        }
        pub type Focus = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct FocusArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "selector")]
            pub(crate) selector: &'a str,
            #[serde(rename = "strict")]
            pub(crate) strict: Option<bool>,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct FrameElement {
            #[serde(rename = "element")]
            pub(crate) element: crate::protocol::generated::ElementHandle
        }
        pub type FrameElementArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetAttribute {
            #[serde(rename = "value")]
            pub(crate) value: Option<String>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetAttributeArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "name")]
            pub(crate) name: &'a str,
            #[serde(borrow)]
            #[serde(rename = "selector")]
            pub(crate) selector: &'a str,
            #[serde(rename = "strict")]
            pub(crate) strict: Option<bool>,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Goto {
            #[serde(rename = "response")]
            pub(crate) response: Option<crate::protocol::generated::Response>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct GotoArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "referer")]
            pub(crate) referer: Option<&'a str>,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>,
            #[serde(borrow)]
            #[serde(rename = "url")]
            pub(crate) url: &'a str,
            #[serde(rename = "waitUntil")]
            pub(crate) wait_until: Option<crate::protocol::generated::LifecycleEvent>
        }
        pub type Highlight = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct HighlightArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "selector")]
            pub(crate) selector: &'a str
        }
        pub type Hover = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct HoverArgs<'a> {
            #[serde(rename = "force")]
            pub(crate) force: Option<bool>,
            #[serde(rename = "modifiers")]
            pub(crate) modifiers: Option<Vec<HoverArgsModifiers>>,
            #[serde(rename = "position")]
            pub(crate) position: Option<crate::protocol::generated::Point>,
            #[serde(borrow)]
            #[serde(rename = "selector")]
            pub(crate) selector: &'a str,
            #[serde(rename = "strict")]
            pub(crate) strict: Option<bool>,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>,
            #[serde(rename = "trial")]
            pub(crate) trial: Option<bool>
        }
        #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
        pub enum HoverArgsModifiers {
            Alt,
            Control,
            Meta,
            Shift
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct InnerHtml {
            #[serde(rename = "value")]
            pub(crate) value: String
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct InnerHtmlArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "selector")]
            pub(crate) selector: &'a str,
            #[serde(rename = "strict")]
            pub(crate) strict: Option<bool>,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct InnerText {
            #[serde(rename = "value")]
            pub(crate) value: String
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct InnerTextArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "selector")]
            pub(crate) selector: &'a str,
            #[serde(rename = "strict")]
            pub(crate) strict: Option<bool>,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct InputValue {
            #[serde(rename = "value")]
            pub(crate) value: String
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct InputValueArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "selector")]
            pub(crate) selector: &'a str,
            #[serde(rename = "strict")]
            pub(crate) strict: Option<bool>,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct IsChecked {
            #[serde(rename = "value")]
            pub(crate) value: bool
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct IsCheckedArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "selector")]
            pub(crate) selector: &'a str,
            #[serde(rename = "strict")]
            pub(crate) strict: Option<bool>,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct IsDisabled {
            #[serde(rename = "value")]
            pub(crate) value: bool
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct IsDisabledArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "selector")]
            pub(crate) selector: &'a str,
            #[serde(rename = "strict")]
            pub(crate) strict: Option<bool>,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct IsEditable {
            #[serde(rename = "value")]
            pub(crate) value: bool
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct IsEditableArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "selector")]
            pub(crate) selector: &'a str,
            #[serde(rename = "strict")]
            pub(crate) strict: Option<bool>,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct IsEnabled {
            #[serde(rename = "value")]
            pub(crate) value: bool
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct IsEnabledArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "selector")]
            pub(crate) selector: &'a str,
            #[serde(rename = "strict")]
            pub(crate) strict: Option<bool>,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct IsHidden {
            #[serde(rename = "value")]
            pub(crate) value: bool
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct IsHiddenArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "selector")]
            pub(crate) selector: &'a str,
            #[serde(rename = "strict")]
            pub(crate) strict: Option<bool>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct IsVisible {
            #[serde(rename = "value")]
            pub(crate) value: bool
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct IsVisibleArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "selector")]
            pub(crate) selector: &'a str,
            #[serde(rename = "strict")]
            pub(crate) strict: Option<bool>
        }
        pub type Press = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct PressArgs<'a> {
            #[serde(rename = "delay")]
            pub(crate) delay: Option<serde_json::Number>,
            #[serde(borrow)]
            #[serde(rename = "key")]
            pub(crate) key: &'a str,
            #[serde(rename = "noWaitAfter")]
            pub(crate) no_wait_after: Option<bool>,
            #[serde(borrow)]
            #[serde(rename = "selector")]
            pub(crate) selector: &'a str,
            #[serde(rename = "strict")]
            pub(crate) strict: Option<bool>,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct QueryCount {
            #[serde(rename = "value")]
            pub(crate) value: serde_json::Number
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct QueryCountArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "selector")]
            pub(crate) selector: &'a str
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct QuerySelector {
            #[serde(rename = "element")]
            pub(crate) element: Option<crate::protocol::generated::ElementHandle>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct QuerySelectorArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "selector")]
            pub(crate) selector: &'a str,
            #[serde(rename = "strict")]
            pub(crate) strict: Option<bool>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct QuerySelectorAll {
            #[serde(rename = "elements")]
            pub(crate) elements: Vec<crate::protocol::generated::ElementHandle>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct QuerySelectorAllArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "selector")]
            pub(crate) selector: &'a str
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SelectOption {
            #[serde(rename = "values")]
            pub(crate) values: Vec<String>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SelectOptionArgs<'a> {
            #[serde(rename = "elements")]
            pub(crate) elements: Option<Vec<crate::protocol::generated::ElementHandle>>,
            #[serde(rename = "force")]
            pub(crate) force: Option<bool>,
            #[serde(rename = "noWaitAfter")]
            pub(crate) no_wait_after: Option<bool>,
            #[serde(borrow)]
            #[serde(rename = "options")]
            pub(crate) options: Option<Vec<SelectOptionArgsOptions<'a>>>,
            #[serde(borrow)]
            #[serde(rename = "selector")]
            pub(crate) selector: &'a str,
            #[serde(rename = "strict")]
            pub(crate) strict: Option<bool>,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SelectOptionArgsOptions<'a> {
            #[serde(rename = "index")]
            pub(crate) index: Option<serde_json::Number>,
            #[serde(borrow)]
            #[serde(rename = "label")]
            pub(crate) label: Option<&'a str>,
            #[serde(borrow)]
            #[serde(rename = "value")]
            pub(crate) value: Option<&'a str>
        }
        pub type SetContent = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetContentArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "html")]
            pub(crate) html: &'a str,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>,
            #[serde(rename = "waitUntil")]
            pub(crate) wait_until: Option<crate::protocol::generated::LifecycleEvent>
        }
        pub type SetInputFilePaths = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetInputFilePathsArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "localPaths")]
            pub(crate) local_paths: Option<Vec<&'a str>>,
            #[serde(rename = "noWaitAfter")]
            pub(crate) no_wait_after: Option<bool>,
            #[serde(borrow)]
            #[serde(rename = "selector")]
            pub(crate) selector: &'a str,
            #[serde(rename = "streams")]
            pub(crate) streams: Option<Vec<crate::protocol::generated::WritableStream>>,
            #[serde(rename = "strict")]
            pub(crate) strict: Option<bool>,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>
        }
        pub type SetInputFiles = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetInputFilesArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "files")]
            pub(crate) files: Vec<SetInputFilesArgsFiles<'a>>,
            #[serde(rename = "noWaitAfter")]
            pub(crate) no_wait_after: Option<bool>,
            #[serde(borrow)]
            #[serde(rename = "selector")]
            pub(crate) selector: &'a str,
            #[serde(rename = "strict")]
            pub(crate) strict: Option<bool>,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetInputFilesArgsFiles<'a> {
            #[serde(borrow)]
            #[serde(rename = "buffer")]
            pub(crate) buffer: &'a [u8],
            #[serde(borrow)]
            #[serde(rename = "mimeType")]
            pub(crate) mime_type: Option<&'a str>,
            #[serde(borrow)]
            #[serde(rename = "name")]
            pub(crate) name: &'a str
        }
        pub type Tap = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct TapArgs<'a> {
            #[serde(rename = "force")]
            pub(crate) force: Option<bool>,
            #[serde(rename = "modifiers")]
            pub(crate) modifiers: Option<Vec<TapArgsModifiers>>,
            #[serde(rename = "noWaitAfter")]
            pub(crate) no_wait_after: Option<bool>,
            #[serde(rename = "position")]
            pub(crate) position: Option<crate::protocol::generated::Point>,
            #[serde(borrow)]
            #[serde(rename = "selector")]
            pub(crate) selector: &'a str,
            #[serde(rename = "strict")]
            pub(crate) strict: Option<bool>,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>,
            #[serde(rename = "trial")]
            pub(crate) trial: Option<bool>
        }
        #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
        pub enum TapArgsModifiers {
            Alt,
            Control,
            Meta,
            Shift
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct TextContent {
            #[serde(rename = "value")]
            pub(crate) value: Option<String>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct TextContentArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "selector")]
            pub(crate) selector: &'a str,
            #[serde(rename = "strict")]
            pub(crate) strict: Option<bool>,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Title {
            #[serde(rename = "value")]
            pub(crate) value: String
        }
        pub type TitleArgs = ();
        pub type Type = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct TypeArgs<'a> {
            #[serde(rename = "delay")]
            pub(crate) delay: Option<serde_json::Number>,
            #[serde(rename = "noWaitAfter")]
            pub(crate) no_wait_after: Option<bool>,
            #[serde(borrow)]
            #[serde(rename = "selector")]
            pub(crate) selector: &'a str,
            #[serde(rename = "strict")]
            pub(crate) strict: Option<bool>,
            #[serde(borrow)]
            #[serde(rename = "text")]
            pub(crate) text: &'a str,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>
        }
        pub type Uncheck = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct UncheckArgs<'a> {
            #[serde(rename = "force")]
            pub(crate) force: Option<bool>,
            #[serde(rename = "noWaitAfter")]
            pub(crate) no_wait_after: Option<bool>,
            #[serde(rename = "position")]
            pub(crate) position: Option<crate::protocol::generated::Point>,
            #[serde(borrow)]
            #[serde(rename = "selector")]
            pub(crate) selector: &'a str,
            #[serde(rename = "strict")]
            pub(crate) strict: Option<bool>,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>,
            #[serde(rename = "trial")]
            pub(crate) trial: Option<bool>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct WaitForFunction {
            #[serde(rename = "handle")]
            pub(crate) handle: crate::protocol::generated::JsHandle
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct WaitForFunctionArgs<'a> {
            #[serde(rename = "arg")]
            pub(crate) arg: crate::protocol::generated::SerializedArgument,
            #[serde(borrow)]
            #[serde(rename = "expression")]
            pub(crate) expression: &'a str,
            #[serde(rename = "isFunction")]
            pub(crate) is_function: Option<bool>,
            #[serde(rename = "pollingInterval")]
            pub(crate) polling_interval: Option<serde_json::Number>,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct WaitForSelector {
            #[serde(rename = "element")]
            pub(crate) element: Option<crate::protocol::generated::ElementHandle>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct WaitForSelectorArgs<'a> {
            #[serde(rename = "omitReturnValue")]
            pub(crate) omit_return_value: Option<bool>,
            #[serde(borrow)]
            #[serde(rename = "selector")]
            pub(crate) selector: &'a str,
            #[serde(rename = "state")]
            pub(crate) state: Option<WaitForSelectorArgsState>,
            #[serde(rename = "strict")]
            pub(crate) strict: Option<bool>,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
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
            pub(crate) timeout: serde_json::Number
        }
    }
}
pub(crate) type JsHandle = OnlyGuid;
pub mod js_handle {
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Initializer {
        #[serde(rename = "preview")]
        pub(crate) preview: String
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
            pub(crate) preview: String
        }
    }
    pub mod commands {
        pub type Dispose = ();
        pub type DisposeArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct EvaluateExpression {
            #[serde(rename = "value")]
            pub(crate) value: crate::protocol::generated::SerializedValue
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct EvaluateExpressionArgs<'a> {
            #[serde(rename = "arg")]
            pub(crate) arg: crate::protocol::generated::SerializedArgument,
            #[serde(borrow)]
            #[serde(rename = "expression")]
            pub(crate) expression: &'a str,
            #[serde(rename = "isFunction")]
            pub(crate) is_function: Option<bool>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct EvaluateExpressionHandle {
            #[serde(rename = "handle")]
            pub(crate) handle: crate::protocol::generated::JsHandle
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct EvaluateExpressionHandleArgs<'a> {
            #[serde(rename = "arg")]
            pub(crate) arg: crate::protocol::generated::SerializedArgument,
            #[serde(borrow)]
            #[serde(rename = "expression")]
            pub(crate) expression: &'a str,
            #[serde(rename = "isFunction")]
            pub(crate) is_function: Option<bool>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetProperty {
            #[serde(rename = "handle")]
            pub(crate) handle: crate::protocol::generated::JsHandle
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetPropertyArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "name")]
            pub(crate) name: &'a str
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetPropertyList {
            #[serde(rename = "properties")]
            pub(crate) properties: Vec<GetPropertyListProperties>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetPropertyListProperties {
            #[serde(rename = "name")]
            pub(crate) name: String,
            #[serde(rename = "value")]
            pub(crate) value: crate::protocol::generated::JsHandle
        }
        pub type GetPropertyListArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct JsonValue {
            #[serde(rename = "value")]
            pub(crate) value: crate::protocol::generated::SerializedValue
        }
        pub type JsonValueArgs = ();
    }
}
pub(crate) type JsonPipe = OnlyGuid;
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
            pub(crate) error: Option<crate::protocol::generated::SerializedError>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Message {
            #[serde(rename = "message")]
            pub(crate) message: String
        }
    }
    pub mod commands {
        pub type Close = ();
        pub type CloseArgs = ();
        pub type Send = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SendArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "message")]
            pub(crate) message: &'a str
        }
    }
}
pub(crate) type LocalUtils = OnlyGuid;
pub mod local_utils {
    pub mod commands {
        pub type HarClose = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct HarCloseArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "harId")]
            pub(crate) har_id: &'a str
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct HarLookup {
            #[serde(rename = "action")]
            pub(crate) action: HarLookupAction,
            #[serde(rename = "body")]
            pub(crate) body: Option<Vec<u8>>,
            #[serde(rename = "headers")]
            pub(crate) headers: Option<Vec<crate::protocol::generated::NameValue>>,
            #[serde(rename = "message")]
            pub(crate) message: Option<String>,
            #[serde(rename = "redirectURL")]
            pub(crate) redirect_url: Option<String>,
            #[serde(rename = "status")]
            pub(crate) status: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
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
        pub struct HarLookupArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "harId")]
            pub(crate) har_id: &'a str,
            #[serde(rename = "headers")]
            pub(crate) headers: Vec<crate::protocol::generated::NameValue>,
            #[serde(rename = "isNavigationRequest")]
            pub(crate) is_navigation_request: bool,
            #[serde(borrow)]
            #[serde(rename = "method")]
            pub(crate) method: &'a str,
            #[serde(borrow)]
            #[serde(rename = "postData")]
            pub(crate) post_data: Option<&'a [u8]>,
            #[serde(borrow)]
            #[serde(rename = "url")]
            pub(crate) url: &'a str
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct HarOpen {
            #[serde(rename = "error")]
            pub(crate) error: Option<String>,
            #[serde(rename = "harId")]
            pub(crate) har_id: Option<String>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct HarOpenArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "file")]
            pub(crate) file: &'a str
        }
        pub type HarUnzip = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct HarUnzipArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "harFile")]
            pub(crate) har_file: &'a str,
            #[serde(borrow)]
            #[serde(rename = "zipFile")]
            pub(crate) zip_file: &'a str
        }
        pub type Zip = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ZipArgs<'a> {
            #[serde(rename = "entries")]
            pub(crate) entries: Vec<crate::protocol::generated::NameValue>,
            #[serde(borrow)]
            #[serde(rename = "zipFile")]
            pub(crate) zip_file: &'a str
        }
    }
}
pub(crate) type Page = OnlyGuid;
pub mod page {
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Initializer {
        #[serde(rename = "isClosed")]
        pub(crate) is_closed: bool,
        #[serde(rename = "mainFrame")]
        pub(crate) main_frame: crate::protocol::generated::Frame,
        #[serde(rename = "opener")]
        pub(crate) opener: Option<crate::protocol::generated::Page>,
        #[serde(rename = "viewportSize")]
        pub(crate) viewport_size: Option<InitializerViewportSize>
    }
    #[derive(Debug, Serialize, Deserialize)]
    pub struct InitializerViewportSize {
        #[serde(rename = "height")]
        pub(crate) height: serde_json::Number,
        #[serde(rename = "width")]
        pub(crate) width: serde_json::Number
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
            pub(crate) binding: crate::protocol::generated::BindingCall
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Console {
            #[serde(rename = "message")]
            pub(crate) message: crate::protocol::generated::ConsoleMessage
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Dialog {
            #[serde(rename = "dialog")]
            pub(crate) dialog: crate::protocol::generated::Dialog
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Download {
            #[serde(rename = "artifact")]
            pub(crate) artifact: crate::protocol::generated::Artifact,
            #[serde(rename = "suggestedFilename")]
            pub(crate) suggested_filename: String,
            #[serde(rename = "url")]
            pub(crate) url: String
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct FileChooser {
            #[serde(rename = "element")]
            pub(crate) element: crate::protocol::generated::ElementHandle,
            #[serde(rename = "isMultiple")]
            pub(crate) is_multiple: bool
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct FrameAttached {
            #[serde(rename = "frame")]
            pub(crate) frame: crate::protocol::generated::Frame
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct FrameDetached {
            #[serde(rename = "frame")]
            pub(crate) frame: crate::protocol::generated::Frame
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct PageError {
            #[serde(rename = "error")]
            pub(crate) error: crate::protocol::generated::SerializedError
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Route {
            #[serde(rename = "request")]
            pub(crate) request: crate::protocol::generated::Request,
            #[serde(rename = "route")]
            pub(crate) route: crate::protocol::generated::Route
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Video {
            #[serde(rename = "artifact")]
            pub(crate) artifact: crate::protocol::generated::Artifact
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct WebSocket {
            #[serde(rename = "webSocket")]
            pub(crate) web_socket: crate::protocol::generated::WebSocket
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Worker {
            #[serde(rename = "worker")]
            pub(crate) worker: crate::protocol::generated::Worker
        }
    }
    pub mod commands {
        #[derive(Debug, Serialize, Deserialize)]
        pub struct AccessibilitySnapshot {
            #[serde(rename = "rootAXNode")]
            pub(crate) root_ax_node: Option<crate::protocol::generated::AxNode>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct AccessibilitySnapshotArgs {
            #[serde(rename = "interestingOnly")]
            pub(crate) interesting_only: Option<bool>,
            #[serde(rename = "root")]
            pub(crate) root: Option<crate::protocol::generated::ElementHandle>
        }
        pub type AddInitScript = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct AddInitScriptArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "source")]
            pub(crate) source: &'a str
        }
        pub type BringToFront = ();
        pub type BringToFrontArgs = ();
        pub type Close = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct CloseArgs {
            #[serde(rename = "runBeforeUnload")]
            pub(crate) run_before_unload: Option<bool>
        }
        pub type EmulateMedia = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct EmulateMediaArgs {
            #[serde(rename = "colorScheme")]
            pub(crate) color_scheme: Option<EmulateMediaArgsColorScheme>,
            #[serde(rename = "forcedColors")]
            pub(crate) forced_colors: Option<EmulateMediaArgsForcedColors>,
            #[serde(rename = "media")]
            pub(crate) media: Option<EmulateMediaArgsMedia>,
            #[serde(rename = "reducedMotion")]
            pub(crate) reduced_motion: Option<EmulateMediaArgsReducedMotion>
        }
        #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
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
        #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
        pub enum EmulateMediaArgsForcedColors {
            #[serde(rename = "active")]
            Active,
            #[serde(rename = "none")]
            None,
            #[serde(rename = "null")]
            Null
        }
        #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
        pub enum EmulateMediaArgsMedia {
            #[serde(rename = "screen")]
            Screen,
            #[serde(rename = "print")]
            Print,
            #[serde(rename = "null")]
            Null
        }
        #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
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
            pub(crate) actual: Option<Vec<u8>>,
            #[serde(rename = "diff")]
            pub(crate) diff: Option<Vec<u8>>,
            #[serde(rename = "errorMessage")]
            pub(crate) error_message: Option<String>,
            #[serde(rename = "log")]
            pub(crate) log: Option<Vec<String>>,
            #[serde(rename = "previous")]
            pub(crate) previous: Option<Vec<u8>>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ExpectScreenshotArgs<'a> {
            #[serde(rename = "comparatorOptions")]
            pub(crate) comparator_options: Option<ExpectScreenshotArgsComparatorOptions>,
            #[serde(borrow)]
            #[serde(rename = "expected")]
            pub(crate) expected: Option<&'a [u8]>,
            #[serde(rename = "isNot")]
            pub(crate) is_not: bool,
            #[serde(borrow)]
            #[serde(rename = "locator")]
            pub(crate) locator: Option<ExpectScreenshotArgsLocator<'a>>,
            #[serde(rename = "screenshotOptions")]
            pub(crate) screenshot_options: Option<ExpectScreenshotArgsScreenshotOptions>,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ExpectScreenshotArgsComparatorOptions {
            #[serde(rename = "maxDiffPixelRatio")]
            pub(crate) max_diff_pixel_ratio: Option<serde_json::Number>,
            #[serde(rename = "maxDiffPixels")]
            pub(crate) max_diff_pixels: Option<serde_json::Number>,
            #[serde(rename = "threshold")]
            pub(crate) threshold: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ExpectScreenshotArgsLocator<'a> {
            #[serde(rename = "frame")]
            pub(crate) frame: crate::protocol::generated::Frame,
            #[serde(borrow)]
            #[serde(rename = "selector")]
            pub(crate) selector: &'a str
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ExpectScreenshotArgsScreenshotOptions {
            #[serde(flatten)]
            #[serde(rename = "$mixin")]
            pub(crate) mixin: crate::protocol::generated::CommonScreenshotOptions,
            #[serde(rename = "clip")]
            pub(crate) clip: Option<crate::protocol::generated::Rect>,
            #[serde(rename = "fullPage")]
            pub(crate) full_page: Option<bool>
        }
        pub type ExposeBinding = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ExposeBindingArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "name")]
            pub(crate) name: &'a str,
            #[serde(rename = "needsHandle")]
            pub(crate) needs_handle: Option<bool>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct GoBack {
            #[serde(rename = "response")]
            pub(crate) response: Option<crate::protocol::generated::Response>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct GoBackArgs {
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>,
            #[serde(rename = "waitUntil")]
            pub(crate) wait_until: Option<crate::protocol::generated::LifecycleEvent>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct GoForward {
            #[serde(rename = "response")]
            pub(crate) response: Option<crate::protocol::generated::Response>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct GoForwardArgs {
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>,
            #[serde(rename = "waitUntil")]
            pub(crate) wait_until: Option<crate::protocol::generated::LifecycleEvent>
        }
        pub type KeyboardDown = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct KeyboardDownArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "key")]
            pub(crate) key: &'a str
        }
        pub type KeyboardInsertText = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct KeyboardInsertTextArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "text")]
            pub(crate) text: &'a str
        }
        pub type KeyboardPress = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct KeyboardPressArgs<'a> {
            #[serde(rename = "delay")]
            pub(crate) delay: Option<serde_json::Number>,
            #[serde(borrow)]
            #[serde(rename = "key")]
            pub(crate) key: &'a str
        }
        pub type KeyboardType = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct KeyboardTypeArgs<'a> {
            #[serde(rename = "delay")]
            pub(crate) delay: Option<serde_json::Number>,
            #[serde(borrow)]
            #[serde(rename = "text")]
            pub(crate) text: &'a str
        }
        pub type KeyboardUp = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct KeyboardUpArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "key")]
            pub(crate) key: &'a str
        }
        pub type MouseClick = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct MouseClickArgs {
            #[serde(rename = "button")]
            pub(crate) button: Option<MouseClickArgsButton>,
            #[serde(rename = "clickCount")]
            pub(crate) click_count: Option<serde_json::Number>,
            #[serde(rename = "delay")]
            pub(crate) delay: Option<serde_json::Number>,
            #[serde(rename = "x")]
            pub(crate) x: serde_json::Number,
            #[serde(rename = "y")]
            pub(crate) y: serde_json::Number
        }
        #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
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
            pub(crate) button: Option<MouseDownArgsButton>,
            #[serde(rename = "clickCount")]
            pub(crate) click_count: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
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
            pub(crate) steps: Option<serde_json::Number>,
            #[serde(rename = "x")]
            pub(crate) x: serde_json::Number,
            #[serde(rename = "y")]
            pub(crate) y: serde_json::Number
        }
        pub type MouseUp = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct MouseUpArgs {
            #[serde(rename = "button")]
            pub(crate) button: Option<MouseUpArgsButton>,
            #[serde(rename = "clickCount")]
            pub(crate) click_count: Option<serde_json::Number>
        }
        #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
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
            pub(crate) delta_x: serde_json::Number,
            #[serde(rename = "deltaY")]
            pub(crate) delta_y: serde_json::Number
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Pdf {
            #[serde(rename = "pdf")]
            pub(crate) pdf: Vec<u8>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct PdfArgs<'a> {
            #[serde(rename = "displayHeaderFooter")]
            pub(crate) display_header_footer: Option<bool>,
            #[serde(borrow)]
            #[serde(rename = "footerTemplate")]
            pub(crate) footer_template: Option<&'a str>,
            #[serde(borrow)]
            #[serde(rename = "format")]
            pub(crate) format: Option<&'a str>,
            #[serde(borrow)]
            #[serde(rename = "headerTemplate")]
            pub(crate) header_template: Option<&'a str>,
            #[serde(borrow)]
            #[serde(rename = "height")]
            pub(crate) height: Option<&'a str>,
            #[serde(rename = "landscape")]
            pub(crate) landscape: Option<bool>,
            #[serde(borrow)]
            #[serde(rename = "margin")]
            pub(crate) margin: Option<PdfArgsMargin<'a>>,
            #[serde(borrow)]
            #[serde(rename = "pageRanges")]
            pub(crate) page_ranges: Option<&'a str>,
            #[serde(rename = "preferCSSPageSize")]
            pub(crate) prefer_css_page_size: Option<bool>,
            #[serde(rename = "printBackground")]
            pub(crate) print_background: Option<bool>,
            #[serde(rename = "scale")]
            pub(crate) scale: Option<serde_json::Number>,
            #[serde(borrow)]
            #[serde(rename = "width")]
            pub(crate) width: Option<&'a str>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct PdfArgsMargin<'a> {
            #[serde(borrow)]
            #[serde(rename = "bottom")]
            pub(crate) bottom: Option<&'a str>,
            #[serde(borrow)]
            #[serde(rename = "left")]
            pub(crate) left: Option<&'a str>,
            #[serde(borrow)]
            #[serde(rename = "right")]
            pub(crate) right: Option<&'a str>,
            #[serde(borrow)]
            #[serde(rename = "top")]
            pub(crate) top: Option<&'a str>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Reload {
            #[serde(rename = "response")]
            pub(crate) response: Option<crate::protocol::generated::Response>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ReloadArgs {
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>,
            #[serde(rename = "waitUntil")]
            pub(crate) wait_until: Option<crate::protocol::generated::LifecycleEvent>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Screenshot {
            #[serde(rename = "binary")]
            pub(crate) binary: Vec<u8>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ScreenshotArgs {
            #[serde(flatten)]
            #[serde(rename = "$mixin")]
            pub(crate) mixin: crate::protocol::generated::CommonScreenshotOptions,
            #[serde(rename = "clip")]
            pub(crate) clip: Option<crate::protocol::generated::Rect>,
            #[serde(rename = "fullPage")]
            pub(crate) full_page: Option<bool>,
            #[serde(rename = "quality")]
            pub(crate) quality: Option<serde_json::Number>,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>,
            #[serde(rename = "type")]
            pub(crate) r#type: Option<ScreenshotArgsType>
        }
        #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
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
            pub(crate) timeout: Option<serde_json::Number>
        }
        pub type SetDefaultTimeoutNoReply = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetDefaultTimeoutNoReplyArgs {
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>
        }
        pub type SetExtraHttpHeaders = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetExtraHttpHeadersArgs {
            #[serde(rename = "headers")]
            pub(crate) headers: Vec<crate::protocol::generated::NameValue>
        }
        pub type SetFileChooserInterceptedNoReply = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetFileChooserInterceptedNoReplyArgs {
            #[serde(rename = "intercepted")]
            pub(crate) intercepted: bool
        }
        pub type SetNetworkInterceptionEnabled = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetNetworkInterceptionEnabledArgs {
            #[serde(rename = "enabled")]
            pub(crate) enabled: bool
        }
        pub type SetViewportSize = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetViewportSizeArgs {
            #[serde(rename = "viewportSize")]
            pub(crate) viewport_size: SetViewportSizeArgsViewportSize
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetViewportSizeArgsViewportSize {
            #[serde(rename = "height")]
            pub(crate) height: serde_json::Number,
            #[serde(rename = "width")]
            pub(crate) width: serde_json::Number
        }
        pub type StartCssCoverage = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct StartCssCoverageArgs {
            #[serde(rename = "resetOnNavigation")]
            pub(crate) reset_on_navigation: Option<bool>
        }
        pub type StartJsCoverage = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct StartJsCoverageArgs {
            #[serde(rename = "reportAnonymousScripts")]
            pub(crate) report_anonymous_scripts: Option<bool>,
            #[serde(rename = "resetOnNavigation")]
            pub(crate) reset_on_navigation: Option<bool>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct StopCssCoverage {
            #[serde(rename = "entries")]
            pub(crate) entries: Vec<StopCssCoverageEntries>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct StopCssCoverageEntries {
            #[serde(rename = "ranges")]
            pub(crate) ranges: Vec<StopCssCoverageEntriesRanges>,
            #[serde(rename = "text")]
            pub(crate) text: Option<String>,
            #[serde(rename = "url")]
            pub(crate) url: String
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct StopCssCoverageEntriesRanges {
            #[serde(rename = "end")]
            pub(crate) end: serde_json::Number,
            #[serde(rename = "start")]
            pub(crate) start: serde_json::Number
        }
        pub type StopCssCoverageArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct StopJsCoverage {
            #[serde(rename = "entries")]
            pub(crate) entries: Vec<StopJsCoverageEntries>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct StopJsCoverageEntries {
            #[serde(rename = "functions")]
            pub(crate) functions: Vec<StopJsCoverageEntriesFunctions>,
            #[serde(rename = "scriptId")]
            pub(crate) script_id: String,
            #[serde(rename = "source")]
            pub(crate) source: Option<String>,
            #[serde(rename = "url")]
            pub(crate) url: String
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct StopJsCoverageEntriesFunctions {
            #[serde(rename = "functionName")]
            pub(crate) function_name: String,
            #[serde(rename = "isBlockCoverage")]
            pub(crate) is_block_coverage: bool,
            #[serde(rename = "ranges")]
            pub(crate) ranges: Vec<StopJsCoverageEntriesFunctionsRanges>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct StopJsCoverageEntriesFunctionsRanges {
            #[serde(rename = "count")]
            pub(crate) count: serde_json::Number,
            #[serde(rename = "endOffset")]
            pub(crate) end_offset: serde_json::Number,
            #[serde(rename = "startOffset")]
            pub(crate) start_offset: serde_json::Number
        }
        pub type StopJsCoverageArgs = ();
        pub type TouchscreenTap = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct TouchscreenTapArgs {
            #[serde(rename = "x")]
            pub(crate) x: serde_json::Number,
            #[serde(rename = "y")]
            pub(crate) y: serde_json::Number
        }
    }
}
pub(crate) type Playwright = OnlyGuid;
pub mod playwright {
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Initializer {
        #[serde(rename = "android")]
        pub(crate) android: crate::protocol::generated::Android,
        #[serde(rename = "chromium")]
        pub(crate) chromium: crate::protocol::generated::BrowserType,
        #[serde(rename = "deviceDescriptors")]
        pub(crate) device_descriptors: Vec<InitializerDeviceDescriptors>,
        #[serde(rename = "electron")]
        pub(crate) electron: crate::protocol::generated::Electron,
        #[serde(rename = "firefox")]
        pub(crate) firefox: crate::protocol::generated::BrowserType,
        #[serde(rename = "preLaunchedBrowser")]
        pub(crate) pre_launched_browser: Option<crate::protocol::generated::Browser>,
        #[serde(rename = "selectors")]
        pub(crate) selectors: crate::protocol::generated::Selectors,
        #[serde(rename = "socksSupport")]
        pub(crate) socks_support: Option<crate::protocol::generated::SocksSupport>,
        #[serde(rename = "utils")]
        pub(crate) utils: crate::protocol::generated::LocalUtils,
        #[serde(rename = "webkit")]
        pub(crate) webkit: crate::protocol::generated::BrowserType
    }
    #[derive(Debug, Serialize, Deserialize)]
    pub struct InitializerDeviceDescriptors {
        #[serde(rename = "descriptor")]
        pub(crate) descriptor: InitializerDeviceDescriptorsDescriptor,
        #[serde(rename = "name")]
        pub(crate) name: String
    }
    #[derive(Debug, Serialize, Deserialize)]
    pub struct InitializerDeviceDescriptorsDescriptor {
        #[serde(rename = "defaultBrowserType")]
        pub(crate) default_browser_type: InitializerDeviceDescriptorsDescriptorDefaultBrowserType,
        #[serde(rename = "deviceScaleFactor")]
        pub(crate) device_scale_factor: serde_json::Number,
        #[serde(rename = "hasTouch")]
        pub(crate) has_touch: bool,
        #[serde(rename = "isMobile")]
        pub(crate) is_mobile: bool,
        #[serde(rename = "screen")]
        pub(crate) screen: Option<InitializerDeviceDescriptorsDescriptorScreen>,
        #[serde(rename = "userAgent")]
        pub(crate) user_agent: String,
        #[serde(rename = "viewport")]
        pub(crate) viewport: InitializerDeviceDescriptorsDescriptorViewport
    }
    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
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
        pub(crate) height: serde_json::Number,
        #[serde(rename = "width")]
        pub(crate) width: serde_json::Number
    }
    #[derive(Debug, Serialize, Deserialize)]
    pub struct InitializerDeviceDescriptorsDescriptorViewport {
        #[serde(rename = "height")]
        pub(crate) height: serde_json::Number,
        #[serde(rename = "width")]
        pub(crate) width: serde_json::Number
    }
    pub mod commands {
        pub type HideHighlight = ();
        pub type HideHighlightArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct NewRequest {
            #[serde(rename = "request")]
            pub(crate) request: crate::protocol::generated::ApiRequestContext
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct NewRequestArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "baseURL")]
            pub(crate) base_url: Option<&'a str>,
            #[serde(rename = "extraHTTPHeaders")]
            pub(crate) extra_http_headers: Option<Vec<crate::protocol::generated::NameValue>>,
            #[serde(borrow)]
            #[serde(rename = "httpCredentials")]
            pub(crate) http_credentials: Option<NewRequestArgsHttpCredentials<'a>>,
            #[serde(rename = "ignoreHTTPSErrors")]
            pub(crate) ignore_https_errors: Option<bool>,
            #[serde(borrow)]
            #[serde(rename = "proxy")]
            pub(crate) proxy: Option<NewRequestArgsProxy<'a>>,
            #[serde(rename = "storageState")]
            pub(crate) storage_state: Option<NewRequestArgsStorageState>,
            #[serde(rename = "timeout")]
            pub(crate) timeout: Option<serde_json::Number>,
            #[serde(borrow)]
            #[serde(rename = "tracesDir")]
            pub(crate) traces_dir: Option<&'a str>,
            #[serde(borrow)]
            #[serde(rename = "userAgent")]
            pub(crate) user_agent: Option<&'a str>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct NewRequestArgsHttpCredentials<'a> {
            #[serde(borrow)]
            #[serde(rename = "password")]
            pub(crate) password: &'a str,
            #[serde(borrow)]
            #[serde(rename = "username")]
            pub(crate) username: &'a str
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct NewRequestArgsProxy<'a> {
            #[serde(borrow)]
            #[serde(rename = "bypass")]
            pub(crate) bypass: Option<&'a str>,
            #[serde(borrow)]
            #[serde(rename = "password")]
            pub(crate) password: Option<&'a str>,
            #[serde(borrow)]
            #[serde(rename = "server")]
            pub(crate) server: &'a str,
            #[serde(borrow)]
            #[serde(rename = "username")]
            pub(crate) username: Option<&'a str>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct NewRequestArgsStorageState {
            #[serde(rename = "cookies")]
            pub(crate) cookies: Vec<crate::protocol::generated::NetworkCookie>,
            #[serde(rename = "origins")]
            pub(crate) origins: Vec<crate::protocol::generated::OriginStorage>
        }
    }
}
pub(crate) type Request = OnlyGuid;
pub mod request {
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Initializer {
        #[serde(rename = "frame")]
        pub(crate) frame: Option<crate::protocol::generated::Frame>,
        #[serde(rename = "headers")]
        pub(crate) headers: Vec<crate::protocol::generated::NameValue>,
        #[serde(rename = "isNavigationRequest")]
        pub(crate) is_navigation_request: bool,
        #[serde(rename = "method")]
        pub(crate) method: String,
        #[serde(rename = "postData")]
        pub(crate) post_data: Option<Vec<u8>>,
        #[serde(rename = "redirectedFrom")]
        pub(crate) redirected_from: Option<crate::protocol::generated::Request>,
        #[serde(rename = "resourceType")]
        pub(crate) resource_type: String,
        #[serde(rename = "serviceWorker")]
        pub(crate) service_worker: Option<crate::protocol::generated::Worker>,
        #[serde(rename = "url")]
        pub(crate) url: String
    }
    pub mod commands {
        #[derive(Debug, Serialize, Deserialize)]
        pub struct RawRequestHeaders {
            #[serde(rename = "headers")]
            pub(crate) headers: Vec<crate::protocol::generated::NameValue>
        }
        pub type RawRequestHeadersArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Response {
            #[serde(rename = "response")]
            pub(crate) response: Option<crate::protocol::generated::Response>
        }
        pub type ResponseArgs = ();
    }
}
pub(crate) type Response = OnlyGuid;
pub mod response {
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Initializer {
        #[serde(rename = "fromServiceWorker")]
        pub(crate) from_service_worker: bool,
        #[serde(rename = "headers")]
        pub(crate) headers: Vec<crate::protocol::generated::NameValue>,
        #[serde(rename = "request")]
        pub(crate) request: crate::protocol::generated::Request,
        #[serde(rename = "status")]
        pub(crate) status: serde_json::Number,
        #[serde(rename = "statusText")]
        pub(crate) status_text: String,
        #[serde(rename = "timing")]
        pub(crate) timing: crate::protocol::generated::ResourceTiming,
        #[serde(rename = "url")]
        pub(crate) url: String
    }
    pub mod commands {
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Body {
            #[serde(rename = "binary")]
            pub(crate) binary: Vec<u8>
        }
        pub type BodyArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct RawResponseHeaders {
            #[serde(rename = "headers")]
            pub(crate) headers: Vec<crate::protocol::generated::NameValue>
        }
        pub type RawResponseHeadersArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SecurityDetails {
            #[serde(rename = "value")]
            pub(crate) value: Option<crate::protocol::generated::SecurityDetails>
        }
        pub type SecurityDetailsArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ServerAddr {
            #[serde(rename = "value")]
            pub(crate) value: Option<crate::protocol::generated::RemoteAddr>
        }
        pub type ServerAddrArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Sizes {
            #[serde(rename = "sizes")]
            pub(crate) sizes: crate::protocol::generated::RequestSizes
        }
        pub type SizesArgs = ();
    }
}
pub(crate) type Root = OnlyGuid;
pub mod root {
    pub mod commands {
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Initialize {
            #[serde(rename = "playwright")]
            pub(crate) playwright: crate::protocol::generated::Playwright
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct InitializeArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "sdkLanguage")]
            pub(crate) sdk_language: &'a str
        }
    }
}
pub(crate) type Route = OnlyGuid;
pub mod route {
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Initializer {
        #[serde(rename = "request")]
        pub(crate) request: crate::protocol::generated::Request
    }
    pub mod commands {
        pub type Abort = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct AbortArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "errorCode")]
            pub(crate) error_code: Option<&'a str>
        }
        pub type Continue = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ContinueArgs<'a> {
            #[serde(rename = "headers")]
            pub(crate) headers: Option<Vec<crate::protocol::generated::NameValue>>,
            #[serde(borrow)]
            #[serde(rename = "method")]
            pub(crate) method: Option<&'a str>,
            #[serde(borrow)]
            #[serde(rename = "postData")]
            pub(crate) post_data: Option<&'a [u8]>,
            #[serde(borrow)]
            #[serde(rename = "url")]
            pub(crate) url: Option<&'a str>
        }
        pub type Fulfill = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct FulfillArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "body")]
            pub(crate) body: Option<&'a str>,
            #[serde(borrow)]
            #[serde(rename = "fetchResponseUid")]
            pub(crate) fetch_response_uid: Option<&'a str>,
            #[serde(rename = "headers")]
            pub(crate) headers: Option<Vec<crate::protocol::generated::NameValue>>,
            #[serde(rename = "isBase64")]
            pub(crate) is_base64: Option<bool>,
            #[serde(rename = "status")]
            pub(crate) status: Option<serde_json::Number>
        }
        pub type RedirectNavigationRequest = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct RedirectNavigationRequestArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "url")]
            pub(crate) url: &'a str
        }
    }
}
pub(crate) type Selectors = OnlyGuid;
pub mod selectors {
    pub mod commands {
        pub type Register = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct RegisterArgs<'a> {
            #[serde(rename = "contentScript")]
            pub(crate) content_script: Option<bool>,
            #[serde(borrow)]
            #[serde(rename = "name")]
            pub(crate) name: &'a str,
            #[serde(borrow)]
            #[serde(rename = "source")]
            pub(crate) source: &'a str
        }
    }
}
pub(crate) type SocksSupport = OnlyGuid;
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
            pub(crate) uid: String
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SocksData {
            #[serde(rename = "data")]
            pub(crate) data: Vec<u8>,
            #[serde(rename = "uid")]
            pub(crate) uid: String
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SocksRequested {
            #[serde(rename = "host")]
            pub(crate) host: String,
            #[serde(rename = "port")]
            pub(crate) port: serde_json::Number,
            #[serde(rename = "uid")]
            pub(crate) uid: String
        }
    }
    pub mod commands {
        pub type SocksConnected = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SocksConnectedArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "host")]
            pub(crate) host: &'a str,
            #[serde(rename = "port")]
            pub(crate) port: serde_json::Number,
            #[serde(borrow)]
            #[serde(rename = "uid")]
            pub(crate) uid: &'a str
        }
        pub type SocksData = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SocksDataArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "data")]
            pub(crate) data: &'a [u8],
            #[serde(borrow)]
            #[serde(rename = "uid")]
            pub(crate) uid: &'a str
        }
        pub type SocksEnd = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SocksEndArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "uid")]
            pub(crate) uid: &'a str
        }
        pub type SocksError = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SocksErrorArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "error")]
            pub(crate) error: &'a str,
            #[serde(borrow)]
            #[serde(rename = "uid")]
            pub(crate) uid: &'a str
        }
        pub type SocksFailed = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SocksFailedArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "errorCode")]
            pub(crate) error_code: &'a str,
            #[serde(borrow)]
            #[serde(rename = "uid")]
            pub(crate) uid: &'a str
        }
    }
}
pub(crate) type Stream = OnlyGuid;
pub mod stream {
    pub mod commands {
        pub type Close = ();
        pub type CloseArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Read {
            #[serde(rename = "binary")]
            pub(crate) binary: Vec<u8>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ReadArgs {
            #[serde(rename = "size")]
            pub(crate) size: Option<serde_json::Number>
        }
    }
}
pub(crate) type Tracing = OnlyGuid;
pub mod tracing {
    pub mod commands {
        pub type TracingStart = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct TracingStartArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "name")]
            pub(crate) name: Option<&'a str>,
            #[serde(rename = "screenshots")]
            pub(crate) screenshots: Option<bool>,
            #[serde(rename = "snapshots")]
            pub(crate) snapshots: Option<bool>,
            #[serde(rename = "sources")]
            pub(crate) sources: Option<bool>
        }
        pub type TracingStartChunk = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct TracingStartChunkArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "title")]
            pub(crate) title: Option<&'a str>
        }
        pub type TracingStop = ();
        pub type TracingStopArgs = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct TracingStopChunk {
            #[serde(rename = "artifact")]
            pub(crate) artifact: Option<crate::protocol::generated::Artifact>,
            #[serde(rename = "sourceEntries")]
            pub(crate) source_entries: Option<Vec<crate::protocol::generated::NameValue>>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct TracingStopChunkArgs {
            #[serde(rename = "mode")]
            pub(crate) mode: TracingStopChunkArgsMode
        }
        #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
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
pub(crate) type WebSocket = OnlyGuid;
pub mod web_socket {
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Initializer {
        #[serde(rename = "url")]
        pub(crate) url: String
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
            pub(crate) data: String,
            #[serde(rename = "opcode")]
            pub(crate) opcode: serde_json::Number
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct FrameSent {
            #[serde(rename = "data")]
            pub(crate) data: String,
            #[serde(rename = "opcode")]
            pub(crate) opcode: serde_json::Number
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SocketError {
            #[serde(rename = "error")]
            pub(crate) error: String
        }
    }
}
pub(crate) type Worker = OnlyGuid;
pub mod worker {
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Initializer {
        #[serde(rename = "url")]
        pub(crate) url: String
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
            pub(crate) value: crate::protocol::generated::SerializedValue
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct EvaluateExpressionArgs<'a> {
            #[serde(rename = "arg")]
            pub(crate) arg: crate::protocol::generated::SerializedArgument,
            #[serde(borrow)]
            #[serde(rename = "expression")]
            pub(crate) expression: &'a str,
            #[serde(rename = "isFunction")]
            pub(crate) is_function: Option<bool>
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct EvaluateExpressionHandle {
            #[serde(rename = "handle")]
            pub(crate) handle: crate::protocol::generated::JsHandle
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct EvaluateExpressionHandleArgs<'a> {
            #[serde(rename = "arg")]
            pub(crate) arg: crate::protocol::generated::SerializedArgument,
            #[serde(borrow)]
            #[serde(rename = "expression")]
            pub(crate) expression: &'a str,
            #[serde(rename = "isFunction")]
            pub(crate) is_function: Option<bool>
        }
    }
}
pub(crate) type WritableStream = OnlyGuid;
pub mod writable_stream {
    pub mod commands {
        pub type Close = ();
        pub type CloseArgs = ();
        pub type Write = ();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct WriteArgs<'a> {
            #[serde(borrow)]
            #[serde(rename = "binary")]
            pub(crate) binary: &'a [u8]
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    #[serde(rename = "fetchUid")]
    pub(crate) fetch_uid: String,
    #[serde(rename = "headers")]
    pub(crate) headers: Vec<crate::protocol::generated::NameValue>,
    #[serde(rename = "status")]
    pub(crate) status: serde_json::Number,
    #[serde(rename = "statusText")]
    pub(crate) status_text: String,
    #[serde(rename = "url")]
    pub(crate) url: String
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AxNode {
    #[serde(rename = "autocomplete")]
    pub(crate) autocomplete: Option<String>,
    #[serde(rename = "checked")]
    pub(crate) checked: Option<AxNodeChecked>,
    #[serde(rename = "children")]
    pub(crate) children: Option<Vec<crate::protocol::generated::AxNode>>,
    #[serde(rename = "description")]
    pub(crate) description: Option<String>,
    #[serde(rename = "disabled")]
    pub(crate) disabled: Option<bool>,
    #[serde(rename = "expanded")]
    pub(crate) expanded: Option<bool>,
    #[serde(rename = "focused")]
    pub(crate) focused: Option<bool>,
    #[serde(rename = "haspopup")]
    pub(crate) haspopup: Option<String>,
    #[serde(rename = "invalid")]
    pub(crate) invalid: Option<String>,
    #[serde(rename = "keyshortcuts")]
    pub(crate) keyshortcuts: Option<String>,
    #[serde(rename = "level")]
    pub(crate) level: Option<serde_json::Number>,
    #[serde(rename = "modal")]
    pub(crate) modal: Option<bool>,
    #[serde(rename = "multiline")]
    pub(crate) multiline: Option<bool>,
    #[serde(rename = "multiselectable")]
    pub(crate) multiselectable: Option<bool>,
    #[serde(rename = "name")]
    pub(crate) name: String,
    #[serde(rename = "orientation")]
    pub(crate) orientation: Option<String>,
    #[serde(rename = "pressed")]
    pub(crate) pressed: Option<AxNodePressed>,
    #[serde(rename = "readonly")]
    pub(crate) readonly: Option<bool>,
    #[serde(rename = "required")]
    pub(crate) required: Option<bool>,
    #[serde(rename = "role")]
    pub(crate) role: String,
    #[serde(rename = "roledescription")]
    pub(crate) roledescription: Option<String>,
    #[serde(rename = "selected")]
    pub(crate) selected: Option<bool>,
    #[serde(rename = "valueNumber")]
    pub(crate) value_number: Option<serde_json::Number>,
    #[serde(rename = "valueString")]
    pub(crate) value_string: Option<String>,
    #[serde(rename = "valuemax")]
    pub(crate) valuemax: Option<serde_json::Number>,
    #[serde(rename = "valuemin")]
    pub(crate) valuemin: Option<serde_json::Number>,
    #[serde(rename = "valuetext")]
    pub(crate) valuetext: Option<String>
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum AxNodeChecked {
    #[serde(rename = "checked")]
    Checked,
    #[serde(rename = "unchecked")]
    Unchecked,
    #[serde(rename = "mixed")]
    Mixed
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
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
    pub(crate) bounds: crate::protocol::generated::Rect,
    #[serde(rename = "checkable")]
    pub(crate) checkable: bool,
    #[serde(rename = "checked")]
    pub(crate) checked: bool,
    #[serde(rename = "children")]
    pub(crate) children: Option<Vec<crate::protocol::generated::AndroidElementInfo>>,
    #[serde(rename = "clazz")]
    pub(crate) clazz: String,
    #[serde(rename = "clickable")]
    pub(crate) clickable: bool,
    #[serde(rename = "desc")]
    pub(crate) desc: String,
    #[serde(rename = "enabled")]
    pub(crate) enabled: bool,
    #[serde(rename = "focusable")]
    pub(crate) focusable: bool,
    #[serde(rename = "focused")]
    pub(crate) focused: bool,
    #[serde(rename = "longClickable")]
    pub(crate) long_clickable: bool,
    #[serde(rename = "pkg")]
    pub(crate) pkg: String,
    #[serde(rename = "res")]
    pub(crate) res: String,
    #[serde(rename = "scrollable")]
    pub(crate) scrollable: bool,
    #[serde(rename = "selected")]
    pub(crate) selected: bool,
    #[serde(rename = "text")]
    pub(crate) text: String
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AndroidSelector {
    #[serde(rename = "checkable")]
    pub(crate) checkable: Option<bool>,
    #[serde(rename = "checked")]
    pub(crate) checked: Option<bool>,
    #[serde(rename = "clazz")]
    pub(crate) clazz: Option<String>,
    #[serde(rename = "clickable")]
    pub(crate) clickable: Option<bool>,
    #[serde(rename = "depth")]
    pub(crate) depth: Option<serde_json::Number>,
    #[serde(rename = "desc")]
    pub(crate) desc: Option<String>,
    #[serde(rename = "enabled")]
    pub(crate) enabled: Option<bool>,
    #[serde(rename = "focusable")]
    pub(crate) focusable: Option<bool>,
    #[serde(rename = "focused")]
    pub(crate) focused: Option<bool>,
    #[serde(rename = "hasChild")]
    pub(crate) has_child: Option<Box<AndroidSelectorHasChild>>,
    #[serde(rename = "hasDescendant")]
    pub(crate) has_descendant: Option<Box<AndroidSelectorHasDescendant>>,
    #[serde(rename = "longClickable")]
    pub(crate) long_clickable: Option<bool>,
    #[serde(rename = "pkg")]
    pub(crate) pkg: Option<String>,
    #[serde(rename = "res")]
    pub(crate) res: Option<String>,
    #[serde(rename = "scrollable")]
    pub(crate) scrollable: Option<bool>,
    #[serde(rename = "selected")]
    pub(crate) selected: Option<bool>,
    #[serde(rename = "text")]
    pub(crate) text: Option<String>
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AndroidSelectorHasChild {
    #[serde(rename = "selector")]
    pub(crate) selector: crate::protocol::generated::AndroidSelector
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AndroidSelectorHasDescendant {
    #[serde(rename = "maxDepth")]
    pub(crate) max_depth: Option<serde_json::Number>,
    #[serde(rename = "selector")]
    pub(crate) selector: crate::protocol::generated::AndroidSelector
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AndroidWebView {
    #[serde(rename = "pid")]
    pub(crate) pid: serde_json::Number,
    #[serde(rename = "pkg")]
    pub(crate) pkg: String,
    #[serde(rename = "socketName")]
    pub(crate) socket_name: String
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ExpectedTextValue {
    #[serde(rename = "ignoreCase")]
    pub(crate) ignore_case: Option<bool>,
    #[serde(rename = "matchSubstring")]
    pub(crate) match_substring: Option<bool>,
    #[serde(rename = "normalizeWhiteSpace")]
    pub(crate) normalize_white_space: Option<bool>,
    #[serde(rename = "regexFlags")]
    pub(crate) regex_flags: Option<String>,
    #[serde(rename = "regexSource")]
    pub(crate) regex_source: Option<String>,
    #[serde(rename = "string")]
    pub(crate) string: Option<String>
}
#[derive(Debug, Serialize, Deserialize)]
pub struct FormField {
    #[serde(rename = "file")]
    pub(crate) file: Option<FormFieldFile>,
    #[serde(rename = "name")]
    pub(crate) name: String,
    #[serde(rename = "value")]
    pub(crate) value: Option<String>
}
#[derive(Debug, Serialize, Deserialize)]
pub struct FormFieldFile {
    #[serde(rename = "buffer")]
    pub(crate) buffer: Vec<u8>,
    #[serde(rename = "mimeType")]
    pub(crate) mime_type: Option<String>,
    #[serde(rename = "name")]
    pub(crate) name: String
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(rename = "apiName")]
    pub(crate) api_name: Option<String>,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(rename = "internal")]
    pub(crate) internal: Option<bool>,
    #[serde(rename = "stack")]
    pub(crate) stack: Option<Vec<crate::protocol::generated::StackFrame>>
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NameValue {
    #[serde(rename = "name")]
    pub(crate) name: String,
    #[serde(rename = "value")]
    pub(crate) value: String
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkCookie {
    #[serde(rename = "domain")]
    pub(crate) domain: String,
    #[serde(rename = "expires")]
    pub(crate) expires: serde_json::Number,
    #[serde(rename = "httpOnly")]
    pub(crate) http_only: bool,
    #[serde(rename = "name")]
    pub(crate) name: String,
    #[serde(rename = "path")]
    pub(crate) path: String,
    #[serde(rename = "sameSite")]
    pub(crate) same_site: NetworkCookieSameSite,
    #[serde(rename = "secure")]
    pub(crate) secure: bool,
    #[serde(rename = "value")]
    pub(crate) value: String
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum NetworkCookieSameSite {
    Strict,
    Lax,
    None
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OriginStorage {
    #[serde(rename = "localStorage")]
    pub(crate) local_storage: Vec<crate::protocol::generated::NameValue>,
    #[serde(rename = "origin")]
    pub(crate) origin: String
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Point {
    #[serde(rename = "x")]
    pub(crate) x: serde_json::Number,
    #[serde(rename = "y")]
    pub(crate) y: serde_json::Number
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RecordHarOptions {
    #[serde(rename = "content")]
    pub(crate) content: Option<RecordHarOptionsContent>,
    #[serde(rename = "mode")]
    pub(crate) mode: Option<RecordHarOptionsMode>,
    #[serde(rename = "path")]
    pub(crate) path: String,
    #[serde(rename = "urlGlob")]
    pub(crate) url_glob: Option<String>,
    #[serde(rename = "urlRegexFlags")]
    pub(crate) url_regex_flags: Option<String>,
    #[serde(rename = "urlRegexSource")]
    pub(crate) url_regex_source: Option<String>
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum RecordHarOptionsContent {
    #[serde(rename = "embed")]
    Embed,
    #[serde(rename = "attach")]
    Attach,
    #[serde(rename = "omit")]
    Omit
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum RecordHarOptionsMode {
    #[serde(rename = "full")]
    Full,
    #[serde(rename = "minimal")]
    Minimal
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Rect {
    #[serde(rename = "height")]
    pub(crate) height: serde_json::Number,
    #[serde(rename = "width")]
    pub(crate) width: serde_json::Number,
    #[serde(rename = "x")]
    pub(crate) x: serde_json::Number,
    #[serde(rename = "y")]
    pub(crate) y: serde_json::Number
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RemoteAddr {
    #[serde(rename = "ipAddress")]
    pub(crate) ip_address: String,
    #[serde(rename = "port")]
    pub(crate) port: serde_json::Number
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestSizes {
    #[serde(rename = "requestBodySize")]
    pub(crate) request_body_size: serde_json::Number,
    #[serde(rename = "requestHeadersSize")]
    pub(crate) request_headers_size: serde_json::Number,
    #[serde(rename = "responseBodySize")]
    pub(crate) response_body_size: serde_json::Number,
    #[serde(rename = "responseHeadersSize")]
    pub(crate) response_headers_size: serde_json::Number
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceTiming {
    #[serde(rename = "connectEnd")]
    pub(crate) connect_end: serde_json::Number,
    #[serde(rename = "connectStart")]
    pub(crate) connect_start: serde_json::Number,
    #[serde(rename = "domainLookupEnd")]
    pub(crate) domain_lookup_end: serde_json::Number,
    #[serde(rename = "domainLookupStart")]
    pub(crate) domain_lookup_start: serde_json::Number,
    #[serde(rename = "requestStart")]
    pub(crate) request_start: serde_json::Number,
    #[serde(rename = "responseStart")]
    pub(crate) response_start: serde_json::Number,
    #[serde(rename = "secureConnectionStart")]
    pub(crate) secure_connection_start: serde_json::Number,
    #[serde(rename = "startTime")]
    pub(crate) start_time: serde_json::Number
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityDetails {
    #[serde(rename = "issuer")]
    pub(crate) issuer: Option<String>,
    #[serde(rename = "protocol")]
    pub(crate) protocol: Option<String>,
    #[serde(rename = "subjectName")]
    pub(crate) subject_name: Option<String>,
    #[serde(rename = "validFrom")]
    pub(crate) valid_from: Option<serde_json::Number>,
    #[serde(rename = "validTo")]
    pub(crate) valid_to: Option<serde_json::Number>
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedArgument {
    #[serde(rename = "handles")]
    pub(crate) handles: Vec<crate::protocol::generated::Channel>,
    #[serde(rename = "value")]
    pub(crate) value: crate::protocol::generated::SerializedValue
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedError {
    #[serde(rename = "error")]
    pub(crate) error: Option<SerializedErrorError>,
    #[serde(rename = "value")]
    pub(crate) value: Option<crate::protocol::generated::SerializedValue>
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedErrorError {
    #[serde(rename = "message")]
    pub(crate) message: String,
    #[serde(rename = "name")]
    pub(crate) name: String,
    #[serde(rename = "stack")]
    pub(crate) stack: Option<String>
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedValue {
    #[serde(rename = "a")]
    pub(crate) a: Option<Vec<crate::protocol::generated::SerializedValue>>,
    #[serde(rename = "b")]
    pub(crate) b: Option<bool>,
    #[serde(rename = "d")]
    pub(crate) d: Option<String>,
    #[serde(rename = "h")]
    pub(crate) h: Option<serde_json::Number>,
    #[serde(rename = "id")]
    pub(crate) id: Option<serde_json::Number>,
    #[serde(rename = "n")]
    pub(crate) n: Option<serde_json::Number>,
    #[serde(rename = "o")]
    pub(crate) o: Option<Vec<SerializedValueO>>,
    #[serde(rename = "r")]
    pub(crate) r: Option<SerializedValueR>,
    #[serde(rename = "ref")]
    pub(crate) r#ref: Option<serde_json::Number>,
    #[serde(rename = "s")]
    pub(crate) s: Option<String>,
    #[serde(rename = "u")]
    pub(crate) u: Option<String>,
    #[serde(rename = "v")]
    pub(crate) v: Option<SerializedValueV>
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedValueO {
    #[serde(rename = "k")]
    pub(crate) k: String,
    #[serde(rename = "v")]
    pub(crate) v: crate::protocol::generated::SerializedValue
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedValueR {
    #[serde(rename = "f")]
    pub(crate) f: String,
    #[serde(rename = "p")]
    pub(crate) p: String
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
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
    pub(crate) domain: Option<String>,
    #[serde(rename = "expires")]
    pub(crate) expires: Option<serde_json::Number>,
    #[serde(rename = "httpOnly")]
    pub(crate) http_only: Option<bool>,
    #[serde(rename = "name")]
    pub(crate) name: String,
    #[serde(rename = "path")]
    pub(crate) path: Option<String>,
    #[serde(rename = "sameSite")]
    pub(crate) same_site: Option<SetNetworkCookieSameSite>,
    #[serde(rename = "secure")]
    pub(crate) secure: Option<bool>,
    #[serde(rename = "url")]
    pub(crate) url: Option<String>,
    #[serde(rename = "value")]
    pub(crate) value: String
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum SetNetworkCookieSameSite {
    Strict,
    Lax,
    None
}
#[derive(Debug, Serialize, Deserialize)]
pub struct StackFrame {
    #[serde(rename = "column")]
    pub(crate) column: Option<serde_json::Number>,
    #[serde(rename = "file")]
    pub(crate) file: String,
    #[serde(rename = "function")]
    pub(crate) function: Option<String>,
    #[serde(rename = "line")]
    pub(crate) line: Option<serde_json::Number>
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CommonScreenshotOptions {
    #[serde(rename = "animations")]
    pub(crate) animations: Option<CommonScreenshotOptionsAnimations>,
    #[serde(rename = "caret")]
    pub(crate) caret: Option<CommonScreenshotOptionsCaret>,
    #[serde(rename = "mask")]
    pub(crate) mask: Option<Vec<CommonScreenshotOptionsMask>>,
    #[serde(rename = "omitBackground")]
    pub(crate) omit_background: Option<bool>,
    #[serde(rename = "scale")]
    pub(crate) scale: Option<CommonScreenshotOptionsScale>
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum CommonScreenshotOptionsAnimations {
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "allow")]
    Allow
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum CommonScreenshotOptionsCaret {
    #[serde(rename = "hide")]
    Hide,
    #[serde(rename = "initial")]
    Initial
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CommonScreenshotOptionsMask {
    #[serde(rename = "frame")]
    pub(crate) frame: crate::protocol::generated::Frame,
    #[serde(rename = "selector")]
    pub(crate) selector: String
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum CommonScreenshotOptionsScale {
    #[serde(rename = "css")]
    Css,
    #[serde(rename = "device")]
    Device
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ContextOptions {
    #[serde(rename = "acceptDownloads")]
    pub(crate) accept_downloads: Option<bool>,
    #[serde(rename = "baseURL")]
    pub(crate) base_url: Option<String>,
    #[serde(rename = "bypassCSP")]
    pub(crate) bypass_csp: Option<bool>,
    #[serde(rename = "colorScheme")]
    pub(crate) color_scheme: Option<ContextOptionsColorScheme>,
    #[serde(rename = "deviceScaleFactor")]
    pub(crate) device_scale_factor: Option<serde_json::Number>,
    #[serde(rename = "extraHTTPHeaders")]
    pub(crate) extra_http_headers: Option<Vec<crate::protocol::generated::NameValue>>,
    #[serde(rename = "forcedColors")]
    pub(crate) forced_colors: Option<ContextOptionsForcedColors>,
    #[serde(rename = "geolocation")]
    pub(crate) geolocation: Option<ContextOptionsGeolocation>,
    #[serde(rename = "hasTouch")]
    pub(crate) has_touch: Option<bool>,
    #[serde(rename = "httpCredentials")]
    pub(crate) http_credentials: Option<ContextOptionsHttpCredentials>,
    #[serde(rename = "ignoreHTTPSErrors")]
    pub(crate) ignore_https_errors: Option<bool>,
    #[serde(rename = "isMobile")]
    pub(crate) is_mobile: Option<bool>,
    #[serde(rename = "javaScriptEnabled")]
    pub(crate) java_script_enabled: Option<bool>,
    #[serde(rename = "locale")]
    pub(crate) locale: Option<String>,
    #[serde(rename = "noDefaultViewport")]
    pub(crate) no_default_viewport: Option<bool>,
    #[serde(rename = "offline")]
    pub(crate) offline: Option<bool>,
    #[serde(rename = "permissions")]
    pub(crate) permissions: Option<Vec<String>>,
    #[serde(rename = "recordHar")]
    pub(crate) record_har: Option<crate::protocol::generated::RecordHarOptions>,
    #[serde(rename = "recordVideo")]
    pub(crate) record_video: Option<ContextOptionsRecordVideo>,
    #[serde(rename = "reducedMotion")]
    pub(crate) reduced_motion: Option<ContextOptionsReducedMotion>,
    #[serde(rename = "screen")]
    pub(crate) screen: Option<ContextOptionsScreen>,
    #[serde(rename = "serviceWorkers")]
    pub(crate) service_workers: Option<ContextOptionsServiceWorkers>,
    #[serde(rename = "strictSelectors")]
    pub(crate) strict_selectors: Option<bool>,
    #[serde(rename = "timezoneId")]
    pub(crate) timezone_id: Option<String>,
    #[serde(rename = "userAgent")]
    pub(crate) user_agent: Option<String>,
    #[serde(rename = "viewport")]
    pub(crate) viewport: Option<ContextOptionsViewport>
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum ContextOptionsColorScheme {
    #[serde(rename = "dark")]
    Dark,
    #[serde(rename = "light")]
    Light,
    #[serde(rename = "no-preference")]
    NoPreference
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum ContextOptionsForcedColors {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "none")]
    None
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ContextOptionsGeolocation {
    #[serde(rename = "accuracy")]
    pub(crate) accuracy: Option<serde_json::Number>,
    #[serde(rename = "latitude")]
    pub(crate) latitude: serde_json::Number,
    #[serde(rename = "longitude")]
    pub(crate) longitude: serde_json::Number
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ContextOptionsHttpCredentials {
    #[serde(rename = "password")]
    pub(crate) password: String,
    #[serde(rename = "username")]
    pub(crate) username: String
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ContextOptionsRecordVideo {
    #[serde(rename = "dir")]
    pub(crate) dir: String,
    #[serde(rename = "size")]
    pub(crate) size: Option<ContextOptionsRecordVideoSize>
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ContextOptionsRecordVideoSize {
    #[serde(rename = "height")]
    pub(crate) height: serde_json::Number,
    #[serde(rename = "width")]
    pub(crate) width: serde_json::Number
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum ContextOptionsReducedMotion {
    #[serde(rename = "reduce")]
    Reduce,
    #[serde(rename = "no-preference")]
    NoPreference
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ContextOptionsScreen {
    #[serde(rename = "height")]
    pub(crate) height: serde_json::Number,
    #[serde(rename = "width")]
    pub(crate) width: serde_json::Number
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum ContextOptionsServiceWorkers {
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "block")]
    Block
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ContextOptionsViewport {
    #[serde(rename = "height")]
    pub(crate) height: serde_json::Number,
    #[serde(rename = "width")]
    pub(crate) width: serde_json::Number
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LaunchOptions {
    #[serde(rename = "args")]
    pub(crate) args: Option<Vec<String>>,
    #[serde(rename = "channel")]
    pub(crate) channel: Option<String>,
    #[serde(rename = "chromiumSandbox")]
    pub(crate) chromium_sandbox: Option<bool>,
    #[serde(rename = "devtools")]
    pub(crate) devtools: Option<bool>,
    #[serde(rename = "downloadsPath")]
    pub(crate) downloads_path: Option<String>,
    #[serde(rename = "env")]
    pub(crate) env: Option<Vec<crate::protocol::generated::NameValue>>,
    #[serde(rename = "executablePath")]
    pub(crate) executable_path: Option<String>,
    #[serde(rename = "handleSIGHUP")]
    pub(crate) handle_sighup: Option<bool>,
    #[serde(rename = "handleSIGINT")]
    pub(crate) handle_sigint: Option<bool>,
    #[serde(rename = "handleSIGTERM")]
    pub(crate) handle_sigterm: Option<bool>,
    #[serde(rename = "headless")]
    pub(crate) headless: Option<bool>,
    #[serde(rename = "ignoreAllDefaultArgs")]
    pub(crate) ignore_all_default_args: Option<bool>,
    #[serde(rename = "ignoreDefaultArgs")]
    pub(crate) ignore_default_args: Option<Vec<String>>,
    #[serde(rename = "proxy")]
    pub(crate) proxy: Option<LaunchOptionsProxy>,
    #[serde(rename = "timeout")]
    pub(crate) timeout: Option<serde_json::Number>,
    #[serde(rename = "tracesDir")]
    pub(crate) traces_dir: Option<String>
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LaunchOptionsProxy {
    #[serde(rename = "bypass")]
    pub(crate) bypass: Option<String>,
    #[serde(rename = "password")]
    pub(crate) password: Option<String>,
    #[serde(rename = "server")]
    pub(crate) server: String,
    #[serde(rename = "username")]
    pub(crate) username: Option<String>
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Hash, Copy)]
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
