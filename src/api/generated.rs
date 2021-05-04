#[doc = "The Accessibility class provides methods for inspecting Chromium's accessibility tree. The accessibility tree is used by\nassistive technology such as [screen readers](https://en.wikipedia.org/wiki/Screen_reader) or\n[switches](https://en.wikipedia.org/wiki/Switch_access).\n\nAccessibility is a very platform-specific thing. On different platforms, there are different screen readers that might\nhave wildly different output.\n\nRendering engines of Chromium, Firefox and WebKit have a concept of \"accessibility tree\", which is then translated into\ndifferent platform-specific APIs. Accessibility namespace gives access to this Accessibility Tree.\n\nMost of the accessibility tree gets filtered out when converting from internal browser AX Tree to Platform-specific\nAX-Tree or by assistive technologies themselves. By default, Playwright tries to approximate this filtering, exposing\nonly the \"interesting\" nodes of the tree."]
impl Accessibility {
    #[doc = "Captures the current state of the accessibility tree. The returned object represents the root accessible node of the\npage.\n\n> NOTE: The Chromium accessibility tree contains nodes that go unused on most platforms and by most screen readers.\nPlaywright will discard them as well for an easier to process tree, unless `interestingOnly` is set to `false`.\n\nAn example of dumping the entire accessibility tree:\n\n```js\nconst snapshot = await page.accessibility.snapshot();\nconsole.log(snapshot);\n```\n\n```java\nString snapshot = page.accessibility().snapshot();\nSystem.out.println(snapshot);\n```\n\n```python async\nsnapshot = await page.accessibility.snapshot()\nprint(snapshot)\n```\n\n```python sync\nsnapshot = page.accessibility.snapshot()\nprint(snapshot)\n```\n\n```csharp\nvar accessibilitySnapshot = await Page.Accessibility.SnapshotAsync();\nConsole.WriteLine(accessibilitySnapshot);\n```\n\nAn example of logging the focused node's name:\n\n```js\nconst snapshot = await page.accessibility.snapshot();\nconst node = findFocusedNode(snapshot);\nconsole.log(node && node.name);\n\nfunction findFocusedNode(node) {\n  if (node.focused)\n    return node;\n  for (const child of node.children || []) {\n    const foundNode = findFocusedNode(child);\n    return foundNode;\n  }\n  return null;\n}\n```\n\n```csharp\nFunc<AccessibilitySnapshotResult, AccessibilitySnapshotResult> findFocusedNode = root =>\n{\n    var nodes = new Stack<AccessibilitySnapshotResult>(new[] { root });\n    while (nodes.Count > 0)\n    {\n        var node = nodes.Pop();\n        if (node.Focused) return node;\n        foreach (var innerNode in node.Children)\n        {\n            nodes.Push(innerNode);\n        }\n    }\n\n    return null;\n};\n\nvar accessibilitySnapshot = await Page.Accessibility.SnapshotAsync();\nvar focusedNode = findFocusedNode(accessibilitySnapshot);\nif(focusedNode != null)\n  Console.WriteLine(focusedNode.Name);\n```\n\n```java\n// FIXME\nString snapshot = page.accessibility().snapshot();\n```\n\n```python async\ndef find_focused_node(node):\n    if (node.get(\"focused\"))\n        return node\n    for child in (node.get(\"children\") or []):\n        found_node = find_focused_node(child)\n        return found_node\n    return None\n\nsnapshot = await page.accessibility.snapshot()\nnode = find_focused_node(snapshot)\nif node:\n    print(node[\"name\"])\n```\n\n```python sync\ndef find_focused_node(node):\n    if (node.get(\"focused\"))\n        return node\n    for child in (node.get(\"children\") or []):\n        found_node = find_focused_node(child)\n        return found_node\n    return None\n\nsnapshot = page.accessibility.snapshot()\nnode = find_focused_node(snapshot)\nif node:\n    print(node[\"name\"])\n```\n"]
    fn snapshot(
        &self,
        #[doc = "options"]
        #[doc = "Prune uninteresting nodes from the tree. Defaults to `true`."]
        interesting_only: Option<bool>,
        #[doc = "The root DOM element for the snapshot. Defaults to the whole page."] root: Option<
            ElementHandle
        >
    ) -> Result<Option<NotImplementedYet>, Arc<Error>> {
        todo!()
    }
}
#[doc = "Playwright has **experimental** support for Android automation. You can access android namespace via:\n\n```js\nconst { _android: android } = require('playwright');\n```\n\nAn example of the Android automation script would be:\n\n```js\nconst { _android: android } = require('playwright');\n\n(async () => {\n  // Connect to the device.\n  const [device] = await android.devices();\n  console.log(`Model: ${device.model()}`);\n  console.log(`Serial: ${device.serial()}`);\n  // Take screenshot of the whole device.\n  await device.screenshot({ path: 'device.png' });\n\n  {\n    // --------------------- WebView -----------------------\n\n    // Launch an application with WebView.\n    await device.shell('am force-stop org.chromium.webview_shell');\n    await device.shell('am start org.chromium.webview_shell/.WebViewBrowserActivity');\n    // Get the WebView.\n    const webview = await device.webView({ pkg: 'org.chromium.webview_shell' });\n\n    // Fill the input box.\n    await device.fill({ res: 'org.chromium.webview_shell:id/url_field' }, 'github.com/microsoft/playwright');\n    await device.press({ res: 'org.chromium.webview_shell:id/url_field' }, 'Enter');\n\n    // Work with WebView's page as usual.\n    const page = await webview.page();\n    await page.waitForNavigation({ url: /.*microsoft\\/playwright.*/ });\n    console.log(await page.title());\n  }\n\n  {\n    // --------------------- Browser -----------------------\n\n    // Launch Chrome browser.\n    await device.shell('am force-stop com.android.chrome');\n    const context = await device.launchBrowser();\n\n    // Use BrowserContext as usual.\n    const page = await context.newPage();\n    await page.goto('https://webkit.org/');\n    console.log(await page.evaluate(() => window.location.href));\n    await page.screenshot({ path: 'page.png' });\n\n    await context.close();\n  }\n\n  // Close the device.\n  await device.close();\n})();\n```\n\nNote that since you don't need Playwright to install web browsers when testing Android, you can omit browser download\nvia setting the following environment variable when installing Playwright:\n\n```sh js\n$ PLAYWRIGHT_SKIP_BROWSER_DOWNLOAD=1 npm i -D playwright\n```\n"]
impl Android {
    #[doc = "Returns the list of detected Android devices."]
    fn devices(&self) -> Result<Vec<AndroidDevice>, Arc<Error>> { todo!() }
    #[doc = "This setting will change the default maximum time for all the methods accepting `timeout` option."]
    fn set_default_timeout(
        &self,
        #[doc = "Maximum time in milliseconds"] timeout: f64
    ) -> Result<(), Error> {
        todo!()
    }
}
#[doc = "`AndroidDevice` represents a connected device, either real hardware or emulated. Devices can be obtained using\n[`method: Android.devices`]."]
impl AndroidDevice {
    #[doc = ""]
    pub fn input(&self) -> AndroidInput {}
    #[doc = "Disconnects from the device."]
    fn close(&self) -> Result<(), Arc<Error>> { todo!() }
    #[doc = "Drags the widget defined by `selector` towards `dest` point."]
    fn drag(
        &self,
        #[doc = "Selector to drag."] selector: AndroidSelector,
        #[doc = "Point to drag to."] dest: NotImplementedYet,
        #[doc = "options"]
        #[doc = "Optional speed of the drag in pixels per second."]
        speed: Option<f64>,
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: AndroidDevice.setDefaultTimeout`] method."]
        timeout: Option<f64>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Fills the specific `selector` input box with `text`."]
    fn fill(
        &self,
        #[doc = "Selector to fill."] selector: AndroidSelector,
        #[doc = "Text to be filled in the input box."] text: String,
        #[doc = "options"]
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: AndroidDevice.setDefaultTimeout`] method."]
        timeout: Option<f64>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Flings the widget defined by `selector` in  the specified `direction`."]
    fn fling(
        &self,
        #[doc = "Selector to fling."] selector: AndroidSelector,
        #[doc = "Fling direction."] direction: AndroidFlingDirection,
        #[doc = "options"]
        #[doc = "Optional speed of the fling in pixels per second."]
        speed: Option<f64>,
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: AndroidDevice.setDefaultTimeout`] method."]
        timeout: Option<f64>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Returns information about a widget defined by `selector`."]
    fn info(
        &self,
        #[doc = "Selector to return information about."] selector: AndroidSelector
    ) -> Result<AndroidElementInfo, Arc<Error>> {
        todo!()
    }
    #[doc = "Installs an apk on the device."]
    fn install_apk(
        &self,
        #[doc = "Either a path to the apk file, or apk file content."] file: NotImplementedYet,
        #[doc = "options"]
        #[doc = "Optional arguments to pass to the `shell:cmd package install` call. Defaults to `-r -t -S`."]
        args: Option<Vec<String>>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Launches Chrome browser on the device, and returns its persistent context."]
    fn launch_browser(
        &self,
        #[doc = "options"]
        #[doc = "Whether to automatically download all the attachments. Defaults to `false` where all the downloads are canceled."]
        accept_downloads: Option<bool>,
        #[doc = "Toggles bypassing page's Content-Security-Policy."] bypass_c_s_p: Option<bool>,
        #[doc = "Emulates `'prefers-colors-scheme'` media feature, supported values are `'light'`, `'dark'`, `'no-preference'`. See\n[`method: Page.emulateMedia`] for more details. Defaults to `'light'`."]
        color_scheme: Option<ColorScheme>,
        command : Option < String >,
        #[doc = "Specify device scale factor (can be thought of as dpr). Defaults to `1`."]
        device_scale_factor: Option<f64>,
        #[doc = "An object containing additional HTTP headers to be sent with every request. All header values must be strings."]
        extra_h_t_t_p_headers: Option<Map<String, String>>,
        #[doc = ""] geolocation: Option<NotImplementedYet>,
        has_touch : Option < bool >,
        #[doc = "Credentials for [HTTP authentication](https://developer.mozilla.org/en-US/docs/Web/HTTP/Authentication)."]
        http_credentials: Option<NotImplementedYet>,
        #[doc = "Whether to ignore HTTPS errors during navigation. Defaults to `false`."]
        ignore_h_t_t_p_s_errors: Option<bool>,
        #[doc = "Whether the `meta viewport` tag is taken into account and touch events are enabled. Defaults to `false`. Not supported\nin Firefox."]
        is_mobile: Option<bool>,
        #[doc = "Whether or not to enable JavaScript in the context. Defaults to `true`."]
        java_script_enabled: Option<bool>,
        #[doc = "Specify user locale, for example `en-GB`, `de-DE`, etc. Locale will affect `navigator.language` value, `Accept-Language`\nrequest header value as well as number and date formatting rules."]
        locale: Option<String>,
        #[doc = "Logger sink for Playwright logging."] logger: Option<Logger>,
        #[doc = "Does not enforce fixed viewport, allows resizing window in the headed mode."]
        no_viewport: Option<bool>,
        #[doc = "Whether to emulate network being offline. Defaults to `false`."] offline: Option<
            bool
        >,
        #[doc = "A list of permissions to grant to all pages in this context. See [`method: BrowserContext.grantPermissions`] for more\ndetails."]
        permissions: Option<Vec<String>>,
        #[doc = "Enables [HAR](http://www.softwareishard.com/blog/har-12-spec) recording for all pages into `recordHar.path` file. If not\nspecified, the HAR is not recorded. Make sure to await [`method: BrowserContext.close`] for the HAR to be saved."]
        record_har: Option<NotImplementedYet>,
        #[doc = "Optional setting to control whether to omit request content from the HAR. Defaults to `false`."]
        record_har_omit_content: Option<bool>,
        #[doc = "Enables [HAR](http://www.softwareishard.com/blog/har-12-spec) recording for all pages into the specified HAR file on the\nfilesystem. If not specified, the HAR is not recorded. Make sure to call [`method: BrowserContext.close`] for the HAR to\nbe saved."]
        record_har_path: Option<path>,
        #[doc = "Enables video recording for all pages into `recordVideo.dir` directory. If not specified videos are not recorded. Make\nsure to await [`method: BrowserContext.close`] for videos to be saved."]
        record_video: Option<NotImplementedYet>,
        #[doc = "Enables video recording for all pages into the specified directory. If not specified videos are not recorded. Make sure\nto call [`method: BrowserContext.close`] for videos to be saved."]
        record_video_dir: Option<path>,
        #[doc = "Dimensions of the recorded videos. If not specified the size will be equal to `viewport` scaled down to fit into\n800x800. If `viewport` is not configured explicitly the video size defaults to 800x450. Actual picture of each page will\nbe scaled down if necessary to fit the specified size."]
        record_video_size: Option<NotImplementedYet>,
        #[doc = "Emulates consistent window screen size available inside web page via `window.screen`. Is only used when the `viewport`\nis set."]
        screen: Option<NotImplementedYet>,
        #[doc = "Changes the timezone of the context. See\n[ICU's metaZones.txt](https://cs.chromium.org/chromium/src/third_party/icu/source/data/misc/metaZones.txt?rcl=faee8bc70570192d82d2978a71e2a615788597d1)\nfor a list of supported timezone IDs."]
        timezone_id: Option<String>,
        #[doc = "Specific user agent to use in this context."] user_agent: Option<String>,
        #[doc = "**DEPRECATED** Use `recordVideo` instead."] video_size: Option<NotImplementedYet>,
        #[doc = "**DEPRECATED** Use `recordVideo` instead."] videos_path: Option<path>,
        #[doc = "Emulates consistent viewport for each page. Defaults to an 1280x720 viewport. `null` disables the default viewport."]
        viewport: Option<Option<NotImplementedYet>>,
        #[doc = "Sets a consistent viewport for each page. Defaults to an 1280x720 viewport. `no_viewport` disables the fixed viewport."]
        viewport: Option<Option<NotImplementedYet>>
    ) -> Result<BrowserContext, Arc<Error>> {
        todo!()
    }
    #[doc = "Performs a long tap on the widget defined by `selector`."]
    fn long_tap(
        &self,
        #[doc = "Selector to tap on."] selector: AndroidSelector,
        #[doc = "options"]
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: AndroidDevice.setDefaultTimeout`] method."]
        timeout: Option<f64>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Device model."]
    fn model(&self) -> Result<String, Error> { todo!() }
    #[doc = "Launches a process in the shell on the device and returns a socket to communicate with the launched process."]
    fn open(&self, #[doc = ""] command: String) -> Result<AndroidSocket, Arc<Error>> { todo!() }
    #[doc = "Pinches the widget defined by `selector` in the closing direction."]
    fn pinch_close(
        &self,
        #[doc = "Selector to pinch close."] selector: AndroidSelector,
        #[doc = "The size of the pinch as a percentage of the widget's size."] percent: f64,
        #[doc = "options"]
        #[doc = "Optional speed of the pinch in pixels per second."]
        speed: Option<f64>,
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: AndroidDevice.setDefaultTimeout`] method."]
        timeout: Option<f64>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Pinches the widget defined by `selector` in the open direction."]
    fn pinch_open(
        &self,
        #[doc = "Selector to pinch open."] selector: AndroidSelector,
        #[doc = "The size of the pinch as a percentage of the widget's size."] percent: f64,
        #[doc = "options"]
        #[doc = "Optional speed of the pinch in pixels per second."]
        speed: Option<f64>,
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: AndroidDevice.setDefaultTimeout`] method."]
        timeout: Option<f64>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Presses the specific `key` in the widget defined by `selector`."]
    fn press(
        &self,
        #[doc = "Selector to press the key in."] selector: AndroidSelector,
        #[doc = "The key to press."] key: AndroidKey,
        #[doc = "options"]
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: AndroidDevice.setDefaultTimeout`] method."]
        timeout: Option<f64>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Copies a file to the device."]
    fn push(
        &self,
        #[doc = "Either a path to the file, or file content."] file: NotImplementedYet,
        #[doc = "Path to the file on the device."] path: String,
        #[doc = "options"]
        #[doc = "Optional file mode, defaults to `644` (`rw-r--r--`)."]
        mode: Option<i64>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Returns the buffer with the captured screenshot of the device."]
    fn screenshot(
        &self,
        #[doc = "options"]
        #[doc = "The file path to save the image to. If `path` is a relative path, then it is resolved relative to the current working\ndirectory. If no path is provided, the image won't be saved to the disk."]
        path: Option<path>
    ) -> Result<Buffer, Arc<Error>> {
        todo!()
    }
    #[doc = "Scrolls the widget defined by `selector` in  the specified `direction`."]
    fn scroll(
        &self,
        #[doc = "Selector to scroll."] selector: AndroidSelector,
        #[doc = "Scroll direction."] direction: AndroidScrollDirection,
        #[doc = "Distance to scroll as a percentage of the widget's size."] percent: f64,
        #[doc = "options"]
        #[doc = "Optional speed of the scroll in pixels per second."]
        speed: Option<f64>,
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: AndroidDevice.setDefaultTimeout`] method."]
        timeout: Option<f64>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Device serial number."]
    fn serial(&self) -> Result<String, Error> { todo!() }
    #[doc = "This setting will change the default maximum time for all the methods accepting `timeout` option."]
    fn set_default_timeout(
        &self,
        #[doc = "Maximum time in milliseconds"] timeout: f64
    ) -> Result<(), Error> {
        todo!()
    }
    #[doc = "Executes a shell command on the device and returns its output."]
    fn shell(
        &self,
        #[doc = "Shell command to execute."] command: String
    ) -> Result<Buffer, Arc<Error>> {
        todo!()
    }
    #[doc = "Swipes the widget defined by `selector` in  the specified `direction`."]
    fn swipe(
        &self,
        #[doc = "Selector to swipe."] selector: AndroidSelector,
        #[doc = "Swipe direction."] direction: AndroidSwipeDirection,
        #[doc = "Distance to swipe as a percentage of the widget's size."] percent: f64,
        #[doc = "options"]
        #[doc = "Optional speed of the swipe in pixels per second."]
        speed: Option<f64>,
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: AndroidDevice.setDefaultTimeout`] method."]
        timeout: Option<f64>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Taps on the widget defined by `selector`."]
    fn tap(
        &self,
        #[doc = "Selector to tap on."] selector: AndroidSelector,
        #[doc = "options"]
        #[doc = "Optional duration of the tap in milliseconds."]
        duration: Option<f64>,
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: AndroidDevice.setDefaultTimeout`] method."]
        timeout: Option<f64>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Waits for the specific `selector` to either appear or disappear, depending on the `state`."]
    fn wait(
        &self,
        #[doc = "Selector to wait for."] selector: AndroidSelector,
        #[doc = "options"]
        #[doc = "Optional state. Can be either:\n- default - wait for element to be present.\n- `'gone'` - wait for element to not be present."]
        state: Option<Gone>,
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: AndroidDevice.setDefaultTimeout`] method."]
        timeout: Option<f64>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Waits for event to fire and passes its value into the predicate function. Returns when the predicate returns truthy\nvalue."]
    fn wait_for_event(
        &self,
        #[doc = "Event name, same one typically passed into `*.on(event)`."] event: String,
        #[doc = "Either a predicate that receives an event or an options object. Optional."]
        options_or_predicate: Option<NotImplementedYet>
    ) -> Result<any, Arc<Error>> {
        todo!()
    }
    #[doc = "This method waits until `AndroidWebView` matching the `selector` is opened and returns it. If there is already an open\n`AndroidWebView` matching the `selector`, returns immediately."]
    fn web_view(
        &self,
        #[doc = ""] selector: NotImplementedYet,
        #[doc = "options"]
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: AndroidDevice.setDefaultTimeout`] method."]
        timeout: Option<f64>
    ) -> Result<AndroidWebView, Arc<Error>> {
        todo!()
    }
    #[doc = "Currently open WebViews."]
    fn web_views(&self) -> Result<Vec<AndroidWebView>, Error> { todo!() }
}
struct NotImplementedYetdest {
    #[doc = ""]
    x: f64,
    #[doc = ""]
    y: f64
}
enum NotImplementedYetdirection {
    NotImplementedYet(down),
    NotImplementedYet(up),
    NotImplementedYet(left),
    NotImplementedYet(right)
}
enum NotImplementedYetfile {
    NotImplementedYet(String),
    NotImplementedYet(Buffer)
}
enum NotImplementedYetfile {
    NotImplementedYet(String),
    NotImplementedYet(Buffer)
}
enum NotImplementedYetdirection {
    NotImplementedYet(down),
    NotImplementedYet(up),
    NotImplementedYet(left),
    NotImplementedYet(right)
}
enum NotImplementedYetdirection {
    NotImplementedYet(down),
    NotImplementedYet(up),
    NotImplementedYet(left),
    NotImplementedYet(right)
}
enum NotImplementedYetoptionsOrPredicate {
    NotImplementedYet(function),
    NotImplementedYet(NotImplementedYet)
}
struct NotImplementedYet {
    #[doc = "receives the event data and resolves to truthy value when the waiting should resolve."]
    predicate: function,
    #[doc = "maximum time to wait for in milliseconds. Defaults to `30000` (30 seconds). Pass `0` to disable timeout. The default\nvalue can be changed by using the [`method: AndroidDevice.setDefaultTimeout`]."]
    timeout: Option<f64>
}
struct NotImplementedYetselector {
    #[doc = "Package identifier."]
    pkg: String
}
enum AndroidDeviceEventType {
    #[doc = "Emitted when a new WebView instance is detected."]
    WebView
}
enum AndroidDeviceEvent {
    #[doc = "Emitted when a new WebView instance is detected."]
    WebView(AndroidWebView)
}
#[doc = ""]
impl AndroidInput {
    #[doc = "Performs a drag between `from` and `to` points."]
    fn drag(
        &self,
        #[doc = "The start point of the drag."] from: NotImplementedYet,
        #[doc = "The end point of the drag."] to: NotImplementedYet,
        #[doc = "The number of steps in the drag. Each step takes 5 milliseconds to complete."]
        steps: i64
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Presses the `key`."]
    fn press(&self, #[doc = "Key to press."] key: AndroidKey) -> Result<(), Arc<Error>> { todo!() }
    #[doc = "Swipes following the path defined by `segments`."]
    fn swipe(
        &self,
        #[doc = "The point to start swiping from."] from: NotImplementedYet,
        #[doc = "Points following the `from` point in the swipe gesture."] segments: Vec<
            NotImplementedYet
        >,
        #[doc = "The number of steps for each segment. Each step takes 5 milliseconds to complete, so 100 steps means half a second per\neach segment."]
        steps: i64
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Taps at the specified `point`."]
    fn tap(
        &self,
        #[doc = "The point to tap at."] point: NotImplementedYet
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Types `text` into currently focused widget."]
    fn r#type(&self, #[doc = "Text to type."] text: String) -> Result<(), Arc<Error>> { todo!() }
}
struct NotImplementedYetfrom {
    #[doc = ""]
    x: f64,
    #[doc = ""]
    y: f64
}
struct NotImplementedYetto {
    #[doc = ""]
    x: f64,
    #[doc = ""]
    y: f64
}
struct NotImplementedYetfrom {
    #[doc = ""]
    x: f64,
    #[doc = ""]
    y: f64
}
struct NotImplementedYetpoint {
    #[doc = ""]
    x: f64,
    #[doc = ""]
    y: f64
}
#[doc = "`AndroidSocket` is a way to communicate with a process launched on the `AndroidDevice`. Use\n[`method: AndroidDevice.open`] to open a socket."]
impl AndroidSocket {
    #[doc = "Closes the socket."]
    fn close(&self) -> Result<(), Arc<Error>> { todo!() }
    #[doc = "Writes some `data` to the socket."]
    fn write(&self, #[doc = "Data to write."] data: Buffer) -> Result<(), Arc<Error>> { todo!() }
}
enum AndroidSocketEventType {
    #[doc = "Emitted when the socket is closed."]
    Close,
    #[doc = "Emitted when data is available to read from the socket."]
    Data
}
enum AndroidSocketEvent {
    #[doc = "Emitted when the socket is closed."]
    Close(()),
    #[doc = "Emitted when data is available to read from the socket."]
    Data(Buffer)
}
#[doc = "`AndroidWebView` represents a WebView open on the `AndroidDevice`. WebView is usually obtained using\n[`method: AndroidDevice.webView`]."]
impl AndroidWebView {
    #[doc = "Connects to the WebView and returns a regular Playwright `Page` to interact with."]
    fn page(&self) -> Result<Page, Arc<Error>> { todo!() }
    #[doc = "WebView process PID."]
    fn pid(&self) -> Result<i64, Error> { todo!() }
    #[doc = "WebView package identifier."]
    fn pkg(&self) -> Result<String, Error> { todo!() }
}
enum AndroidWebViewEventType {
    #[doc = "Emitted when the WebView is closed."]
    Close
}
enum AndroidWebViewEvent {
    #[doc = "Emitted when the WebView is closed."]
    Close(())
}
#[doc = "- extends: [EventEmitter]\n\nA Browser is created via [`method: BrowserType.launch`]. An example of using a `Browser` to create a `Page`:\n\n```js\nconst { firefox } = require('playwright');  // Or 'chromium' or 'webkit'.\n\n(async () => {\n  const browser = await firefox.launch();\n  const page = await browser.newPage();\n  await page.goto('https://example.com');\n  await browser.close();\n})();\n```\n\n```java\nimport com.microsoft.playwright.*;\n\npublic class Example {\n  public static void main(String[] args) {\n    try (Playwright playwright = Playwright.create()) {\n      BrowserType firefox = playwright.firefox()\n      Browser browser = firefox.launch();\n      Page page = browser.newPage();\n      page.navigate('https://example.com');\n      browser.close();\n    }\n  }\n}\n```\n\n```python async\nimport asyncio\nfrom playwright.async_api import async_playwright\n\nasync def run(playwright):\n    firefox = playwright.firefox\n    browser = await firefox.launch()\n    page = await browser.new_page()\n    await page.goto(\"https://example.com\")\n    await browser.close()\n\nasync def main():\n    async with async_playwright() as playwright:\n        await run(playwright)\nasyncio.run(main())\n```\n\n```python sync\nfrom playwright.sync_api import sync_playwright\n\ndef run(playwright):\n    firefox = playwright.firefox\n    browser = firefox.launch()\n    page = browser.new_page()\n    page.goto(\"https://example.com\")\n    browser.close()\n\nwith sync_playwright() as playwright:\n    run(playwright)\n```\n"]
#[doc = "Extends EventEmitter"]
impl Browser {
    #[doc = "In case this browser is obtained using [`method: BrowserType.launch`], closes the browser and all of its pages (if any\nwere opened).\n\nIn case this browser is connected to, clears all created contexts belonging to this browser and disconnects from the\nbrowser server.\n\nThe `Browser` object itself is considered to be disposed and cannot be used anymore."]
    fn close(&self) -> Result<(), Arc<Error>> { todo!() }
    #[doc = "Returns an array of all open browser contexts. In a newly created browser, this will return zero browser contexts.\n\n```js\nconst browser = await pw.webkit.launch();\nconsole.log(browser.contexts().length); // prints `0`\n\nconst context = await browser.newContext();\nconsole.log(browser.contexts().length); // prints `1`\n```\n\n```java\nBrowser browser = pw.webkit().launch();\nSystem.out.println(browser.contexts().size()); // prints \"0\"\nBrowserContext context = browser.newContext();\nSystem.out.println(browser.contexts().size()); // prints \"1\"\n```\n\n```python async\nbrowser = await pw.webkit.launch()\nprint(len(browser.contexts())) # prints `0`\ncontext = await browser.new_context()\nprint(len(browser.contexts())) # prints `1`\n```\n\n```python sync\nbrowser = pw.webkit.launch()\nprint(len(browser.contexts())) # prints `0`\ncontext = browser.new_context()\nprint(len(browser.contexts())) # prints `1`\n```\n"]
    fn contexts(&self) -> Result<Vec<BrowserContext>, Error> { todo!() }
    #[doc = "Indicates that the browser is connected."]
    fn is_connected(&self) -> Result<bool, Error> { todo!() }
    #[doc = "> NOTE: CDP Sessions are only supported on Chromium-based browsers.\n\nReturns the newly created browser session."]
    fn new_browser_c_d_p_session(&self) -> Result<CDPSession, Arc<Error>> { todo!() }
    #[doc = "Creates a new browser context. It won't share cookies/cache with other browser contexts.\n\n```js\n(async () => {\n  const browser = await playwright.firefox.launch();  // Or 'chromium' or 'webkit'.\n  // Create a new incognito browser context.\n  const context = await browser.newContext();\n  // Create a new page in a pristine context.\n  const page = await context.newPage();\n  await page.goto('https://example.com');\n})();\n```\n\n```java\nBrowser browser = playwright.firefox().launch();  // Or 'chromium' or 'webkit'.\n// Create a new incognito browser context.\nBrowserContext context = browser.newContext();\n// Create a new page in a pristine context.\nPage page = context.newPage();\npage.navigate('https://example.com');\n```\n\n```python async\nbrowser = await playwright.firefox.launch() # or \"chromium\" or \"webkit\".\n# create a new incognito browser context.\ncontext = await browser.new_context()\n# create a new page in a pristine context.\npage = await context.new_page()\nawait page.goto(\"https://example.com\")\n```\n\n```python sync\nbrowser = playwright.firefox.launch() # or \"chromium\" or \"webkit\".\n# create a new incognito browser context.\ncontext = browser.new_context()\n# create a new page in a pristine context.\npage = context.new_page()\npage.goto(\"https://example.com\")\n```\n"]
    fn new_context(
        &self,
        #[doc = "options"]
        #[doc = "Whether to automatically download all the attachments. Defaults to `false` where all the downloads are canceled."]
        accept_downloads: Option<bool>,
        #[doc = "Toggles bypassing page's Content-Security-Policy."] bypass_c_s_p: Option<bool>,
        #[doc = "Emulates `'prefers-colors-scheme'` media feature, supported values are `'light'`, `'dark'`, `'no-preference'`. See\n[`method: Page.emulateMedia`] for more details. Defaults to `'light'`."]
        color_scheme: Option<ColorScheme>,
        #[doc = "Specify device scale factor (can be thought of as dpr). Defaults to `1`."]
        device_scale_factor: Option<f64>,
        #[doc = "An object containing additional HTTP headers to be sent with every request. All header values must be strings."]
        extra_h_t_t_p_headers: Option<Map<String, String>>,
        #[doc = ""] geolocation: Option<NotImplementedYet>,
        has_touch : Option < bool >,
        #[doc = "Credentials for [HTTP authentication](https://developer.mozilla.org/en-US/docs/Web/HTTP/Authentication)."]
        http_credentials: Option<NotImplementedYet>,
        #[doc = "Whether to ignore HTTPS errors during navigation. Defaults to `false`."]
        ignore_h_t_t_p_s_errors: Option<bool>,
        #[doc = "Whether the `meta viewport` tag is taken into account and touch events are enabled. Defaults to `false`. Not supported\nin Firefox."]
        is_mobile: Option<bool>,
        #[doc = "Whether or not to enable JavaScript in the context. Defaults to `true`."]
        java_script_enabled: Option<bool>,
        #[doc = "Specify user locale, for example `en-GB`, `de-DE`, etc. Locale will affect `navigator.language` value, `Accept-Language`\nrequest header value as well as number and date formatting rules."]
        locale: Option<String>,
        #[doc = "Logger sink for Playwright logging."] logger: Option<Logger>,
        #[doc = "Does not enforce fixed viewport, allows resizing window in the headed mode."]
        no_viewport: Option<bool>,
        #[doc = "Whether to emulate network being offline. Defaults to `false`."] offline: Option<
            bool
        >,
        #[doc = "A list of permissions to grant to all pages in this context. See [`method: BrowserContext.grantPermissions`] for more\ndetails."]
        permissions: Option<Vec<String>>,
        #[doc = "Network proxy settings to use with this context.\n\n> NOTE: For Chromium on Windows the browser needs to be launched with the global proxy for this option to work. If all\ncontexts override the proxy, global proxy will be never used and can be any string, for example `launch({ proxy: {\nserver: 'http://per-context' } })`."]
        proxy: Option<NotImplementedYet>,
        #[doc = "Enables [HAR](http://www.softwareishard.com/blog/har-12-spec) recording for all pages into `recordHar.path` file. If not\nspecified, the HAR is not recorded. Make sure to await [`method: BrowserContext.close`] for the HAR to be saved."]
        record_har: Option<NotImplementedYet>,
        #[doc = "Optional setting to control whether to omit request content from the HAR. Defaults to `false`."]
        record_har_omit_content: Option<bool>,
        #[doc = "Enables [HAR](http://www.softwareishard.com/blog/har-12-spec) recording for all pages into the specified HAR file on the\nfilesystem. If not specified, the HAR is not recorded. Make sure to call [`method: BrowserContext.close`] for the HAR to\nbe saved."]
        record_har_path: Option<path>,
        #[doc = "Enables video recording for all pages into `recordVideo.dir` directory. If not specified videos are not recorded. Make\nsure to await [`method: BrowserContext.close`] for videos to be saved."]
        record_video: Option<NotImplementedYet>,
        #[doc = "Enables video recording for all pages into the specified directory. If not specified videos are not recorded. Make sure\nto call [`method: BrowserContext.close`] for videos to be saved."]
        record_video_dir: Option<path>,
        #[doc = "Dimensions of the recorded videos. If not specified the size will be equal to `viewport` scaled down to fit into\n800x800. If `viewport` is not configured explicitly the video size defaults to 800x450. Actual picture of each page will\nbe scaled down if necessary to fit the specified size."]
        record_video_size: Option<NotImplementedYet>,
        #[doc = "Emulates consistent window screen size available inside web page via `window.screen`. Is only used when the `viewport`\nis set."]
        screen: Option<NotImplementedYet>,
        #[doc = "Populates context with given storage state. This option can be used to initialize context with logged-in information\nobtained via [`method: BrowserContext.storageState`]. Either a path to the file with saved storage, or an object with\nthe following fields:"]
        storage_state: Option<NotImplementedYet>,
        #[doc = "Populates context with given storage state. This option can be used to initialize context with logged-in information\nobtained via [`method: BrowserContext.storageState`]."]
        storage_state: Option<String>,
        #[doc = "Populates context with given storage state. This option can be used to initialize context with logged-in information\nobtained via [`method: BrowserContext.storageState`]. Path to the file with saved storage state."]
        storage_state_path: Option<path>,
        #[doc = "Changes the timezone of the context. See\n[ICU's metaZones.txt](https://cs.chromium.org/chromium/src/third_party/icu/source/data/misc/metaZones.txt?rcl=faee8bc70570192d82d2978a71e2a615788597d1)\nfor a list of supported timezone IDs."]
        timezone_id: Option<String>,
        #[doc = "Specific user agent to use in this context."] user_agent: Option<String>,
        #[doc = "**DEPRECATED** Use `recordVideo` instead."] video_size: Option<NotImplementedYet>,
        #[doc = "**DEPRECATED** Use `recordVideo` instead."] videos_path: Option<path>,
        #[doc = "Emulates consistent viewport for each page. Defaults to an 1280x720 viewport. `null` disables the default viewport."]
        viewport: Option<Option<NotImplementedYet>>,
        #[doc = "Sets a consistent viewport for each page. Defaults to an 1280x720 viewport. `no_viewport` disables the fixed viewport."]
        viewport: Option<Option<NotImplementedYet>>
    ) -> Result<BrowserContext, Arc<Error>> {
        todo!()
    }
    #[doc = "Creates a new page in a new browser context. Closing this page will close the context as well.\n\nThis is a convenience API that should only be used for the single-page scenarios and short snippets. Production code and\ntesting frameworks should explicitly create [`method: Browser.newContext`] followed by the\n[`method: BrowserContext.newPage`] to control their exact life times."]
    fn new_page(
        &self,
        #[doc = "options"]
        #[doc = "Whether to automatically download all the attachments. Defaults to `false` where all the downloads are canceled."]
        accept_downloads: Option<bool>,
        #[doc = "Toggles bypassing page's Content-Security-Policy."] bypass_c_s_p: Option<bool>,
        #[doc = "Emulates `'prefers-colors-scheme'` media feature, supported values are `'light'`, `'dark'`, `'no-preference'`. See\n[`method: Page.emulateMedia`] for more details. Defaults to `'light'`."]
        color_scheme: Option<ColorScheme>,
        #[doc = "Specify device scale factor (can be thought of as dpr). Defaults to `1`."]
        device_scale_factor: Option<f64>,
        #[doc = "An object containing additional HTTP headers to be sent with every request. All header values must be strings."]
        extra_h_t_t_p_headers: Option<Map<String, String>>,
        #[doc = ""] geolocation: Option<NotImplementedYet>,
        has_touch : Option < bool >,
        #[doc = "Credentials for [HTTP authentication](https://developer.mozilla.org/en-US/docs/Web/HTTP/Authentication)."]
        http_credentials: Option<NotImplementedYet>,
        #[doc = "Whether to ignore HTTPS errors during navigation. Defaults to `false`."]
        ignore_h_t_t_p_s_errors: Option<bool>,
        #[doc = "Whether the `meta viewport` tag is taken into account and touch events are enabled. Defaults to `false`. Not supported\nin Firefox."]
        is_mobile: Option<bool>,
        #[doc = "Whether or not to enable JavaScript in the context. Defaults to `true`."]
        java_script_enabled: Option<bool>,
        #[doc = "Specify user locale, for example `en-GB`, `de-DE`, etc. Locale will affect `navigator.language` value, `Accept-Language`\nrequest header value as well as number and date formatting rules."]
        locale: Option<String>,
        #[doc = "Logger sink for Playwright logging."] logger: Option<Logger>,
        #[doc = "Does not enforce fixed viewport, allows resizing window in the headed mode."]
        no_viewport: Option<bool>,
        #[doc = "Whether to emulate network being offline. Defaults to `false`."] offline: Option<
            bool
        >,
        #[doc = "A list of permissions to grant to all pages in this context. See [`method: BrowserContext.grantPermissions`] for more\ndetails."]
        permissions: Option<Vec<String>>,
        #[doc = "Network proxy settings to use with this context.\n\n> NOTE: For Chromium on Windows the browser needs to be launched with the global proxy for this option to work. If all\ncontexts override the proxy, global proxy will be never used and can be any string, for example `launch({ proxy: {\nserver: 'http://per-context' } })`."]
        proxy: Option<NotImplementedYet>,
        #[doc = "Enables [HAR](http://www.softwareishard.com/blog/har-12-spec) recording for all pages into `recordHar.path` file. If not\nspecified, the HAR is not recorded. Make sure to await [`method: BrowserContext.close`] for the HAR to be saved."]
        record_har: Option<NotImplementedYet>,
        #[doc = "Optional setting to control whether to omit request content from the HAR. Defaults to `false`."]
        record_har_omit_content: Option<bool>,
        #[doc = "Enables [HAR](http://www.softwareishard.com/blog/har-12-spec) recording for all pages into the specified HAR file on the\nfilesystem. If not specified, the HAR is not recorded. Make sure to call [`method: BrowserContext.close`] for the HAR to\nbe saved."]
        record_har_path: Option<path>,
        #[doc = "Enables video recording for all pages into `recordVideo.dir` directory. If not specified videos are not recorded. Make\nsure to await [`method: BrowserContext.close`] for videos to be saved."]
        record_video: Option<NotImplementedYet>,
        #[doc = "Enables video recording for all pages into the specified directory. If not specified videos are not recorded. Make sure\nto call [`method: BrowserContext.close`] for videos to be saved."]
        record_video_dir: Option<path>,
        #[doc = "Dimensions of the recorded videos. If not specified the size will be equal to `viewport` scaled down to fit into\n800x800. If `viewport` is not configured explicitly the video size defaults to 800x450. Actual picture of each page will\nbe scaled down if necessary to fit the specified size."]
        record_video_size: Option<NotImplementedYet>,
        #[doc = "Emulates consistent window screen size available inside web page via `window.screen`. Is only used when the `viewport`\nis set."]
        screen: Option<NotImplementedYet>,
        #[doc = "Populates context with given storage state. This option can be used to initialize context with logged-in information\nobtained via [`method: BrowserContext.storageState`]. Either a path to the file with saved storage, or an object with\nthe following fields:"]
        storage_state: Option<NotImplementedYet>,
        #[doc = "Populates context with given storage state. This option can be used to initialize context with logged-in information\nobtained via [`method: BrowserContext.storageState`]."]
        storage_state: Option<String>,
        #[doc = "Populates context with given storage state. This option can be used to initialize context with logged-in information\nobtained via [`method: BrowserContext.storageState`]. Path to the file with saved storage state."]
        storage_state_path: Option<path>,
        #[doc = "Changes the timezone of the context. See\n[ICU's metaZones.txt](https://cs.chromium.org/chromium/src/third_party/icu/source/data/misc/metaZones.txt?rcl=faee8bc70570192d82d2978a71e2a615788597d1)\nfor a list of supported timezone IDs."]
        timezone_id: Option<String>,
        #[doc = "Specific user agent to use in this context."] user_agent: Option<String>,
        #[doc = "**DEPRECATED** Use `recordVideo` instead."] video_size: Option<NotImplementedYet>,
        #[doc = "**DEPRECATED** Use `recordVideo` instead."] videos_path: Option<path>,
        #[doc = "Emulates consistent viewport for each page. Defaults to an 1280x720 viewport. `null` disables the default viewport."]
        viewport: Option<Option<NotImplementedYet>>,
        #[doc = "Sets a consistent viewport for each page. Defaults to an 1280x720 viewport. `no_viewport` disables the fixed viewport."]
        viewport: Option<Option<NotImplementedYet>>
    ) -> Result<Page, Arc<Error>> {
        todo!()
    }
    #[doc = "> NOTE: Tracing is only supported on Chromium-based browsers.\n\nYou can use [`method: Browser.startTracing`] and [`method: Browser.stopTracing`] to create a trace file that can be\nopened in Chrome DevTools performance panel.\n\n```js\nawait browser.startTracing(page, {path: 'trace.json'});\nawait page.goto('https://www.google.com');\nawait browser.stopTracing();\n```\n\n```java\nbrowser.startTracing(page, new Browser.StartTracingOptions()\n  .setPath(Paths.get(\"trace.json\")));\npage.goto('https://www.google.com');\nbrowser.stopTracing();\n```\n\n```python async\nawait browser.start_tracing(page, path=\"trace.json\")\nawait page.goto(\"https://www.google.com\")\nawait browser.stop_tracing()\n```\n\n```python sync\nbrowser.start_tracing(page, path=\"trace.json\")\npage.goto(\"https://www.google.com\")\nbrowser.stop_tracing()\n```\n"]
    fn start_tracing(
        &self,
        page : Option < Page >,
        #[doc = "options"]
        #[doc = "specify custom categories to use instead of default."]
        categories: Option<Vec<String>>,
        #[doc = "A path to write the trace file to."] path: Option<path>,
        #[doc = "captures screenshots in the trace."] screenshots: Option<bool>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "> NOTE: Tracing is only supported on Chromium-based browsers.\n\nReturns the buffer with trace data."]
    fn stop_tracing(&self) -> Result<Buffer, Arc<Error>> { todo!() }
    #[doc = "Returns the browser version."]
    fn version(&self) -> Result<String, Error> { todo!() }
}
enum BrowserEventType {
    #[doc = "Emitted when Browser gets disconnected from the browser application. This might happen because of one of the following:\n- Browser application is closed or crashed.\n- The [`method: Browser.close`] method was called."]
    Disconnected
}
enum BrowserEvent {
    #[doc = "Emitted when Browser gets disconnected from the browser application. This might happen because of one of the following:\n- Browser application is closed or crashed.\n- The [`method: Browser.close`] method was called."]
    Disconnected(Browser)
}
#[doc = "- extends: [EventEmitter]\n\nBrowserContexts provide a way to operate multiple independent browser sessions.\n\nIf a page opens another page, e.g. with a `window.open` call, the popup will belong to the parent page's browser\ncontext.\n\nPlaywright allows creation of \"incognito\" browser contexts with `browser.newContext()` method. \"Incognito\" browser\ncontexts don't write any browsing data to disk.\n\n```js\n// Create a new incognito browser context\nconst context = await browser.newContext();\n// Create a new page inside context.\nconst page = await context.newPage();\nawait page.goto('https://example.com');\n// Dispose context once it's no longer needed.\nawait context.close();\n```\n\n```java\n// Create a new incognito browser context\nBrowserContext context = browser.newContext();\n// Create a new page inside context.\nPage page = context.newPage();\npage.navigate(\"https://example.com\");\n// Dispose context once it\"s no longer needed.\ncontext.close();\n```\n\n```python async\n# create a new incognito browser context\ncontext = await browser.new_context()\n# create a new page inside context.\npage = await context.new_page()\nawait page.goto(\"https://example.com\")\n# dispose context once it\"s no longer needed.\nawait context.close()\n```\n\n```python sync\n# create a new incognito browser context\ncontext = browser.new_context()\n# create a new page inside context.\npage = context.new_page()\npage.goto(\"https://example.com\")\n# dispose context once it\"s no longer needed.\ncontext.close()\n```\n"]
#[doc = "Extends EventEmitter"]
impl BrowserContext {
    #[doc = "Adds cookies into this browser context. All pages within this context will have these cookies installed. Cookies can be\nobtained via [`method: BrowserContext.cookies`].\n\n```js\nawait browserContext.addCookies([cookieObject1, cookieObject2]);\n```\n\n```java\nbrowserContext.addCookies(Arrays.asList(cookieObject1, cookieObject2));\n```\n\n```python async\nawait browser_context.add_cookies([cookie_object1, cookie_object2])\n```\n\n```python sync\nbrowser_context.add_cookies([cookie_object1, cookie_object2])\n```\n"]
    fn add_cookies(&self, #[doc = ""] cookies: Vec<NotImplementedYet>) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Adds a script which would be evaluated in one of the following scenarios:\n- Whenever a page is created in the browser context or is navigated.\n- Whenever a child frame is attached or navigated in any page in the browser context. In this case, the script is\n  evaluated in the context of the newly attached frame.\n\nThe script is evaluated after the document was created but before any of its scripts were run. This is useful to amend\nthe JavaScript environment, e.g. to seed `Math.random`.\n\nAn example of overriding `Math.random` before the page loads:\n\n```js browser\n// preload.js\nMath.random = () => 42;\n```\n\n```js\n// In your playwright script, assuming the preload.js file is in same directory.\nawait browserContext.addInitScript({\n  path: 'preload.js'\n});\n```\n\n```java\n// In your playwright script, assuming the preload.js file is in same directory.\nbrowserContext.addInitScript(Paths.get(\"preload.js\"));\n```\n\n```python async\n# in your playwright script, assuming the preload.js file is in same directory.\nawait browser_context.add_init_script(path=\"preload.js\")\n```\n\n```python sync\n# in your playwright script, assuming the preload.js file is in same directory.\nbrowser_context.add_init_script(path=\"preload.js\")\n```\n\n> NOTE: The order of evaluation of multiple scripts installed via [`method: BrowserContext.addInitScript`] and\n[`method: Page.addInitScript`] is not defined."]
    fn add_init_script(
        &self,
        script : NotImplementedYet,
        script : NotImplementedYet,
        #[doc = "Optional argument to pass to `script` (only supported when passing a function)."]
        arg: Option<Serializable>,
        #[doc = "Path to the JavaScript file. If `path` is a relative path, then it is resolved relative to the current working\ndirectory. Optional."]
        path: Option<path>,
        script : Option < String >
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "> NOTE: Background pages are only supported on Chromium-based browsers.\n\nAll existing background pages in the context."]
    fn background_pages(&self) -> Result<Vec<Page>, Error> { todo!() }
    #[doc = "Returns the browser instance of the context. If it was launched as a persistent context null gets returned."]
    fn browser(&self) -> Result<Option<Browser>, Error> { todo!() }
    #[doc = "Clears context cookies."]
    fn clear_cookies(&self) -> Result<(), Arc<Error>> { todo!() }
    #[doc = "Clears all permission overrides for the browser context.\n\n```js\nconst context = await browser.newContext();\nawait context.grantPermissions(['clipboard-read']);\n// do stuff ..\ncontext.clearPermissions();\n```\n\n```java\nBrowserContext context = browser.newContext();\ncontext.grantPermissions(Arrays.asList(\"clipboard-read\"));\n// do stuff ..\ncontext.clearPermissions();\n```\n\n```python async\ncontext = await browser.new_context()\nawait context.grant_permissions([\"clipboard-read\"])\n# do stuff ..\ncontext.clear_permissions()\n```\n\n```python sync\ncontext = browser.new_context()\ncontext.grant_permissions([\"clipboard-read\"])\n# do stuff ..\ncontext.clear_permissions()\n```\n"]
    fn clear_permissions(&self) -> Result<(), Arc<Error>> { todo!() }
    #[doc = "Closes the browser context. All the pages that belong to the browser context will be closed.\n\n> NOTE: The default browser context cannot be closed."]
    fn close(&self) -> Result<(), Arc<Error>> { todo!() }
    #[doc = "If no URLs are specified, this method returns all cookies. If URLs are specified, only cookies that affect those URLs\nare returned."]
    fn cookies(
        &self,
        #[doc = "Optional list of URLs."] urls: Option<NotImplementedYet>
    ) -> Result<Vec<NotImplementedYet>, Arc<Error>> {
        todo!()
    }
    #[doc = "The method adds a function called `name` on the `window` object of every frame in every page in the context. When\ncalled, the function executes `callback` and returns a [Promise] which resolves to the return value of `callback`. If\nthe `callback` returns a [Promise], it will be awaited.\n\nThe first argument of the `callback` function contains information about the caller: `{ browserContext: BrowserContext,\npage: Page, frame: Frame }`.\n\nSee [`method: Page.exposeBinding`] for page-only version.\n\nAn example of exposing page URL to all frames in all pages in the context:\n\n```js\nconst { webkit } = require('playwright');  // Or 'chromium' or 'firefox'.\n\n(async () => {\n  const browser = await webkit.launch({ headless: false });\n  const context = await browser.newContext();\n  await context.exposeBinding('pageURL', ({ page }) => page.url());\n  const page = await context.newPage();\n  await page.setContent(`\n    <script>\n      async function onClick() {\n        document.querySelector('div').textContent = await window.pageURL();\n      }\n    </script>\n    <button onclick=\"onClick()\">Click me</button>\n    <div></div>\n  `);\n  await page.click('button');\n})();\n```\n\n```java\nimport com.microsoft.playwright.*;\n\npublic class Example {\n  public static void main(String[] args) {\n    try (Playwright playwright = Playwright.create()) {\n      BrowserType webkit = playwright.webkit()\n      Browser browser = webkit.launch(new BrowserType.LaunchOptions().setHeadless(false));\n      BrowserContext context = browser.newContext();\n      context.exposeBinding(\"pageURL\", (source, args) -> source.page().url());\n      Page page = context.newPage();\n      page.setContent(\"<script>\\n\" +\n        \"  async function onClick() {\\n\" +\n        \"    document.querySelector('div').textContent = await window.pageURL();\\n\" +\n        \"  }\\n\" +\n        \"</script>\\n\" +\n        \"<button onclick=\\\"onClick()\\\">Click me</button>\\n\" +\n        \"<div></div>\");\n      page.click(\"button\");\n    }\n  }\n}\n```\n\n```python async\nimport asyncio\nfrom playwright.async_api import async_playwright\n\nasync def run(playwright):\n    webkit = playwright.webkit\n    browser = await webkit.launch(headless=false)\n    context = await browser.new_context()\n    await context.expose_binding(\"pageURL\", lambda source: source[\"page\"].url)\n    page = await context.new_page()\n    await page.set_content(\"\"\"\n    <script>\n      async function onClick() {\n        document.querySelector('div').textContent = await window.pageURL();\n      }\n    </script>\n    <button onclick=\"onClick()\">Click me</button>\n    <div></div>\n    \"\"\")\n    await page.click(\"button\")\n\nasync def main():\n    async with async_playwright() as playwright:\n        await run(playwright)\nasyncio.run(main())\n```\n\n```python sync\nfrom playwright.sync_api import sync_playwright\n\ndef run(playwright):\n    webkit = playwright.webkit\n    browser = webkit.launch(headless=false)\n    context = browser.new_context()\n    context.expose_binding(\"pageURL\", lambda source: source[\"page\"].url)\n    page = context.new_page()\n    page.set_content(\"\"\"\n    <script>\n      async function onClick() {\n        document.querySelector('div').textContent = await window.pageURL();\n      }\n    </script>\n    <button onclick=\"onClick()\">Click me</button>\n    <div></div>\n    \"\"\")\n    page.click(\"button\")\n\nwith sync_playwright() as playwright:\n    run(playwright)\n```\n\nAn example of passing an element handle:\n\n```js\nawait context.exposeBinding('clicked', async (source, element) => {\n  console.log(await element.textContent());\n}, { handle: true });\nawait page.setContent(`\n  <script>\n    document.addEventListener('click', event => window.clicked(event.target));\n  </script>\n  <div>Click me</div>\n  <div>Or click me</div>\n`);\n```\n\n```java\ncontext.exposeBinding(\"clicked\", (source, args) -> {\n  ElementHandle element = (ElementHandle) args[0];\n  System.out.println(element.textContent());\n  return null;\n}, new BrowserContext.ExposeBindingOptions().setHandle(true));\npage.setContent(\"\" +\n  \"<script>\\n\" +\n  \"  document.addEventListener('click', event => window.clicked(event.target));\\n\" +\n  \"</script>\\n\" +\n  \"<div>Click me</div>\\n\" +\n  \"<div>Or click me</div>\\n\");\n```\n\n```python async\nasync def print(source, element):\n    print(await element.text_content())\n\nawait context.expose_binding(\"clicked\", print, handle=true)\nawait page.set_content(\"\"\"\n  <script>\n    document.addEventListener('click', event => window.clicked(event.target));\n  </script>\n  <div>Click me</div>\n  <div>Or click me</div>\n\"\"\")\n```\n\n```python sync\ndef print(source, element):\n    print(element.text_content())\n\ncontext.expose_binding(\"clicked\", print, handle=true)\npage.set_content(\"\"\"\n  <script>\n    document.addEventListener('click', event => window.clicked(event.target));\n  </script>\n  <div>Click me</div>\n  <div>Or click me</div>\n\"\"\")\n```\n"]
    fn expose_binding(
        &self,
        #[doc = "Name of the function on the window object."] name: String,
        callback : function,
        #[doc = "options"]
        #[doc = "Whether to pass the argument as a handle, instead of passing by value. When passing a handle, only one argument is\nsupported. When passing by value, multiple arguments are supported."]
        handle: Option<bool>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "The method adds a function called `name` on the `window` object of every frame in every page in the context. When\ncalled, the function executes `callback` and returns a [Promise] which resolves to the return value of `callback`.\n\nIf the `callback` returns a [Promise], it will be awaited.\n\nSee [`method: Page.exposeFunction`] for page-only version.\n\nAn example of adding an `md5` function to all pages in the context:\n\n```js\nconst { webkit } = require('playwright');  // Or 'chromium' or 'firefox'.\nconst crypto = require('crypto');\n\n(async () => {\n  const browser = await webkit.launch({ headless: false });\n  const context = await browser.newContext();\n  await context.exposeFunction('md5', text => crypto.createHash('md5').update(text).digest('hex'));\n  const page = await context.newPage();\n  await page.setContent(`\n    <script>\n      async function onClick() {\n        document.querySelector('div').textContent = await window.md5('PLAYWRIGHT');\n      }\n    </script>\n    <button onclick=\"onClick()\">Click me</button>\n    <div></div>\n  `);\n  await page.click('button');\n})();\n```\n\n```java\nimport com.microsoft.playwright.*;\n\nimport java.nio.charset.StandardCharsets;\nimport java.security.MessageDigest;\nimport java.security.NoSuchAlgorithmException;\nimport java.util.Base64;\n\npublic class Example {\n  public static void main(String[] args) {\n    try (Playwright playwright = Playwright.create()) {\n      BrowserType webkit = playwright.webkit()\n      Browser browser = webkit.launch(new BrowserType.LaunchOptions().setHeadless(false));\n      context.exposeFunction(\"sha1\", args -> {\n        String text = (String) args[0];\n        MessageDigest crypto;\n        try {\n          crypto = MessageDigest.getInstance(\"SHA-1\");\n        } catch (NoSuchAlgorithmException e) {\n          return null;\n        }\n        byte[] token = crypto.digest(text.getBytes(StandardCharsets.UTF_8));\n        return Base64.getEncoder().encodeToString(token);\n      });\n      Page page = context.newPage();\n      page.setContent(\"<script>\\n\" +\n        \"  async function onClick() {\\n\" +\n        \"    document.querySelector('div').textContent = await window.sha1('PLAYWRIGHT');\\n\" +\n        \"  }\\n\" +\n        \"</script>\\n\" +\n        \"<button onclick=\\\"onClick()\\\">Click me</button>\\n\" +\n        \"<div></div>\\n\");\n      page.click(\"button\");\n    }\n  }\n}\n```\n\n```python async\nimport asyncio\nimport hashlib\nfrom playwright.async_api import async_playwright\n\nasync def sha1(text):\n    m = hashlib.sha1()\n    m.update(bytes(text, \"utf8\"))\n    return m.hexdigest()\n\n\nasync def run(playwright):\n    webkit = playwright.webkit\n    browser = await webkit.launch(headless=False)\n    context = await browser.new_context()\n    await context.expose_function(\"sha1\", sha1)\n    page = await context.new_page()\n    await page.set_content(\"\"\"\n        <script>\n          async function onClick() {\n            document.querySelector('div').textContent = await window.sha1('PLAYWRIGHT');\n          }\n        </script>\n        <button onclick=\"onClick()\">Click me</button>\n        <div></div>\n    \"\"\")\n    await page.click(\"button\")\n\nasync def main():\n    async with async_playwright() as playwright:\n        await run(playwright)\nasyncio.run(main())\n```\n\n```python sync\nimport hashlib\nfrom playwright.sync_api import sync_playwright\n\ndef sha1(text):\n    m = hashlib.sha1()\n    m.update(bytes(text, \"utf8\"))\n    return m.hexdigest()\n\n\ndef run(playwright):\n    webkit = playwright.webkit\n    browser = webkit.launch(headless=False)\n    context = browser.new_context()\n    context.expose_function(\"sha1\", sha1)\n    page = context.new_page()\n    page.expose_function(\"sha1\", sha1)\n    page.set_content(\"\"\"\n        <script>\n          async function onClick() {\n            document.querySelector('div').textContent = await window.sha1('PLAYWRIGHT');\n          }\n        </script>\n        <button onclick=\"onClick()\">Click me</button>\n        <div></div>\n    \"\"\")\n    page.click(\"button\")\n\nwith sync_playwright() as playwright:\n    run(playwright)\n```\n"]
    fn expose_function(
        &self,
        #[doc = "Name of the function on the window object."] name: String,
        callback : function
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Grants specified permissions to the browser context. Only grants corresponding permissions to the given origin if\nspecified."]
    fn grant_permissions(
        &self,
        #[doc = "A permission or an array of permissions to grant. Permissions can be one of the following values:\n- `'geolocation'`\n- `'midi'`\n- `'midi-sysex'` (system-exclusive midi)\n- `'notifications'`\n- `'push'`\n- `'camera'`\n- `'microphone'`\n- `'background-sync'`\n- `'ambient-light-sensor'`\n- `'accelerometer'`\n- `'gyroscope'`\n- `'magnetometer'`\n- `'accessibility-events'`\n- `'clipboard-read'`\n- `'clipboard-write'`\n- `'payment-handler'`"]
        permissions: Vec<String>,
        #[doc = "options"]
        #[doc = "The [origin] to grant permissions to, e.g. \"https://example.com\"."]
        origin: Option<String>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "> NOTE: CDP sessions are only supported on Chromium-based browsers.\n\nReturns the newly created session."]
    fn new_c_d_p_session(
        &self,
        #[doc = "Page to create new session for."] page: Page
    ) -> Result<CDPSession, Arc<Error>> {
        todo!()
    }
    #[doc = "Creates a new page in the browser context."]
    fn new_page(&self) -> Result<Page, Arc<Error>> { todo!() }
    #[doc = "Returns all open pages in the context."]
    fn pages(&self) -> Result<Vec<Page>, Error> { todo!() }
    #[doc = "Routing provides the capability to modify network requests that are made by any page in the browser context. Once route\nis enabled, every request matching the url pattern will stall unless it's continued, fulfilled or aborted.\n\nAn example of a naive handler that aborts all image requests:\n\n```js\nconst context = await browser.newContext();\nawait context.route('**/*.{png,jpg,jpeg}', route => route.abort());\nconst page = await context.newPage();\nawait page.goto('https://example.com');\nawait browser.close();\n```\n\n```java\nBrowserContext context = browser.newContext();\ncontext.route(\"**/*.{png,jpg,jpeg}\", route -> route.abort());\nPage page = context.newPage();\npage.navigate(\"https://example.com\");\nbrowser.close();\n```\n\n```python async\ncontext = await browser.new_context()\npage = await context.new_page()\nawait context.route(\"**/*.{png,jpg,jpeg}\", lambda route: route.abort())\nawait page.goto(\"https://example.com\")\nawait browser.close()\n```\n\n```python sync\ncontext = browser.new_context()\npage = context.new_page()\ncontext.route(\"**/*.{png,jpg,jpeg}\", lambda route: route.abort())\npage.goto(\"https://example.com\")\nbrowser.close()\n```\n\nor the same snippet using a regex pattern instead:\n\n```js\nconst context = await browser.newContext();\nawait context.route(/(\\.png$)|(\\.jpg$)/, route => route.abort());\nconst page = await context.newPage();\nawait page.goto('https://example.com');\nawait browser.close();\n```\n\n```java\nBrowserContext context = browser.newContext();\ncontext.route(Pattern.compile(\"(\\\\.png$)|(\\\\.jpg$)\"), route -> route.abort());\nPage page = context.newPage();\npage.navigate(\"https://example.com\");\nbrowser.close();\n```\n\n```python async\ncontext = await browser.new_context()\npage = await context.new_page()\nawait context.route(re.compile(r\"(\\.png$)|(\\.jpg$)\"), lambda route: route.abort())\npage = await context.new_page()\nawait page.goto(\"https://example.com\")\nawait browser.close()\n```\n\n```python sync\ncontext = browser.new_context()\npage = context.new_page()\ncontext.route(re.compile(r\"(\\.png$)|(\\.jpg$)\"), lambda route: route.abort())\npage = await context.new_page()\npage = context.new_page()\npage.goto(\"https://example.com\")\nbrowser.close()\n```\n\nIt is possible to examine the request to decide the route action. For example, mocking all requests that contain some\npost data, and leaving all other requests as is:\n\n```js\nawait context.route('/api/**', route => {\n  if (route.request().postData().includes('my-string'))\n    route.fulfill({ body: 'mocked-data' });\n  else\n    route.continue();\n});\n```\n\n```java\ncontext.route(\"/api/**\", route -> {\n  if (route.request().postData().contains(\"my-string\"))\n    route.fulfill(new Route.FulfillOptions().setBody(\"mocked-data\"));\n  else\n    route.resume();\n});\n```\n\n```python async\ndef handle_route(route):\n  if (\"my-string\" in route.request.post_data)\n    route.fulfill(body=\"mocked-data\")\n  else\n    route.continue_()\nawait context.route(\"/api/**\", handle_route)\n```\n\n```python sync\ndef handle_route(route):\n  if (\"my-string\" in route.request.post_data)\n    route.fulfill(body=\"mocked-data\")\n  else\n    route.continue_()\ncontext.route(\"/api/**\", handle_route)\n```\n\nPage routes (set up with [`method: Page.route`]) take precedence over browser context routes when request matches both\nhandlers.\n\nTo remove a route with its handler you can use [`method: BrowserContext.unroute`].\n\n> NOTE: Enabling routing disables http cache."]
    fn route(
        &self,
        #[doc = "A glob pattern, regex pattern or predicate receiving [URL] to match while routing."]
        url: NotImplementedYet,
        #[doc = "handler function to route the request."] handler: function,
        #[doc = "handler function to route the request."] handler: function
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "> NOTE: Service workers are only supported on Chromium-based browsers.\n\nAll existing service workers in the context."]
    fn service_workers(&self) -> Result<Vec<Worker>, Error> { todo!() }
    #[doc = "This setting will change the default maximum navigation time for the following methods and related shortcuts:\n- [`method: Page.goBack`]\n- [`method: Page.goForward`]\n- [`method: Page.goto`]\n- [`method: Page.reload`]\n- [`method: Page.setContent`]\n- [`method: Page.waitForNavigation`]\n\n> NOTE: [`method: Page.setDefaultNavigationTimeout`] and [`method: Page.setDefaultTimeout`] take priority over\n[`method: BrowserContext.setDefaultNavigationTimeout`]."]
    fn set_default_navigation_timeout(
        &self,
        #[doc = "Maximum navigation time in milliseconds"] timeout: f64
    ) -> Result<(), Error> {
        todo!()
    }
    #[doc = "This setting will change the default maximum time for all the methods accepting `timeout` option.\n\n> NOTE: [`method: Page.setDefaultNavigationTimeout`], [`method: Page.setDefaultTimeout`] and\n[`method: BrowserContext.setDefaultNavigationTimeout`] take priority over [`method: BrowserContext.setDefaultTimeout`]."]
    fn set_default_timeout(
        &self,
        #[doc = "Maximum time in milliseconds"] timeout: f64
    ) -> Result<(), Error> {
        todo!()
    }
    #[doc = "The extra HTTP headers will be sent with every request initiated by any page in the context. These headers are merged\nwith page-specific extra HTTP headers set with [`method: Page.setExtraHTTPHeaders`]. If page overrides a particular\nheader, page-specific header value will be used instead of the browser context header value.\n\n> NOTE: [`method: BrowserContext.setExtraHTTPHeaders`] does not guarantee the order of headers in the outgoing requests."]
    fn set_extra_h_t_t_p_headers(
        &self,
        #[doc = "An object containing additional HTTP headers to be sent with every request. All header values must be strings."]
        headers: Map<String, String>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Sets the context's geolocation. Passing `null` or `undefined` emulates position unavailable.\n\n```js\nawait browserContext.setGeolocation({latitude: 59.95, longitude: 30.31667});\n```\n\n```java\nbrowserContext.setGeolocation(new Geolocation(59.95, 30.31667));\n```\n\n```python async\nawait browser_context.set_geolocation({\"latitude\": 59.95, \"longitude\": 30.31667})\n```\n\n```python sync\nbrowser_context.set_geolocation({\"latitude\": 59.95, \"longitude\": 30.31667})\n```\n\n> NOTE: Consider using [`method: BrowserContext.grantPermissions`] to grant permissions for the browser context pages to\nread its geolocation."]
    fn set_geolocation(
        &self,
        #[doc = ""] geolocation: Option<NotImplementedYet>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "**DEPRECATED** Browsers may cache credentials after successful authentication. Create a new browser context instead."]
    fn set_h_t_t_p_credentials(
        &self,
        #[doc = ""] http_credentials: Option<NotImplementedYet>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = ""]
    fn set_offline(
        &self,
        #[doc = "Whether to emulate network being offline for the browser context."] offline: bool
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Returns storage state for this browser context, contains current cookies and local storage snapshot."]
    fn storage_state(
        &self,
        #[doc = "options"]
        #[doc = "The file path to save the storage state to. If `path` is a relative path, then it is resolved relative to current\nworking directory. If no path is provided, storage state is still returned, but won't be saved to the disk."]
        path: Option<path>
    ) -> Result<NotImplementedYet, Arc<Error>> {
        todo!()
    }
    #[doc = "Removes a route created with [`method: BrowserContext.route`]. When `handler` is not specified, removes all routes for\nthe `url`."]
    fn unroute(
        &self,
        #[doc = "A glob pattern, regex pattern or predicate receiving [URL] used to register a routing with\n[`method: BrowserContext.route`]."]
        url: NotImplementedYet,
        #[doc = "Optional handler function used to register a routing with [`method: BrowserContext.route`]."]
        handler: Option<function>,
        #[doc = "Optional handler function used to register a routing with [`method: BrowserContext.route`]."]
        handler: Option<function>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Waits for event to fire and passes its value into the predicate function. Returns when the predicate returns truthy\nvalue. Will throw an error if the context closes before the event is fired. Returns the event data value.\n\n```js\nconst [page, _] = await Promise.all([\n  context.waitForEvent('page'),\n  page.click('button')\n]);\n```\n\n```java\nPage newPage = context.waitForPage(() -> page.click(\"button\"));\n```\n\n```python async\nasync with context.expect_event(\"page\") as event_info:\n    await page.click(\"button\")\npage = await event_info.value\n```\n\n```python sync\nwith context.expect_event(\"page\") as event_info:\n    page.click(\"button\")\npage = event_info.value\n```\n"]
    fn wait_for_event(
        &self,
        #[doc = "Event name, same one would pass into `browserContext.on(event)`."] event: String,
        #[doc = "Either a predicate that receives an event or an options object. Optional."]
        options_or_predicate: Option<NotImplementedYet>,
        #[doc = "options"]
        #[doc = "Receives the event data and resolves to truthy value when the waiting should resolve."]
        predicate: Option<function>,
        #[doc = "Maximum time to wait for in milliseconds. Defaults to `30000` (30 seconds). Pass `0` to disable timeout. The default\nvalue can be changed by using the [`method: BrowserContext.setDefaultTimeout`]."]
        timeout: Option<f64>
    ) -> Result<any, Arc<Error>> {
        todo!()
    }
    #[doc = "Performs action and waits for a new `Page` to be created in the context. If predicate is provided, it passes `Page`\nvalue into the `predicate` function and waits for `predicate(event)` to return a truthy value. Will throw an error if\nthe context closes before new `Page` is created."]
    fn wait_for_page(
        &self,
        #[doc = "Callback that performs the action triggering the event."] callback: Runnable,
        #[doc = "options"]
        #[doc = "Receives the `Page` object and resolves to truthy value when the waiting should resolve."]
        predicate: Option<function>,
        #[doc = "Maximum time to wait for in milliseconds. Defaults to `30000` (30 seconds). Pass `0` to disable timeout. The default\nvalue can be changed by using the [`method: BrowserContext.setDefaultTimeout`]."]
        timeout: Option<f64>
    ) -> Result<Page, Arc<Error>> {
        todo!()
    }
    #[doc = "> NOTE: In most cases, you should use [`method: BrowserContext.waitForEvent`].\n\nWaits for given `event` to fire. If predicate is provided, it passes event's value into the `predicate` function and\nwaits for `predicate(event)` to return a truthy value. Will throw an error if the socket is closed before the `event` is\nfired."]
    fn wait_for_event2(
        &self,
        #[doc = "Event name, same one typically passed into `*.on(event)`."] event: String,
        #[doc = "options"]
        #[doc = "Receives the event data and resolves to truthy value when the waiting should resolve."]
        predicate: Option<function>,
        #[doc = "Maximum time to wait for in milliseconds. Defaults to `30000` (30 seconds). Pass `0` to disable timeout. The default\nvalue can be changed by using the [`method: BrowserContext.setDefaultTimeout`]."]
        timeout: Option<f64>
    ) -> Result<Any, Arc<Error>> {
        todo!()
    }
}
enum NotImplementedYetscript {
    NotImplementedYet(function),
    NotImplementedYet(String),
    NotImplementedYet(NotImplementedYet)
}
struct NotImplementedYet {
    #[doc = "Path to the JavaScript file. If `path` is a relative path, then it is resolved relative to the current working\ndirectory. Optional."]
    path: Option<path>,
    #[doc = "Raw script content. Optional."]
    content: Option<String>
}
enum NotImplementedYetscript {
    NotImplementedYet(String),
    NotImplementedYet(path)
}
enum NotImplementedYeturls {
    NotImplementedYet(String),
    NotImplementedYet(Vec<String>)
}
enum NotImplementedYeturl {
    NotImplementedYet(String),
    NotImplementedYet(RegExp),
    NotImplementedYet(function)
}
struct NotImplementedYetstorageState {
    #[doc = ""]
    cookies: Vec<NotImplementedYet>,
    #[doc = ""]
    origins: Vec<NotImplementedYet>
}
enum NotImplementedYeturl {
    NotImplementedYet(String),
    NotImplementedYet(RegExp),
    NotImplementedYet(function)
}
enum NotImplementedYetoptionsOrPredicate {
    NotImplementedYet(function),
    NotImplementedYet(NotImplementedYet)
}
struct NotImplementedYet {
    #[doc = "receives the event data and resolves to truthy value when the waiting should resolve."]
    predicate: function,
    #[doc = "maximum time to wait for in milliseconds. Defaults to `30000` (30 seconds). Pass `0` to disable timeout. The default\nvalue can be changed by using the [`method: BrowserContext.setDefaultTimeout`]."]
    timeout: Option<f64>
}
enum BrowserContextEventType {
    #[doc = "> NOTE: Only works with Chromium browser's persistent context.\n\nEmitted when new background page is created in the context.\n\n```js\nconst backgroundPage = await context.waitForEvent('backgroundpage');\n```\n\n```python async\nbackground_page = await context.wait_for_event(\"backgroundpage\")\n```\n\n```python sync\nbackground_page = context.wait_for_event(\"backgroundpage\")\n```\n"]
    BackgroundPage,
    #[doc = "Emitted when Browser context gets closed. This might happen because of one of the following:\n- Browser context is closed.\n- Browser application is closed or crashed.\n- The [`method: Browser.close`] method was called."]
    Close,
    #[doc = "The event is emitted when a new Page is created in the BrowserContext. The page may still be loading. The event will\nalso fire for popup pages. See also [`event: Page.popup`] to receive events about popups relevant to a specific page.\n\nThe earliest moment that page is available is when it has navigated to the initial url. For example, when opening a\npopup with `window.open('http://example.com')`, this event will fire when the network request to \"http://example.com\" is\ndone and its response has started loading in the popup.\n\n```js\nconst [newPage] = await Promise.all([\n  context.waitForEvent('page'),\n  page.click('a[target=_blank]'),\n]);\nconsole.log(await newPage.evaluate('location.href'));\n```\n\n```java\nPage newPage = context.waitForPage(() -> {\n  page.click(\"a[target=_blank]\");\n});\nSystem.out.println(newPage.evaluate(\"location.href\"));\n```\n\n```python async\nasync with context.expect_page() as page_info:\n    await page.click(\"a[target=_blank]\"),\npage = await page_info.value\nprint(await page.evaluate(\"location.href\"))\n```\n\n```python sync\nwith context.expect_page() as page_info:\n    page.click(\"a[target=_blank]\"),\npage = page_info.value\nprint(page.evaluate(\"location.href\"))\n```\n\n> NOTE: Use [`method: Page.waitForLoadState`] to wait until the page gets to a particular state (you should not need it\nin most cases)."]
    Page,
    #[doc = "> NOTE: Service workers are only supported on Chromium-based browsers.\n\nEmitted when new service worker is created in the context."]
    ServiceWorker
}
enum BrowserContextEvent {
    #[doc = "> NOTE: Only works with Chromium browser's persistent context.\n\nEmitted when new background page is created in the context.\n\n```js\nconst backgroundPage = await context.waitForEvent('backgroundpage');\n```\n\n```python async\nbackground_page = await context.wait_for_event(\"backgroundpage\")\n```\n\n```python sync\nbackground_page = context.wait_for_event(\"backgroundpage\")\n```\n"]
    BackgroundPage(Page),
    #[doc = "Emitted when Browser context gets closed. This might happen because of one of the following:\n- Browser context is closed.\n- Browser application is closed or crashed.\n- The [`method: Browser.close`] method was called."]
    Close(BrowserContext),
    #[doc = "The event is emitted when a new Page is created in the BrowserContext. The page may still be loading. The event will\nalso fire for popup pages. See also [`event: Page.popup`] to receive events about popups relevant to a specific page.\n\nThe earliest moment that page is available is when it has navigated to the initial url. For example, when opening a\npopup with `window.open('http://example.com')`, this event will fire when the network request to \"http://example.com\" is\ndone and its response has started loading in the popup.\n\n```js\nconst [newPage] = await Promise.all([\n  context.waitForEvent('page'),\n  page.click('a[target=_blank]'),\n]);\nconsole.log(await newPage.evaluate('location.href'));\n```\n\n```java\nPage newPage = context.waitForPage(() -> {\n  page.click(\"a[target=_blank]\");\n});\nSystem.out.println(newPage.evaluate(\"location.href\"));\n```\n\n```python async\nasync with context.expect_page() as page_info:\n    await page.click(\"a[target=_blank]\"),\npage = await page_info.value\nprint(await page.evaluate(\"location.href\"))\n```\n\n```python sync\nwith context.expect_page() as page_info:\n    page.click(\"a[target=_blank]\"),\npage = page_info.value\nprint(page.evaluate(\"location.href\"))\n```\n\n> NOTE: Use [`method: Page.waitForLoadState`] to wait until the page gets to a particular state (you should not need it\nin most cases)."]
    Page(Page),
    #[doc = "> NOTE: Service workers are only supported on Chromium-based browsers.\n\nEmitted when new service worker is created in the context."]
    ServiceWorker(Worker)
}
#[doc = ""]
impl BrowserServer {
    #[doc = "Closes the browser gracefully and makes sure the process is terminated."]
    fn close(&self) -> Result<(), Arc<Error>> { todo!() }
    #[doc = "Kills the browser process and waits for the process to exit."]
    fn kill(&self) -> Result<(), Arc<Error>> { todo!() }
    #[doc = "Spawned browser application process."]
    fn process(&self) -> Result<ChildProcess, Error> { todo!() }
    #[doc = "Browser websocket url.\n\nBrowser websocket endpoint which can be used as an argument to [`method: BrowserType.connect`] to establish connection\nto the browser."]
    fn ws_endpoint(&self) -> Result<String, Error> { todo!() }
}
enum BrowserServerEventType {
    #[doc = "Emitted when the browser server closes."]
    Close
}
enum BrowserServerEvent {
    #[doc = "Emitted when the browser server closes."]
    Close(())
}
#[doc = "BrowserType provides methods to launch a specific browser instance or connect to an existing one. The following is a\ntypical example of using Playwright to drive automation:\n\n```js\nconst { chromium } = require('playwright');  // Or 'firefox' or 'webkit'.\n\n(async () => {\n  const browser = await chromium.launch();\n  const page = await browser.newPage();\n  await page.goto('https://example.com');\n  // other actions...\n  await browser.close();\n})();\n```\n\n```java\nimport com.microsoft.playwright.*;\n\npublic class Example {\n  public static void main(String[] args) {\n    try (Playwright playwright = Playwright.create()) {\n      BrowserType chromium = playwright.chromium();\n      Browser browser = chromium.launch();\n      Page page = browser.newPage();\n      page.navigate(\"https://example.com\");\n      // other actions...\n      browser.close();\n    }\n  }\n}\n```\n\n```python async\nimport asyncio\nfrom playwright.async_api import async_playwright\n\nasync def run(playwright):\n    chromium = playwright.chromium\n    browser = await chromium.launch()\n    page = await browser.new_page()\n    await page.goto(\"https://example.com\")\n    # other actions...\n    await browser.close()\n\nasync def main():\n    async with async_playwright() as playwright:\n        await run(playwright)\nasyncio.run(main())\n```\n\n```python sync\nfrom playwright.sync_api import sync_playwright\n\ndef run(playwright):\n    chromium = playwright.chromium\n    browser = chromium.launch()\n    page = browser.new_page()\n    page.goto(\"https://example.com\")\n    # other actions...\n    browser.close()\n\nwith sync_playwright() as playwright:\n    run(playwright)\n```\n"]
impl BrowserType {
    #[doc = "This methods attaches Playwright to an existing browser instance."]
    fn connect(
        &self,
        #[doc = ""] params: NotImplementedYet,
        #[doc = "A browser websocket endpoint to connect to."] ws_endpoint: String,
        #[doc = "options"]
        #[doc = "Additional HTTP headers to be sent with web socket connect request. Optional."]
        headers: Option<Map<String, String>>,
        #[doc = "Slows down Playwright operations by the specified amount of milliseconds. Useful so that you can see what is going on.\nDefaults to 0."]
        slow_mo: Option<f64>,
        #[doc = "Maximum time in milliseconds to wait for the connection to be established. Defaults to `30000` (30 seconds). Pass `0` to\ndisable timeout."]
        timeout: Option<f64>
    ) -> Result<Browser, Arc<Error>> {
        todo!()
    }
    #[doc = "This methods attaches Playwright to an existing browser instance using the Chrome DevTools Protocol.\n\nThe default browser context is accessible via [`method: Browser.contexts`].\n\n> NOTE: Connecting over the Chrome DevTools Protocol is only supported for Chromium-based browsers."]
    fn connect_over_c_d_p(
        &self,
        #[doc = ""] params: NotImplementedYet,
        #[doc = "A CDP websocket endpoint or http url to connect to. For example `http://localhost:9222/` or\n`ws://127.0.0.1:9222/devtools/browser/387adf4c-243f-4051-a181-46798f4a46f4`."]
        endpoint_u_r_l: String,
        #[doc = "options"]
        #[doc = "Additional HTTP headers to be sent with connect request. Optional."]
        headers: Option<Map<String, String>>,
        #[doc = "Slows down Playwright operations by the specified amount of milliseconds. Useful so that you can see what is going on.\nDefaults to 0."]
        slow_mo: Option<f64>,
        #[doc = "Maximum time in milliseconds to wait for the connection to be established. Defaults to `30000` (30 seconds). Pass `0` to\ndisable timeout."]
        timeout: Option<f64>
    ) -> Result<Browser, Arc<Error>> {
        todo!()
    }
    #[doc = "A path where Playwright expects to find a bundled browser executable."]
    fn executable_path(&self) -> Result<String, Error> { todo!() }
    #[doc = "Returns the browser instance.\n\nYou can use `ignoreDefaultArgs` to filter out `--mute-audio` from default arguments:\n\n```js\nconst browser = await chromium.launch({  // Or 'firefox' or 'webkit'.\n  ignoreDefaultArgs: ['--mute-audio']\n});\n```\n\n```java\n// Or \"firefox\" or \"webkit\".\nBrowser browser = chromium.launch(new BrowserType.LaunchOptions()\n  .setIgnoreDefaultArgs(Arrays.asList(\"--mute-audio\")));\n```\n\n```python async\nbrowser = await playwright.chromium.launch( # or \"firefox\" or \"webkit\".\n    ignore_default_args=[\"--mute-audio\"]\n)\n```\n\n```python sync\nbrowser = playwright.chromium.launch( # or \"firefox\" or \"webkit\".\n    ignore_default_args=[\"--mute-audio\"]\n)\n```\n\n> **Chromium-only** Playwright can also be used to control the Google Chrome or Microsoft Edge browsers, but it works\nbest with the version of Chromium it is bundled with. There is no guarantee it will work with any other version. Use\n`executablePath` option with extreme caution.\n>\n> If Google Chrome (rather than Chromium) is preferred, a\n[Chrome Canary](https://www.google.com/chrome/browser/canary.html) or\n[Dev Channel](https://www.chromium.org/getting-involved/dev-channel) build is suggested.\n>\n> Stock browsers like Google Chrome and Microsoft Edge are suitable for tests that require proprietary media codecs for\nvideo playback. See\n[this article](https://www.howtogeek.com/202825/what%E2%80%99s-the-difference-between-chromium-and-chrome/) for other\ndifferences between Chromium and Chrome.\n[This article](https://chromium.googlesource.com/chromium/src/+/lkgr/docs/chromium_browser_vs_google_chrome.md)\ndescribes some differences for Linux users."]
    fn launch(
        &self,
        #[doc = "options"]
        #[doc = "Additional arguments to pass to the browser instance. The list of Chromium flags can be found\n[here](http://peter.sh/experiments/chromium-command-line-switches/)."]
        args: Option<Vec<String>>,
        #[doc = "Browser distribution channel. Read more about using\n[Google Chrome and Microsoft Edge](./browsers.md#google-chrome--microsoft-edge)."]
        channel: Option<BrowserChannel>,
        #[doc = "Enable Chromium sandboxing. Defaults to `false`."] chromium_sandbox: Option<bool>,
        #[doc = "**Chromium-only** Whether to auto-open a Developer Tools panel for each tab. If this option is `true`, the `headless`\noption will be set `false`."]
        devtools: Option<bool>,
        #[doc = "If specified, accepted downloads are downloaded into this directory. Otherwise, temporary directory is created and is\ndeleted when browser is closed."]
        downloads_path: Option<path>,
        #[doc = "Specify environment variables that will be visible to the browser. Defaults to `process.env`."]
        env: Option<Map<String, String>>,
        #[doc = "Specify environment variables that will be visible to the browser. Defaults to `process.env`."]
        env: Option<Map<String, String>>,
        #[doc = "Path to a browser executable to run instead of the bundled one. If `executablePath` is a relative path, then it is\nresolved relative to the current working directory. Note that Playwright only works with the bundled Chromium, Firefox\nor WebKit, use at your own risk."]
        executable_path: Option<path>,
        #[doc = "Firefox user preferences. Learn more about the Firefox user preferences at\n[`about:config`](https://support.mozilla.org/en-US/kb/about-config-editor-firefox)."]
        firefox_user_prefs: Option<Map<String, String>>,
        #[doc = "Firefox user preferences. Learn more about the Firefox user preferences at\n[`about:config`](https://support.mozilla.org/en-US/kb/about-config-editor-firefox)."]
        firefox_user_prefs: Option<Map<String, String>>,
        handle_s_i_g_h_u_p : Option < bool >,
        handle_s_i_g_i_n_t : Option < bool >,
        handle_s_i_g_t_e_r_m : Option < bool >,
        #[doc = "Whether to run browser in headless mode. More details for\n[Chromium](https://developers.google.com/web/updates/2017/04/headless-chrome) and\n[Firefox](https://developer.mozilla.org/en-US/docs/Mozilla/Firefox/Headless_mode). Defaults to `true` unless the\n`devtools` option is `true`."]
        headless: Option<bool>,
        #[doc = "If `true`, Playwright does not pass its own configurations args and only uses the ones from `args`. Dangerous option;\nuse with care. Defaults to `false`."]
        ignore_all_default_args: Option<bool>,
        #[doc = "If `true`, Playwright does not pass its own configurations args and only uses the ones from `args`. If an array is\ngiven, then filters out the given default arguments. Dangerous option; use with care. Defaults to `false`."]
        ignore_default_args: Option<NotImplementedYet>,
        #[doc = "If `true`, Playwright does not pass its own configurations args and only uses the ones from `args`. Dangerous option;\nuse with care."]
        ignore_default_args: Option<Vec<String>>,
        #[doc = "Logger sink for Playwright logging."] logger: Option<Logger>,
        #[doc = "Network proxy settings."] proxy: Option<NotImplementedYet>,
        #[doc = "Slows down Playwright operations by the specified amount of milliseconds. Useful so that you can see what is going on."]
        slow_mo: Option<f64>,
        #[doc = "Maximum time in milliseconds to wait for the browser instance to start. Defaults to `30000` (30 seconds). Pass `0` to\ndisable timeout."]
        timeout: Option<f64>
    ) -> Result<Browser, Arc<Error>> {
        todo!()
    }
    #[doc = "Returns the persistent browser context instance.\n\nLaunches browser that uses persistent storage located at `userDataDir` and returns the only context. Closing this\ncontext will automatically close the browser."]
    fn launch_persistent_context(
        &self,
        #[doc = "Path to a User Data Directory, which stores browser session data like cookies and local storage. More details for\n[Chromium](https://chromium.googlesource.com/chromium/src/+/master/docs/user_data_dir.md#introduction) and\n[Firefox](https://developer.mozilla.org/en-US/docs/Mozilla/Command_Line_Options#User_Profile). Note that Chromium's user\ndata directory is the **parent** directory of the \"Profile Path\" seen at `chrome://version`."]
        user_data_dir: path,
        #[doc = "options"]
        #[doc = "Whether to automatically download all the attachments. Defaults to `false` where all the downloads are canceled."]
        accept_downloads: Option<bool>,
        #[doc = "Additional arguments to pass to the browser instance. The list of Chromium flags can be found\n[here](http://peter.sh/experiments/chromium-command-line-switches/)."]
        args: Option<Vec<String>>,
        #[doc = "Toggles bypassing page's Content-Security-Policy."] bypass_c_s_p: Option<bool>,
        #[doc = "Browser distribution channel. Read more about using\n[Google Chrome and Microsoft Edge](./browsers.md#google-chrome--microsoft-edge)."]
        channel: Option<BrowserChannel>,
        #[doc = "Enable Chromium sandboxing. Defaults to `true`."] chromium_sandbox: Option<bool>,
        #[doc = "Emulates `'prefers-colors-scheme'` media feature, supported values are `'light'`, `'dark'`, `'no-preference'`. See\n[`method: Page.emulateMedia`] for more details. Defaults to `'light'`."]
        color_scheme: Option<ColorScheme>,
        #[doc = "Specify device scale factor (can be thought of as dpr). Defaults to `1`."]
        device_scale_factor: Option<f64>,
        #[doc = "**Chromium-only** Whether to auto-open a Developer Tools panel for each tab. If this option is `true`, the `headless`\noption will be set `false`."]
        devtools: Option<bool>,
        #[doc = "If specified, accepted downloads are downloaded into this directory. Otherwise, temporary directory is created and is\ndeleted when browser is closed."]
        downloads_path: Option<path>,
        #[doc = "Specify environment variables that will be visible to the browser. Defaults to `process.env`."]
        env: Option<Map<String, String>>,
        #[doc = "Specify environment variables that will be visible to the browser. Defaults to `process.env`."]
        env: Option<Map<String, String>>,
        #[doc = "Path to a browser executable to run instead of the bundled one. If `executablePath` is a relative path, then it is\nresolved relative to the current working directory. **BEWARE**: Playwright is only guaranteed to work with the bundled\nChromium, Firefox or WebKit, use at your own risk."]
        executable_path: Option<path>,
        #[doc = "An object containing additional HTTP headers to be sent with every request. All header values must be strings."]
        extra_h_t_t_p_headers: Option<Map<String, String>>,
        #[doc = ""] geolocation: Option<NotImplementedYet>,
        handle_s_i_g_h_u_p : Option < bool >,
        handle_s_i_g_i_n_t : Option < bool >,
        handle_s_i_g_t_e_r_m : Option < bool >,
        has_touch : Option < bool >,
        #[doc = "Whether to run browser in headless mode. More details for\n[Chromium](https://developers.google.com/web/updates/2017/04/headless-chrome) and\n[Firefox](https://developer.mozilla.org/en-US/docs/Mozilla/Firefox/Headless_mode). Defaults to `true` unless the\n`devtools` option is `true`."]
        headless: Option<bool>,
        #[doc = "Credentials for [HTTP authentication](https://developer.mozilla.org/en-US/docs/Web/HTTP/Authentication)."]
        http_credentials: Option<NotImplementedYet>,
        #[doc = "If `true`, Playwright does not pass its own configurations args and only uses the ones from `args`. Dangerous option;\nuse with care. Defaults to `false`."]
        ignore_all_default_args: Option<bool>,
        #[doc = "If `true`, Playwright does not pass its own configurations args and only uses the ones from `args`. If an array is\ngiven, then filters out the given default arguments. Dangerous option; use with care. Defaults to `false`."]
        ignore_default_args: Option<NotImplementedYet>,
        #[doc = "If `true`, Playwright does not pass its own configurations args and only uses the ones from `args`. Dangerous option;\nuse with care."]
        ignore_default_args: Option<Vec<String>>,
        #[doc = "Whether to ignore HTTPS errors during navigation. Defaults to `false`."]
        ignore_h_t_t_p_s_errors: Option<bool>,
        #[doc = "Whether the `meta viewport` tag is taken into account and touch events are enabled. Defaults to `false`. Not supported\nin Firefox."]
        is_mobile: Option<bool>,
        #[doc = "Whether or not to enable JavaScript in the context. Defaults to `true`."]
        java_script_enabled: Option<bool>,
        #[doc = "Specify user locale, for example `en-GB`, `de-DE`, etc. Locale will affect `navigator.language` value, `Accept-Language`\nrequest header value as well as number and date formatting rules."]
        locale: Option<String>,
        #[doc = "Logger sink for Playwright logging."] logger: Option<Logger>,
        #[doc = "Does not enforce fixed viewport, allows resizing window in the headed mode."]
        no_viewport: Option<bool>,
        #[doc = "Whether to emulate network being offline. Defaults to `false`."] offline: Option<
            bool
        >,
        #[doc = "A list of permissions to grant to all pages in this context. See [`method: BrowserContext.grantPermissions`] for more\ndetails."]
        permissions: Option<Vec<String>>,
        #[doc = "Network proxy settings."] proxy: Option<NotImplementedYet>,
        #[doc = "Enables [HAR](http://www.softwareishard.com/blog/har-12-spec) recording for all pages into `recordHar.path` file. If not\nspecified, the HAR is not recorded. Make sure to await [`method: BrowserContext.close`] for the HAR to be saved."]
        record_har: Option<NotImplementedYet>,
        #[doc = "Optional setting to control whether to omit request content from the HAR. Defaults to `false`."]
        record_har_omit_content: Option<bool>,
        #[doc = "Enables [HAR](http://www.softwareishard.com/blog/har-12-spec) recording for all pages into the specified HAR file on the\nfilesystem. If not specified, the HAR is not recorded. Make sure to call [`method: BrowserContext.close`] for the HAR to\nbe saved."]
        record_har_path: Option<path>,
        #[doc = "Enables video recording for all pages into `recordVideo.dir` directory. If not specified videos are not recorded. Make\nsure to await [`method: BrowserContext.close`] for videos to be saved."]
        record_video: Option<NotImplementedYet>,
        #[doc = "Enables video recording for all pages into the specified directory. If not specified videos are not recorded. Make sure\nto call [`method: BrowserContext.close`] for videos to be saved."]
        record_video_dir: Option<path>,
        #[doc = "Dimensions of the recorded videos. If not specified the size will be equal to `viewport` scaled down to fit into\n800x800. If `viewport` is not configured explicitly the video size defaults to 800x450. Actual picture of each page will\nbe scaled down if necessary to fit the specified size."]
        record_video_size: Option<NotImplementedYet>,
        #[doc = "Emulates consistent window screen size available inside web page via `window.screen`. Is only used when the `viewport`\nis set."]
        screen: Option<NotImplementedYet>,
        #[doc = "Slows down Playwright operations by the specified amount of milliseconds. Useful so that you can see what is going on.\nDefaults to 0."]
        slow_mo: Option<f64>,
        #[doc = "Maximum time in milliseconds to wait for the browser instance to start. Defaults to `30000` (30 seconds). Pass `0` to\ndisable timeout."]
        timeout: Option<f64>,
        #[doc = "Changes the timezone of the context. See\n[ICU's metaZones.txt](https://cs.chromium.org/chromium/src/third_party/icu/source/data/misc/metaZones.txt?rcl=faee8bc70570192d82d2978a71e2a615788597d1)\nfor a list of supported timezone IDs."]
        timezone_id: Option<String>,
        #[doc = "Specific user agent to use in this context."] user_agent: Option<String>,
        #[doc = "**DEPRECATED** Use `recordVideo` instead."] video_size: Option<NotImplementedYet>,
        #[doc = "**DEPRECATED** Use `recordVideo` instead."] videos_path: Option<path>,
        #[doc = "Emulates consistent viewport for each page. Defaults to an 1280x720 viewport. `null` disables the default viewport."]
        viewport: Option<Option<NotImplementedYet>>,
        #[doc = "Sets a consistent viewport for each page. Defaults to an 1280x720 viewport. `no_viewport` disables the fixed viewport."]
        viewport: Option<Option<NotImplementedYet>>
    ) -> Result<BrowserContext, Arc<Error>> {
        todo!()
    }
    #[doc = "Returns the browser app instance.\n\nLaunches browser server that client can connect to. An example of launching a browser executable and connecting to it\nlater:\n\n```js\nconst { chromium } = require('playwright');  // Or 'webkit' or 'firefox'.\n\n(async () => {\n  const browserServer = await chromium.launchServer();\n  const wsEndpoint = browserServer.wsEndpoint();\n  // Use web socket endpoint later to establish a connection.\n  const browser = await chromium.connect({ wsEndpoint });\n  // Close browser instance.\n  await browserServer.close();\n})();\n```\n"]
    fn launch_server(
        &self,
        #[doc = "options"]
        #[doc = "Additional arguments to pass to the browser instance. The list of Chromium flags can be found\n[here](http://peter.sh/experiments/chromium-command-line-switches/)."]
        args: Option<Vec<String>>,
        #[doc = "Browser distribution channel. Read more about using\n[Google Chrome and Microsoft Edge](./browsers.md#google-chrome--microsoft-edge)."]
        channel: Option<BrowserChannel>,
        #[doc = "Enable Chromium sandboxing. Defaults to `true`."] chromium_sandbox: Option<bool>,
        #[doc = "**Chromium-only** Whether to auto-open a Developer Tools panel for each tab. If this option is `true`, the `headless`\noption will be set `false`."]
        devtools: Option<bool>,
        #[doc = "If specified, accepted downloads are downloaded into this directory. Otherwise, temporary directory is created and is\ndeleted when browser is closed."]
        downloads_path: Option<path>,
        #[doc = "Specify environment variables that will be visible to the browser. Defaults to `process.env`."]
        env: Option<Map<String, String>>,
        #[doc = "Path to a browser executable to run instead of the bundled one. If `executablePath` is a relative path, then it is\nresolved relative to the current working directory. **BEWARE**: Playwright is only guaranteed to work with the bundled\nChromium, Firefox or WebKit, use at your own risk."]
        executable_path: Option<path>,
        #[doc = "Firefox user preferences. Learn more about the Firefox user preferences at\n[`about:config`](https://support.mozilla.org/en-US/kb/about-config-editor-firefox)."]
        firefox_user_prefs: Option<Map<String, String>>,
        handle_s_i_g_h_u_p : Option < bool >,
        handle_s_i_g_i_n_t : Option < bool >,
        handle_s_i_g_t_e_r_m : Option < bool >,
        #[doc = "Whether to run browser in headless mode. More details for\n[Chromium](https://developers.google.com/web/updates/2017/04/headless-chrome) and\n[Firefox](https://developer.mozilla.org/en-US/docs/Mozilla/Firefox/Headless_mode). Defaults to `true` unless the\n`devtools` option is `true`."]
        headless: Option<bool>,
        #[doc = "If `true`, Playwright does not pass its own configurations args and only uses the ones from `args`. If an array is\ngiven, then filters out the given default arguments. Dangerous option; use with care. Defaults to `false`."]
        ignore_default_args: Option<NotImplementedYet>,
        #[doc = "Logger sink for Playwright logging."] logger: Option<Logger>,
        #[doc = "Port to use for the web socket. Defaults to 0 that picks any available port."]
        port: Option<i64>,
        #[doc = "Network proxy settings."] proxy: Option<NotImplementedYet>,
        #[doc = "Maximum time in milliseconds to wait for the browser instance to start. Defaults to `30000` (30 seconds). Pass `0` to\ndisable timeout."]
        timeout: Option<f64>
    ) -> Result<BrowserServer, Arc<Error>> {
        todo!()
    }
    #[doc = "Returns browser name. For example: `'chromium'`, `'webkit'` or `'firefox'`."]
    fn name(&self) -> Result<String, Error> { todo!() }
}
struct NotImplementedYetparams {
    #[doc = "A browser websocket endpoint to connect to."]
    ws_endpoint: String,
    #[doc = "Additional HTTP headers to be sent with web socket connect request. Optional."]
    headers: Option<Map<String, String>>,
    #[doc = "Slows down Playwright operations by the specified amount of milliseconds. Useful so that you can see what is going on.\nDefaults to 0."]
    slow_mo: Option<f64>,
    #[doc = "Logger sink for Playwright logging. Optional."]
    logger: Option<Logger>,
    #[doc = "Maximum time in milliseconds to wait for the connection to be established. Defaults to `30000` (30 seconds). Pass `0` to\ndisable timeout."]
    timeout: Option<f64>
}
struct NotImplementedYetparams {
    #[doc = "A CDP websocket endpoint or http url to connect to. For example `http://localhost:9222/` or\n`ws://127.0.0.1:9222/devtools/browser/387adf4c-243f-4051-a181-46798f4a46f4`."]
    endpoint_u_r_l: String,
    #[doc = "Additional HTTP headers to be sent with connect request. Optional."]
    headers: Option<Map<String, String>>,
    #[doc = "Slows down Playwright operations by the specified amount of milliseconds. Useful so that you can see what is going on.\nDefaults to 0."]
    slow_mo: Option<f64>,
    #[doc = "Logger sink for Playwright logging. Optional."]
    logger: Option<Logger>,
    #[doc = "Maximum time in milliseconds to wait for the connection to be established. Defaults to `30000` (30 seconds). Pass `0` to\ndisable timeout."]
    timeout: Option<f64>
}
#[doc = "- extends: [EventEmitter]\n\nThe `CDPSession` instances are used to talk raw Chrome Devtools Protocol:\n- protocol methods can be called with `session.send` method.\n- protocol events can be subscribed to with `session.on` method.\n\nUseful links:\n- Documentation on DevTools Protocol can be found here:\n  [DevTools Protocol Viewer](https://chromedevtools.github.io/devtools-protocol/).\n- Getting Started with DevTools Protocol:\n  https://github.com/aslushnikov/getting-started-with-cdp/blob/master/README.md\n\n```js\nconst client = await page.context().newCDPSession(page);\nawait client.send('Animation.enable');\nclient.on('Animation.animationCreated', () => console.log('Animation created!'));\nconst response = await client.send('Animation.getPlaybackRate');\nconsole.log('playback rate is ' + response.playbackRate);\nawait client.send('Animation.setPlaybackRate', {\n  playbackRate: response.playbackRate / 2\n});\n```\n\n```python async\nclient = await page.context().new_cdp_session(page)\nawait client.send(\"animation.enable\")\nclient.on(\"animation.animation_created\", lambda: print(\"animation created!\"))\nresponse = await client.send(\"animation.get_playback_rate\")\nprint(\"playback rate is \" + response[\"playback_rate\"])\nawait client.send(\"animation.set_playback_rate\", {\n    playback_rate: response[\"playback_rate\"] / 2\n})\n```\n\n```python sync\nclient = page.context().new_cdp_session(page)\nclient.send(\"animation.enable\")\nclient.on(\"animation.animation_created\", lambda: print(\"animation created!\"))\nresponse = client.send(\"animation.get_playback_rate\")\nprint(\"playback rate is \" + response[\"playback_rate\"])\nclient.send(\"animation.set_playback_rate\", {\n    playback_rate: response[\"playback_rate\"] / 2\n})\n```\n"]
#[doc = "Extends EventEmitter"]
impl CDPSession {
    #[doc = "Detaches the CDPSession from the target. Once detached, the CDPSession object won't emit any events and can't be used to\nsend messages."]
    fn detach(&self) -> Result<(), Arc<Error>> { todo!() }
    #[doc = ""]
    fn send(
        &self,
        #[doc = "protocol method name"] method: String,
        #[doc = "Optional method parameters"] params: Option<Object>
    ) -> Result<Object, Arc<Error>> {
        todo!()
    }
}
#[doc = "`ConsoleMessage` objects are dispatched by page via the [`event: Page.console`] event."]
impl ConsoleMessage {
    #[doc = ""]
    fn args(&self) -> Result<Vec<JsHandle>, Error> { todo!() }
    #[doc = ""]
    fn location(&self) -> Result<NotImplementedYet, Error> { todo!() }
    #[doc = "URL of the resource followed by 0-based line and column numbers in the resource formatted as `URL:line:column`."]
    fn location(&self) -> Result<String, Error> { todo!() }
    #[doc = ""]
    fn text(&self) -> Result<String, Error> { todo!() }
    #[doc = "One of the following values: `'log'`, `'debug'`, `'info'`, `'error'`, `'warning'`, `'dir'`, `'dirxml'`, `'table'`,\n`'trace'`, `'clear'`, `'startGroup'`, `'startGroupCollapsed'`, `'endGroup'`, `'assert'`, `'profile'`, `'profileEnd'`,\n`'count'`, `'timeEnd'`."]
    fn r#type(&self) -> Result<String, Error> { todo!() }
}
struct NotImplementedYetlocation {
    #[doc = "URL of the resource."]
    url: String,
    #[doc = "0-based line number in the resource."]
    line_number: i64,
    #[doc = "0-based column number in the resource."]
    column_number: i64
}
#[doc = "Coverage gathers information about parts of JavaScript and CSS that were used by the page.\n\nAn example of using JavaScript coverage to produce Istanbul report for page load:\n\n> NOTE: Coverage APIs are only supported on Chromium-based browsers.\n\n```js\nconst { chromium } = require('playwright');\nconst v8toIstanbul = require('v8-to-istanbul');\n\n(async() => {\n  const browser = await chromium.launch();\n  const page = await browser.newPage();\n  await page.coverage.startJSCoverage();\n  await page.goto('https://chromium.org');\n  const coverage = await page.coverage.stopJSCoverage();\n  for (const entry of coverage) {\n    const converter = new v8toIstanbul('', 0, { source: entry.source });\n    await converter.load();\n    converter.applyCoverage(entry.functions);\n    console.log(JSON.stringify(converter.toIstanbul()));\n  }\n  await browser.close();\n})();\n```\n"]
impl Coverage {
    #[doc = "Returns coverage is started"]
    fn start_c_s_s_coverage(
        &self,
        #[doc = "options"]
        #[doc = "Whether to reset coverage on every navigation. Defaults to `true`."]
        reset_on_navigation: Option<bool>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Returns coverage is started\n\n> NOTE: Anonymous scripts are ones that don't have an associated url. These are scripts that are dynamically created on\nthe page using `eval` or `new Function`. If `reportAnonymousScripts` is set to `true`, anonymous scripts will have\n`__playwright_evaluation_script__` as their URL."]
    fn start_j_s_coverage(
        &self,
        #[doc = "options"]
        #[doc = "Whether anonymous scripts generated by the page should be reported. Defaults to `false`."]
        report_anonymous_scripts: Option<bool>,
        #[doc = "Whether to reset coverage on every navigation. Defaults to `true`."]
        reset_on_navigation: Option<bool>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Returns the array of coverage reports for all stylesheets\n\n> NOTE: CSS Coverage doesn't include dynamically injected style tags without sourceURLs."]
    fn stop_c_s_s_coverage(&self) -> Result<Vec<NotImplementedYet>, Arc<Error>> { todo!() }
    #[doc = "Returns the array of coverage reports for all scripts\n\n> NOTE: JavaScript Coverage doesn't include anonymous scripts by default. However, scripts with sourceURLs are reported."]
    fn stop_j_s_coverage(&self) -> Result<Vec<NotImplementedYet>, Arc<Error>> { todo!() }
}
#[doc = "`Dialog` objects are dispatched by page via the [`event: Page.dialog`] event.\n\nAn example of using `Dialog` class:\n\n```js\nconst { chromium } = require('playwright');  // Or 'firefox' or 'webkit'.\n\n(async () => {\n  const browser = await chromium.launch();\n  const page = await browser.newPage();\n  page.on('dialog', async dialog => {\n    console.log(dialog.message());\n    await dialog.dismiss();\n  });\n  await page.evaluate(() => alert('1'));\n  await browser.close();\n})();\n```\n\n```java\nimport com.microsoft.playwright.*;\n\npublic class Example {\n  public static void main(String[] args) {\n    try (Playwright playwright = Playwright.create()) {\n      BrowserType chromium = playwright.chromium();\n      Browser browser = chromium.launch();\n      Page page = browser.newPage();\n      page.onDialog(dialog -> {\n        System.out.println(dialog.message());\n        dialog.dismiss();\n      });\n      page.evaluate(\"alert('1')\");\n      browser.close();\n    }\n  }\n}\n```\n\n```python async\nimport asyncio\nfrom playwright.async_api import async_playwright\n\nasync def handle_dialog(dialog):\n    print(dialog.message)\n    await dialog.dismiss()\n\nasync def run(playwright):\n    chromium = playwright.chromium\n    browser = await chromium.launch()\n    page = await browser.new_page()\n    page.on(\"dialog\", handle_dialog)\n    page.evaluate(\"alert('1')\")\n    await browser.close()\n\nasync def main():\n    async with async_playwright() as playwright:\n        await run(playwright)\nasyncio.run(main())\n```\n\n```python sync\nfrom playwright.sync_api import sync_playwright\n\ndef handle_dialog(dialog):\n    print(dialog.message)\n    dialog.dismiss()\n\ndef run(playwright):\n    chromium = playwright.chromium\n    browser = chromium.launch()\n    page = browser.new_page()\n    page.on(\"dialog\", handle_dialog)\n    page.evaluate(\"alert('1')\")\n    browser.close()\n\nwith sync_playwright() as playwright:\n    run(playwright)\n```\n\n> NOTE: Dialogs are dismissed automatically, unless there is a [`event: Page.dialog`] listener. When listener is\npresent, it **must** either [`method: Dialog.accept`] or [`method: Dialog.dismiss`] the dialog - otherwise the page will\n[freeze](https://developer.mozilla.org/en-US/docs/Web/JavaScript/EventLoop#never_blocking) waiting for the dialog, and\nactions like click will never finish."]
impl Dialog {
    #[doc = "Returns when the dialog has been accepted."]
    fn accept(
        &self,
        #[doc = "A text to enter in prompt. Does not cause any effects if the dialog's `type` is not prompt. Optional."]
        prompt_text: Option<String>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "If dialog is prompt, returns default prompt value. Otherwise, returns empty string."]
    fn default_value(&self) -> Result<String, Error> { todo!() }
    #[doc = "Returns when the dialog has been dismissed."]
    fn dismiss(&self) -> Result<(), Arc<Error>> { todo!() }
    #[doc = "A message displayed in the dialog."]
    fn message(&self) -> Result<String, Error> { todo!() }
    #[doc = "Returns dialog's type, can be one of `alert`, `beforeunload`, `confirm` or `prompt`."]
    fn r#type(&self) -> Result<String, Error> { todo!() }
}
#[doc = "`Download` objects are dispatched by page via the [`event: Page.download`] event.\n\nAll the downloaded files belonging to the browser context are deleted when the browser context is closed. All downloaded\nfiles are deleted when the browser closes.\n\nDownload event is emitted once the download starts. Download path becomes available once download completes:\n\n```js\nconst [ download ] = await Promise.all([\n  page.waitForEvent('download'), // wait for download to start\n  page.click('a')\n]);\n// wait for download to complete\nconst path = await download.path();\n```\n\n```java\n// wait for download to start\nDownload download  = page.waitForDownload(() -> page.click(\"a\"));\n// wait for download to complete\nPath path = download.path();\n```\n\n```java\n// wait for download to start\nDownload download = page.waitForDownload(() -> {\n  page.click(\"a\");\n});\n// wait for download to complete\nPath path = download.path();\n```\n\n```python async\nasync with page.expect_download() as download_info:\n    await page.click(\"a\")\ndownload = await download_info.value\n# waits for download to complete\npath = await download.path()\n```\n\n```python sync\nwith page.expect_download() as download_info:\n    page.click(\"a\")\ndownload = download_info.value\n# wait for download to complete\npath = download.path()\n```\n\n> NOTE: Browser context **must** be created with the `acceptDownloads` set to `true` when user needs access to the\ndownloaded content. If `acceptDownloads` is not set, download events are emitted, but the actual download is not\nperformed and user has no access to the downloaded files."]
impl Download {
    #[doc = "Returns readable stream for current download or `null` if download failed."]
    fn create_read_stream(&self) -> Result<Option<Readable>, Arc<Error>> { todo!() }
    #[doc = "Deletes the downloaded file. Will wait for the download to finish if necessary."]
    fn delete(&self) -> Result<(), Arc<Error>> { todo!() }
    #[doc = "Returns download error if any. Will wait for the download to finish if necessary."]
    fn failure(&self) -> Result<Option<String>, Arc<Error>> { todo!() }
    #[doc = "Returns path to the downloaded file in case of successful download. The method will wait for the download to finish if\nnecessary. The method throws when connected remotely."]
    fn path(&self) -> Result<Option<path>, Arc<Error>> { todo!() }
    #[doc = "Saves the download to a user-specified path. It is safe to call this method while the download is still in progress."]
    fn save_as(
        &self,
        #[doc = "Path where the download should be saved."] path: path
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Returns suggested filename for this download. It is typically computed by the browser from the\n[`Content-Disposition`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Disposition) response header\nor the `download` attribute. See the spec on [whatwg](https://html.spec.whatwg.org/#downloading-resources). Different\nbrowsers can use different logic for computing it."]
    fn suggested_filename(&self) -> Result<String, Error> { todo!() }
    #[doc = "Returns downloaded url."]
    fn url(&self) -> Result<String, Error> { todo!() }
}
#[doc = "Playwright has **experimental** support for Electron automation. You can access electron namespace via:\n\n```js\nconst { _electron } = require('playwright');\n```\n\nAn example of the Electron automation script would be:\n\n```js\nconst { _electron: electron } = require('playwright');\n\n(async () => {\n  // Launch Electron app.\n  const electronApp = await electron.launch({ args: ['main.js'] });\n\n  // Evaluation expression in the Electron context.\n  const appPath = await electronApp.evaluate(async ({ app }) => {\n    // This runs in the main Electron process, parameter here is always\n    // the result of the require('electron') in the main app script.\n    return app.getAppPath();\n  });\n  console.log(appPath);\n\n  // Get the first window that the app opens, wait if necessary.\n  const window = await electronApp.firstWindow();\n  // Print the title.\n  console.log(await window.title());\n  // Capture a screenshot.\n  await window.screenshot({ path: 'intro.png' });\n  // Direct Electron console to Node terminal.\n  window.on('console', console.log);\n  // Click button.\n  await window.click('text=Click me');\n  // Exit app.\n  await electronApp.close();\n})();\n```\n\nNote that since you don't need Playwright to install web browsers when testing Electron, you can omit browser download\nvia setting the following environment variable when installing Playwright:\n\n```sh js\n$ PLAYWRIGHT_SKIP_BROWSER_DOWNLOAD=1 npm i -D playwright\n```\n"]
impl Electron {
    #[doc = "Launches electron application specified with the `executablePath`."]
    fn launch(
        &self,
        #[doc = "options"]
        #[doc = "Additional arguments to pass to the application when launching. You typically pass the main script name here."]
        args: Option<Vec<String>>,
        #[doc = "Current working directory to launch application from."] cwd: Option<String>,
        #[doc = "Specifies environment variables that will be visible to Electron. Defaults to `process.env`."]
        env: Option<Map<String, String>>,
        #[doc = "Launches given Electron application. If not specified, launches the default Electron executable installed in this\npackage, located at `node_modules/.bin/electron`."]
        executable_path: Option<String>
    ) -> Result<ElectronApplication, Arc<Error>> {
        todo!()
    }
}
#[doc = "Electron application representation. You can use [`method: Electron.launch`] to obtain the application instance. This\ninstance you can control main electron process as well as work with Electron windows:\n\n```js\nconst { _electron: electron } = require('playwright');\n\n(async () => {\n  // Launch Electron app.\n  const electronApp = await electron.launch({ args: ['main.js'] });\n\n  // Evaluation expression in the Electron context.\n  const appPath = await electronApp.evaluate(async ({ app }) => {\n    // This runs in the main Electron process, parameter here is always\n    // the result of the require('electron') in the main app script.\n    return app.getAppPath();\n  });\n  console.log(appPath);\n\n  // Get the first window that the app opens, wait if necessary.\n  const window = await electronApp.firstWindow();\n  // Print the title.\n  console.log(await window.title());\n  // Capture a screenshot.\n  await window.screenshot({ path: 'intro.png' });\n  // Direct Electron console to Node terminal.\n  window.on('console', console.log);\n  // Click button.\n  await window.click('text=Click me');\n  // Exit app.\n  await electronApp.close();\n})();\n```\n"]
impl ElectronApplication {
    #[doc = "Returns the BrowserWindow object that corresponds to the given Playwright page."]
    fn browser_window(
        &self,
        #[doc = "Page to retrieve the window for."] page: Page
    ) -> Result<JsHandle, Arc<Error>> {
        todo!()
    }
    #[doc = "Closes Electron application."]
    fn close(&self) -> Result<(), Arc<Error>> { todo!() }
    #[doc = "This method returns browser context that can be used for setting up context-wide routing, etc."]
    fn context(&self) -> Result<BrowserContext, Error> { todo!() }
    #[doc = "Returns the return value of `expression`.\n\nIf the function passed to the [`method: ElectronApplication.evaluate`] returns a [Promise], then\n[`method: ElectronApplication.evaluate`] would wait for the promise to resolve and return its value.\n\nIf the function passed to the [`method: ElectronApplication.evaluate`] returns a non-[Serializable] value, then\n[`method: ElectronApplication.evaluate`] returns `undefined`. Playwright also supports transferring some additional\nvalues that are not serializable by `JSON`: `-0`, `NaN`, `Infinity`, `-Infinity`."]
    fn evaluate(
        &self,
        #[doc = "JavaScript expression to be evaluated in the browser context. If it looks like a function declaration, it is interpreted\nas a function. Otherwise, evaluated as an expression."]
        expression: String,
        #[doc = "Optional argument to pass to `expression`."] arg: Option<EvaluationArgument>
    ) -> Result<Serializable, Arc<Error>> {
        todo!()
    }
    #[doc = "Returns the return value of `expression` as a `JSHandle`.\n\nThe only difference between [`method: ElectronApplication.evaluate`] and [`method: ElectronApplication.evaluateHandle`]\nis that [`method: ElectronApplication.evaluateHandle`] returns `JSHandle`.\n\nIf the function passed to the [`method: ElectronApplication.evaluateHandle`] returns a [Promise], then\n[`method: ElectronApplication.evaluateHandle`] would wait for the promise to resolve and return its value."]
    fn evaluate_handle(
        &self,
        #[doc = "JavaScript expression to be evaluated in the browser context. If it looks like a function declaration, it is interpreted\nas a function. Otherwise, evaluated as an expression."]
        expression: String,
        #[doc = ""] arg: EvaluationArgument
    ) -> Result<JsHandle, Arc<Error>> {
        todo!()
    }
    #[doc = "Convenience method that waits for the first application window to be opened. Typically your script will start with:\n\n```js\n  const electronApp = await electron.launch({\n    args: ['main.js']\n  });\n  const window = await electronApp.firstWindow();\n  // ...\n```\n"]
    fn first_window(&self) -> Result<Page, Arc<Error>> { todo!() }
    #[doc = "Waits for event to fire and passes its value into the predicate function. Returns when the predicate returns truthy\nvalue. Will throw an error if the application is closed before the event is fired. Returns the event data value.\n\n```js\nconst [window] = await Promise.all([\n  electronApp.waitForEvent('window'),\n  mainWindow.click('button')\n]);\n```\n"]
    fn wait_for_event(
        &self,
        #[doc = "Event name, same one typically passed into `*.on(event)`."] event: String,
        #[doc = "Either a predicate that receives an event or an options object. Optional."]
        options_or_predicate: Option<NotImplementedYet>
    ) -> Result<any, Arc<Error>> {
        todo!()
    }
    #[doc = "Convenience method that returns all the opened windows."]
    fn windows(&self) -> Result<Vec<Page>, Error> { todo!() }
}
enum NotImplementedYetoptionsOrPredicate {
    NotImplementedYet(function),
    NotImplementedYet(NotImplementedYet)
}
struct NotImplementedYet {
    #[doc = "receives the event data and resolves to truthy value when the waiting should resolve."]
    predicate: function,
    #[doc = "maximum time to wait for in milliseconds. Defaults to `30000` (30 seconds). Pass `0` to disable timeout. The default\nvalue can be changed by using the [`method: BrowserContext.setDefaultTimeout`]."]
    timeout: Option<f64>
}
enum ElectronApplicationEventType {
    #[doc = "This event is issued when the application closes."]
    Close,
    #[doc = "This event is issued for every window that is created **and loaded** in Electron. It contains a `Page` that can be used\nfor Playwright automation."]
    Window
}
enum ElectronApplicationEvent {
    #[doc = "This event is issued when the application closes."]
    Close(()),
    #[doc = "This event is issued for every window that is created **and loaded** in Electron. It contains a `Page` that can be used\nfor Playwright automation."]
    Window(Page)
}
#[doc = "- extends: `JSHandle`\n\nElementHandle represents an in-page DOM element. ElementHandles can be created with the [`method: Page.querySelector`]\nmethod.\n\n```js\nconst { chromium } = require('playwright');  // Or 'firefox' or 'webkit'.\n\n(async () => {\n  const browser = await chromium.launch();\n  const page = await browser.newPage();\n  await page.goto('https://example.com');\n  const hrefElement = await page.$('a');\n  await hrefElement.click();\n  // ...\n})();\n```\n\n```java\nimport com.microsoft.playwright.*;\n\npublic class Example {\n  public static void main(String[] args) {\n    try (Playwright playwright = Playwright.create()) {\n      BrowserType chromium = playwright.chromium();\n      Browser browser = chromium.launch();\n      Page page = browser.newPage();\n      page.navigate(\"https://example.com\");\n      ElementHandle hrefElement = page.querySelector(\"a\");\n      hrefElement.click();\n      // ...\n    }\n  }\n}\n```\n\n```python async\nimport asyncio\nfrom playwright.async_api import async_playwright\n\nasync def run(playwright):\n    chromium = playwright.chromium\n    browser = await chromium.launch()\n    page = await browser.new_page()\n    await page.goto(\"https://example.com\")\n    href_element = await page.query_selector(\"a\")\n    await href_element.click()\n    # ...\n\nasync def main():\n    async with async_playwright() as playwright:\n        await run(playwright)\nasyncio.run(main())\n```\n\n```python sync\nfrom playwright.sync_api import sync_playwright\n\ndef run(playwright):\n    chromium = playwright.chromium\n    browser = chromium.launch()\n    page = browser.new_page()\n    page.goto(\"https://example.com\")\n    href_element = page.query_selector(\"a\")\n    href_element.click()\n    # ...\n\nwith sync_playwright() as playwright:\n    run(playwright)\n```\n\nElementHandle prevents DOM element from garbage collection unless the handle is disposed with\n[`method: JSHandle.dispose`]. ElementHandles are auto-disposed when their origin frame gets navigated.\n\nElementHandle instances can be used as an argument in [`method: Page.evalOnSelector`] and [`method: Page.evaluate`]\nmethods."]
#[doc = "Extends JSHandle"]
impl ElementHandle {
    #[doc = "This method returns the bounding box of the element, or `null` if the element is not visible. The bounding box is\ncalculated relative to the main frame viewport - which is usually the same as the browser window.\n\nScrolling affects the returned bonding box, similarly to\n[Element.getBoundingClientRect](https://developer.mozilla.org/en-US/docs/Web/API/Element/getBoundingClientRect). That\nmeans `x` and/or `y` may be negative.\n\nElements from child frames return the bounding box relative to the main frame, unlike the\n[Element.getBoundingClientRect](https://developer.mozilla.org/en-US/docs/Web/API/Element/getBoundingClientRect).\n\nAssuming the page is static, it is safe to use bounding box coordinates to perform input. For example, the following\nsnippet should click the center of the element.\n\n```js\nconst box = await elementHandle.boundingBox();\nawait page.mouse.click(box.x + box.width / 2, box.y + box.height / 2);\n```\n\n```java\nBoundingBox box = elementHandle.boundingBox();\npage.mouse().click(box.x + box.width / 2, box.y + box.height / 2);\n```\n\n```python async\nbox = await element_handle.bounding_box()\nawait page.mouse.click(box[\"x\"] + box[\"width\"] / 2, box[\"y\"] + box[\"height\"] / 2)\n```\n\n```python sync\nbox = element_handle.bounding_box()\npage.mouse.click(box[\"x\"] + box[\"width\"] / 2, box[\"y\"] + box[\"height\"] / 2)\n```\n"]
    fn bounding_box(&self) -> Result<Option<NotImplementedYet>, Arc<Error>> { todo!() }
    #[doc = "This method checks the element by performing the following steps:\n1. Ensure that element is a checkbox or a radio input. If not, this method throws. If the element is already checked,\n   this method returns immediately.\n1. Wait for [actionability](https://playwright.dev/docs/actionability/) checks on the element, unless `force` option is set.\n1. Scroll the element into view if needed.\n1. Use [`property: Page.mouse`] to click in the center of the element.\n1. Wait for initiated navigations to either succeed or fail, unless `noWaitAfter` option is set.\n1. Ensure that the element is now checked. If not, this method throws.\n\nIf the element is detached from the DOM at any moment during the action, this method throws.\n\nWhen all steps combined have not finished during the specified `timeout`, this method throws a `TimeoutError`. Passing\nzero timeout disables this."]
    fn check(
        &self,
        #[doc = "options"]
        #[doc = "Whether to bypass the [actionability](https://playwright.dev/docs/actionability/) checks. Defaults to `false`."]
        force: Option<bool>,
        #[doc = "Actions that initiate navigations are waiting for these navigations to happen and for pages to start loading. You can\nopt out of waiting via setting this flag. You would only need this option in the exceptional cases such as navigating to\ninaccessible pages. Defaults to `false`."]
        no_wait_after: Option<bool>,
        #[doc = "A point to use relative to the top-left corner of element padding box. If not specified, uses some visible point of the\nelement."]
        position: Option<NotImplementedYet>,
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>,
        #[doc = "When set, this method only performs the [actionability](https://playwright.dev/docs/actionability/) checks and skips the action. Defaults to\n`false`. Useful to wait until the element is ready for the action without performing it."]
        trial: Option<bool>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "This method clicks the element by performing the following steps:\n1. Wait for [actionability](https://playwright.dev/docs/actionability/) checks on the element, unless `force` option is set.\n1. Scroll the element into view if needed.\n1. Use [`property: Page.mouse`] to click in the center of the element, or the specified `position`.\n1. Wait for initiated navigations to either succeed or fail, unless `noWaitAfter` option is set.\n\nIf the element is detached from the DOM at any moment during the action, this method throws.\n\nWhen all steps combined have not finished during the specified `timeout`, this method throws a `TimeoutError`. Passing\nzero timeout disables this."]
    fn click(
        &self,
        #[doc = "options"]
        #[doc = "Defaults to `left`."]
        button: Option<MouseButton>,
        #[doc = "defaults to 1. See [UIEvent.detail]."] click_count: Option<i64>,
        #[doc = "Time to wait between `mousedown` and `mouseup` in milliseconds. Defaults to 0."]
        delay: Option<f64>,
        #[doc = "Whether to bypass the [actionability](https://playwright.dev/docs/actionability/) checks. Defaults to `false`."]
        force: Option<bool>,
        #[doc = "Modifier keys to press. Ensures that only these modifiers are pressed during the operation, and then restores current\nmodifiers back. If not specified, currently pressed modifiers are used."]
        modifiers: Option<Vec<KeyboardModifier>>,
        #[doc = "Actions that initiate navigations are waiting for these navigations to happen and for pages to start loading. You can\nopt out of waiting via setting this flag. You would only need this option in the exceptional cases such as navigating to\ninaccessible pages. Defaults to `false`."]
        no_wait_after: Option<bool>,
        #[doc = "A point to use relative to the top-left corner of element padding box. If not specified, uses some visible point of the\nelement."]
        position: Option<NotImplementedYet>,
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>,
        #[doc = "When set, this method only performs the [actionability](https://playwright.dev/docs/actionability/) checks and skips the action. Defaults to\n`false`. Useful to wait until the element is ready for the action without performing it."]
        trial: Option<bool>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Returns the content frame for element handles referencing iframe nodes, or `null` otherwise"]
    fn content_frame(&self) -> Result<Option<Frame>, Arc<Error>> { todo!() }
    #[doc = "This method double clicks the element by performing the following steps:\n1. Wait for [actionability](https://playwright.dev/docs/actionability/) checks on the element, unless `force` option is set.\n1. Scroll the element into view if needed.\n1. Use [`property: Page.mouse`] to double click in the center of the element, or the specified `position`.\n1. Wait for initiated navigations to either succeed or fail, unless `noWaitAfter` option is set. Note that if the\n   first click of the `dblclick()` triggers a navigation event, this method will throw.\n\nIf the element is detached from the DOM at any moment during the action, this method throws.\n\nWhen all steps combined have not finished during the specified `timeout`, this method throws a `TimeoutError`. Passing\nzero timeout disables this.\n\n> NOTE: `elementHandle.dblclick()` dispatches two `click` events and a single `dblclick` event."]
    fn dblclick(
        &self,
        #[doc = "options"]
        #[doc = "Defaults to `left`."]
        button: Option<MouseButton>,
        #[doc = "Time to wait between `mousedown` and `mouseup` in milliseconds. Defaults to 0."]
        delay: Option<f64>,
        #[doc = "Whether to bypass the [actionability](https://playwright.dev/docs/actionability/) checks. Defaults to `false`."]
        force: Option<bool>,
        #[doc = "Modifier keys to press. Ensures that only these modifiers are pressed during the operation, and then restores current\nmodifiers back. If not specified, currently pressed modifiers are used."]
        modifiers: Option<Vec<KeyboardModifier>>,
        #[doc = "Actions that initiate navigations are waiting for these navigations to happen and for pages to start loading. You can\nopt out of waiting via setting this flag. You would only need this option in the exceptional cases such as navigating to\ninaccessible pages. Defaults to `false`."]
        no_wait_after: Option<bool>,
        #[doc = "A point to use relative to the top-left corner of element padding box. If not specified, uses some visible point of the\nelement."]
        position: Option<NotImplementedYet>,
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>,
        #[doc = "When set, this method only performs the [actionability](https://playwright.dev/docs/actionability/) checks and skips the action. Defaults to\n`false`. Useful to wait until the element is ready for the action without performing it."]
        trial: Option<bool>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "The snippet below dispatches the `click` event on the element. Regardless of the visibility state of the element,\n`click` is dispatched. This is equivalent to calling\n[element.click()](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/click).\n\n```js\nawait elementHandle.dispatchEvent('click');\n```\n\n```java\nelementHandle.dispatchEvent(\"click\");\n```\n\n```python async\nawait element_handle.dispatch_event(\"click\")\n```\n\n```python sync\nelement_handle.dispatch_event(\"click\")\n```\n\nUnder the hood, it creates an instance of an event based on the given `type`, initializes it with `eventInit` properties\nand dispatches it on the element. Events are `composed`, `cancelable` and bubble by default.\n\nSince `eventInit` is event-specific, please refer to the events documentation for the lists of initial properties:\n- [DragEvent](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/DragEvent)\n- [FocusEvent](https://developer.mozilla.org/en-US/docs/Web/API/FocusEvent/FocusEvent)\n- [KeyboardEvent](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/KeyboardEvent)\n- [MouseEvent](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/MouseEvent)\n- [PointerEvent](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/PointerEvent)\n- [TouchEvent](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/TouchEvent)\n- [Event](https://developer.mozilla.org/en-US/docs/Web/API/Event/Event)\n\nYou can also specify `JSHandle` as the property value if you want live objects to be passed into the event:\n\n```js\n// Note you can only create DataTransfer in Chromium and Firefox\nconst dataTransfer = await page.evaluateHandle(() => new DataTransfer());\nawait elementHandle.dispatchEvent('dragstart', { dataTransfer });\n```\n\n```java\n// Note you can only create DataTransfer in Chromium and Firefox\nJSHandle dataTransfer = page.evaluateHandle(\"() => new DataTransfer()\");\nMap<String, Object> arg = new HashMap<>();\narg.put(\"dataTransfer\", dataTransfer);\nelementHandle.dispatchEvent(\"dragstart\", arg);\n```\n\n```python async\n# note you can only create data_transfer in chromium and firefox\ndata_transfer = await page.evaluate_handle(\"new DataTransfer()\")\nawait element_handle.dispatch_event(\"#source\", \"dragstart\", {\"dataTransfer\": data_transfer})\n```\n\n```python sync\n# note you can only create data_transfer in chromium and firefox\ndata_transfer = page.evaluate_handle(\"new DataTransfer()\")\nelement_handle.dispatch_event(\"#source\", \"dragstart\", {\"dataTransfer\": data_transfer})\n```\n"]
    fn dispatch_event(
        &self,
        #[doc = "DOM event type: `\"click\"`, `\"dragstart\"`, etc."] r#type: String,
        #[doc = "Optional event-specific initialization properties."] event_init: Option<
            EvaluationArgument
        >
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Returns the return value of `expression`.\n\nThe method finds an element matching the specified selector in the `ElementHandle`s subtree and passes it as a first\nargument to `expression`. See [Working with selectors](./selectors.md) for more details. If no elements match the\nselector, the method throws an error.\n\nIf `expression` returns a [Promise], then [`method: ElementHandle.evalOnSelector`] would wait for the promise to resolve\nand return its value.\n\nExamples:\n\n```js\nconst tweetHandle = await page.$('.tweet');\nexpect(await tweetHandle.$eval('.like', node => node.innerText)).toBe('100');\nexpect(await tweetHandle.$eval('.retweets', node => node.innerText)).toBe('10');\n```\n\n```java\nElementHandle tweetHandle = page.querySelector(\".tweet\");\nassertEquals(\"100\", tweetHandle.evalOnSelector(\".like\", \"node => node.innerText\"));\nassertEquals(\"10\", tweetHandle.evalOnSelector(\".retweets\", \"node => node.innerText\"));\n```\n\n```python async\ntweet_handle = await page.query_selector(\".tweet\")\nassert await tweet_handle.eval_on_selector(\".like\", \"node => node.innerText\") == \"100\"\nassert await tweet_handle.eval_on_selector(\".retweets\", \"node => node.innerText\") = \"10\"\n```\n\n```python sync\ntweet_handle = page.query_selector(\".tweet\")\nassert tweet_handle.eval_on_selector(\".like\", \"node => node.innerText\") == \"100\"\nassert tweet_handle.eval_on_selector(\".retweets\", \"node => node.innerText\") = \"10\"\n```\n"]
    fn eval_on_selector(
        &self,
        #[doc = "A selector to query for. See [working with selectors](./selectors.md) for more details."]
        selector: String,
        #[doc = "JavaScript expression to be evaluated in the browser context. If it looks like a function declaration, it is interpreted\nas a function. Otherwise, evaluated as an expression."]
        expression: String,
        #[doc = "Optional argument to pass to `expression`."] arg: Option<EvaluationArgument>
    ) -> Result<Serializable, Arc<Error>> {
        todo!()
    }
    #[doc = "Returns the return value of `expression`.\n\nThe method finds all elements matching the specified selector in the `ElementHandle`'s subtree and passes an array of\nmatched elements as a first argument to `expression`. See [Working with selectors](./selectors.md) for more details.\n\nIf `expression` returns a [Promise], then [`method: ElementHandle.evalOnSelectorAll`] would wait for the promise to\nresolve and return its value.\n\nExamples:\n\n```html\n<div class=\"feed\">\n  <div class=\"tweet\">Hello!</div>\n  <div class=\"tweet\">Hi!</div>\n</div>\n```\n\n```js\nconst feedHandle = await page.$('.feed');\nexpect(await feedHandle.$$eval('.tweet', nodes => nodes.map(n => n.innerText))).toEqual(['Hello!', 'Hi!']);\n```\n\n```java\nElementHandle feedHandle = page.querySelector(\".feed\");\nassertEquals(Arrays.asList(\"Hello!\", \"Hi!\"), feedHandle.evalOnSelectorAll(\".tweet\", \"nodes => nodes.map(n => n.innerText)\"));\n```\n\n```python async\nfeed_handle = await page.query_selector(\".feed\")\nassert await feed_handle.eval_on_selector_all(\".tweet\", \"nodes => nodes.map(n => n.innerText)\") == [\"hello!\", \"hi!\"]\n```\n\n```python sync\nfeed_handle = page.query_selector(\".feed\")\nassert feed_handle.eval_on_selector_all(\".tweet\", \"nodes => nodes.map(n => n.innerText)\") == [\"hello!\", \"hi!\"]\n```\n"]
    fn eval_on_selector_all(
        &self,
        #[doc = "A selector to query for. See [working with selectors](./selectors.md) for more details."]
        selector: String,
        #[doc = "JavaScript expression to be evaluated in the browser context. If it looks like a function declaration, it is interpreted\nas a function. Otherwise, evaluated as an expression."]
        expression: String,
        #[doc = "Optional argument to pass to `expression`."] arg: Option<EvaluationArgument>
    ) -> Result<Serializable, Arc<Error>> {
        todo!()
    }
    #[doc = "This method waits for [actionability](https://playwright.dev/docs/actionability/) checks, focuses the element, fills it and triggers an `input`\nevent after filling. If the element is inside the `<label>` element that has associated\n[control](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLabelElement/control), that control will be filled\ninstead. If the element to be filled is not an `<input>`, `<textarea>` or `[contenteditable]` element, this method\nthrows an error. Note that you can pass an empty string to clear the input field."]
    fn fill(
        &self,
        #[doc = "Value to set for the `<input>`, `<textarea>` or `[contenteditable]` element."]
        value: String,
        #[doc = "options"]
        #[doc = "Actions that initiate navigations are waiting for these navigations to happen and for pages to start loading. You can\nopt out of waiting via setting this flag. You would only need this option in the exceptional cases such as navigating to\ninaccessible pages. Defaults to `false`."]
        no_wait_after: Option<bool>,
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Calls [focus](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/focus) on the element."]
    fn focus(&self) -> Result<(), Arc<Error>> { todo!() }
    #[doc = "Returns element attribute value."]
    fn get_attribute(
        &self,
        #[doc = "Attribute name to get the value for."] name: String
    ) -> Result<Option<String>, Arc<Error>> {
        todo!()
    }
    #[doc = "This method hovers over the element by performing the following steps:\n1. Wait for [actionability](https://playwright.dev/docs/actionability/) checks on the element, unless `force` option is set.\n1. Scroll the element into view if needed.\n1. Use [`property: Page.mouse`] to hover over the center of the element, or the specified `position`.\n1. Wait for initiated navigations to either succeed or fail, unless `noWaitAfter` option is set.\n\nIf the element is detached from the DOM at any moment during the action, this method throws.\n\nWhen all steps combined have not finished during the specified `timeout`, this method throws a `TimeoutError`. Passing\nzero timeout disables this."]
    fn hover(
        &self,
        #[doc = "options"]
        #[doc = "Whether to bypass the [actionability](https://playwright.dev/docs/actionability/) checks. Defaults to `false`."]
        force: Option<bool>,
        #[doc = "Modifier keys to press. Ensures that only these modifiers are pressed during the operation, and then restores current\nmodifiers back. If not specified, currently pressed modifiers are used."]
        modifiers: Option<Vec<KeyboardModifier>>,
        #[doc = "A point to use relative to the top-left corner of element padding box. If not specified, uses some visible point of the\nelement."]
        position: Option<NotImplementedYet>,
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>,
        #[doc = "When set, this method only performs the [actionability](https://playwright.dev/docs/actionability/) checks and skips the action. Defaults to\n`false`. Useful to wait until the element is ready for the action without performing it."]
        trial: Option<bool>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Returns the `element.innerHTML`."]
    fn inner_h_t_m_l(&self) -> Result<String, Arc<Error>> { todo!() }
    #[doc = "Returns the `element.innerText`."]
    fn inner_text(&self) -> Result<String, Arc<Error>> { todo!() }
    #[doc = "Returns whether the element is checked. Throws if the element is not a checkbox or radio input."]
    fn is_checked(&self) -> Result<bool, Arc<Error>> { todo!() }
    #[doc = "Returns whether the element is disabled, the opposite of [enabled](./actionability.md#enabled)."]
    fn is_disabled(&self) -> Result<bool, Arc<Error>> { todo!() }
    #[doc = "Returns whether the element is [editable](./actionability.md#editable)."]
    fn is_editable(&self) -> Result<bool, Arc<Error>> { todo!() }
    #[doc = "Returns whether the element is [enabled](./actionability.md#enabled)."]
    fn is_enabled(&self) -> Result<bool, Arc<Error>> { todo!() }
    #[doc = "Returns whether the element is hidden, the opposite of [visible](./actionability.md#visible)."]
    fn is_hidden(&self) -> Result<bool, Arc<Error>> { todo!() }
    #[doc = "Returns whether the element is [visible](./actionability.md#visible)."]
    fn is_visible(&self) -> Result<bool, Arc<Error>> { todo!() }
    #[doc = "Returns the frame containing the given element."]
    fn owner_frame(&self) -> Result<Option<Frame>, Arc<Error>> { todo!() }
    #[doc = "Focuses the element, and then uses [`method: Keyboard.down`] and [`method: Keyboard.up`].\n\n`key` can specify the intended [keyboardEvent.key](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key)\nvalue or a single character to generate the text for. A superset of the `key` values can be found\n[here](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key/Key_Values). Examples of the keys are:\n\n`F1` - `F12`, `Digit0`- `Digit9`, `KeyA`- `KeyZ`, `Backquote`, `Minus`, `Equal`, `Backslash`, `Backspace`, `Tab`,\n`Delete`, `Escape`, `ArrowDown`, `End`, `Enter`, `Home`, `Insert`, `PageDown`, `PageUp`, `ArrowRight`, `ArrowUp`, etc.\n\nFollowing modification shortcuts are also supported: `Shift`, `Control`, `Alt`, `Meta`, `ShiftLeft`.\n\nHolding down `Shift` will type the text that corresponds to the `key` in the upper case.\n\nIf `key` is a single character, it is case-sensitive, so the values `a` and `A` will generate different respective\ntexts.\n\nShortcuts such as `key: \"Control+o\"` or `key: \"Control+Shift+T\"` are supported as well. When specified with the\nmodifier, modifier is pressed and being held while the subsequent key is being pressed."]
    fn press(
        &self,
        #[doc = "Name of the key to press or a character to generate, such as `ArrowLeft` or `a`."]
        key: String,
        #[doc = "options"]
        #[doc = "Time to wait between `keydown` and `keyup` in milliseconds. Defaults to 0."]
        delay: Option<f64>,
        #[doc = "Actions that initiate navigations are waiting for these navigations to happen and for pages to start loading. You can\nopt out of waiting via setting this flag. You would only need this option in the exceptional cases such as navigating to\ninaccessible pages. Defaults to `false`."]
        no_wait_after: Option<bool>,
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "The method finds an element matching the specified selector in the `ElementHandle`'s subtree. See\n[Working with selectors](./selectors.md) for more details. If no elements match the selector, returns `null`."]
    fn query_selector(
        &self,
        #[doc = "A selector to query for. See [working with selectors](./selectors.md) for more details."]
        selector: String
    ) -> Result<Option<ElementHandle>, Arc<Error>> {
        todo!()
    }
    #[doc = "The method finds all elements matching the specified selector in the `ElementHandle`s subtree. See\n[Working with selectors](./selectors.md) for more details. If no elements match the selector, returns empty array."]
    fn query_selector_all(
        &self,
        #[doc = "A selector to query for. See [working with selectors](./selectors.md) for more details."]
        selector: String
    ) -> Result<Vec<ElementHandle>, Arc<Error>> {
        todo!()
    }
    #[doc = "Returns the buffer with the captured screenshot.\n\nThis method waits for the [actionability](https://playwright.dev/docs/actionability/) checks, then scrolls element into view before taking a\nscreenshot. If the element is detached from DOM, the method throws an error."]
    fn screenshot(
        &self,
        #[doc = "options"]
        #[doc = "Hides default white background and allows capturing screenshots with transparency. Not applicable to `jpeg` images.\nDefaults to `false`."]
        omit_background: Option<bool>,
        #[doc = "The file path to save the image to. The screenshot type will be inferred from file extension. If `path` is a relative\npath, then it is resolved relative to the current working directory. If no path is provided, the image won't be saved to\nthe disk."]
        path: Option<path>,
        quality : Option < i64 >,
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>,
        #[doc = "Specify screenshot type, defaults to `png`."] r#type: Option<ScreenshotType>
    ) -> Result<Buffer, Arc<Error>> {
        todo!()
    }
    #[doc = "This method waits for [actionability](https://playwright.dev/docs/actionability/) checks, then tries to scroll element into view, unless it is\ncompletely visible as defined by\n[IntersectionObserver](https://developer.mozilla.org/en-US/docs/Web/API/Intersection_Observer_API)'s `ratio`.\n\nThrows when `elementHandle` does not point to an element\n[connected](https://developer.mozilla.org/en-US/docs/Web/API/Node/isConnected) to a Document or a ShadowRoot."]
    fn scroll_into_view_if_needed(
        &self,
        #[doc = "options"]
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Returns the array of option values that have been successfully selected.\n\nTriggers a `change` and `input` event once all the provided options have been selected. If element is not a `<select>`\nelement, the method throws an error.\n\nWill wait until all specified options are present in the `<select>` element.\n\n```js\n// single selection matching the value\nhandle.selectOption('blue');\n\n// single selection matching the label\nhandle.selectOption({ label: 'Blue' });\n\n// multiple selection\nhandle.selectOption(['red', 'green', 'blue']);\n```\n\n```java\n// single selection matching the value\nhandle.selectOption(\"blue\");\n// single selection matching the label\nhandle.selectOption(new SelectOption().setLabel(\"Blue\"));\n// multiple selection\nhandle.selectOption(new String[] {\"red\", \"green\", \"blue\"});\n```\n\n```python async\n# single selection matching the value\nawait handle.select_option(\"blue\")\n# single selection matching the label\nawait handle.select_option(label=\"blue\")\n# multiple selection\nawait handle.select_option(value=[\"red\", \"green\", \"blue\"])\n```\n\n```python sync\n# single selection matching the value\nhandle.select_option(\"blue\")\n# single selection matching both the label\nhandle.select_option(label=\"blue\")\n# multiple selection\nhandle.select_option(value=[\"red\", \"green\", \"blue\"])\n```\n\n```python sync\n# single selection matching the value\nhandle.select_option(\"blue\")\n# single selection matching both the value and the label\nhandle.select_option(label=\"blue\")\n# multiple selection\nhandle.select_option(\"red\", \"green\", \"blue\")\n# multiple selection for blue, red and second option\nhandle.select_option(value=\"blue\", { index: 2 }, \"red\")\n```\n"]
    fn select_option(
        &self,
        #[doc = "Options to select. If the `<select>` has the `multiple` attribute, all matching options are selected, otherwise only the\nfirst option matching one of the passed options is selected. String values are equivalent to `{value:'string'}`. Option\nis considered matching if all specified properties match."]
        values: Option<NotImplementedYet>,
        #[doc = "Options to select. If the `<select>` has the `multiple` attribute, all matching options are selected, otherwise only the\nfirst option matching one of the passed options is selected. String values are equivalent to `{value:'string'}`. Option\nis considered matching if all specified properties match."]
        values: Vec<NotImplementedYet>,
        #[doc = "Option elements to select. Optional."] element: Option<NotImplementedYet>,
        #[doc = "Options to select by index. Optional."] index: Option<NotImplementedYet>,
        #[doc = "Options to select by value. If the `<select>` has the `multiple` attribute, all given options are selected, otherwise\nonly the first option matching one of the passed options is selected. Optional."]
        value: Option<NotImplementedYet>,
        #[doc = "Options to select by label. If the `<select>` has the `multiple` attribute, all given options are selected, otherwise\nonly the first option matching one of the passed options is selected. Optional."]
        label: Option<NotImplementedYet>,
        #[doc = "options"]
        #[doc = "Actions that initiate navigations are waiting for these navigations to happen and for pages to start loading. You can\nopt out of waiting via setting this flag. You would only need this option in the exceptional cases such as navigating to\ninaccessible pages. Defaults to `false`."]
        no_wait_after: Option<bool>,
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>
    ) -> Result<Vec<String>, Arc<Error>> {
        todo!()
    }
    #[doc = "This method waits for [actionability](https://playwright.dev/docs/actionability/) checks, then focuses the element and selects all its text\ncontent."]
    fn select_text(
        &self,
        #[doc = "options"]
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "This method expects `elementHandle` to point to an\n[input element](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input).\n\nSets the value of the file input to these file paths or files. If some of the `filePaths` are relative paths, then they\nare resolved relative to the the current working directory. For empty array, clears the selected files."]
    fn set_input_files(
        &self,
        #[doc = ""] files: NotImplementedYet,
        #[doc = "options"]
        #[doc = "Actions that initiate navigations are waiting for these navigations to happen and for pages to start loading. You can\nopt out of waiting via setting this flag. You would only need this option in the exceptional cases such as navigating to\ninaccessible pages. Defaults to `false`."]
        no_wait_after: Option<bool>,
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "This method taps the element by performing the following steps:\n1. Wait for [actionability](https://playwright.dev/docs/actionability/) checks on the element, unless `force` option is set.\n1. Scroll the element into view if needed.\n1. Use [`property: Page.touchscreen`] to tap the center of the element, or the specified `position`.\n1. Wait for initiated navigations to either succeed or fail, unless `noWaitAfter` option is set.\n\nIf the element is detached from the DOM at any moment during the action, this method throws.\n\nWhen all steps combined have not finished during the specified `timeout`, this method throws a `TimeoutError`. Passing\nzero timeout disables this.\n\n> NOTE: `elementHandle.tap()` requires that the `hasTouch` option of the browser context be set to true."]
    fn tap(
        &self,
        #[doc = "options"]
        #[doc = "Whether to bypass the [actionability](https://playwright.dev/docs/actionability/) checks. Defaults to `false`."]
        force: Option<bool>,
        #[doc = "Modifier keys to press. Ensures that only these modifiers are pressed during the operation, and then restores current\nmodifiers back. If not specified, currently pressed modifiers are used."]
        modifiers: Option<Vec<KeyboardModifier>>,
        #[doc = "Actions that initiate navigations are waiting for these navigations to happen and for pages to start loading. You can\nopt out of waiting via setting this flag. You would only need this option in the exceptional cases such as navigating to\ninaccessible pages. Defaults to `false`."]
        no_wait_after: Option<bool>,
        #[doc = "A point to use relative to the top-left corner of element padding box. If not specified, uses some visible point of the\nelement."]
        position: Option<NotImplementedYet>,
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>,
        #[doc = "When set, this method only performs the [actionability](https://playwright.dev/docs/actionability/) checks and skips the action. Defaults to\n`false`. Useful to wait until the element is ready for the action without performing it."]
        trial: Option<bool>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Returns the `node.textContent`."]
    fn text_content(&self) -> Result<Option<String>, Arc<Error>> { todo!() }
    #[doc = "Focuses the element, and then sends a `keydown`, `keypress`/`input`, and `keyup` event for each character in the text.\n\nTo press a special key, like `Control` or `ArrowDown`, use [`method: ElementHandle.press`].\n\n```js\nawait elementHandle.type('Hello'); // Types instantly\nawait elementHandle.type('World', {delay: 100}); // Types slower, like a user\n```\n\n```java\nelementHandle.type(\"Hello\"); // Types instantly\nelementHandle.type(\"World\", new ElementHandle.TypeOptions().setDelay(100)); // Types slower, like a user\n```\n\n```python async\nawait element_handle.type(\"hello\") # types instantly\nawait element_handle.type(\"world\", delay=100) # types slower, like a user\n```\n\n```python sync\nelement_handle.type(\"hello\") # types instantly\nelement_handle.type(\"world\", delay=100) # types slower, like a user\n```\n\nAn example of typing into a text field and then submitting the form:\n\n```js\nconst elementHandle = await page.$('input');\nawait elementHandle.type('some text');\nawait elementHandle.press('Enter');\n```\n\n```java\nElementHandle elementHandle = page.querySelector(\"input\");\nelementHandle.type(\"some text\");\nelementHandle.press(\"Enter\");\n```\n\n```python async\nelement_handle = await page.query_selector(\"input\")\nawait element_handle.type(\"some text\")\nawait element_handle.press(\"Enter\")\n```\n\n```python sync\nelement_handle = page.query_selector(\"input\")\nelement_handle.type(\"some text\")\nelement_handle.press(\"Enter\")\n```\n"]
    fn r#type(
        &self,
        #[doc = "A text to type into a focused element."] text: String,
        #[doc = "options"]
        #[doc = "Time to wait between key presses in milliseconds. Defaults to 0."]
        delay: Option<f64>,
        #[doc = "Actions that initiate navigations are waiting for these navigations to happen and for pages to start loading. You can\nopt out of waiting via setting this flag. You would only need this option in the exceptional cases such as navigating to\ninaccessible pages. Defaults to `false`."]
        no_wait_after: Option<bool>,
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "This method checks the element by performing the following steps:\n1. Ensure that element is a checkbox or a radio input. If not, this method throws. If the element is already\n   unchecked, this method returns immediately.\n1. Wait for [actionability](https://playwright.dev/docs/actionability/) checks on the element, unless `force` option is set.\n1. Scroll the element into view if needed.\n1. Use [`property: Page.mouse`] to click in the center of the element.\n1. Wait for initiated navigations to either succeed or fail, unless `noWaitAfter` option is set.\n1. Ensure that the element is now unchecked. If not, this method throws.\n\nIf the element is detached from the DOM at any moment during the action, this method throws.\n\nWhen all steps combined have not finished during the specified `timeout`, this method throws a `TimeoutError`. Passing\nzero timeout disables this."]
    fn uncheck(
        &self,
        #[doc = "options"]
        #[doc = "Whether to bypass the [actionability](https://playwright.dev/docs/actionability/) checks. Defaults to `false`."]
        force: Option<bool>,
        #[doc = "Actions that initiate navigations are waiting for these navigations to happen and for pages to start loading. You can\nopt out of waiting via setting this flag. You would only need this option in the exceptional cases such as navigating to\ninaccessible pages. Defaults to `false`."]
        no_wait_after: Option<bool>,
        #[doc = "A point to use relative to the top-left corner of element padding box. If not specified, uses some visible point of the\nelement."]
        position: Option<NotImplementedYet>,
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>,
        #[doc = "When set, this method only performs the [actionability](https://playwright.dev/docs/actionability/) checks and skips the action. Defaults to\n`false`. Useful to wait until the element is ready for the action without performing it."]
        trial: Option<bool>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Returns when the element satisfies the `state`.\n\nDepending on the `state` parameter, this method waits for one of the [actionability](https://playwright.dev/docs/actionability/) checks to pass.\nThis method throws when the element is detached while waiting, unless waiting for the `\"hidden\"` state.\n- `\"visible\"` Wait until the element is [visible](./actionability.md#visible).\n- `\"hidden\"` Wait until the element is [not visible](./actionability.md#visible) or\n  [not attached](./actionability.md#attached). Note that waiting for hidden does not throw when the element detaches.\n- `\"stable\"` Wait until the element is both [visible](./actionability.md#visible) and\n  [stable](./actionability.md#stable).\n- `\"enabled\"` Wait until the element is [enabled](./actionability.md#enabled).\n- `\"disabled\"` Wait until the element is [not enabled](./actionability.md#enabled).\n- `\"editable\"` Wait until the element is [editable](./actionability.md#editable).\n\nIf the element does not satisfy the condition for the `timeout` milliseconds, this method will throw."]
    fn wait_for_element_state(
        &self,
        #[doc = "A state to wait for, see below for more details."] state: ElementState,
        #[doc = "options"]
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Returns element specified by selector when it satisfies `state` option. Returns `null` if waiting for `hidden` or\n`detached`.\n\nWait for the `selector` relative to the element handle to satisfy `state` option (either appear/disappear from dom, or\nbecome visible/hidden). If at the moment of calling the method `selector` already satisfies the condition, the method\nwill return immediately. If the selector doesn't satisfy the condition for the `timeout` milliseconds, the function will\nthrow.\n\n```js\nawait page.setContent(`<div><span></span></div>`);\nconst div = await page.$('div');\n// Waiting for the 'span' selector relative to the div.\nconst span = await div.waitForSelector('span', { state: 'attached' });\n```\n\n```java\npage.setContent(\"<div><span></span></div>\");\nElementHandle div = page.querySelector(\"div\");\n// Waiting for the \"span\" selector relative to the div.\nElementHandle span = div.waitForSelector(\"span\", new ElementHandle.WaitForSelectorOptions()\n  .setState(WaitForSelectorState.ATTACHED));\n```\n\n```python async\nawait page.set_content(\"<div><span></span></div>\")\ndiv = await page.query_selector(\"div\")\n# waiting for the \"span\" selector relative to the div.\nspan = await div.wait_for_selector(\"span\", state=\"attached\")\n```\n\n```python sync\npage.set_content(\"<div><span></span></div>\")\ndiv = page.query_selector(\"div\")\n# waiting for the \"span\" selector relative to the div.\nspan = div.wait_for_selector(\"span\", state=\"attached\")\n```\n\n> NOTE: This method does not work across navigations, use [`method: Page.waitForSelector`] instead."]
    fn wait_for_selector(
        &self,
        #[doc = "A selector to query for. See [working with selectors](./selectors.md) for more details."]
        selector: String,
        #[doc = "options"]
        #[doc = "Defaults to `'visible'`. Can be either:\n- `'attached'` - wait for element to be present in DOM.\n- `'detached'` - wait for element to not be present in DOM.\n- `'visible'` - wait for element to have non-empty bounding box and no `visibility:hidden`. Note that element without\n  any content or with `display:none` has an empty bounding box and is not considered visible.\n- `'hidden'` - wait for element to be either detached from DOM, or have an empty bounding box or `visibility:hidden`.\n  This is opposite to the `'visible'` option."]
        state: Option<WaitForSelectorState>,
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>
    ) -> Result<Option<ElementHandle>, Arc<Error>> {
        todo!()
    }
}
enum NotImplementedYetvalues {
    NotImplementedYet(String),
    NotImplementedYet(ElementHandle),
    NotImplementedYet(Vec<String>),
    NotImplementedYet(NotImplementedYet),
    NotImplementedYet(Vec<ElementHandle>),
    NotImplementedYet(Vec<NotImplementedYet>)
}
struct NotImplementedYet {
    #[doc = "Matches by `option.value`. Optional."]
    value: Option<String>,
    #[doc = "Matches by `option.label`. Optional."]
    label: Option<String>,
    #[doc = "Matches by the index. Optional."]
    index: Option<i64>
}
enum NotImplementedYetelement {
    NotImplementedYet(ElementHandle),
    NotImplementedYet(Vec<ElementHandle>)
}
enum NotImplementedYetindex {
    NotImplementedYet(i64),
    NotImplementedYet(Vec<i64>)
}
enum NotImplementedYetvalue {
    NotImplementedYet(String),
    NotImplementedYet(Vec<String>)
}
enum NotImplementedYetlabel {
    NotImplementedYet(String),
    NotImplementedYet(Vec<String>)
}
enum NotImplementedYetfiles {
    NotImplementedYet(path),
    NotImplementedYet(Vec<path>),
    NotImplementedYet(NotImplementedYet),
    NotImplementedYet(Vec<NotImplementedYet>)
}
struct NotImplementedYet {
    #[doc = "File name"]
    name: String,
    #[doc = "File type"]
    mime_type: String,
    #[doc = "File content"]
    buffer: Buffer
}
enum NotImplementedYetstate {
    NotImplementedYet(visible),
    NotImplementedYet(hidden),
    NotImplementedYet(stable),
    NotImplementedYet(enabled),
    NotImplementedYet(disabled),
    NotImplementedYet(editable)
}
#[doc = "- extends: [Exception]\n\nError is raised whenever certain operations are terminated abnormally, e.g. browser closes while\n[`method: Page.evaluate`] is running. All Playwright exceptions inherit from this class."]
#[doc = "Extends Exception"]
impl Error {
    #[doc = "Message of the error."]
    pub fn message(&self) -> str {}
    #[doc = "Name of the error which got thrown inside the browser. Optional."]
    pub fn name(&self) -> str {}
    #[doc = "Stack of the error which got thrown inside the browser. Optional."]
    pub fn stack(&self) -> str {}
}
#[doc = "`FileChooser` objects are dispatched by the page in the [`event: Page.fileChooser`] event.\n\n```js\nconst [fileChooser] = await Promise.all([\n  page.waitForEvent('filechooser'),\n  page.click('upload')\n]);\nawait fileChooser.setFiles('myfile.pdf');\n```\n\n```java\nFileChooser fileChooser = page.waitForFileChooser(() -> page.click(\"upload\"));\nfileChooser.setFiles(Paths.get(\"myfile.pdf\"));\n```\n\n```python async\nasync with page.expect_file_chooser() as fc_info:\n    await page.click(\"upload\")\nfile_chooser = await fc_info.value\nawait file_chooser.set_files(\"myfile.pdf\")\n```\n\n```python sync\nwith page.expect_file_chooser() as fc_info:\n    page.click(\"upload\")\nfile_chooser = fc_info.value\nfile_chooser.set_files(\"myfile.pdf\")\n```\n"]
impl FileChooser {
    #[doc = "Returns input element associated with this file chooser."]
    fn element(&self) -> Result<ElementHandle, Error> { todo!() }
    #[doc = "Returns whether this file chooser accepts multiple files."]
    fn is_multiple(&self) -> Result<bool, Error> { todo!() }
    #[doc = "Returns page this file chooser belongs to."]
    fn page(&self) -> Result<Page, Error> { todo!() }
    #[doc = "Sets the value of the file input this chooser is associated with. If some of the `filePaths` are relative paths, then\nthey are resolved relative to the the current working directory. For empty array, clears the selected files."]
    fn set_files(
        &self,
        #[doc = ""] files: NotImplementedYet,
        #[doc = "options"]
        #[doc = "Actions that initiate navigations are waiting for these navigations to happen and for pages to start loading. You can\nopt out of waiting via setting this flag. You would only need this option in the exceptional cases such as navigating to\ninaccessible pages. Defaults to `false`."]
        no_wait_after: Option<bool>,
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
}
enum NotImplementedYetfiles {
    NotImplementedYet(path),
    NotImplementedYet(Vec<path>),
    NotImplementedYet(NotImplementedYet),
    NotImplementedYet(Vec<NotImplementedYet>)
}
struct NotImplementedYet {
    #[doc = "File name"]
    name: String,
    #[doc = "File type"]
    mime_type: String,
    #[doc = "File content"]
    buffer: Buffer
}
#[doc = "At every point of time, page exposes its current frame tree via the [`method: Page.mainFrame`] and\n[`method: Frame.childFrames`] methods.\n\n`Frame` object's lifecycle is controlled by three events, dispatched on the page object:\n- [`event: Page.frameAttached`] - fired when the frame gets attached to the page. A Frame can be attached to the page\n  only once.\n- [`event: Page.frameNavigated`] - fired when the frame commits navigation to a different URL.\n- [`event: Page.frameDetached`] - fired when the frame gets detached from the page.  A Frame can be detached from the\n  page only once.\n\nAn example of dumping frame tree:\n\n```js\nconst { firefox } = require('playwright');  // Or 'chromium' or 'webkit'.\n\n(async () => {\n  const browser = await firefox.launch();\n  const page = await browser.newPage();\n  await page.goto('https://www.google.com/chrome/browser/canary.html');\n  dumpFrameTree(page.mainFrame(), '');\n  await browser.close();\n\n  function dumpFrameTree(frame, indent) {\n    console.log(indent + frame.url());\n    for (const child of frame.childFrames()) {\n      dumpFrameTree(child, indent + '  ');\n    }\n  }\n})();\n```\n\n```java\nimport com.microsoft.playwright.*;\n\npublic class Example {\n  public static void main(String[] args) {\n    try (Playwright playwright = Playwright.create()) {\n      BrowserType firefox = playwright.firefox();\n      Browser browser = firefox.launch();\n      Page page = browser.newPage();\n      page.navigate(\"https://www.google.com/chrome/browser/canary.html\");\n      dumpFrameTree(page.mainFrame(), \"\");\n      browser.close();\n    }\n  }\n  static void dumpFrameTree(Frame frame, String indent) {\n    System.out.println(indent + frame.url());\n    for (Frame child : frame.childFrames()) {\n      dumpFrameTree(child, indent + \"  \");\n    }\n  }\n}\n```\n\n```python async\nimport asyncio\nfrom playwright.async_api import async_playwright\n\nasync def run(playwright):\n    firefox = playwright.firefox\n    browser = await firefox.launch()\n    page = await browser.new_page()\n    await page.goto(\"https://www.theverge.com\")\n    dump_frame_tree(page.main_frame, \"\")\n    await browser.close()\n\ndef dump_frame_tree(frame, indent):\n    print(indent + frame.name + '@' + frame.url)\n    for child in frame.child_frames:\n        dump_frame_tree(child, indent + \"    \")\n\nasync def main():\n    async with async_playwright() as playwright:\n        await run(playwright)\nasyncio.run(main())\n```\n\n```python sync\nfrom playwright.sync_api import sync_playwright\n\ndef run(playwright):\n    firefox = playwright.firefox\n    browser = firefox.launch()\n    page = browser.new_page()\n    page.goto(\"https://www.theverge.com\")\n    dump_frame_tree(page.main_frame, \"\")\n    browser.close()\n\ndef dump_frame_tree(frame, indent):\n    print(indent + frame.name + '@' + frame.url)\n    for child in frame.child_frames:\n        dump_frame_tree(child, indent + \"    \")\n\nwith sync_playwright() as playwright:\n    run(playwright)\n```\n"]
impl Frame {
    #[doc = "Returns the added tag when the script's onload fires or when the script content was injected into frame.\n\nAdds a `<script>` tag into the page with the desired url or content."]
    fn add_script_tag(
        &self,
        #[doc = "options"]
        #[doc = "Raw JavaScript content to be injected into frame."]
        content: Option<String>,
        #[doc = "Path to the JavaScript file to be injected into frame. If `path` is a relative path, then it is resolved relative to the\ncurrent working directory."]
        path: Option<path>,
        #[doc = "Script type. Use 'module' in order to load a Javascript ES6 module. See\n[script](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/script) for more details."]
        r#type: Option<String>,
        #[doc = "URL of a script to be added."] url: Option<String>
    ) -> Result<ElementHandle, Arc<Error>> {
        todo!()
    }
    #[doc = "Returns the added tag when the stylesheet's onload fires or when the CSS content was injected into frame.\n\nAdds a `<link rel=\"stylesheet\">` tag into the page with the desired url or a `<style type=\"text/css\">` tag with the\ncontent."]
    fn add_style_tag(
        &self,
        #[doc = "options"]
        #[doc = "Raw CSS content to be injected into frame."]
        content: Option<String>,
        #[doc = "Path to the CSS file to be injected into frame. If `path` is a relative path, then it is resolved relative to the\ncurrent working directory."]
        path: Option<path>,
        #[doc = "URL of the `<link>` tag."] url: Option<String>
    ) -> Result<ElementHandle, Arc<Error>> {
        todo!()
    }
    #[doc = "This method checks an element matching `selector` by performing the following steps:\n1. Find an element matching `selector`. If there is none, wait until a matching element is attached to the DOM.\n1. Ensure that matched element is a checkbox or a radio input. If not, this method throws. If the element is already\n   checked, this method returns immediately.\n1. Wait for [actionability](https://playwright.dev/docs/actionability/) checks on the matched element, unless `force` option is set. If the\n   element is detached during the checks, the whole action is retried.\n1. Scroll the element into view if needed.\n1. Use [`property: Page.mouse`] to click in the center of the element.\n1. Wait for initiated navigations to either succeed or fail, unless `noWaitAfter` option is set.\n1. Ensure that the element is now checked. If not, this method throws.\n\nWhen all steps combined have not finished during the specified `timeout`, this method throws a `TimeoutError`. Passing\nzero timeout disables this."]
    fn check(
        &self,
        #[doc = "A selector to search for element. If there are multiple elements satisfying the selector, the first will be used. See\n[working with selectors](./selectors.md) for more details."]
        selector: String,
        #[doc = "options"]
        #[doc = "Whether to bypass the [actionability](https://playwright.dev/docs/actionability/) checks. Defaults to `false`."]
        force: Option<bool>,
        #[doc = "Actions that initiate navigations are waiting for these navigations to happen and for pages to start loading. You can\nopt out of waiting via setting this flag. You would only need this option in the exceptional cases such as navigating to\ninaccessible pages. Defaults to `false`."]
        no_wait_after: Option<bool>,
        #[doc = "A point to use relative to the top-left corner of element padding box. If not specified, uses some visible point of the\nelement."]
        position: Option<NotImplementedYet>,
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>,
        #[doc = "When set, this method only performs the [actionability](https://playwright.dev/docs/actionability/) checks and skips the action. Defaults to\n`false`. Useful to wait until the element is ready for the action without performing it."]
        trial: Option<bool>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = ""]
    fn child_frames(&self) -> Result<Vec<Frame>, Error> { todo!() }
    #[doc = "This method clicks an element matching `selector` by performing the following steps:\n1. Find an element matching `selector`. If there is none, wait until a matching element is attached to the DOM.\n1. Wait for [actionability](https://playwright.dev/docs/actionability/) checks on the matched element, unless `force` option is set. If the\n   element is detached during the checks, the whole action is retried.\n1. Scroll the element into view if needed.\n1. Use [`property: Page.mouse`] to click in the center of the element, or the specified `position`.\n1. Wait for initiated navigations to either succeed or fail, unless `noWaitAfter` option is set.\n\nWhen all steps combined have not finished during the specified `timeout`, this method throws a `TimeoutError`. Passing\nzero timeout disables this."]
    fn click(
        &self,
        #[doc = "A selector to search for element. If there are multiple elements satisfying the selector, the first will be used. See\n[working with selectors](./selectors.md) for more details."]
        selector: String,
        #[doc = "options"]
        #[doc = "Defaults to `left`."]
        button: Option<MouseButton>,
        #[doc = "defaults to 1. See [UIEvent.detail]."] click_count: Option<i64>,
        #[doc = "Time to wait between `mousedown` and `mouseup` in milliseconds. Defaults to 0."]
        delay: Option<f64>,
        #[doc = "Whether to bypass the [actionability](https://playwright.dev/docs/actionability/) checks. Defaults to `false`."]
        force: Option<bool>,
        #[doc = "Modifier keys to press. Ensures that only these modifiers are pressed during the operation, and then restores current\nmodifiers back. If not specified, currently pressed modifiers are used."]
        modifiers: Option<Vec<KeyboardModifier>>,
        #[doc = "Actions that initiate navigations are waiting for these navigations to happen and for pages to start loading. You can\nopt out of waiting via setting this flag. You would only need this option in the exceptional cases such as navigating to\ninaccessible pages. Defaults to `false`."]
        no_wait_after: Option<bool>,
        #[doc = "A point to use relative to the top-left corner of element padding box. If not specified, uses some visible point of the\nelement."]
        position: Option<NotImplementedYet>,
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>,
        #[doc = "When set, this method only performs the [actionability](https://playwright.dev/docs/actionability/) checks and skips the action. Defaults to\n`false`. Useful to wait until the element is ready for the action without performing it."]
        trial: Option<bool>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Gets the full HTML contents of the frame, including the doctype."]
    fn content(&self) -> Result<String, Arc<Error>> { todo!() }
    #[doc = "This method double clicks an element matching `selector` by performing the following steps:\n1. Find an element matching `selector`. If there is none, wait until a matching element is attached to the DOM.\n1. Wait for [actionability](https://playwright.dev/docs/actionability/) checks on the matched element, unless `force` option is set. If the\n   element is detached during the checks, the whole action is retried.\n1. Scroll the element into view if needed.\n1. Use [`property: Page.mouse`] to double click in the center of the element, or the specified `position`.\n1. Wait for initiated navigations to either succeed or fail, unless `noWaitAfter` option is set. Note that if the\n   first click of the `dblclick()` triggers a navigation event, this method will throw.\n\nWhen all steps combined have not finished during the specified `timeout`, this method throws a `TimeoutError`. Passing\nzero timeout disables this.\n\n> NOTE: `frame.dblclick()` dispatches two `click` events and a single `dblclick` event."]
    fn dblclick(
        &self,
        #[doc = "A selector to search for element. If there are multiple elements satisfying the selector, the first will be used. See\n[working with selectors](./selectors.md) for more details."]
        selector: String,
        #[doc = "options"]
        #[doc = "Defaults to `left`."]
        button: Option<MouseButton>,
        #[doc = "Time to wait between `mousedown` and `mouseup` in milliseconds. Defaults to 0."]
        delay: Option<f64>,
        #[doc = "Whether to bypass the [actionability](https://playwright.dev/docs/actionability/) checks. Defaults to `false`."]
        force: Option<bool>,
        #[doc = "Modifier keys to press. Ensures that only these modifiers are pressed during the operation, and then restores current\nmodifiers back. If not specified, currently pressed modifiers are used."]
        modifiers: Option<Vec<KeyboardModifier>>,
        #[doc = "Actions that initiate navigations are waiting for these navigations to happen and for pages to start loading. You can\nopt out of waiting via setting this flag. You would only need this option in the exceptional cases such as navigating to\ninaccessible pages. Defaults to `false`."]
        no_wait_after: Option<bool>,
        #[doc = "A point to use relative to the top-left corner of element padding box. If not specified, uses some visible point of the\nelement."]
        position: Option<NotImplementedYet>,
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>,
        #[doc = "When set, this method only performs the [actionability](https://playwright.dev/docs/actionability/) checks and skips the action. Defaults to\n`false`. Useful to wait until the element is ready for the action without performing it."]
        trial: Option<bool>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "The snippet below dispatches the `click` event on the element. Regardless of the visibility state of the element,\n`click` is dispatched. This is equivalent to calling\n[element.click()](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/click).\n\n```js\nawait frame.dispatchEvent('button#submit', 'click');\n```\n\n```java\nframe.dispatchEvent(\"button#submit\", \"click\");\n```\n\n```python async\nawait frame.dispatch_event(\"button#submit\", \"click\")\n```\n\n```python sync\nframe.dispatch_event(\"button#submit\", \"click\")\n```\n\nUnder the hood, it creates an instance of an event based on the given `type`, initializes it with `eventInit` properties\nand dispatches it on the element. Events are `composed`, `cancelable` and bubble by default.\n\nSince `eventInit` is event-specific, please refer to the events documentation for the lists of initial properties:\n- [DragEvent](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/DragEvent)\n- [FocusEvent](https://developer.mozilla.org/en-US/docs/Web/API/FocusEvent/FocusEvent)\n- [KeyboardEvent](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/KeyboardEvent)\n- [MouseEvent](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/MouseEvent)\n- [PointerEvent](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/PointerEvent)\n- [TouchEvent](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/TouchEvent)\n- [Event](https://developer.mozilla.org/en-US/docs/Web/API/Event/Event)\n\nYou can also specify `JSHandle` as the property value if you want live objects to be passed into the event:\n\n```js\n// Note you can only create DataTransfer in Chromium and Firefox\nconst dataTransfer = await frame.evaluateHandle(() => new DataTransfer());\nawait frame.dispatchEvent('#source', 'dragstart', { dataTransfer });\n```\n\n```java\n// Note you can only create DataTransfer in Chromium and Firefox\nJSHandle dataTransfer = frame.evaluateHandle(\"() => new DataTransfer()\");\nMap<String, Object> arg = new HashMap<>();\narg.put(\"dataTransfer\", dataTransfer);\nframe.dispatchEvent(\"#source\", \"dragstart\", arg);\n```\n\n```python async\n# note you can only create data_transfer in chromium and firefox\ndata_transfer = await frame.evaluate_handle(\"new DataTransfer()\")\nawait frame.dispatch_event(\"#source\", \"dragstart\", { \"dataTransfer\": data_transfer })\n```\n\n```python sync\n# note you can only create data_transfer in chromium and firefox\ndata_transfer = frame.evaluate_handle(\"new DataTransfer()\")\nframe.dispatch_event(\"#source\", \"dragstart\", { \"dataTransfer\": data_transfer })\n```\n"]
    fn dispatch_event(
        &self,
        #[doc = "A selector to search for element. If there are multiple elements satisfying the selector, the first will be used. See\n[working with selectors](./selectors.md) for more details."]
        selector: String,
        #[doc = "DOM event type: `\"click\"`, `\"dragstart\"`, etc."] r#type: String,
        #[doc = "Optional event-specific initialization properties."] event_init: Option<
            EvaluationArgument
        >,
        #[doc = "options"]
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Returns the return value of `expression`.\n\nThe method finds an element matching the specified selector within the frame and passes it as a first argument to\n`expression`. See [Working with selectors](./selectors.md) for more details. If no elements match the selector, the\nmethod throws an error.\n\nIf `expression` returns a [Promise], then [`method: Frame.evalOnSelector`] would wait for the promise to resolve and\nreturn its value.\n\nExamples:\n\n```js\nconst searchValue = await frame.$eval('#search', el => el.value);\nconst preloadHref = await frame.$eval('link[rel=preload]', el => el.href);\nconst html = await frame.$eval('.main-container', (e, suffix) => e.outerHTML + suffix, 'hello');\n```\n\n```java\nString searchValue = (String) frame.evalOnSelector(\"#search\", \"el => el.value\");\nString preloadHref = (String) frame.evalOnSelector(\"link[rel=preload]\", \"el => el.href\");\nString html = (String) frame.evalOnSelector(\".main-container\", \"(e, suffix) => e.outerHTML + suffix\", \"hello\");\n```\n\n```python async\nsearch_value = await frame.eval_on_selector(\"#search\", \"el => el.value\")\npreload_href = await frame.eval_on_selector(\"link[rel=preload]\", \"el => el.href\")\nhtml = await frame.eval_on_selector(\".main-container\", \"(e, suffix) => e.outerHTML + suffix\", \"hello\")\n```\n\n```python sync\nsearch_value = frame.eval_on_selector(\"#search\", \"el => el.value\")\npreload_href = frame.eval_on_selector(\"link[rel=preload]\", \"el => el.href\")\nhtml = frame.eval_on_selector(\".main-container\", \"(e, suffix) => e.outerHTML + suffix\", \"hello\")\n```\n"]
    fn eval_on_selector(
        &self,
        #[doc = "A selector to query for. See [working with selectors](./selectors.md) for more details."]
        selector: String,
        #[doc = "JavaScript expression to be evaluated in the browser context. If it looks like a function declaration, it is interpreted\nas a function. Otherwise, evaluated as an expression."]
        expression: String,
        #[doc = "Optional argument to pass to `expression`."] arg: Option<EvaluationArgument>
    ) -> Result<Serializable, Arc<Error>> {
        todo!()
    }
    #[doc = "Returns the return value of `expression`.\n\nThe method finds all elements matching the specified selector within the frame and passes an array of matched elements\nas a first argument to `expression`. See [Working with selectors](./selectors.md) for more details.\n\nIf `expression` returns a [Promise], then [`method: Frame.evalOnSelectorAll`] would wait for the promise to resolve and\nreturn its value.\n\nExamples:\n\n```js\nconst divsCounts = await frame.$$eval('div', (divs, min) => divs.length >= min, 10);\n```\n\n```java\nboolean divsCounts = (boolean) page.evalOnSelectorAll(\"div\", \"(divs, min) => divs.length >= min\", 10);\n```\n\n```python async\ndivs_counts = await frame.eval_on_selector_all(\"div\", \"(divs, min) => divs.length >= min\", 10)\n```\n\n```python sync\ndivs_counts = frame.eval_on_selector_all(\"div\", \"(divs, min) => divs.length >= min\", 10)\n```\n"]
    fn eval_on_selector_all(
        &self,
        #[doc = "A selector to query for. See [working with selectors](./selectors.md) for more details."]
        selector: String,
        #[doc = "JavaScript expression to be evaluated in the browser context. If it looks like a function declaration, it is interpreted\nas a function. Otherwise, evaluated as an expression."]
        expression: String,
        #[doc = "Optional argument to pass to `expression`."] arg: Option<EvaluationArgument>
    ) -> Result<Serializable, Arc<Error>> {
        todo!()
    }
    #[doc = "Returns the return value of `expression`.\n\nIf the function passed to the [`method: Frame.evaluate`] returns a [Promise], then [`method: Frame.evaluate`] would wait\nfor the promise to resolve and return its value.\n\nIf the function passed to the [`method: Frame.evaluate`] returns a non-[Serializable] value, then\n[`method: Frame.evaluate`] returns `undefined`. Playwright also supports transferring some additional values that are\nnot serializable by `JSON`: `-0`, `NaN`, `Infinity`, `-Infinity`.\n\n```js\nconst result = await frame.evaluate(([x, y]) => {\n  return Promise.resolve(x * y);\n}, [7, 8]);\nconsole.log(result); // prints \"56\"\n```\n\n```java\nObject result = frame.evaluate(\"([x, y]) => {\\n\" +\n  \"  return Promise.resolve(x * y);\\n\" +\n  \"}\", Arrays.asList(7, 8));\nSystem.out.println(result); // prints \"56\"\n```\n\n```python async\nresult = await frame.evaluate(\"([x, y]) => Promise.resolve(x * y)\", [7, 8])\nprint(result) # prints \"56\"\n```\n\n```python sync\nresult = frame.evaluate(\"([x, y]) => Promise.resolve(x * y)\", [7, 8])\nprint(result) # prints \"56\"\n```\n\nA string can also be passed in instead of a function.\n\n```js\nconsole.log(await frame.evaluate('1 + 2')); // prints \"3\"\n```\n\n```java\nSystem.out.println(frame.evaluate(\"1 + 2\")); // prints \"3\"\n```\n\n```python async\nprint(await frame.evaluate(\"1 + 2\")) # prints \"3\"\nx = 10\nprint(await frame.evaluate(f\"1 + {x}\")) # prints \"11\"\n```\n\n```python sync\nprint(frame.evaluate(\"1 + 2\")) # prints \"3\"\nx = 10\nprint(frame.evaluate(f\"1 + {x}\")) # prints \"11\"\n```\n\n`ElementHandle` instances can be passed as an argument to the [`method: Frame.evaluate`]:\n\n```js\nconst bodyHandle = await frame.$('body');\nconst html = await frame.evaluate(([body, suffix]) => body.innerHTML + suffix, [bodyHandle, 'hello']);\nawait bodyHandle.dispose();\n```\n\n```java\nElementHandle bodyHandle = frame.querySelector(\"body\");\nString html = (String) frame.evaluate(\"([body, suffix]) => body.innerHTML + suffix\", Arrays.asList(bodyHandle, \"hello\"));\nbodyHandle.dispose();\n```\n\n```python async\nbody_handle = await frame.query_selector(\"body\")\nhtml = await frame.evaluate(\"([body, suffix]) => body.innerHTML + suffix\", [body_handle, \"hello\"])\nawait body_handle.dispose()\n```\n\n```python sync\nbody_handle = frame.query_selector(\"body\")\nhtml = frame.evaluate(\"([body, suffix]) => body.innerHTML + suffix\", [body_handle, \"hello\"])\nbody_handle.dispose()\n```\n"]
    fn evaluate(
        &self,
        #[doc = "JavaScript expression to be evaluated in the browser context. If it looks like a function declaration, it is interpreted\nas a function. Otherwise, evaluated as an expression."]
        expression: String,
        #[doc = "Optional argument to pass to `expression`."] arg: Option<EvaluationArgument>
    ) -> Result<Serializable, Arc<Error>> {
        todo!()
    }
    #[doc = "Returns the return value of `expression` as a `JSHandle`.\n\nThe only difference between [`method: Frame.evaluate`] and [`method: Frame.evaluateHandle`] is that\n[`method: Frame.evaluateHandle`] returns `JSHandle`.\n\nIf the function, passed to the [`method: Frame.evaluateHandle`], returns a [Promise], then\n[`method: Frame.evaluateHandle`] would wait for the promise to resolve and return its value.\n\n```js\nconst aWindowHandle = await frame.evaluateHandle(() => Promise.resolve(window));\naWindowHandle; // Handle for the window object.\n```\n\n```java\n// Handle for the window object.\nJSHandle aWindowHandle = frame.evaluateHandle(\"() => Promise.resolve(window)\");\n```\n\n```python async\na_window_handle = await frame.evaluate_handle(\"Promise.resolve(window)\")\na_window_handle # handle for the window object.\n```\n\n```python sync\na_window_handle = frame.evaluate_handle(\"Promise.resolve(window)\")\na_window_handle # handle for the window object.\n```\n\nA string can also be passed in instead of a function.\n\n```js\nconst aHandle = await frame.evaluateHandle('document'); // Handle for the 'document'.\n```\n\n```java\nJSHandle aHandle = frame.evaluateHandle(\"document\"); // Handle for the \"document\".\n```\n\n```python async\na_handle = await page.evaluate_handle(\"document\") # handle for the \"document\"\n```\n\n```python sync\na_handle = page.evaluate_handle(\"document\") # handle for the \"document\"\n```\n\n`JSHandle` instances can be passed as an argument to the [`method: Frame.evaluateHandle`]:\n\n```js\nconst aHandle = await frame.evaluateHandle(() => document.body);\nconst resultHandle = await frame.evaluateHandle(([body, suffix]) => body.innerHTML + suffix, [aHandle, 'hello']);\nconsole.log(await resultHandle.jsonValue());\nawait resultHandle.dispose();\n```\n\n```java\nJSHandle aHandle = frame.evaluateHandle(\"() => document.body\");\nJSHandle resultHandle = frame.evaluateHandle(\"([body, suffix]) => body.innerHTML + suffix\", Arrays.asList(aHandle, \"hello\"));\nSystem.out.println(resultHandle.jsonValue());\nresultHandle.dispose();\n```\n\n```python async\na_handle = await page.evaluate_handle(\"document.body\")\nresult_handle = await page.evaluate_handle(\"body => body.innerHTML\", a_handle)\nprint(await result_handle.json_value())\nawait result_handle.dispose()\n```\n\n```python sync\na_handle = page.evaluate_handle(\"document.body\")\nresult_handle = page.evaluate_handle(\"body => body.innerHTML\", a_handle)\nprint(result_handle.json_value())\nresult_handle.dispose()\n```\n"]
    fn evaluate_handle(
        &self,
        #[doc = "JavaScript expression to be evaluated in the browser context. If it looks like a function declaration, it is interpreted\nas a function. Otherwise, evaluated as an expression."]
        expression: String,
        #[doc = "Optional argument to pass to `expression`."] arg: Option<EvaluationArgument>
    ) -> Result<JsHandle, Arc<Error>> {
        todo!()
    }
    #[doc = "This method waits for an element matching `selector`, waits for [actionability](https://playwright.dev/docs/actionability/) checks, focuses the\nelement, fills it and triggers an `input` event after filling. If the element is inside the `<label>` element that has\nassociated [control](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLabelElement/control), that control will be\nfilled instead. If the element to be filled is not an `<input>`, `<textarea>` or `[contenteditable]` element, this\nmethod throws an error. Note that you can pass an empty string to clear the input field.\n\nTo send fine-grained keyboard events, use [`method: Frame.type`]."]
    fn fill(
        &self,
        #[doc = "A selector to search for element. If there are multiple elements satisfying the selector, the first will be used. See\n[working with selectors](./selectors.md) for more details."]
        selector: String,
        #[doc = "Value to fill for the `<input>`, `<textarea>` or `[contenteditable]` element."]
        value: String,
        #[doc = "options"]
        #[doc = "Actions that initiate navigations are waiting for these navigations to happen and for pages to start loading. You can\nopt out of waiting via setting this flag. You would only need this option in the exceptional cases such as navigating to\ninaccessible pages. Defaults to `false`."]
        no_wait_after: Option<bool>,
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "This method fetches an element with `selector` and focuses it. If there's no element matching `selector`, the method\nwaits until a matching element appears in the DOM."]
    fn focus(
        &self,
        #[doc = "A selector to search for element. If there are multiple elements satisfying the selector, the first will be used. See\n[working with selectors](./selectors.md) for more details."]
        selector: String,
        #[doc = "options"]
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Returns the `frame` or `iframe` element handle which corresponds to this frame.\n\nThis is an inverse of [`method: ElementHandle.contentFrame`]. Note that returned handle actually belongs to the parent\nframe.\n\nThis method throws an error if the frame has been detached before `frameElement()` returns.\n\n```js\nconst frameElement = await frame.frameElement();\nconst contentFrame = await frameElement.contentFrame();\nconsole.log(frame === contentFrame);  // -> true\n```\n\n```java\nElementHandle frameElement = frame.frameElement();\nFrame contentFrame = frameElement.contentFrame();\nSystem.out.println(frame == contentFrame);  // -> true\n```\n\n```python async\nframe_element = await frame.frame_element()\ncontent_frame = await frame_element.content_frame()\nassert frame == content_frame\n```\n\n```python sync\nframe_element = frame.frame_element()\ncontent_frame = frame_element.content_frame()\nassert frame == content_frame\n```\n"]
    fn frame_element(&self) -> Result<ElementHandle, Arc<Error>> { todo!() }
    #[doc = "Returns element attribute value."]
    fn get_attribute(
        &self,
        #[doc = "A selector to search for element. If there are multiple elements satisfying the selector, the first will be used. See\n[working with selectors](./selectors.md) for more details."]
        selector: String,
        #[doc = "Attribute name to get the value for."] name: String,
        #[doc = "options"]
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>
    ) -> Result<Option<String>, Arc<Error>> {
        todo!()
    }
    #[doc = "Returns the main resource response. In case of multiple redirects, the navigation will resolve with the response of the\nlast redirect.\n\n`frame.goto` will throw an error if:\n- there's an SSL error (e.g. in case of self-signed certificates).\n- target URL is invalid.\n- the `timeout` is exceeded during navigation.\n- the remote server does not respond or is unreachable.\n- the main resource failed to load.\n\n`frame.goto` will not throw an error when any valid HTTP status code is returned by the remote server, including 404\n\"Not Found\" and 500 \"Internal Server Error\".  The status code for such responses can be retrieved by calling\n[`method: Response.status`].\n\n> NOTE: `frame.goto` either throws an error or returns a main resource response. The only exceptions are navigation to\n`about:blank` or navigation to the same URL with a different hash, which would succeed and return `null`.\n> NOTE: Headless mode doesn't support navigation to a PDF document. See the\n[upstream issue](https://bugs.chromium.org/p/chromium/issues/detail?id=761295)."]
    fn goto(
        &self,
        url : String,
        #[doc = "options"]
        #[doc = "Referer header value. If provided it will take preference over the referer header value set by\n[`method: Page.setExtraHTTPHeaders`]."]
        referer: Option<String>,
        #[doc = "Maximum operation time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be\nchanged by using the [`method: BrowserContext.setDefaultNavigationTimeout`],\n[`method: BrowserContext.setDefaultTimeout`], [`method: Page.setDefaultNavigationTimeout`] or\n[`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>,
        #[doc = "When to consider operation succeeded, defaults to `load`. Events can be either:\n- `'domcontentloaded'` - consider operation to be finished when the `DOMContentLoaded` event is fired.\n- `'load'` - consider operation to be finished when the `load` event is fired.\n- `'networkidle'` - consider operation to be finished when there are no network connections for at least `500` ms."]
        wait_until: Option<WaitUntilState>
    ) -> Result<Option<Response>, Arc<Error>> {
        todo!()
    }
    #[doc = "This method hovers over an element matching `selector` by performing the following steps:\n1. Find an element matching `selector`. If there is none, wait until a matching element is attached to the DOM.\n1. Wait for [actionability](https://playwright.dev/docs/actionability/) checks on the matched element, unless `force` option is set. If the\n   element is detached during the checks, the whole action is retried.\n1. Scroll the element into view if needed.\n1. Use [`property: Page.mouse`] to hover over the center of the element, or the specified `position`.\n1. Wait for initiated navigations to either succeed or fail, unless `noWaitAfter` option is set.\n\nWhen all steps combined have not finished during the specified `timeout`, this method throws a `TimeoutError`. Passing\nzero timeout disables this."]
    fn hover(
        &self,
        #[doc = "A selector to search for element. If there are multiple elements satisfying the selector, the first will be used. See\n[working with selectors](./selectors.md) for more details."]
        selector: String,
        #[doc = "options"]
        #[doc = "Whether to bypass the [actionability](https://playwright.dev/docs/actionability/) checks. Defaults to `false`."]
        force: Option<bool>,
        #[doc = "Modifier keys to press. Ensures that only these modifiers are pressed during the operation, and then restores current\nmodifiers back. If not specified, currently pressed modifiers are used."]
        modifiers: Option<Vec<KeyboardModifier>>,
        #[doc = "A point to use relative to the top-left corner of element padding box. If not specified, uses some visible point of the\nelement."]
        position: Option<NotImplementedYet>,
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>,
        #[doc = "When set, this method only performs the [actionability](https://playwright.dev/docs/actionability/) checks and skips the action. Defaults to\n`false`. Useful to wait until the element is ready for the action without performing it."]
        trial: Option<bool>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Returns `element.innerHTML`."]
    fn inner_h_t_m_l(
        &self,
        #[doc = "A selector to search for element. If there are multiple elements satisfying the selector, the first will be used. See\n[working with selectors](./selectors.md) for more details."]
        selector: String,
        #[doc = "options"]
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>
    ) -> Result<String, Arc<Error>> {
        todo!()
    }
    #[doc = "Returns `element.innerText`."]
    fn inner_text(
        &self,
        #[doc = "A selector to search for element. If there are multiple elements satisfying the selector, the first will be used. See\n[working with selectors](./selectors.md) for more details."]
        selector: String,
        #[doc = "options"]
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>
    ) -> Result<String, Arc<Error>> {
        todo!()
    }
    #[doc = "Returns whether the element is checked. Throws if the element is not a checkbox or radio input."]
    fn is_checked(
        &self,
        #[doc = "A selector to search for element. If there are multiple elements satisfying the selector, the first will be used. See\n[working with selectors](./selectors.md) for more details."]
        selector: String,
        #[doc = "options"]
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>
    ) -> Result<bool, Arc<Error>> {
        todo!()
    }
    #[doc = "Returns `true` if the frame has been detached, or `false` otherwise."]
    fn is_detached(&self) -> Result<bool, Error> { todo!() }
    #[doc = "Returns whether the element is disabled, the opposite of [enabled](./actionability.md#enabled)."]
    fn is_disabled(
        &self,
        #[doc = "A selector to search for element. If there are multiple elements satisfying the selector, the first will be used. See\n[working with selectors](./selectors.md) for more details."]
        selector: String,
        #[doc = "options"]
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>
    ) -> Result<bool, Arc<Error>> {
        todo!()
    }
    #[doc = "Returns whether the element is [editable](./actionability.md#editable)."]
    fn is_editable(
        &self,
        #[doc = "A selector to search for element. If there are multiple elements satisfying the selector, the first will be used. See\n[working with selectors](./selectors.md) for more details."]
        selector: String,
        #[doc = "options"]
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>
    ) -> Result<bool, Arc<Error>> {
        todo!()
    }
    #[doc = "Returns whether the element is [enabled](./actionability.md#enabled)."]
    fn is_enabled(
        &self,
        #[doc = "A selector to search for element. If there are multiple elements satisfying the selector, the first will be used. See\n[working with selectors](./selectors.md) for more details."]
        selector: String,
        #[doc = "options"]
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>
    ) -> Result<bool, Arc<Error>> {
        todo!()
    }
    #[doc = "Returns whether the element is hidden, the opposite of [visible](./actionability.md#visible).  `selector` that does not\nmatch any elements is considered hidden."]
    fn is_hidden(
        &self,
        #[doc = "A selector to search for element. If there are multiple elements satisfying the selector, the first will be used. See\n[working with selectors](./selectors.md) for more details."]
        selector: String,
        #[doc = "options"]
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>
    ) -> Result<bool, Arc<Error>> {
        todo!()
    }
    #[doc = "Returns whether the element is [visible](./actionability.md#visible). `selector` that does not match any elements is\nconsidered not visible."]
    fn is_visible(
        &self,
        #[doc = "A selector to search for element. If there are multiple elements satisfying the selector, the first will be used. See\n[working with selectors](./selectors.md) for more details."]
        selector: String,
        #[doc = "options"]
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>
    ) -> Result<bool, Arc<Error>> {
        todo!()
    }
    #[doc = "Returns frame's name attribute as specified in the tag.\n\nIf the name is empty, returns the id attribute instead.\n\n> NOTE: This value is calculated once when the frame is created, and will not update if the attribute is changed later."]
    fn name(&self) -> Result<String, Error> { todo!() }
    #[doc = "Returns the page containing this frame."]
    fn page(&self) -> Result<Page, Error> { todo!() }
    #[doc = "Parent frame, if any. Detached frames and main frames return `null`."]
    fn parent_frame(&self) -> Result<Option<Frame>, Error> { todo!() }
    #[doc = "`key` can specify the intended [keyboardEvent.key](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key)\nvalue or a single character to generate the text for. A superset of the `key` values can be found\n[here](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key/Key_Values). Examples of the keys are:\n\n`F1` - `F12`, `Digit0`- `Digit9`, `KeyA`- `KeyZ`, `Backquote`, `Minus`, `Equal`, `Backslash`, `Backspace`, `Tab`,\n`Delete`, `Escape`, `ArrowDown`, `End`, `Enter`, `Home`, `Insert`, `PageDown`, `PageUp`, `ArrowRight`, `ArrowUp`, etc.\n\nFollowing modification shortcuts are also supported: `Shift`, `Control`, `Alt`, `Meta`, `ShiftLeft`.\n\nHolding down `Shift` will type the text that corresponds to the `key` in the upper case.\n\nIf `key` is a single character, it is case-sensitive, so the values `a` and `A` will generate different respective\ntexts.\n\nShortcuts such as `key: \"Control+o\"` or `key: \"Control+Shift+T\"` are supported as well. When specified with the\nmodifier, modifier is pressed and being held while the subsequent key is being pressed."]
    fn press(
        &self,
        #[doc = "A selector to search for element. If there are multiple elements satisfying the selector, the first will be used. See\n[working with selectors](./selectors.md) for more details."]
        selector: String,
        #[doc = "Name of the key to press or a character to generate, such as `ArrowLeft` or `a`."]
        key: String,
        #[doc = "options"]
        #[doc = "Time to wait between `keydown` and `keyup` in milliseconds. Defaults to 0."]
        delay: Option<f64>,
        #[doc = "Actions that initiate navigations are waiting for these navigations to happen and for pages to start loading. You can\nopt out of waiting via setting this flag. You would only need this option in the exceptional cases such as navigating to\ninaccessible pages. Defaults to `false`."]
        no_wait_after: Option<bool>,
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Returns the ElementHandle pointing to the frame element.\n\nThe method finds an element matching the specified selector within the frame. See\n[Working with selectors](./selectors.md) for more details. If no elements match the selector, returns `null`."]
    fn query_selector(
        &self,
        #[doc = "A selector to query for. See [working with selectors](./selectors.md) for more details."]
        selector: String
    ) -> Result<Option<ElementHandle>, Arc<Error>> {
        todo!()
    }
    #[doc = "Returns the ElementHandles pointing to the frame elements.\n\nThe method finds all elements matching the specified selector within the frame. See\n[Working with selectors](./selectors.md) for more details. If no elements match the selector, returns empty array."]
    fn query_selector_all(
        &self,
        #[doc = "A selector to query for. See [working with selectors](./selectors.md) for more details."]
        selector: String
    ) -> Result<Vec<ElementHandle>, Arc<Error>> {
        todo!()
    }
    #[doc = "Returns the array of option values that have been successfully selected.\n\nTriggers a `change` and `input` event once all the provided options have been selected. If there's no `<select>` element\nmatching `selector`, the method throws an error.\n\nWill wait until all specified options are present in the `<select>` element.\n\n```js\n// single selection matching the value\nframe.selectOption('select#colors', 'blue');\n\n// single selection matching both the value and the label\nframe.selectOption('select#colors', { label: 'Blue' });\n\n// multiple selection\nframe.selectOption('select#colors', 'red', 'green', 'blue');\n```\n\n```java\n// single selection matching the value\nframe.selectOption(\"select#colors\", \"blue\");\n// single selection matching both the value and the label\nframe.selectOption(\"select#colors\", new SelectOption().setLabel(\"Blue\"));\n// multiple selection\nframe.selectOption(\"select#colors\", new String[] {\"red\", \"green\", \"blue\"});\n```\n\n```python async\n# single selection matching the value\nawait frame.select_option(\"select#colors\", \"blue\")\n# single selection matching the label\nawait frame.select_option(\"select#colors\", label=\"blue\")\n# multiple selection\nawait frame.select_option(\"select#colors\", value=[\"red\", \"green\", \"blue\"])\n```\n\n```python sync\n# single selection matching the value\nframe.select_option(\"select#colors\", \"blue\")\n# single selection matching both the label\nframe.select_option(\"select#colors\", label=\"blue\")\n# multiple selection\nframe.select_option(\"select#colors\", value=[\"red\", \"green\", \"blue\"])\n```\n"]
    fn select_option(
        &self,
        #[doc = "A selector to query for. See [working with selectors](./selectors.md) for more details."]
        selector: String,
        #[doc = "Options to select. If the `<select>` has the `multiple` attribute, all matching options are selected, otherwise only the\nfirst option matching one of the passed options is selected. String values are equivalent to `{value:'string'}`. Option\nis considered matching if all specified properties match."]
        values: Option<NotImplementedYet>,
        #[doc = "Options to select. If the `<select>` has the `multiple` attribute, all matching options are selected, otherwise only the\nfirst option matching one of the passed options is selected. String values are equivalent to `{value:'string'}`. Option\nis considered matching if all specified properties match."]
        values: Vec<NotImplementedYet>,
        #[doc = "Option elements to select. Optional."] element: Option<NotImplementedYet>,
        #[doc = "Options to select by index. Optional."] index: Option<NotImplementedYet>,
        #[doc = "Options to select by value. If the `<select>` has the `multiple` attribute, all given options are selected, otherwise\nonly the first option matching one of the passed options is selected. Optional."]
        value: Option<NotImplementedYet>,
        #[doc = "Options to select by label. If the `<select>` has the `multiple` attribute, all given options are selected, otherwise\nonly the first option matching one of the passed options is selected. Optional."]
        label: Option<NotImplementedYet>,
        #[doc = "options"]
        #[doc = "Actions that initiate navigations are waiting for these navigations to happen and for pages to start loading. You can\nopt out of waiting via setting this flag. You would only need this option in the exceptional cases such as navigating to\ninaccessible pages. Defaults to `false`."]
        no_wait_after: Option<bool>,
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>
    ) -> Result<Vec<String>, Arc<Error>> {
        todo!()
    }
    #[doc = ""]
    fn set_content(
        &self,
        #[doc = "HTML markup to assign to the page."] html: String,
        #[doc = "options"]
        #[doc = "Maximum operation time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be\nchanged by using the [`method: BrowserContext.setDefaultNavigationTimeout`],\n[`method: BrowserContext.setDefaultTimeout`], [`method: Page.setDefaultNavigationTimeout`] or\n[`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>,
        #[doc = "When to consider operation succeeded, defaults to `load`. Events can be either:\n- `'domcontentloaded'` - consider operation to be finished when the `DOMContentLoaded` event is fired.\n- `'load'` - consider operation to be finished when the `load` event is fired.\n- `'networkidle'` - consider operation to be finished when there are no network connections for at least `500` ms."]
        wait_until: Option<WaitUntilState>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "This method expects `selector` to point to an\n[input element](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input).\n\nSets the value of the file input to these file paths or files. If some of the `filePaths` are relative paths, then they\nare resolved relative to the the current working directory. For empty array, clears the selected files."]
    fn set_input_files(
        &self,
        #[doc = "A selector to search for element. If there are multiple elements satisfying the selector, the first will be used. See\n[working with selectors](./selectors.md) for more details."]
        selector: String,
        #[doc = ""] files: NotImplementedYet,
        #[doc = "options"]
        #[doc = "Actions that initiate navigations are waiting for these navigations to happen and for pages to start loading. You can\nopt out of waiting via setting this flag. You would only need this option in the exceptional cases such as navigating to\ninaccessible pages. Defaults to `false`."]
        no_wait_after: Option<bool>,
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "This method taps an element matching `selector` by performing the following steps:\n1. Find an element matching `selector`. If there is none, wait until a matching element is attached to the DOM.\n1. Wait for [actionability](https://playwright.dev/docs/actionability/) checks on the matched element, unless `force` option is set. If the\n   element is detached during the checks, the whole action is retried.\n1. Scroll the element into view if needed.\n1. Use [`property: Page.touchscreen`] to tap the center of the element, or the specified `position`.\n1. Wait for initiated navigations to either succeed or fail, unless `noWaitAfter` option is set.\n\nWhen all steps combined have not finished during the specified `timeout`, this method throws a `TimeoutError`. Passing\nzero timeout disables this.\n\n> NOTE: `frame.tap()` requires that the `hasTouch` option of the browser context be set to true."]
    fn tap(
        &self,
        #[doc = "A selector to search for element. If there are multiple elements satisfying the selector, the first will be used. See\n[working with selectors](./selectors.md) for more details."]
        selector: String,
        #[doc = "options"]
        #[doc = "Whether to bypass the [actionability](https://playwright.dev/docs/actionability/) checks. Defaults to `false`."]
        force: Option<bool>,
        #[doc = "Modifier keys to press. Ensures that only these modifiers are pressed during the operation, and then restores current\nmodifiers back. If not specified, currently pressed modifiers are used."]
        modifiers: Option<Vec<KeyboardModifier>>,
        #[doc = "Actions that initiate navigations are waiting for these navigations to happen and for pages to start loading. You can\nopt out of waiting via setting this flag. You would only need this option in the exceptional cases such as navigating to\ninaccessible pages. Defaults to `false`."]
        no_wait_after: Option<bool>,
        #[doc = "A point to use relative to the top-left corner of element padding box. If not specified, uses some visible point of the\nelement."]
        position: Option<NotImplementedYet>,
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>,
        #[doc = "When set, this method only performs the [actionability](https://playwright.dev/docs/actionability/) checks and skips the action. Defaults to\n`false`. Useful to wait until the element is ready for the action without performing it."]
        trial: Option<bool>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Returns `element.textContent`."]
    fn text_content(
        &self,
        #[doc = "A selector to search for element. If there are multiple elements satisfying the selector, the first will be used. See\n[working with selectors](./selectors.md) for more details."]
        selector: String,
        #[doc = "options"]
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>
    ) -> Result<Option<String>, Arc<Error>> {
        todo!()
    }
    #[doc = "Returns the page title."]
    fn title(&self) -> Result<String, Arc<Error>> { todo!() }
    #[doc = "Sends a `keydown`, `keypress`/`input`, and `keyup` event for each character in the text. `frame.type` can be used to\nsend fine-grained keyboard events. To fill values in form fields, use [`method: Frame.fill`].\n\nTo press a special key, like `Control` or `ArrowDown`, use [`method: Keyboard.press`].\n\n```js\nawait frame.type('#mytextarea', 'Hello'); // Types instantly\nawait frame.type('#mytextarea', 'World', {delay: 100}); // Types slower, like a user\n```\n\n```java\n// Types instantly\nframe.type(\"#mytextarea\", \"Hello\");\n// Types slower, like a user\nframe.type(\"#mytextarea\", \"World\", new Frame.TypeOptions().setDelay(100));\n```\n\n```python async\nawait frame.type(\"#mytextarea\", \"hello\") # types instantly\nawait frame.type(\"#mytextarea\", \"world\", delay=100) # types slower, like a user\n```\n\n```python sync\nframe.type(\"#mytextarea\", \"hello\") # types instantly\nframe.type(\"#mytextarea\", \"world\", delay=100) # types slower, like a user\n```\n"]
    fn r#type(
        &self,
        #[doc = "A selector to search for element. If there are multiple elements satisfying the selector, the first will be used. See\n[working with selectors](./selectors.md) for more details."]
        selector: String,
        #[doc = "A text to type into a focused element."] text: String,
        #[doc = "options"]
        #[doc = "Time to wait between key presses in milliseconds. Defaults to 0."]
        delay: Option<f64>,
        #[doc = "Actions that initiate navigations are waiting for these navigations to happen and for pages to start loading. You can\nopt out of waiting via setting this flag. You would only need this option in the exceptional cases such as navigating to\ninaccessible pages. Defaults to `false`."]
        no_wait_after: Option<bool>,
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "This method checks an element matching `selector` by performing the following steps:\n1. Find an element matching `selector`. If there is none, wait until a matching element is attached to the DOM.\n1. Ensure that matched element is a checkbox or a radio input. If not, this method throws. If the element is already\n   unchecked, this method returns immediately.\n1. Wait for [actionability](https://playwright.dev/docs/actionability/) checks on the matched element, unless `force` option is set. If the\n   element is detached during the checks, the whole action is retried.\n1. Scroll the element into view if needed.\n1. Use [`property: Page.mouse`] to click in the center of the element.\n1. Wait for initiated navigations to either succeed or fail, unless `noWaitAfter` option is set.\n1. Ensure that the element is now unchecked. If not, this method throws.\n\nWhen all steps combined have not finished during the specified `timeout`, this method throws a `TimeoutError`. Passing\nzero timeout disables this."]
    fn uncheck(
        &self,
        #[doc = "A selector to search for element. If there are multiple elements satisfying the selector, the first will be used. See\n[working with selectors](./selectors.md) for more details."]
        selector: String,
        #[doc = "options"]
        #[doc = "Whether to bypass the [actionability](https://playwright.dev/docs/actionability/) checks. Defaults to `false`."]
        force: Option<bool>,
        #[doc = "Actions that initiate navigations are waiting for these navigations to happen and for pages to start loading. You can\nopt out of waiting via setting this flag. You would only need this option in the exceptional cases such as navigating to\ninaccessible pages. Defaults to `false`."]
        no_wait_after: Option<bool>,
        #[doc = "A point to use relative to the top-left corner of element padding box. If not specified, uses some visible point of the\nelement."]
        position: Option<NotImplementedYet>,
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>,
        #[doc = "When set, this method only performs the [actionability](https://playwright.dev/docs/actionability/) checks and skips the action. Defaults to\n`false`. Useful to wait until the element is ready for the action without performing it."]
        trial: Option<bool>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Returns frame's url."]
    fn url(&self) -> Result<String, Error> { todo!() }
    #[doc = "Returns when the `expression` returns a truthy value, returns that value.\n\nThe [`method: Frame.waitForFunction`] can be used to observe viewport size change:\n\n```js\nconst { firefox } = require('playwright');  // Or 'chromium' or 'webkit'.\n\n(async () => {\n  const browser = await firefox.launch();\n  const page = await browser.newPage();\n  const watchDog = page.mainFrame().waitForFunction('window.innerWidth < 100');\n  page.setViewportSize({width: 50, height: 50});\n  await watchDog;\n  await browser.close();\n})();\n```\n\n```java\nimport com.microsoft.playwright.*;\n\npublic class Example {\n  public static void main(String[] args) {\n    try (Playwright playwright = Playwright.create()) {\n      BrowserType firefox = playwright.firefox();\n      Browser browser = firefox.launch();\n      Page page = browser.newPage();\n      page.setViewportSize(50, 50);\n      page.mainFrame().waitForFunction(\"window.innerWidth < 100\");\n      browser.close();\n    }\n  }\n}\n```\n\n```python async\nimport asyncio\nfrom playwright.async_api import async_playwright\n\nasync def run(playwright):\n    webkit = playwright.webkit\n    browser = await webkit.launch()\n    page = await browser.new_page()\n    await page.evaluate(\"window.x = 0; setTimeout(() => { window.x = 100 }, 1000);\")\n    await page.main_frame.wait_for_function(\"() => window.x > 0\")\n    await browser.close()\n\nasync def main():\n    async with async_playwright() as playwright:\n        await run(playwright)\nasyncio.run(main())\n```\n\n```python sync\nfrom playwright.sync_api import sync_playwright\n\ndef run(playwright):\n    webkit = playwright.webkit\n    browser = webkit.launch()\n    page = browser.new_page()\n    page.evaluate(\"window.x = 0; setTimeout(() => { window.x = 100 }, 1000);\")\n    page.main_frame.wait_for_function(\"() => window.x > 0\")\n    browser.close()\n\nwith sync_playwright() as playwright:\n    run(playwright)\n```\n\nTo pass an argument to the predicate of `frame.waitForFunction` function:\n\n```js\nconst selector = '.foo';\nawait frame.waitForFunction(selector => !!document.querySelector(selector), selector);\n```\n\n```java\nString selector = \".foo\";\nframe.waitForFunction(\"selector => !!document.querySelector(selector)\", selector);\n```\n\n```python async\nselector = \".foo\"\nawait frame.wait_for_function(\"selector => !!document.querySelector(selector)\", selector)\n```\n\n```python sync\nselector = \".foo\"\nframe.wait_for_function(\"selector => !!document.querySelector(selector)\", selector)\n```\n"]
    fn wait_for_function(
        &self,
        #[doc = "JavaScript expression to be evaluated in the browser context. If it looks like a function declaration, it is interpreted\nas a function. Otherwise, evaluated as an expression."]
        expression: String,
        #[doc = "Optional argument to pass to `expression`."] arg: Option<EvaluationArgument>,
        #[doc = "options"]
        #[doc = "If `polling` is `'raf'`, then `expression` is constantly executed in `requestAnimationFrame` callback. If `polling` is a\nnumber, then it is treated as an interval in milliseconds at which the function would be executed. Defaults to `raf`."]
        polling: Option<NotImplementedYet>,
        #[doc = "If specified, then it is treated as an interval in milliseconds at which the function would be executed. By default if\nthe option is not specified `expression` is executed in `requestAnimationFrame` callback."]
        polling_interval: Option<f64>,
        #[doc = "maximum time to wait for in milliseconds. Defaults to `30000` (30 seconds). Pass `0` to disable timeout. The default\nvalue can be changed by using the [`method: BrowserContext.setDefaultTimeout`]."]
        timeout: Option<f64>
    ) -> Result<JsHandle, Arc<Error>> {
        todo!()
    }
    #[doc = "Waits for the required load state to be reached.\n\nThis returns when the frame reaches a required load state, `load` by default. The navigation must have been committed\nwhen this method is called. If current document has already reached the required state, resolves immediately.\n\n```js\nawait frame.click('button'); // Click triggers navigation.\nawait frame.waitForLoadState(); // Waits for 'load' state by default.\n```\n\n```java\nframe.click(\"button\"); // Click triggers navigation.\nframe.waitForLoadState(); // Waits for \"load\" state by default.\n```\n\n```python async\nawait frame.click(\"button\") # click triggers navigation.\nawait frame.wait_for_load_state() # the promise resolves after \"load\" event.\n```\n\n```python sync\nframe.click(\"button\") # click triggers navigation.\nframe.wait_for_load_state() # the promise resolves after \"load\" event.\n```\n"]
    fn wait_for_load_state(
        &self,
        #[doc = "Optional load state to wait for, defaults to `load`. If the state has been already reached while loading current\ndocument, the method resolves immediately. Can be one of:\n- `'load'` - wait for the `load` event to be fired.\n- `'domcontentloaded'` - wait for the `DOMContentLoaded` event to be fired.\n- `'networkidle'` - wait until there are no network connections for at least `500` ms."]
        state: Option<LoadState>,
        #[doc = "options"]
        #[doc = "Maximum operation time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be\nchanged by using the [`method: BrowserContext.setDefaultNavigationTimeout`],\n[`method: BrowserContext.setDefaultTimeout`], [`method: Page.setDefaultNavigationTimeout`] or\n[`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Waits for the frame navigation and returns the main resource response. In case of multiple redirects, the navigation\nwill resolve with the response of the last redirect. In case of navigation to a different anchor or navigation due to\nHistory API usage, the navigation will resolve with `null`.\n\nThis method waits for the frame to navigate to a new URL. It is useful for when you run code which will indirectly cause\nthe frame to navigate. Consider this example:\n\n```js\nconst [response] = await Promise.all([\n  frame.waitForNavigation(), // The promise resolves after navigation has finished\n  frame.click('a.delayed-navigation'), // Clicking the link will indirectly cause a navigation\n]);\n```\n\n```java\n// The method returns after navigation has finished\nResponse response = frame.waitForNavigation(() -> {\n  // Clicking the link will indirectly cause a navigation\n  frame.click(\"a.delayed-navigation\");\n});\n```\n\n```python async\nasync with frame.expect_navigation():\n    await frame.click(\"a.delayed-navigation\") # clicking the link will indirectly cause a navigation\n# Resolves after navigation has finished\n```\n\n```python sync\nwith frame.expect_navigation():\n    frame.click(\"a.delayed-navigation\") # clicking the link will indirectly cause a navigation\n# Resolves after navigation has finished\n```\n\n> NOTE: Usage of the [History API](https://developer.mozilla.org/en-US/docs/Web/API/History_API) to change the URL is\nconsidered a navigation."]
    fn wait_for_navigation(
        &self,
        #[doc = "Callback that performs the action triggering the event."] callback: Runnable,
        #[doc = "options"]
        #[doc = "Maximum operation time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be\nchanged by using the [`method: BrowserContext.setDefaultNavigationTimeout`],\n[`method: BrowserContext.setDefaultTimeout`], [`method: Page.setDefaultNavigationTimeout`] or\n[`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>,
        #[doc = "A glob pattern, regex pattern or predicate receiving [URL] to match while waiting for the navigation."]
        url: Option<NotImplementedYet>,
        #[doc = "When to consider operation succeeded, defaults to `load`. Events can be either:\n- `'domcontentloaded'` - consider operation to be finished when the `DOMContentLoaded` event is fired.\n- `'load'` - consider operation to be finished when the `load` event is fired.\n- `'networkidle'` - consider operation to be finished when there are no network connections for at least `500` ms."]
        wait_until: Option<WaitUntilState>
    ) -> Result<Option<Response>, Arc<Error>> {
        todo!()
    }
    #[doc = "Returns when element specified by selector satisfies `state` option. Returns `null` if waiting for `hidden` or\n`detached`.\n\nWait for the `selector` to satisfy `state` option (either appear/disappear from dom, or become visible/hidden). If at\nthe moment of calling the method `selector` already satisfies the condition, the method will return immediately. If the\nselector doesn't satisfy the condition for the `timeout` milliseconds, the function will throw.\n\nThis method works across navigations:\n\n```js\nconst { chromium } = require('playwright');  // Or 'firefox' or 'webkit'.\n\n(async () => {\n  const browser = await chromium.launch();\n  const page = await browser.newPage();\n  for (let currentURL of ['https://google.com', 'https://bbc.com']) {\n    await page.goto(currentURL);\n    const element = await page.mainFrame().waitForSelector('img');\n    console.log('Loaded image: ' + await element.getAttribute('src'));\n  }\n  await browser.close();\n})();\n```\n\n```java\nimport com.microsoft.playwright.*;\n\npublic class Example {\n  public static void main(String[] args) {\n    try (Playwright playwright = Playwright.create()) {\n      BrowserType chromium = playwright.chromium();\n      Browser browser = chromium.launch();\n      Page page = browser.newPage();\n      for (String currentURL : Arrays.asList(\"https://google.com\", \"https://bbc.com\")) {\n        page.navigate(currentURL);\n        ElementHandle element = page.mainFrame().waitForSelector(\"img\");\n        System.out.println(\"Loaded image: \" + element.getAttribute(\"src\"));\n      }\n      browser.close();\n    }\n  }\n}\n```\n\n```python async\nimport asyncio\nfrom playwright.async_api import async_playwright\n\nasync def run(playwright):\n    chromium = playwright.chromium\n    browser = await chromium.launch()\n    page = await browser.new_page()\n    for current_url in [\"https://google.com\", \"https://bbc.com\"]:\n        await page.goto(current_url, wait_until=\"domcontentloaded\")\n        element = await page.main_frame.wait_for_selector(\"img\")\n        print(\"Loaded image: \" + str(await element.get_attribute(\"src\")))\n    await browser.close()\n\nasync def main():\n    async with async_playwright() as playwright:\n        await run(playwright)\nasyncio.run(main())\n```\n\n```python sync\nfrom playwright.sync_api import sync_playwright\n\ndef run(playwright):\n    chromium = playwright.chromium\n    browser = chromium.launch()\n    page = browser.new_page()\n    for current_url in [\"https://google.com\", \"https://bbc.com\"]:\n        page.goto(current_url, wait_until=\"domcontentloaded\")\n        element = page.main_frame.wait_for_selector(\"img\")\n        print(\"Loaded image: \" + str(element.get_attribute(\"src\")))\n    browser.close()\n\nwith sync_playwright() as playwright:\n    run(playwright)\n```\n"]
    fn wait_for_selector(
        &self,
        #[doc = "A selector to query for. See [working with selectors](./selectors.md) for more details."]
        selector: String,
        #[doc = "options"]
        #[doc = "Defaults to `'visible'`. Can be either:\n- `'attached'` - wait for element to be present in DOM.\n- `'detached'` - wait for element to not be present in DOM.\n- `'visible'` - wait for element to have non-empty bounding box and no `visibility:hidden`. Note that element without\n  any content or with `display:none` has an empty bounding box and is not considered visible.\n- `'hidden'` - wait for element to be either detached from DOM, or have an empty bounding box or `visibility:hidden`.\n  This is opposite to the `'visible'` option."]
        state: Option<WaitForSelectorState>,
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>
    ) -> Result<Option<ElementHandle>, Arc<Error>> {
        todo!()
    }
    #[doc = "Waits for the given `timeout` in milliseconds.\n\nNote that `frame.waitForTimeout()` should only be used for debugging. Tests using the timer in production are going to\nbe flaky. Use signals such as network events, selectors becoming visible and others instead."]
    fn wait_for_timeout(
        &self,
        #[doc = "A timeout to wait for"] timeout: f64
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Waits for the frame to navigate to the given URL.\n\n```js\nawait frame.click('a.delayed-navigation'); // Clicking the link will indirectly cause a navigation\nawait frame.waitForURL('**/target.html');\n```\n\n```java\nframe.click(\"a.delayed-navigation\"); // Clicking the link will indirectly cause a navigation\nframe.waitForURL(\"**/target.html\");\n```\n\n```python async\nawait frame.click(\"a.delayed-navigation\") # clicking the link will indirectly cause a navigation\nawait frame.wait_for_url(\"**/target.html\")\n```\n\n```python sync\nframe.click(\"a.delayed-navigation\") # clicking the link will indirectly cause a navigation\nframe.wait_for_url(\"**/target.html\")\n```\n"]
    fn wait_for_u_r_l(
        &self,
        #[doc = "A glob pattern, regex pattern or predicate receiving [URL] to match while waiting for the navigation."]
        url: NotImplementedYet,
        #[doc = "options"]
        #[doc = "Maximum operation time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be\nchanged by using the [`method: BrowserContext.setDefaultNavigationTimeout`],\n[`method: BrowserContext.setDefaultTimeout`], [`method: Page.setDefaultNavigationTimeout`] or\n[`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>,
        #[doc = "When to consider operation succeeded, defaults to `load`. Events can be either:\n- `'domcontentloaded'` - consider operation to be finished when the `DOMContentLoaded` event is fired.\n- `'load'` - consider operation to be finished when the `load` event is fired.\n- `'networkidle'` - consider operation to be finished when there are no network connections for at least `500` ms."]
        wait_until: Option<WaitUntilState>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
}
enum NotImplementedYetvalues {
    NotImplementedYet(String),
    NotImplementedYet(ElementHandle),
    NotImplementedYet(Vec<String>),
    NotImplementedYet(NotImplementedYet),
    NotImplementedYet(Vec<ElementHandle>),
    NotImplementedYet(Vec<NotImplementedYet>)
}
struct NotImplementedYet {
    #[doc = "Matches by `option.value`. Optional."]
    value: Option<String>,
    #[doc = "Matches by `option.label`. Optional."]
    label: Option<String>,
    #[doc = "Matches by the index. Optional."]
    index: Option<i64>
}
enum NotImplementedYetelement {
    NotImplementedYet(ElementHandle),
    NotImplementedYet(Vec<ElementHandle>)
}
enum NotImplementedYetindex {
    NotImplementedYet(i64),
    NotImplementedYet(Vec<i64>)
}
enum NotImplementedYetvalue {
    NotImplementedYet(String),
    NotImplementedYet(Vec<String>)
}
enum NotImplementedYetlabel {
    NotImplementedYet(String),
    NotImplementedYet(Vec<String>)
}
enum NotImplementedYetfiles {
    NotImplementedYet(path),
    NotImplementedYet(Vec<path>),
    NotImplementedYet(NotImplementedYet),
    NotImplementedYet(Vec<NotImplementedYet>)
}
struct NotImplementedYet {
    #[doc = "File name"]
    name: String,
    #[doc = "File type"]
    mime_type: String,
    #[doc = "File content"]
    buffer: Buffer
}
enum NotImplementedYetstate {
    NotImplementedYet(load),
    NotImplementedYet(domcontentloaded),
    NotImplementedYet(networkidle)
}
enum NotImplementedYeturl {
    NotImplementedYet(String),
    NotImplementedYet(RegExp),
    NotImplementedYet(function)
}
#[doc = "JSHandle represents an in-page JavaScript object. JSHandles can be created with the [`method: Page.evaluateHandle`]\nmethod.\n\n```js\nconst windowHandle = await page.evaluateHandle(() => window);\n// ...\n```\n\n```java\nJSHandle windowHandle = page.evaluateHandle(\"() => window\");\n// ...\n```\n\n```python async\nwindow_handle = await page.evaluate_handle(\"window\")\n# ...\n```\n\n```python sync\nwindow_handle = page.evaluate_handle(\"window\")\n# ...\n```\n\nJSHandle prevents the referenced JavaScript object being garbage collected unless the handle is exposed with\n[`method: JSHandle.dispose`]. JSHandles are auto-disposed when their origin frame gets navigated or the parent context\ngets destroyed.\n\nJSHandle instances can be used as an argument in [`method: Page.evalOnSelector`], [`method: Page.evaluate`] and\n[`method: Page.evaluateHandle`] methods."]
impl JsHandle {
    #[doc = "Returns either `null` or the object handle itself, if the object handle is an instance of `ElementHandle`."]
    fn as_element(&self) -> Result<Option<ElementHandle>, Error> { todo!() }
    #[doc = "The `jsHandle.dispose` method stops referencing the element handle."]
    fn dispose(&self) -> Result<(), Arc<Error>> { todo!() }
    #[doc = "Returns the return value of `expression`.\n\nThis method passes this handle as the first argument to `expression`.\n\nIf `expression` returns a [Promise], then `handle.evaluate` would wait for the promise to resolve and return its value.\n\nExamples:\n\n```js\nconst tweetHandle = await page.$('.tweet .retweets');\nexpect(await tweetHandle.evaluate(node => node.innerText)).toBe('10 retweets');\n```\n\n```java\nElementHandle tweetHandle = page.querySelector(\".tweet .retweets\");\nassertEquals(\"10 retweets\", tweetHandle.evaluate(\"node => node.innerText\"));\n```\n\n```python async\ntweet_handle = await page.query_selector(\".tweet .retweets\")\nassert await tweet_handle.evaluate(\"node => node.innerText\") == \"10 retweets\"\n```\n\n```python sync\ntweet_handle = page.query_selector(\".tweet .retweets\")\nassert tweet_handle.evaluate(\"node => node.innerText\") == \"10 retweets\"\n```\n"]
    fn evaluate(
        &self,
        #[doc = "JavaScript expression to be evaluated in the browser context. If it looks like a function declaration, it is interpreted\nas a function. Otherwise, evaluated as an expression."]
        expression: String,
        #[doc = "Optional argument to pass to `expression`."] arg: Option<EvaluationArgument>
    ) -> Result<Serializable, Arc<Error>> {
        todo!()
    }
    #[doc = "Returns the return value of `expression` as a `JSHandle`.\n\nThis method passes this handle as the first argument to `expression`.\n\nThe only difference between `jsHandle.evaluate` and `jsHandle.evaluateHandle` is that `jsHandle.evaluateHandle` returns\n`JSHandle`.\n\nIf the function passed to the `jsHandle.evaluateHandle` returns a [Promise], then `jsHandle.evaluateHandle` would wait\nfor the promise to resolve and return its value.\n\nSee [`method: Page.evaluateHandle`] for more details."]
    fn evaluate_handle(
        &self,
        #[doc = "JavaScript expression to be evaluated in the browser context. If it looks like a function declaration, it is interpreted\nas a function. Otherwise, evaluated as an expression."]
        expression: String,
        #[doc = "Optional argument to pass to `expression`."] arg: Option<EvaluationArgument>
    ) -> Result<JsHandle, Arc<Error>> {
        todo!()
    }
    #[doc = "The method returns a map with **own property names** as keys and JSHandle instances for the property values.\n\n```js\nconst handle = await page.evaluateHandle(() => ({window, document}));\nconst properties = await handle.getProperties();\nconst windowHandle = properties.get('window');\nconst documentHandle = properties.get('document');\nawait handle.dispose();\n```\n\n```java\nJSHandle handle = page.evaluateHandle(\"() => ({window, document}\"););\nMap<String, JSHandle> properties = handle.getProperties();\nJSHandle windowHandle = properties.get(\"window\");\nJSHandle documentHandle = properties.get(\"document\");\nhandle.dispose();\n```\n\n```python async\nhandle = await page.evaluate_handle(\"{window, document}\")\nproperties = await handle.get_properties()\nwindow_handle = properties.get(\"window\")\ndocument_handle = properties.get(\"document\")\nawait handle.dispose()\n```\n\n```python sync\nhandle = page.evaluate_handle(\"{window, document}\")\nproperties = handle.get_properties()\nwindow_handle = properties.get(\"window\")\ndocument_handle = properties.get(\"document\")\nhandle.dispose()\n```\n"]
    fn get_properties(&self) -> Result<Map<String, String>, Arc<Error>> { todo!() }
    #[doc = "Fetches a single property from the referenced object."]
    fn get_property(
        &self,
        #[doc = "property to get"] property_name: String
    ) -> Result<JsHandle, Arc<Error>> {
        todo!()
    }
    #[doc = "Returns a JSON representation of the object. If the object has a `toJSON` function, it **will not be called**.\n\n> NOTE: The method will return an empty JSON object if the referenced object is not stringifiable. It will throw an\nerror if the object has circular references."]
    fn json_value(&self) -> Result<Serializable, Arc<Error>> { todo!() }
}
#[doc = "Keyboard provides an api for managing a virtual keyboard. The high level api is [`method: Keyboard.type`], which takes\nraw characters and generates proper keydown, keypress/input, and keyup events on your page.\n\nFor finer control, you can use [`method: Keyboard.down`], [`method: Keyboard.up`], and [`method: Keyboard.insertText`]\nto manually fire events as if they were generated from a real keyboard.\n\nAn example of holding down `Shift` in order to select and delete some text:\n\n```js\nawait page.keyboard.type('Hello World!');\nawait page.keyboard.press('ArrowLeft');\n\nawait page.keyboard.down('Shift');\nfor (let i = 0; i < ' World'.length; i++)\n  await page.keyboard.press('ArrowLeft');\nawait page.keyboard.up('Shift');\n\nawait page.keyboard.press('Backspace');\n// Result text will end up saying 'Hello!'\n```\n\n```java\npage.keyboard().type(\"Hello World!\");\npage.keyboard().press(\"ArrowLeft\");\npage.keyboard().down(\"Shift\");\nfor (int i = 0; i < \" World\".length(); i++)\n  page.keyboard().press(\"ArrowLeft\");\npage.keyboard().up(\"Shift\");\npage.keyboard().press(\"Backspace\");\n// Result text will end up saying \"Hello!\"\n```\n\n```python async\nawait page.keyboard.type(\"Hello World!\")\nawait page.keyboard.press(\"ArrowLeft\")\nawait page.keyboard.down(\"Shift\")\nfor i in range(6):\n    await page.keyboard.press(\"ArrowLeft\")\nawait page.keyboard.up(\"Shift\")\nawait page.keyboard.press(\"Backspace\")\n# result text will end up saying \"Hello!\"\n```\n\n```python sync\npage.keyboard.type(\"Hello World!\")\npage.keyboard.press(\"ArrowLeft\")\npage.keyboard.down(\"Shift\")\nfor i in range(6):\n    page.keyboard.press(\"ArrowLeft\")\npage.keyboard.up(\"Shift\")\npage.keyboard.press(\"Backspace\")\n# result text will end up saying \"Hello!\"\n```\n\nAn example of pressing uppercase `A`\n\n```js\nawait page.keyboard.press('Shift+KeyA');\n// or\nawait page.keyboard.press('Shift+A');\n```\n\n```java\npage.keyboard().press(\"Shift+KeyA\");\n// or\npage.keyboard().press(\"Shift+A\");\n```\n\n```python async\nawait page.keyboard.press(\"Shift+KeyA\")\n# or\nawait page.keyboard.press(\"Shift+A\")\n```\n\n```python sync\npage.keyboard.press(\"Shift+KeyA\")\n# or\npage.keyboard.press(\"Shift+A\")\n```\n\nAn example to trigger select-all with the keyboard\n\n```js\n// on Windows and Linux\nawait page.keyboard.press('Control+A');\n// on macOS\nawait page.keyboard.press('Meta+A');\n```\n\n```java\n// on Windows and Linux\npage.keyboard().press(\"Control+A\");\n// on macOS\npage.keyboard().press(\"Meta+A\");\n```\n\n```python async\n# on windows and linux\nawait page.keyboard.press(\"Control+A\")\n# on mac_os\nawait page.keyboard.press(\"Meta+A\")\n```\n\n```python sync\n# on windows and linux\npage.keyboard.press(\"Control+A\")\n# on mac_os\npage.keyboard.press(\"Meta+A\")\n```\n"]
impl Keyboard {
    #[doc = "Dispatches a `keydown` event.\n\n`key` can specify the intended [keyboardEvent.key](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key)\nvalue or a single character to generate the text for. A superset of the `key` values can be found\n[here](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key/Key_Values). Examples of the keys are:\n\n`F1` - `F12`, `Digit0`- `Digit9`, `KeyA`- `KeyZ`, `Backquote`, `Minus`, `Equal`, `Backslash`, `Backspace`, `Tab`,\n`Delete`, `Escape`, `ArrowDown`, `End`, `Enter`, `Home`, `Insert`, `PageDown`, `PageUp`, `ArrowRight`, `ArrowUp`, etc.\n\nFollowing modification shortcuts are also supported: `Shift`, `Control`, `Alt`, `Meta`, `ShiftLeft`.\n\nHolding down `Shift` will type the text that corresponds to the `key` in the upper case.\n\nIf `key` is a single character, it is case-sensitive, so the values `a` and `A` will generate different respective\ntexts.\n\nIf `key` is a modifier key, `Shift`, `Meta`, `Control`, or `Alt`, subsequent key presses will be sent with that modifier\nactive. To release the modifier key, use [`method: Keyboard.up`].\n\nAfter the key is pressed once, subsequent calls to [`method: Keyboard.down`] will have\n[repeat](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/repeat) set to true. To release the key, use\n[`method: Keyboard.up`].\n\n> NOTE: Modifier keys DO influence `keyboard.down`. Holding down `Shift` will type the text in upper case."]
    fn down(
        &self,
        #[doc = "Name of the key to press or a character to generate, such as `ArrowLeft` or `a`."]
        key: String
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Dispatches only `input` event, does not emit the `keydown`, `keyup` or `keypress` events.\n\n```js\npage.keyboard.insertText('嗨');\n```\n\n```java\npage.keyboard().insertText(\"嗨\");\n```\n\n```python async\nawait page.keyboard.insert_text(\"嗨\")\n```\n\n```python sync\npage.keyboard.insert_text(\"嗨\")\n```\n\n> NOTE: Modifier keys DO NOT effect `keyboard.insertText`. Holding down `Shift` will not type the text in upper case."]
    fn insert_text(
        &self,
        #[doc = "Sets input to the specified text value."] text: String
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "`key` can specify the intended [keyboardEvent.key](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key)\nvalue or a single character to generate the text for. A superset of the `key` values can be found\n[here](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key/Key_Values). Examples of the keys are:\n\n`F1` - `F12`, `Digit0`- `Digit9`, `KeyA`- `KeyZ`, `Backquote`, `Minus`, `Equal`, `Backslash`, `Backspace`, `Tab`,\n`Delete`, `Escape`, `ArrowDown`, `End`, `Enter`, `Home`, `Insert`, `PageDown`, `PageUp`, `ArrowRight`, `ArrowUp`, etc.\n\nFollowing modification shortcuts are also supported: `Shift`, `Control`, `Alt`, `Meta`, `ShiftLeft`.\n\nHolding down `Shift` will type the text that corresponds to the `key` in the upper case.\n\nIf `key` is a single character, it is case-sensitive, so the values `a` and `A` will generate different respective\ntexts.\n\nShortcuts such as `key: \"Control+o\"` or `key: \"Control+Shift+T\"` are supported as well. When specified with the\nmodifier, modifier is pressed and being held while the subsequent key is being pressed.\n\n```js\nconst page = await browser.newPage();\nawait page.goto('https://keycode.info');\nawait page.keyboard.press('A');\nawait page.screenshot({ path: 'A.png' });\nawait page.keyboard.press('ArrowLeft');\nawait page.screenshot({ path: 'ArrowLeft.png' });\nawait page.keyboard.press('Shift+O');\nawait page.screenshot({ path: 'O.png' });\nawait browser.close();\n```\n\n```java\nPage page = browser.newPage();\npage.navigate(\"https://keycode.info\");\npage.keyboard().press(\"A\");\npage.screenshot(new Page.ScreenshotOptions().setPath(Paths.get(\"A.png\"));\npage.keyboard().press(\"ArrowLeft\");\npage.screenshot(new Page.ScreenshotOptions().setPath(Paths.get(\"ArrowLeft.png\")));\npage.keyboard().press(\"Shift+O\");\npage.screenshot(new Page.ScreenshotOptions().setPath(Paths.get(\"O.png\")));\nbrowser.close();\n```\n\n```python async\npage = await browser.new_page()\nawait page.goto(\"https://keycode.info\")\nawait page.keyboard.press(\"a\")\nawait page.screenshot(path=\"a.png\")\nawait page.keyboard.press(\"ArrowLeft\")\nawait page.screenshot(path=\"arrow_left.png\")\nawait page.keyboard.press(\"Shift+O\")\nawait page.screenshot(path=\"o.png\")\nawait browser.close()\n```\n\n```python sync\npage = browser.new_page()\npage.goto(\"https://keycode.info\")\npage.keyboard.press(\"a\")\npage.screenshot(path=\"a.png\")\npage.keyboard.press(\"ArrowLeft\")\npage.screenshot(path=\"arrow_left.png\")\npage.keyboard.press(\"Shift+O\")\npage.screenshot(path=\"o.png\")\nbrowser.close()\n```\n\nShortcut for [`method: Keyboard.down`] and [`method: Keyboard.up`]."]
    fn press(
        &self,
        #[doc = "Name of the key to press or a character to generate, such as `ArrowLeft` or `a`."]
        key: String,
        #[doc = "options"]
        #[doc = "Time to wait between `keydown` and `keyup` in milliseconds. Defaults to 0."]
        delay: Option<f64>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Sends a `keydown`, `keypress`/`input`, and `keyup` event for each character in the text.\n\nTo press a special key, like `Control` or `ArrowDown`, use [`method: Keyboard.press`].\n\n```js\nawait page.keyboard.type('Hello'); // Types instantly\nawait page.keyboard.type('World', {delay: 100}); // Types slower, like a user\n```\n\n```java\n// Types instantly\npage.keyboard().type(\"Hello\");\n// Types slower, like a user\npage.keyboard().type(\"World\", new Keyboard.TypeOptions().setDelay(100));\n```\n\n```python async\nawait page.keyboard.type(\"Hello\") # types instantly\nawait page.keyboard.type(\"World\", delay=100) # types slower, like a user\n```\n\n```python sync\npage.keyboard.type(\"Hello\") # types instantly\npage.keyboard.type(\"World\", delay=100) # types slower, like a user\n```\n\n> NOTE: Modifier keys DO NOT effect `keyboard.type`. Holding down `Shift` will not type the text in upper case.\n> NOTE: For characters that are not on a US keyboard, only an `input` event will be sent."]
    fn r#type(
        &self,
        #[doc = "A text to type into a focused element."] text: String,
        #[doc = "options"]
        #[doc = "Time to wait between key presses in milliseconds. Defaults to 0."]
        delay: Option<f64>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Dispatches a `keyup` event."]
    fn up(
        &self,
        #[doc = "Name of the key to press or a character to generate, such as `ArrowLeft` or `a`."]
        key: String
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
}
#[doc = "Playwright generates a lot of logs and they are accessible via the pluggable logger sink.\n\n```js\nconst { chromium } = require('playwright');  // Or 'firefox' or 'webkit'.\n\n(async () => {\n  const browser = await chromium.launch({\n    logger: {\n      isEnabled: (name, severity) => name === 'browser',\n      log: (name, severity, message, args) => console.log(`${name} ${message}`)\n    }\n  });\n  ...\n})();\n```\n"]
impl Logger {
    #[doc = "Determines whether sink is interested in the logger with the given name and severity."]
    fn is_enabled(
        &self,
        #[doc = "logger name"] name: String,
        #[doc = ""] severity: NotImplementedYet
    ) -> Result<bool, Error> {
        todo!()
    }
    #[doc = ""]
    fn log(
        &self,
        #[doc = "logger name"] name: String,
        #[doc = ""] severity: NotImplementedYet,
        #[doc = "log message format"] message: NotImplementedYet,
        #[doc = "message arguments"] args: Vec<Object>,
        #[doc = "optional formatting hints"] hints: NotImplementedYet
    ) -> Result<(), Error> {
        todo!()
    }
}
enum NotImplementedYetseverity {
    NotImplementedYet(verbose),
    NotImplementedYet(info),
    NotImplementedYet(warning),
    NotImplementedYet(error)
}
enum NotImplementedYetseverity {
    NotImplementedYet(verbose),
    NotImplementedYet(info),
    NotImplementedYet(warning),
    NotImplementedYet(error)
}
enum NotImplementedYetmessage {
    NotImplementedYet(String),
    NotImplementedYet(Error)
}
struct NotImplementedYethints {
    #[doc = "Optional preferred logger color."]
    color: Option<String>
}
#[doc = "The Mouse class operates in main-frame CSS pixels relative to the top-left corner of the viewport.\n\nEvery `page` object has its own Mouse, accessible with [`property: Page.mouse`].\n\n```js\n// Using ‘page.mouse’ to trace a 100x100 square.\nawait page.mouse.move(0, 0);\nawait page.mouse.down();\nawait page.mouse.move(0, 100);\nawait page.mouse.move(100, 100);\nawait page.mouse.move(100, 0);\nawait page.mouse.move(0, 0);\nawait page.mouse.up();\n```\n\n```java\n// Using ‘page.mouse’ to trace a 100x100 square.\npage.mouse().move(0, 0);\npage.mouse().down();\npage.mouse().move(0, 100);\npage.mouse().move(100, 100);\npage.mouse().move(100, 0);\npage.mouse().move(0, 0);\npage.mouse().up();\n```\n\n```python async\n# using ‘page.mouse’ to trace a 100x100 square.\nawait page.mouse.move(0, 0)\nawait page.mouse.down()\nawait page.mouse.move(0, 100)\nawait page.mouse.move(100, 100)\nawait page.mouse.move(100, 0)\nawait page.mouse.move(0, 0)\nawait page.mouse.up()\n```\n\n```python sync\n# using ‘page.mouse’ to trace a 100x100 square.\npage.mouse.move(0, 0)\npage.mouse.down()\npage.mouse.move(0, 100)\npage.mouse.move(100, 100)\npage.mouse.move(100, 0)\npage.mouse.move(0, 0)\npage.mouse.up()\n```\n\n```csharp\nawait Page.Mouse.MoveAsync(0, 0);\nawait Page.Mouse.DownAsync();\nawait Page.Mouse.MoveAsync(0, 100);\nawait Page.Mouse.MoveAsync(100, 100);\nawait Page.Mouse.MoveAsync(100, 0);\nawait Page.Mouse.MoveAsync(0, 0);\nawait Page.Mouse.UpAsync();\n```\n"]
impl Mouse {
    #[doc = "Shortcut for [`method: Mouse.move`], [`method: Mouse.down`], [`method: Mouse.up`]."]
    fn click(
        &self,
        #[doc = ""] x: f64,
        #[doc = ""] y: f64,
        #[doc = "options"]
        #[doc = "Defaults to `left`."]
        button: Option<MouseButton>,
        #[doc = "defaults to 1. See [UIEvent.detail]."] click_count: Option<i64>,
        #[doc = "Time to wait between `mousedown` and `mouseup` in milliseconds. Defaults to 0."]
        delay: Option<f64>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Shortcut for [`method: Mouse.move`], [`method: Mouse.down`], [`method: Mouse.up`], [`method: Mouse.down`] and\n[`method: Mouse.up`]."]
    fn dblclick(
        &self,
        #[doc = ""] x: f64,
        #[doc = ""] y: f64,
        #[doc = "options"]
        #[doc = "Defaults to `left`."]
        button: Option<MouseButton>,
        #[doc = "Time to wait between `mousedown` and `mouseup` in milliseconds. Defaults to 0."]
        delay: Option<f64>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Dispatches a `mousedown` event."]
    fn down(
        &self,
        #[doc = "options"]
        #[doc = "Defaults to `left`."]
        button: Option<MouseButton>,
        #[doc = "defaults to 1. See [UIEvent.detail]."] click_count: Option<i64>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Dispatches a `mousemove` event."]
    fn r#move(
        &self,
        #[doc = ""] x: f64,
        #[doc = ""] y: f64,
        #[doc = "options"]
        #[doc = "defaults to 1. Sends intermediate `mousemove` events."]
        steps: Option<i64>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Dispatches a `mouseup` event."]
    fn up(
        &self,
        #[doc = "options"]
        #[doc = "Defaults to `left`."]
        button: Option<MouseButton>,
        #[doc = "defaults to 1. See [UIEvent.detail]."] click_count: Option<i64>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
}
#[doc = "- extends: [EventEmitter]\n\nPage provides methods to interact with a single tab in a `Browser`, or an\n[extension background page](https://developer.chrome.com/extensions/background_pages) in Chromium. One `Browser`\ninstance might have multiple `Page` instances.\n\nThis example creates a page, navigates it to a URL, and then saves a screenshot:\n\n```js\nconst { webkit } = require('playwright');  // Or 'chromium' or 'firefox'.\n\n(async () => {\n  const browser = await webkit.launch();\n  const context = await browser.newContext();\n  const page = await context.newPage();\n  await page.goto('https://example.com');\n  await page.screenshot({path: 'screenshot.png'});\n  await browser.close();\n})();\n```\n\n```java\nimport com.microsoft.playwright.*;\n\npublic class Example {\n  public static void main(String[] args) {\n    try (Playwright playwright = Playwright.create()) {\n      BrowserType webkit = playwright.webkit();\n      Browser browser = webkit.launch();\n      BrowserContext context = browser.newContext();\n      Page page = context.newPage();\n      page.navigate(\"https://example.com\");\n      page.screenshot(new Page.ScreenshotOptions().setPath(Paths.get(\"screenshot.png\")));\n      browser.close();\n    }\n  }\n}\n```\n\n```python async\nimport asyncio\nfrom playwright.async_api import async_playwright\n\nasync def run(playwright):\n    webkit = playwright.webkit\n    browser = await webkit.launch()\n    context = await browser.new_context()\n    page = await context.new_page()\n    await page.goto(\"https://example.com\")\n    await page.screenshot(path=\"screenshot.png\")\n    await browser.close()\n\nasync def main():\n    async with async_playwright() as playwright:\n        await run(playwright)\nasyncio.run(main())\n```\n\n```python sync\nfrom playwright.sync_api import sync_playwright\n\ndef run(playwright):\n    webkit = playwright.webkit\n    browser = webkit.launch()\n    context = browser.new_context()\n    page = context.new_page()\n    page.goto(\"https://example.com\")\n    page.screenshot(path=\"screenshot.png\")\n    browser.close()\n\nwith sync_playwright() as playwright:\n    run(playwright)\n```\n\nThe Page class emits various events (described below) which can be handled using any of Node's native\n[`EventEmitter`](https://nodejs.org/api/events.html#events_class_eventemitter) methods, such as `on`, `once` or\n`removeListener`.\n\nThis example logs a message for a single page `load` event:\n\n```js\npage.once('load', () => console.log('Page loaded!'));\n```\n\n```java\npage.onLoad(p -> System.out.println(\"Page loaded!\"));\n```\n\n```py\npage.once(\"load\", lambda: print(\"page loaded!\"))\n```\n\nTo unsubscribe from events use the `removeListener` method:\n\n```js\nfunction logRequest(interceptedRequest) {\n  console.log('A request was made:', interceptedRequest.url());\n}\npage.on('request', logRequest);\n// Sometime later...\npage.removeListener('request', logRequest);\n```\n\n```java\nConsumer<Request> logRequest = interceptedRequest -> {\n  System.out.println(\"A request was made: \" + interceptedRequest.url());\n};\npage.onRequest(logRequest);\n// Sometime later...\npage.offRequest(logRequest);\n```\n\n```py\ndef log_request(intercepted_request):\n    print(\"a request was made:\", intercepted_request.url)\npage.on(\"request\", log_request)\n# sometime later...\npage.remove_listener(\"request\", log_request)\n```\n"]
#[doc = "Extends EventEmitter"]
impl Page {
    #[doc = ""]
    pub fn accessibility(&self) -> Accessibility {}
    #[doc = "> NOTE: Only available for Chromium atm.\n\nBrowser-specific Coverage implementation. See `Coverage`(#class-coverage) for more details."]
    pub fn coverage(&self) -> Coverage {}
    #[doc = ""]
    pub fn keyboard(&self) -> Keyboard {}
    #[doc = ""]
    pub fn mouse(&self) -> Mouse {}
    #[doc = ""]
    pub fn touchscreen(&self) -> Touchscreen {}
    #[doc = "Adds a script which would be evaluated in one of the following scenarios:\n- Whenever the page is navigated.\n- Whenever the child frame is attached or navigated. In this case, the script is evaluated in the context of the newly\n  attached frame.\n\nThe script is evaluated after the document was created but before any of its scripts were run. This is useful to amend\nthe JavaScript environment, e.g. to seed `Math.random`.\n\nAn example of overriding `Math.random` before the page loads:\n\n```js browser\n// preload.js\nMath.random = () => 42;\n```\n\n```js\n// In your playwright script, assuming the preload.js file is in same directory\nawait page.addInitScript({ path: './preload.js' });\n```\n\n```java\n// In your playwright script, assuming the preload.js file is in same directory\npage.addInitScript(Paths.get(\"./preload.js\"));\n```\n\n```python async\n# in your playwright script, assuming the preload.js file is in same directory\nawait page.add_init_script(path=\"./preload.js\")\n```\n\n```python sync\n# in your playwright script, assuming the preload.js file is in same directory\npage.add_init_script(path=\"./preload.js\")\n```\n\n> NOTE: The order of evaluation of multiple scripts installed via [`method: BrowserContext.addInitScript`] and\n[`method: Page.addInitScript`] is not defined."]
    fn add_init_script(
        &self,
        #[doc = "Script to be evaluated in the page."] script: NotImplementedYet,
        script : NotImplementedYet,
        #[doc = "Optional argument to pass to `script` (only supported when passing a function)."]
        arg: Option<Serializable>,
        #[doc = "Path to the JavaScript file. If `path` is a relative path, then it is resolved relative to the current working\ndirectory. Optional."]
        path: Option<path>,
        script : Option < String >
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Adds a `<script>` tag into the page with the desired url or content. Returns the added tag when the script's onload\nfires or when the script content was injected into frame.\n\nShortcut for main frame's [`method: Frame.addScriptTag`]."]
    fn add_script_tag(
        &self,
        #[doc = "options"]
        #[doc = "Raw JavaScript content to be injected into frame."]
        content: Option<String>,
        #[doc = "Path to the JavaScript file to be injected into frame. If `path` is a relative path, then it is resolved relative to the\ncurrent working directory."]
        path: Option<path>,
        #[doc = "Script type. Use 'module' in order to load a Javascript ES6 module. See\n[script](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/script) for more details."]
        r#type: Option<String>,
        #[doc = "URL of a script to be added."] url: Option<String>
    ) -> Result<ElementHandle, Arc<Error>> {
        todo!()
    }
    #[doc = "Adds a `<link rel=\"stylesheet\">` tag into the page with the desired url or a `<style type=\"text/css\">` tag with the\ncontent. Returns the added tag when the stylesheet's onload fires or when the CSS content was injected into frame.\n\nShortcut for main frame's [`method: Frame.addStyleTag`]."]
    fn add_style_tag(
        &self,
        #[doc = "options"]
        #[doc = "Raw CSS content to be injected into frame."]
        content: Option<String>,
        #[doc = "Path to the CSS file to be injected into frame. If `path` is a relative path, then it is resolved relative to the\ncurrent working directory."]
        path: Option<path>,
        #[doc = "URL of the `<link>` tag."] url: Option<String>
    ) -> Result<ElementHandle, Arc<Error>> {
        todo!()
    }
    #[doc = "Brings page to front (activates tab)."]
    fn bring_to_front(&self) -> Result<(), Arc<Error>> { todo!() }
    #[doc = "This method checks an element matching `selector` by performing the following steps:\n1. Find an element matching `selector`. If there is none, wait until a matching element is attached to the DOM.\n1. Ensure that matched element is a checkbox or a radio input. If not, this method throws. If the element is already\n   checked, this method returns immediately.\n1. Wait for [actionability](https://playwright.dev/docs/actionability/) checks on the matched element, unless `force` option is set. If the\n   element is detached during the checks, the whole action is retried.\n1. Scroll the element into view if needed.\n1. Use [`property: Page.mouse`] to click in the center of the element.\n1. Wait for initiated navigations to either succeed or fail, unless `noWaitAfter` option is set.\n1. Ensure that the element is now checked. If not, this method throws.\n\nWhen all steps combined have not finished during the specified `timeout`, this method throws a `TimeoutError`. Passing\nzero timeout disables this.\n\nShortcut for main frame's [`method: Frame.check`]."]
    fn check(
        &self,
        #[doc = "A selector to search for element. If there are multiple elements satisfying the selector, the first will be used. See\n[working with selectors](./selectors.md) for more details."]
        selector: String,
        #[doc = "options"]
        #[doc = "Whether to bypass the [actionability](https://playwright.dev/docs/actionability/) checks. Defaults to `false`."]
        force: Option<bool>,
        #[doc = "Actions that initiate navigations are waiting for these navigations to happen and for pages to start loading. You can\nopt out of waiting via setting this flag. You would only need this option in the exceptional cases such as navigating to\ninaccessible pages. Defaults to `false`."]
        no_wait_after: Option<bool>,
        #[doc = "A point to use relative to the top-left corner of element padding box. If not specified, uses some visible point of the\nelement."]
        position: Option<NotImplementedYet>,
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>,
        #[doc = "When set, this method only performs the [actionability](https://playwright.dev/docs/actionability/) checks and skips the action. Defaults to\n`false`. Useful to wait until the element is ready for the action without performing it."]
        trial: Option<bool>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "This method clicks an element matching `selector` by performing the following steps:\n1. Find an element matching `selector`. If there is none, wait until a matching element is attached to the DOM.\n1. Wait for [actionability](https://playwright.dev/docs/actionability/) checks on the matched element, unless `force` option is set. If the\n   element is detached during the checks, the whole action is retried.\n1. Scroll the element into view if needed.\n1. Use [`property: Page.mouse`] to click in the center of the element, or the specified `position`.\n1. Wait for initiated navigations to either succeed or fail, unless `noWaitAfter` option is set.\n\nWhen all steps combined have not finished during the specified `timeout`, this method throws a `TimeoutError`. Passing\nzero timeout disables this.\n\nShortcut for main frame's [`method: Frame.click`]."]
    fn click(
        &self,
        #[doc = "A selector to search for element. If there are multiple elements satisfying the selector, the first will be used. See\n[working with selectors](./selectors.md) for more details."]
        selector: String,
        #[doc = "options"]
        #[doc = "Defaults to `left`."]
        button: Option<MouseButton>,
        #[doc = "defaults to 1. See [UIEvent.detail]."] click_count: Option<i64>,
        #[doc = "Time to wait between `mousedown` and `mouseup` in milliseconds. Defaults to 0."]
        delay: Option<f64>,
        #[doc = "Whether to bypass the [actionability](https://playwright.dev/docs/actionability/) checks. Defaults to `false`."]
        force: Option<bool>,
        #[doc = "Modifier keys to press. Ensures that only these modifiers are pressed during the operation, and then restores current\nmodifiers back. If not specified, currently pressed modifiers are used."]
        modifiers: Option<Vec<KeyboardModifier>>,
        #[doc = "Actions that initiate navigations are waiting for these navigations to happen and for pages to start loading. You can\nopt out of waiting via setting this flag. You would only need this option in the exceptional cases such as navigating to\ninaccessible pages. Defaults to `false`."]
        no_wait_after: Option<bool>,
        #[doc = "A point to use relative to the top-left corner of element padding box. If not specified, uses some visible point of the\nelement."]
        position: Option<NotImplementedYet>,
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>,
        #[doc = "When set, this method only performs the [actionability](https://playwright.dev/docs/actionability/) checks and skips the action. Defaults to\n`false`. Useful to wait until the element is ready for the action without performing it."]
        trial: Option<bool>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "If `runBeforeUnload` is `false`, does not run any unload handlers and waits for the page to be closed. If\n`runBeforeUnload` is `true` the method will run unload handlers, but will **not** wait for the page to close.\n\nBy default, `page.close()` **does not** run `beforeunload` handlers.\n\n> NOTE: if `runBeforeUnload` is passed as true, a `beforeunload` dialog might be summoned and should be handled manually\nvia [`event: Page.dialog`] event."]
    fn close(
        &self,
        #[doc = "options"]
        #[doc = "Defaults to `false`. Whether to run the\n[before unload](https://developer.mozilla.org/en-US/docs/Web/Events/beforeunload) page handlers."]
        run_before_unload: Option<bool>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Gets the full HTML contents of the page, including the doctype."]
    fn content(&self) -> Result<String, Arc<Error>> { todo!() }
    #[doc = "Get the browser context that the page belongs to."]
    fn context(&self) -> Result<BrowserContext, Error> { todo!() }
    #[doc = "This method double clicks an element matching `selector` by performing the following steps:\n1. Find an element matching `selector`. If there is none, wait until a matching element is attached to the DOM.\n1. Wait for [actionability](https://playwright.dev/docs/actionability/) checks on the matched element, unless `force` option is set. If the\n   element is detached during the checks, the whole action is retried.\n1. Scroll the element into view if needed.\n1. Use [`property: Page.mouse`] to double click in the center of the element, or the specified `position`.\n1. Wait for initiated navigations to either succeed or fail, unless `noWaitAfter` option is set. Note that if the\n   first click of the `dblclick()` triggers a navigation event, this method will throw.\n\nWhen all steps combined have not finished during the specified `timeout`, this method throws a `TimeoutError`. Passing\nzero timeout disables this.\n\n> NOTE: `page.dblclick()` dispatches two `click` events and a single `dblclick` event.\n\nShortcut for main frame's [`method: Frame.dblclick`]."]
    fn dblclick(
        &self,
        #[doc = "A selector to search for element. If there are multiple elements satisfying the selector, the first will be used. See\n[working with selectors](./selectors.md) for more details."]
        selector: String,
        #[doc = "options"]
        #[doc = "Defaults to `left`."]
        button: Option<MouseButton>,
        #[doc = "Time to wait between `mousedown` and `mouseup` in milliseconds. Defaults to 0."]
        delay: Option<f64>,
        #[doc = "Whether to bypass the [actionability](https://playwright.dev/docs/actionability/) checks. Defaults to `false`."]
        force: Option<bool>,
        #[doc = "Modifier keys to press. Ensures that only these modifiers are pressed during the operation, and then restores current\nmodifiers back. If not specified, currently pressed modifiers are used."]
        modifiers: Option<Vec<KeyboardModifier>>,
        #[doc = "Actions that initiate navigations are waiting for these navigations to happen and for pages to start loading. You can\nopt out of waiting via setting this flag. You would only need this option in the exceptional cases such as navigating to\ninaccessible pages. Defaults to `false`."]
        no_wait_after: Option<bool>,
        #[doc = "A point to use relative to the top-left corner of element padding box. If not specified, uses some visible point of the\nelement."]
        position: Option<NotImplementedYet>,
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>,
        #[doc = "When set, this method only performs the [actionability](https://playwright.dev/docs/actionability/) checks and skips the action. Defaults to\n`false`. Useful to wait until the element is ready for the action without performing it."]
        trial: Option<bool>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "The snippet below dispatches the `click` event on the element. Regardless of the visibility state of the element,\n`click` is dispatched. This is equivalent to calling\n[element.click()](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/click).\n\n```js\nawait page.dispatchEvent('button#submit', 'click');\n```\n\n```java\npage.dispatchEvent(\"button#submit\", \"click\");\n```\n\n```python async\nawait page.dispatch_event(\"button#submit\", \"click\")\n```\n\n```python sync\npage.dispatch_event(\"button#submit\", \"click\")\n```\n\nUnder the hood, it creates an instance of an event based on the given `type`, initializes it with `eventInit` properties\nand dispatches it on the element. Events are `composed`, `cancelable` and bubble by default.\n\nSince `eventInit` is event-specific, please refer to the events documentation for the lists of initial properties:\n- [DragEvent](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/DragEvent)\n- [FocusEvent](https://developer.mozilla.org/en-US/docs/Web/API/FocusEvent/FocusEvent)\n- [KeyboardEvent](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/KeyboardEvent)\n- [MouseEvent](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/MouseEvent)\n- [PointerEvent](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/PointerEvent)\n- [TouchEvent](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/TouchEvent)\n- [Event](https://developer.mozilla.org/en-US/docs/Web/API/Event/Event)\n\nYou can also specify `JSHandle` as the property value if you want live objects to be passed into the event:\n\n```js\n// Note you can only create DataTransfer in Chromium and Firefox\nconst dataTransfer = await page.evaluateHandle(() => new DataTransfer());\nawait page.dispatchEvent('#source', 'dragstart', { dataTransfer });\n```\n\n```java\n// Note you can only create DataTransfer in Chromium and Firefox\nJSHandle dataTransfer = page.evaluateHandle(\"() => new DataTransfer()\");\nMap<String, Object> arg = new HashMap<>();\narg.put(\"dataTransfer\", dataTransfer);\npage.dispatchEvent(\"#source\", \"dragstart\", arg);\n```\n\n```python async\n# note you can only create data_transfer in chromium and firefox\ndata_transfer = await page.evaluate_handle(\"new DataTransfer()\")\nawait page.dispatch_event(\"#source\", \"dragstart\", { \"dataTransfer\": data_transfer })\n```\n\n```python sync\n# note you can only create data_transfer in chromium and firefox\ndata_transfer = page.evaluate_handle(\"new DataTransfer()\")\npage.dispatch_event(\"#source\", \"dragstart\", { \"dataTransfer\": data_transfer })\n```\n"]
    fn dispatch_event(
        &self,
        #[doc = "A selector to search for element. If there are multiple elements satisfying the selector, the first will be used. See\n[working with selectors](./selectors.md) for more details."]
        selector: String,
        #[doc = "DOM event type: `\"click\"`, `\"dragstart\"`, etc."] r#type: String,
        #[doc = "Optional event-specific initialization properties."] event_init: Option<
            EvaluationArgument
        >,
        #[doc = "options"]
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "This method changes the `CSS media type` through the `media` argument, and/or the `'prefers-colors-scheme'` media\nfeature, using the `colorScheme` argument.\n\n```js\nawait page.evaluate(() => matchMedia('screen').matches);\n// → true\nawait page.evaluate(() => matchMedia('print').matches);\n// → false\n\nawait page.emulateMedia({ media: 'print' });\nawait page.evaluate(() => matchMedia('screen').matches);\n// → false\nawait page.evaluate(() => matchMedia('print').matches);\n// → true\n\nawait page.emulateMedia({});\nawait page.evaluate(() => matchMedia('screen').matches);\n// → true\nawait page.evaluate(() => matchMedia('print').matches);\n// → false\n```\n\n```java\npage.evaluate(\"() => matchMedia('screen').matches\");\n// → true\npage.evaluate(\"() => matchMedia('print').matches\");\n// → false\n\npage.emulateMedia(new Page.EmulateMediaOptions().setMedia(Media.PRINT));\npage.evaluate(\"() => matchMedia('screen').matches\");\n// → false\npage.evaluate(\"() => matchMedia('print').matches\");\n// → true\n\npage.emulateMedia(new Page.EmulateMediaOptions());\npage.evaluate(\"() => matchMedia('screen').matches\");\n// → true\npage.evaluate(\"() => matchMedia('print').matches\");\n// → false\n```\n\n```python async\nawait page.evaluate(\"matchMedia('screen').matches\")\n# → True\nawait page.evaluate(\"matchMedia('print').matches\")\n# → False\n\nawait page.emulate_media(media=\"print\")\nawait page.evaluate(\"matchMedia('screen').matches\")\n# → False\nawait page.evaluate(\"matchMedia('print').matches\")\n# → True\n\nawait page.emulate_media()\nawait page.evaluate(\"matchMedia('screen').matches\")\n# → True\nawait page.evaluate(\"matchMedia('print').matches\")\n# → False\n```\n\n```python sync\npage.evaluate(\"matchMedia('screen').matches\")\n# → True\npage.evaluate(\"matchMedia('print').matches\")\n# → False\n\npage.emulate_media(media=\"print\")\npage.evaluate(\"matchMedia('screen').matches\")\n# → False\npage.evaluate(\"matchMedia('print').matches\")\n# → True\n\npage.emulate_media()\npage.evaluate(\"matchMedia('screen').matches\")\n# → True\npage.evaluate(\"matchMedia('print').matches\")\n# → False\n```\n\n```js\nawait page.emulateMedia({ colorScheme: 'dark' });\nawait page.evaluate(() => matchMedia('(prefers-color-scheme: dark)').matches);\n// → true\nawait page.evaluate(() => matchMedia('(prefers-color-scheme: light)').matches);\n// → false\nawait page.evaluate(() => matchMedia('(prefers-color-scheme: no-preference)').matches);\n// → false\n```\n\n```java\npage.emulateMedia(new Page.EmulateMediaOptions().setColorScheme(ColorScheme.DARK));\npage.evaluate(\"() => matchMedia('(prefers-color-scheme: dark)').matches\");\n// → true\npage.evaluate(\"() => matchMedia('(prefers-color-scheme: light)').matches\");\n// → false\npage.evaluate(\"() => matchMedia('(prefers-color-scheme: no-preference)').matches\");\n// → false\n```\n\n```python async\nawait page.emulate_media(color_scheme=\"dark\")\nawait page.evaluate(\"matchMedia('(prefers-color-scheme: dark)').matches\")\n# → True\nawait page.evaluate(\"matchMedia('(prefers-color-scheme: light)').matches\")\n# → False\nawait page.evaluate(\"matchMedia('(prefers-color-scheme: no-preference)').matches\")\n# → False\n```\n\n```python sync\npage.emulate_media(color_scheme=\"dark\")\npage.evaluate(\"matchMedia('(prefers-color-scheme: dark)').matches\")\n# → True\npage.evaluate(\"matchMedia('(prefers-color-scheme: light)').matches\")\n# → False\npage.evaluate(\"matchMedia('(prefers-color-scheme: no-preference)').matches\")\n```\n"]
    fn emulate_media(
        &self,
        #[doc = "options"]
        #[doc = "Emulates `'prefers-colors-scheme'` media feature, supported values are `'light'`, `'dark'`, `'no-preference'`. Passing\n`null` disables color scheme emulation."]
        color_scheme: Option<Option<ColorScheme>>,
        #[doc = "Changes the CSS media type of the page. The only allowed values are `'screen'`, `'print'` and `null`. Passing `null`\ndisables CSS media emulation."]
        media: Option<Option<Media>>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "The method finds an element matching the specified selector within the page and passes it as a first argument to\n`expression`. If no elements match the selector, the method throws an error. Returns the value of `expression`.\n\nIf `expression` returns a [Promise], then [`method: Page.evalOnSelector`] would wait for the promise to resolve and\nreturn its value.\n\nExamples:\n\n```js\nconst searchValue = await page.$eval('#search', el => el.value);\nconst preloadHref = await page.$eval('link[rel=preload]', el => el.href);\nconst html = await page.$eval('.main-container', (e, suffix) => e.outerHTML + suffix, 'hello');\n```\n\n```java\nString searchValue = (String) page.evalOnSelector(\"#search\", \"el => el.value\");\nString preloadHref = (String) page.evalOnSelector(\"link[rel=preload]\", \"el => el.href\");\nString html = (String) page.evalOnSelector(\".main-container\", \"(e, suffix) => e.outerHTML + suffix\", \"hello\");\n```\n\n```python async\nsearch_value = await page.eval_on_selector(\"#search\", \"el => el.value\")\npreload_href = await page.eval_on_selector(\"link[rel=preload]\", \"el => el.href\")\nhtml = await page.eval_on_selector(\".main-container\", \"(e, suffix) => e.outer_html + suffix\", \"hello\")\n```\n\n```python sync\nsearch_value = page.eval_on_selector(\"#search\", \"el => el.value\")\npreload_href = page.eval_on_selector(\"link[rel=preload]\", \"el => el.href\")\nhtml = page.eval_on_selector(\".main-container\", \"(e, suffix) => e.outer_html + suffix\", \"hello\")\n```\n\nShortcut for main frame's [`method: Frame.evalOnSelector`]."]
    fn eval_on_selector(
        &self,
        #[doc = "A selector to query for. See [working with selectors](./selectors.md) for more details."]
        selector: String,
        #[doc = "JavaScript expression to be evaluated in the browser context. If it looks like a function declaration, it is interpreted\nas a function. Otherwise, evaluated as an expression."]
        expression: String,
        #[doc = "Optional argument to pass to `expression`."] arg: Option<EvaluationArgument>
    ) -> Result<Serializable, Arc<Error>> {
        todo!()
    }
    #[doc = "The method finds all elements matching the specified selector within the page and passes an array of matched elements as\na first argument to `expression`. Returns the result of `expression` invocation.\n\nIf `expression` returns a [Promise], then [`method: Page.evalOnSelectorAll`] would wait for the promise to resolve and\nreturn its value.\n\nExamples:\n\n```js\nconst divCounts = await page.$$eval('div', (divs, min) => divs.length >= min, 10);\n```\n\n```java\nboolean divCounts = (boolean) page.evalOnSelectorAll(\"div\", \"(divs, min) => divs.length >= min\", 10);\n```\n\n```python async\ndiv_counts = await page.eval_on_selector_all(\"div\", \"(divs, min) => divs.length >= min\", 10)\n```\n\n```python sync\ndiv_counts = page.eval_on_selector_all(\"div\", \"(divs, min) => divs.length >= min\", 10)\n```\n"]
    fn eval_on_selector_all(
        &self,
        #[doc = "A selector to query for. See [working with selectors](./selectors.md) for more details."]
        selector: String,
        #[doc = "JavaScript expression to be evaluated in the browser context. If it looks like a function declaration, it is interpreted\nas a function. Otherwise, evaluated as an expression."]
        expression: String,
        #[doc = "Optional argument to pass to `expression`."] arg: Option<EvaluationArgument>
    ) -> Result<Serializable, Arc<Error>> {
        todo!()
    }
    #[doc = "Returns the value of the `expression` invocation.\n\nIf the function passed to the [`method: Page.evaluate`] returns a [Promise], then [`method: Page.evaluate`] would wait\nfor the promise to resolve and return its value.\n\nIf the function passed to the [`method: Page.evaluate`] returns a non-[Serializable] value, then\n[`method: Page.evaluate`] resolves to `undefined`. Playwright also supports transferring some additional values that are\nnot serializable by `JSON`: `-0`, `NaN`, `Infinity`, `-Infinity`.\n\nPassing argument to `expression`:\n\n```js\nconst result = await page.evaluate(([x, y]) => {\n  return Promise.resolve(x * y);\n}, [7, 8]);\nconsole.log(result); // prints \"56\"\n```\n\n```java\nObject result = page.evaluate(\"([x, y]) => {\\n\" +\n  \"  return Promise.resolve(x * y);\\n\" +\n  \"}\", Arrays.asList(7, 8));\nSystem.out.println(result); // prints \"56\"\n```\n\n```python async\nresult = await page.evaluate(\"([x, y]) => Promise.resolve(x * y)\", [7, 8])\nprint(result) # prints \"56\"\n```\n\n```python sync\nresult = page.evaluate(\"([x, y]) => Promise.resolve(x * y)\", [7, 8])\nprint(result) # prints \"56\"\n```\n\nA string can also be passed in instead of a function:\n\n```js\nconsole.log(await page.evaluate('1 + 2')); // prints \"3\"\nconst x = 10;\nconsole.log(await page.evaluate(`1 + ${x}`)); // prints \"11\"\n```\n\n```java\nSystem.out.println(page.evaluate(\"1 + 2\")); // prints \"3\"\n```\n\n```python async\nprint(await page.evaluate(\"1 + 2\")) # prints \"3\"\nx = 10\nprint(await page.evaluate(f\"1 + {x}\")) # prints \"11\"\n```\n\n```python sync\nprint(page.evaluate(\"1 + 2\")) # prints \"3\"\nx = 10\nprint(page.evaluate(f\"1 + {x}\")) # prints \"11\"\n```\n\n`ElementHandle` instances can be passed as an argument to the [`method: Page.evaluate`]:\n\n```js\nconst bodyHandle = await page.$('body');\nconst html = await page.evaluate(([body, suffix]) => body.innerHTML + suffix, [bodyHandle, 'hello']);\nawait bodyHandle.dispose();\n```\n\n```java\nElementHandle bodyHandle = page.querySelector(\"body\");\nString html = (String) page.evaluate(\"([body, suffix]) => body.innerHTML + suffix\", Arrays.asList(bodyHandle, \"hello\"));\nbodyHandle.dispose();\n```\n\n```python async\nbody_handle = await page.query_selector(\"body\")\nhtml = await page.evaluate(\"([body, suffix]) => body.innerHTML + suffix\", [body_handle, \"hello\"])\nawait body_handle.dispose()\n```\n\n```python sync\nbody_handle = page.query_selector(\"body\")\nhtml = page.evaluate(\"([body, suffix]) => body.innerHTML + suffix\", [body_handle, \"hello\"])\nbody_handle.dispose()\n```\n\nShortcut for main frame's [`method: Frame.evaluate`]."]
    fn evaluate(
        &self,
        #[doc = "JavaScript expression to be evaluated in the browser context. If it looks like a function declaration, it is interpreted\nas a function. Otherwise, evaluated as an expression."]
        expression: String,
        #[doc = "Optional argument to pass to `expression`."] arg: Option<EvaluationArgument>
    ) -> Result<Serializable, Arc<Error>> {
        todo!()
    }
    #[doc = "Returns the value of the `expression` invocation as a `JSHandle`.\n\nThe only difference between [`method: Page.evaluate`] and [`method: Page.evaluateHandle`] is that\n[`method: Page.evaluateHandle`] returns `JSHandle`.\n\nIf the function passed to the [`method: Page.evaluateHandle`] returns a [Promise], then [`method: Page.evaluateHandle`]\nwould wait for the promise to resolve and return its value.\n\n```js\nconst aWindowHandle = await page.evaluateHandle(() => Promise.resolve(window));\naWindowHandle; // Handle for the window object.\n```\n\n```java\n// Handle for the window object.\nJSHandle aWindowHandle = page.evaluateHandle(\"() => Promise.resolve(window)\");\n```\n\n```python async\na_window_handle = await page.evaluate_handle(\"Promise.resolve(window)\")\na_window_handle # handle for the window object.\n```\n\n```python sync\na_window_handle = page.evaluate_handle(\"Promise.resolve(window)\")\na_window_handle # handle for the window object.\n```\n\nA string can also be passed in instead of a function:\n\n```js\nconst aHandle = await page.evaluateHandle('document'); // Handle for the 'document'\n```\n\n```java\nJSHandle aHandle = page.evaluateHandle(\"document\"); // Handle for the \"document\".\n```\n\n```python async\na_handle = await page.evaluate_handle(\"document\") # handle for the \"document\"\n```\n\n```python sync\na_handle = page.evaluate_handle(\"document\") # handle for the \"document\"\n```\n\n`JSHandle` instances can be passed as an argument to the [`method: Page.evaluateHandle`]:\n\n```js\nconst aHandle = await page.evaluateHandle(() => document.body);\nconst resultHandle = await page.evaluateHandle(body => body.innerHTML, aHandle);\nconsole.log(await resultHandle.jsonValue());\nawait resultHandle.dispose();\n```\n\n```java\nJSHandle aHandle = page.evaluateHandle(\"() => document.body\");\nJSHandle resultHandle = page.evaluateHandle(\"([body, suffix]) => body.innerHTML + suffix\", Arrays.asList(aHandle, \"hello\"));\nSystem.out.println(resultHandle.jsonValue());\nresultHandle.dispose();\n```\n\n```python async\na_handle = await page.evaluate_handle(\"document.body\")\nresult_handle = await page.evaluate_handle(\"body => body.innerHTML\", a_handle)\nprint(await result_handle.json_value())\nawait result_handle.dispose()\n```\n\n```python sync\na_handle = page.evaluate_handle(\"document.body\")\nresult_handle = page.evaluate_handle(\"body => body.innerHTML\", a_handle)\nprint(result_handle.json_value())\nresult_handle.dispose()\n```\n"]
    fn evaluate_handle(
        &self,
        #[doc = "JavaScript expression to be evaluated in the browser context. If it looks like a function declaration, it is interpreted\nas a function. Otherwise, evaluated as an expression."]
        expression: String,
        #[doc = "Optional argument to pass to `expression`."] arg: Option<EvaluationArgument>
    ) -> Result<JsHandle, Arc<Error>> {
        todo!()
    }
    #[doc = "The method adds a function called `name` on the `window` object of every frame in this page. When called, the function\nexecutes `callback` and returns a [Promise] which resolves to the return value of `callback`. If the `callback` returns\na [Promise], it will be awaited.\n\nThe first argument of the `callback` function contains information about the caller: `{ browserContext: BrowserContext,\npage: Page, frame: Frame }`.\n\nSee [`method: BrowserContext.exposeBinding`] for the context-wide version.\n\n> NOTE: Functions installed via [`method: Page.exposeBinding`] survive navigations.\n\nAn example of exposing page URL to all frames in a page:\n\n```js\nconst { webkit } = require('playwright');  // Or 'chromium' or 'firefox'.\n\n(async () => {\n  const browser = await webkit.launch({ headless: false });\n  const context = await browser.newContext();\n  const page = await context.newPage();\n  await page.exposeBinding('pageURL', ({ page }) => page.url());\n  await page.setContent(`\n    <script>\n      async function onClick() {\n        document.querySelector('div').textContent = await window.pageURL();\n      }\n    </script>\n    <button onclick=\"onClick()\">Click me</button>\n    <div></div>\n  `);\n  await page.click('button');\n})();\n```\n\n```java\nimport com.microsoft.playwright.*;\n\npublic class Example {\n  public static void main(String[] args) {\n    try (Playwright playwright = Playwright.create()) {\n      BrowserType webkit = playwright.webkit();\n      Browser browser = webkit.launch({ headless: false });\n      BrowserContext context = browser.newContext();\n      Page page = context.newPage();\n      page.exposeBinding(\"pageURL\", (source, args) -> source.page().url());\n      page.setContent(\"<script>\\n\" +\n        \"  async function onClick() {\\n\" +\n        \"    document.querySelector('div').textContent = await window.pageURL();\\n\" +\n        \"  }\\n\" +\n        \"</script>\\n\" +\n        \"<button onclick=\\\"onClick()\\\">Click me</button>\\n\" +\n        \"<div></div>\");\n      page.click(\"button\");\n    }\n  }\n}\n```\n\n```python async\nimport asyncio\nfrom playwright.async_api import async_playwright\n\nasync def run(playwright):\n    webkit = playwright.webkit\n    browser = await webkit.launch(headless=false)\n    context = await browser.new_context()\n    page = await context.new_page()\n    await page.expose_binding(\"pageURL\", lambda source: source[\"page\"].url)\n    await page.set_content(\"\"\"\n    <script>\n      async function onClick() {\n        document.querySelector('div').textContent = await window.pageURL();\n      }\n    </script>\n    <button onclick=\"onClick()\">Click me</button>\n    <div></div>\n    \"\"\")\n    await page.click(\"button\")\n\nasync def main():\n    async with async_playwright() as playwright:\n        await run(playwright)\nasyncio.run(main())\n```\n\n```python sync\nfrom playwright.sync_api import sync_playwright\n\ndef run(playwright):\n    webkit = playwright.webkit\n    browser = webkit.launch(headless=false)\n    context = browser.new_context()\n    page = context.new_page()\n    page.expose_binding(\"pageURL\", lambda source: source[\"page\"].url)\n    page.set_content(\"\"\"\n    <script>\n      async function onClick() {\n        document.querySelector('div').textContent = await window.pageURL();\n      }\n    </script>\n    <button onclick=\"onClick()\">Click me</button>\n    <div></div>\n    \"\"\")\n    page.click(\"button\")\n\nwith sync_playwright() as playwright:\n    run(playwright)\n```\n\nAn example of passing an element handle:\n\n```js\nawait page.exposeBinding('clicked', async (source, element) => {\n  console.log(await element.textContent());\n}, { handle: true });\nawait page.setContent(`\n  <script>\n    document.addEventListener('click', event => window.clicked(event.target));\n  </script>\n  <div>Click me</div>\n  <div>Or click me</div>\n`);\n```\n\n```java\npage.exposeBinding(\"clicked\", (source, args) -> {\n  ElementHandle element = (ElementHandle) args[0];\n  System.out.println(element.textContent());\n  return null;\n}, new Page.ExposeBindingOptions().setHandle(true));\npage.setContent(\"\" +\n  \"<script>\\n\" +\n  \"  document.addEventListener('click', event => window.clicked(event.target));\\n\" +\n  \"</script>\\n\" +\n  \"<div>Click me</div>\\n\" +\n  \"<div>Or click me</div>\\n\");\n```\n\n```python async\nasync def print(source, element):\n    print(await element.text_content())\n\nawait page.expose_binding(\"clicked\", print, handle=true)\nawait page.set_content(\"\"\"\n  <script>\n    document.addEventListener('click', event => window.clicked(event.target));\n  </script>\n  <div>Click me</div>\n  <div>Or click me</div>\n\"\"\")\n```\n\n```python sync\ndef print(source, element):\n    print(element.text_content())\n\npage.expose_binding(\"clicked\", print, handle=true)\npage.set_content(\"\"\"\n  <script>\n    document.addEventListener('click', event => window.clicked(event.target));\n  </script>\n  <div>Click me</div>\n  <div>Or click me</div>\n\"\"\")\n```\n"]
    fn expose_binding(
        &self,
        #[doc = "Name of the function on the window object."] name: String,
        callback : function,
        #[doc = "options"]
        #[doc = "Whether to pass the argument as a handle, instead of passing by value. When passing a handle, only one argument is\nsupported. When passing by value, multiple arguments are supported."]
        handle: Option<bool>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "The method adds a function called `name` on the `window` object of every frame in the page. When called, the function\nexecutes `callback` and returns a [Promise] which resolves to the return value of `callback`.\n\nIf the `callback` returns a [Promise], it will be awaited.\n\nSee [`method: BrowserContext.exposeFunction`] for context-wide exposed function.\n\n> NOTE: Functions installed via [`method: Page.exposeFunction`] survive navigations.\n\nAn example of adding an `sha1` function to the page:\n\n```js\nconst { webkit } = require('playwright');  // Or 'chromium' or 'firefox'.\nconst crypto = require('crypto');\n\n(async () => {\n  const browser = await webkit.launch({ headless: false });\n  const page = await browser.newPage();\n  await page.exposeFunction('sha1', text => crypto.createHash('sha1').update(text).digest('hex'));\n  await page.setContent(`\n    <script>\n      async function onClick() {\n        document.querySelector('div').textContent = await window.sha1('PLAYWRIGHT');\n      }\n    </script>\n    <button onclick=\"onClick()\">Click me</button>\n    <div></div>\n  `);\n  await page.click('button');\n})();\n```\n\n```java\nimport com.microsoft.playwright.*;\n\nimport java.nio.charset.StandardCharsets;\nimport java.security.MessageDigest;\nimport java.security.NoSuchAlgorithmException;\nimport java.util.Base64;\n\npublic class Example {\n  public static void main(String[] args) {\n    try (Playwright playwright = Playwright.create()) {\n      BrowserType webkit = playwright.webkit();\n      Browser browser = webkit.launch({ headless: false });\n      Page page = browser.newPage();\n      page.exposeFunction(\"sha1\", args -> {\n        String text = (String) args[0];\n        MessageDigest crypto;\n        try {\n          crypto = MessageDigest.getInstance(\"SHA-1\");\n        } catch (NoSuchAlgorithmException e) {\n          return null;\n        }\n        byte[] token = crypto.digest(text.getBytes(StandardCharsets.UTF_8));\n        return Base64.getEncoder().encodeToString(token);\n      });\n      page.setContent(\"<script>\\n\" +\n        \"  async function onClick() {\\n\" +\n        \"    document.querySelector('div').textContent = await window.sha1('PLAYWRIGHT');\\n\" +\n        \"  }\\n\" +\n        \"</script>\\n\" +\n        \"<button onclick=\\\"onClick()\\\">Click me</button>\\n\" +\n        \"<div></div>\\n\");\n      page.click(\"button\");\n    }\n  }\n}\n```\n\n```python async\nimport asyncio\nimport hashlib\nfrom playwright.async_api import async_playwright\n\nasync def sha1(text):\n    m = hashlib.sha1()\n    m.update(bytes(text, \"utf8\"))\n    return m.hexdigest()\n\n\nasync def run(playwright):\n    webkit = playwright.webkit\n    browser = await webkit.launch(headless=False)\n    page = await browser.new_page()\n    await page.expose_function(\"sha1\", sha1)\n    await page.set_content(\"\"\"\n        <script>\n          async function onClick() {\n            document.querySelector('div').textContent = await window.sha1('PLAYWRIGHT');\n          }\n        </script>\n        <button onclick=\"onClick()\">Click me</button>\n        <div></div>\n    \"\"\")\n    await page.click(\"button\")\n\nasync def main():\n    async with async_playwright() as playwright:\n        await run(playwright)\nasyncio.run(main())\n```\n\n```python sync\nimport hashlib\nfrom playwright.sync_api import sync_playwright\n\ndef sha1(text):\n    m = hashlib.sha1()\n    m.update(bytes(text, \"utf8\"))\n    return m.hexdigest()\n\n\ndef run(playwright):\n    webkit = playwright.webkit\n    browser = webkit.launch(headless=False)\n    page = browser.new_page()\n    page.expose_function(\"sha1\", sha1)\n    page.set_content(\"\"\"\n        <script>\n          async function onClick() {\n            document.querySelector('div').textContent = await window.sha1('PLAYWRIGHT');\n          }\n        </script>\n        <button onclick=\"onClick()\">Click me</button>\n        <div></div>\n    \"\"\")\n    page.click(\"button\")\n\nwith sync_playwright() as playwright:\n    run(playwright)\n```\n"]
    fn expose_function(
        &self,
        #[doc = "Name of the function on the window object"] name: String,
        callback : function
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "This method waits for an element matching `selector`, waits for [actionability](https://playwright.dev/docs/actionability/) checks, focuses the\nelement, fills it and triggers an `input` event after filling. If the element is inside the `<label>` element that has\nassociated [control](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLabelElement/control), that control will be\nfilled instead. If the element to be filled is not an `<input>`, `<textarea>` or `[contenteditable]` element, this\nmethod throws an error. Note that you can pass an empty string to clear the input field.\n\nTo send fine-grained keyboard events, use [`method: Page.type`].\n\nShortcut for main frame's [`method: Frame.fill`]"]
    fn fill(
        &self,
        #[doc = "A selector to search for element. If there are multiple elements satisfying the selector, the first will be used. See\n[working with selectors](./selectors.md) for more details."]
        selector: String,
        #[doc = "Value to fill for the `<input>`, `<textarea>` or `[contenteditable]` element."]
        value: String,
        #[doc = "options"]
        #[doc = "Actions that initiate navigations are waiting for these navigations to happen and for pages to start loading. You can\nopt out of waiting via setting this flag. You would only need this option in the exceptional cases such as navigating to\ninaccessible pages. Defaults to `false`."]
        no_wait_after: Option<bool>,
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "This method fetches an element with `selector` and focuses it. If there's no element matching `selector`, the method\nwaits until a matching element appears in the DOM.\n\nShortcut for main frame's [`method: Frame.focus`]."]
    fn focus(
        &self,
        #[doc = "A selector to search for element. If there are multiple elements satisfying the selector, the first will be used. See\n[working with selectors](./selectors.md) for more details."]
        selector: String,
        #[doc = "options"]
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Returns frame matching the specified criteria. Either `name` or `url` must be specified.\n\n```js\nconst frame = page.frame('frame-name');\n```\n\n```java\nFrame frame = page.frame(\"frame-name\");\n```\n\n```py\nframe = page.frame(name=\"frame-name\")\n```\n\n```js\nconst frame = page.frame({ url: /.*domain.*/ });\n```\n\n```java\nFrame frame = page.frameByUrl(Pattern.compile(\".*domain.*\");\n```\n\n```py\nframe = page.frame(url=r\".*domain.*\")\n```\n"]
    fn frame(
        &self,
        #[doc = "Frame name or other frame lookup options."] frame_selector: NotImplementedYet,
        #[doc = "Frame name specified in the `iframe`'s `name` attribute."] name: String,
        #[doc = "options"]
        #[doc = "Frame name specified in the `iframe`'s `name` attribute. Optional."]
        name: Option<String>,
        #[doc = "A glob pattern, regex pattern or predicate receiving frame's `url` as a [URL] object. Optional."]
        url: Option<NotImplementedYet>
    ) -> Result<Option<Frame>, Error> {
        todo!()
    }
    #[doc = "Returns frame with matching URL."]
    fn frame_by_url(
        &self,
        #[doc = "A glob pattern, regex pattern or predicate receiving frame's `url` as a [URL] object."]
        url: NotImplementedYet
    ) -> Result<Option<Frame>, Error> {
        todo!()
    }
    #[doc = "An array of all frames attached to the page."]
    fn frames(&self) -> Result<Vec<Frame>, Error> { todo!() }
    #[doc = "Returns element attribute value."]
    fn get_attribute(
        &self,
        #[doc = "A selector to search for element. If there are multiple elements satisfying the selector, the first will be used. See\n[working with selectors](./selectors.md) for more details."]
        selector: String,
        #[doc = "Attribute name to get the value for."] name: String,
        #[doc = "options"]
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>
    ) -> Result<Option<String>, Arc<Error>> {
        todo!()
    }
    #[doc = "Returns the main resource response. In case of multiple redirects, the navigation will resolve with the response of the\nlast redirect. If can not go back, returns `null`.\n\nNavigate to the previous page in history."]
    fn go_back(
        &self,
        #[doc = "options"]
        #[doc = "Maximum operation time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be\nchanged by using the [`method: BrowserContext.setDefaultNavigationTimeout`],\n[`method: BrowserContext.setDefaultTimeout`], [`method: Page.setDefaultNavigationTimeout`] or\n[`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>,
        #[doc = "When to consider operation succeeded, defaults to `load`. Events can be either:\n- `'domcontentloaded'` - consider operation to be finished when the `DOMContentLoaded` event is fired.\n- `'load'` - consider operation to be finished when the `load` event is fired.\n- `'networkidle'` - consider operation to be finished when there are no network connections for at least `500` ms."]
        wait_until: Option<WaitUntilState>
    ) -> Result<Option<Response>, Arc<Error>> {
        todo!()
    }
    #[doc = "Returns the main resource response. In case of multiple redirects, the navigation will resolve with the response of the\nlast redirect. If can not go forward, returns `null`.\n\nNavigate to the next page in history."]
    fn go_forward(
        &self,
        #[doc = "options"]
        #[doc = "Maximum operation time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be\nchanged by using the [`method: BrowserContext.setDefaultNavigationTimeout`],\n[`method: BrowserContext.setDefaultTimeout`], [`method: Page.setDefaultNavigationTimeout`] or\n[`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>,
        #[doc = "When to consider operation succeeded, defaults to `load`. Events can be either:\n- `'domcontentloaded'` - consider operation to be finished when the `DOMContentLoaded` event is fired.\n- `'load'` - consider operation to be finished when the `load` event is fired.\n- `'networkidle'` - consider operation to be finished when there are no network connections for at least `500` ms."]
        wait_until: Option<WaitUntilState>
    ) -> Result<Option<Response>, Arc<Error>> {
        todo!()
    }
    #[doc = "Returns the main resource response. In case of multiple redirects, the navigation will resolve with the response of the\nlast redirect.\n\n`page.goto` will throw an error if:\n- there's an SSL error (e.g. in case of self-signed certificates).\n- target URL is invalid.\n- the `timeout` is exceeded during navigation.\n- the remote server does not respond or is unreachable.\n- the main resource failed to load.\n\n`page.goto` will not throw an error when any valid HTTP status code is returned by the remote server, including 404 \"Not\nFound\" and 500 \"Internal Server Error\".  The status code for such responses can be retrieved by calling\n[`method: Response.status`].\n\n> NOTE: `page.goto` either throws an error or returns a main resource response. The only exceptions are navigation to\n`about:blank` or navigation to the same URL with a different hash, which would succeed and return `null`.\n> NOTE: Headless mode doesn't support navigation to a PDF document. See the\n[upstream issue](https://bugs.chromium.org/p/chromium/issues/detail?id=761295).\n\nShortcut for main frame's [`method: Frame.goto`]"]
    fn goto(
        &self,
        url : String,
        #[doc = "options"]
        #[doc = "Referer header value. If provided it will take preference over the referer header value set by\n[`method: Page.setExtraHTTPHeaders`]."]
        referer: Option<String>,
        #[doc = "Maximum operation time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be\nchanged by using the [`method: BrowserContext.setDefaultNavigationTimeout`],\n[`method: BrowserContext.setDefaultTimeout`], [`method: Page.setDefaultNavigationTimeout`] or\n[`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>,
        #[doc = "When to consider operation succeeded, defaults to `load`. Events can be either:\n- `'domcontentloaded'` - consider operation to be finished when the `DOMContentLoaded` event is fired.\n- `'load'` - consider operation to be finished when the `load` event is fired.\n- `'networkidle'` - consider operation to be finished when there are no network connections for at least `500` ms."]
        wait_until: Option<WaitUntilState>
    ) -> Result<Option<Response>, Arc<Error>> {
        todo!()
    }
    #[doc = "This method hovers over an element matching `selector` by performing the following steps:\n1. Find an element matching `selector`. If there is none, wait until a matching element is attached to the DOM.\n1. Wait for [actionability](https://playwright.dev/docs/actionability/) checks on the matched element, unless `force` option is set. If the\n   element is detached during the checks, the whole action is retried.\n1. Scroll the element into view if needed.\n1. Use [`property: Page.mouse`] to hover over the center of the element, or the specified `position`.\n1. Wait for initiated navigations to either succeed or fail, unless `noWaitAfter` option is set.\n\nWhen all steps combined have not finished during the specified `timeout`, this method throws a `TimeoutError`. Passing\nzero timeout disables this.\n\nShortcut for main frame's [`method: Frame.hover`]."]
    fn hover(
        &self,
        #[doc = "A selector to search for element. If there are multiple elements satisfying the selector, the first will be used. See\n[working with selectors](./selectors.md) for more details."]
        selector: String,
        #[doc = "options"]
        #[doc = "Whether to bypass the [actionability](https://playwright.dev/docs/actionability/) checks. Defaults to `false`."]
        force: Option<bool>,
        #[doc = "Modifier keys to press. Ensures that only these modifiers are pressed during the operation, and then restores current\nmodifiers back. If not specified, currently pressed modifiers are used."]
        modifiers: Option<Vec<KeyboardModifier>>,
        #[doc = "A point to use relative to the top-left corner of element padding box. If not specified, uses some visible point of the\nelement."]
        position: Option<NotImplementedYet>,
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>,
        #[doc = "When set, this method only performs the [actionability](https://playwright.dev/docs/actionability/) checks and skips the action. Defaults to\n`false`. Useful to wait until the element is ready for the action without performing it."]
        trial: Option<bool>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Returns `element.innerHTML`."]
    fn inner_h_t_m_l(
        &self,
        #[doc = "A selector to search for element. If there are multiple elements satisfying the selector, the first will be used. See\n[working with selectors](./selectors.md) for more details."]
        selector: String,
        #[doc = "options"]
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>
    ) -> Result<String, Arc<Error>> {
        todo!()
    }
    #[doc = "Returns `element.innerText`."]
    fn inner_text(
        &self,
        #[doc = "A selector to search for element. If there are multiple elements satisfying the selector, the first will be used. See\n[working with selectors](./selectors.md) for more details."]
        selector: String,
        #[doc = "options"]
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>
    ) -> Result<String, Arc<Error>> {
        todo!()
    }
    #[doc = "Returns whether the element is checked. Throws if the element is not a checkbox or radio input."]
    fn is_checked(
        &self,
        #[doc = "A selector to search for element. If there are multiple elements satisfying the selector, the first will be used. See\n[working with selectors](./selectors.md) for more details."]
        selector: String,
        #[doc = "options"]
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>
    ) -> Result<bool, Arc<Error>> {
        todo!()
    }
    #[doc = "Indicates that the page has been closed."]
    fn is_closed(&self) -> Result<bool, Error> { todo!() }
    #[doc = "Returns whether the element is disabled, the opposite of [enabled](./actionability.md#enabled)."]
    fn is_disabled(
        &self,
        #[doc = "A selector to search for element. If there are multiple elements satisfying the selector, the first will be used. See\n[working with selectors](./selectors.md) for more details."]
        selector: String,
        #[doc = "options"]
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>
    ) -> Result<bool, Arc<Error>> {
        todo!()
    }
    #[doc = "Returns whether the element is [editable](./actionability.md#editable)."]
    fn is_editable(
        &self,
        #[doc = "A selector to search for element. If there are multiple elements satisfying the selector, the first will be used. See\n[working with selectors](./selectors.md) for more details."]
        selector: String,
        #[doc = "options"]
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>
    ) -> Result<bool, Arc<Error>> {
        todo!()
    }
    #[doc = "Returns whether the element is [enabled](./actionability.md#enabled)."]
    fn is_enabled(
        &self,
        #[doc = "A selector to search for element. If there are multiple elements satisfying the selector, the first will be used. See\n[working with selectors](./selectors.md) for more details."]
        selector: String,
        #[doc = "options"]
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>
    ) -> Result<bool, Arc<Error>> {
        todo!()
    }
    #[doc = "Returns whether the element is hidden, the opposite of [visible](./actionability.md#visible).  `selector` that does not\nmatch any elements is considered hidden."]
    fn is_hidden(
        &self,
        #[doc = "A selector to search for element. If there are multiple elements satisfying the selector, the first will be used. See\n[working with selectors](./selectors.md) for more details."]
        selector: String,
        #[doc = "options"]
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>
    ) -> Result<bool, Arc<Error>> {
        todo!()
    }
    #[doc = "Returns whether the element is [visible](./actionability.md#visible). `selector` that does not match any elements is\nconsidered not visible."]
    fn is_visible(
        &self,
        #[doc = "A selector to search for element. If there are multiple elements satisfying the selector, the first will be used. See\n[working with selectors](./selectors.md) for more details."]
        selector: String,
        #[doc = "options"]
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>
    ) -> Result<bool, Arc<Error>> {
        todo!()
    }
    #[doc = "The page's main frame. Page is guaranteed to have a main frame which persists during navigations."]
    fn main_frame(&self) -> Result<Frame, Error> { todo!() }
    #[doc = "Returns the opener for popup pages and `null` for others. If the opener has been closed already the returns `null`."]
    fn opener(&self) -> Result<Option<Page>, Arc<Error>> { todo!() }
    #[doc = "Pauses script execution. Playwright will stop executing the script and wait for the user to either press 'Resume' button\nin the page overlay or to call `playwright.resume()` in the DevTools console.\n\nUser can inspect selectors or perform manual steps while paused. Resume will continue running the original script from\nthe place it was paused.\n\n> NOTE: This method requires Playwright to be started in a headed mode, with a falsy `headless` value in the\n[`method: BrowserType.launch`]."]
    fn pause(&self) -> Result<(), Arc<Error>> { todo!() }
    #[doc = "Returns the PDF buffer.\n\n> NOTE: Generating a pdf is currently only supported in Chromium headless.\n\n`page.pdf()` generates a pdf of the page with `print` css media. To generate a pdf with `screen` media, call\n[`method: Page.emulateMedia`] before calling `page.pdf()`:\n\n> NOTE: By default, `page.pdf()` generates a pdf with modified colors for printing. Use the\n[`-webkit-print-color-adjust`](https://developer.mozilla.org/en-US/docs/Web/CSS/-webkit-print-color-adjust) property to\nforce rendering of exact colors.\n\n```js\n// Generates a PDF with 'screen' media type.\nawait page.emulateMedia({media: 'screen'});\nawait page.pdf({path: 'page.pdf'});\n```\n\n```java\n// Generates a PDF with \"screen\" media type.\npage.emulateMedia(new Page.EmulateMediaOptions().setMedia(Media.SCREEN));\npage.pdf(new Page.PdfOptions().setPath(Paths.get(\"page.pdf\")));\n```\n\n```python async\n# generates a pdf with \"screen\" media type.\nawait page.emulate_media(media=\"screen\")\nawait page.pdf(path=\"page.pdf\")\n```\n\n```python sync\n# generates a pdf with \"screen\" media type.\npage.emulate_media(media=\"screen\")\npage.pdf(path=\"page.pdf\")\n```\n\nThe `width`, `height`, and `margin` options accept values labeled with units. Unlabeled values are treated as pixels.\n\nA few examples:\n- `page.pdf({width: 100})` - prints with width set to 100 pixels\n- `page.pdf({width: '100px'})` - prints with width set to 100 pixels\n- `page.pdf({width: '10cm'})` - prints with width set to 10 centimeters.\n\nAll possible units are:\n- `px` - pixel\n- `in` - inch\n- `cm` - centimeter\n- `mm` - millimeter\n\nThe `format` options are:\n- `Letter`: 8.5in x 11in\n- `Legal`: 8.5in x 14in\n- `Tabloid`: 11in x 17in\n- `Ledger`: 17in x 11in\n- `A0`: 33.1in x 46.8in\n- `A1`: 23.4in x 33.1in\n- `A2`: 16.54in x 23.4in\n- `A3`: 11.7in x 16.54in\n- `A4`: 8.27in x 11.7in\n- `A5`: 5.83in x 8.27in\n- `A6`: 4.13in x 5.83in\n\n> NOTE: `headerTemplate` and `footerTemplate` markup have the following limitations: > 1. Script tags inside templates\nare not evaluated. > 2. Page styles are not visible inside templates."]
    fn pdf(
        &self,
        #[doc = "options"]
        #[doc = "Display header and footer. Defaults to `false`."]
        display_header_footer: Option<bool>,
        #[doc = "HTML template for the print footer. Should use the same format as the `headerTemplate`."]
        footer_template: Option<String>,
        #[doc = "Paper format. If set, takes priority over `width` or `height` options. Defaults to 'Letter'."]
        format: Option<String>,
        #[doc = "HTML template for the print header. Should be valid HTML markup with following classes used to inject printing values\ninto them:\n- `'date'` formatted print date\n- `'title'` document title\n- `'url'` document location\n- `'pageNumber'` current page number\n- `'totalPages'` total pages in the document"]
        header_template: Option<String>,
        #[doc = "Paper height, accepts values labeled with units."] height: Option<
            NotImplementedYet
        >,
        #[doc = "Paper height, accepts values labeled with units."] height: Option<String>,
        #[doc = "Paper orientation. Defaults to `false`."] landscape: Option<bool>,
        #[doc = "Paper margins, defaults to none."] margin: Option<NotImplementedYet>,
        #[doc = "Paper margins, defaults to none."] margin: Option<NotImplementedYet>,
        #[doc = "Paper ranges to print, e.g., '1-5, 8, 11-13'. Defaults to the empty string, which means print all pages."]
        page_ranges: Option<String>,
        #[doc = "The file path to save the PDF to. If `path` is a relative path, then it is resolved relative to the current working\ndirectory. If no path is provided, the PDF won't be saved to the disk."]
        path: Option<path>,
        #[doc = "Give any CSS `@page` size declared in the page priority over what is declared in `width` and `height` or `format`\noptions. Defaults to `false`, which will scale the content to fit the paper size."]
        prefer_c_s_s_page_size: Option<bool>,
        #[doc = "Print background graphics. Defaults to `false`."] print_background: Option<bool>,
        #[doc = "Scale of the webpage rendering. Defaults to `1`. Scale amount must be between 0.1 and 2."]
        scale: Option<f64>,
        #[doc = "Paper width, accepts values labeled with units."] width: Option<NotImplementedYet>,
        #[doc = "Paper width, accepts values labeled with units."] width: Option<String>
    ) -> Result<Buffer, Arc<Error>> {
        todo!()
    }
    #[doc = "Focuses the element, and then uses [`method: Keyboard.down`] and [`method: Keyboard.up`].\n\n`key` can specify the intended [keyboardEvent.key](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key)\nvalue or a single character to generate the text for. A superset of the `key` values can be found\n[here](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key/Key_Values). Examples of the keys are:\n\n`F1` - `F12`, `Digit0`- `Digit9`, `KeyA`- `KeyZ`, `Backquote`, `Minus`, `Equal`, `Backslash`, `Backspace`, `Tab`,\n`Delete`, `Escape`, `ArrowDown`, `End`, `Enter`, `Home`, `Insert`, `PageDown`, `PageUp`, `ArrowRight`, `ArrowUp`, etc.\n\nFollowing modification shortcuts are also supported: `Shift`, `Control`, `Alt`, `Meta`, `ShiftLeft`.\n\nHolding down `Shift` will type the text that corresponds to the `key` in the upper case.\n\nIf `key` is a single character, it is case-sensitive, so the values `a` and `A` will generate different respective\ntexts.\n\nShortcuts such as `key: \"Control+o\"` or `key: \"Control+Shift+T\"` are supported as well. When specified with the\nmodifier, modifier is pressed and being held while the subsequent key is being pressed.\n\n```js\nconst page = await browser.newPage();\nawait page.goto('https://keycode.info');\nawait page.press('body', 'A');\nawait page.screenshot({ path: 'A.png' });\nawait page.press('body', 'ArrowLeft');\nawait page.screenshot({ path: 'ArrowLeft.png' });\nawait page.press('body', 'Shift+O');\nawait page.screenshot({ path: 'O.png' });\nawait browser.close();\n```\n\n```java\nPage page = browser.newPage();\npage.navigate(\"https://keycode.info\");\npage.press(\"body\", \"A\");\npage.screenshot(new Page.ScreenshotOptions().setPath(Paths.get(\"A.png\")));\npage.press(\"body\", \"ArrowLeft\");\npage.screenshot(new Page.ScreenshotOptions().setPath(Paths.get(\"ArrowLeft.png\" )));\npage.press(\"body\", \"Shift+O\");\npage.screenshot(new Page.ScreenshotOptions().setPath(Paths.get(\"O.png\" )));\n```\n\n```python async\npage = await browser.new_page()\nawait page.goto(\"https://keycode.info\")\nawait page.press(\"body\", \"A\")\nawait page.screenshot(path=\"a.png\")\nawait page.press(\"body\", \"ArrowLeft\")\nawait page.screenshot(path=\"arrow_left.png\")\nawait page.press(\"body\", \"Shift+O\")\nawait page.screenshot(path=\"o.png\")\nawait browser.close()\n```\n\n```python sync\npage = browser.new_page()\npage.goto(\"https://keycode.info\")\npage.press(\"body\", \"A\")\npage.screenshot(path=\"a.png\")\npage.press(\"body\", \"ArrowLeft\")\npage.screenshot(path=\"arrow_left.png\")\npage.press(\"body\", \"Shift+O\")\npage.screenshot(path=\"o.png\")\nbrowser.close()\n```\n"]
    fn press(
        &self,
        #[doc = "A selector to search for element. If there are multiple elements satisfying the selector, the first will be used. See\n[working with selectors](./selectors.md) for more details."]
        selector: String,
        #[doc = "Name of the key to press or a character to generate, such as `ArrowLeft` or `a`."]
        key: String,
        #[doc = "options"]
        #[doc = "Time to wait between `keydown` and `keyup` in milliseconds. Defaults to 0."]
        delay: Option<f64>,
        #[doc = "Actions that initiate navigations are waiting for these navigations to happen and for pages to start loading. You can\nopt out of waiting via setting this flag. You would only need this option in the exceptional cases such as navigating to\ninaccessible pages. Defaults to `false`."]
        no_wait_after: Option<bool>,
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "The method finds an element matching the specified selector within the page. If no elements match the selector, the\nreturn value resolves to `null`.\n\nShortcut for main frame's [`method: Frame.querySelector`]."]
    fn query_selector(
        &self,
        #[doc = "A selector to query for. See [working with selectors](./selectors.md) for more details."]
        selector: String
    ) -> Result<Option<ElementHandle>, Arc<Error>> {
        todo!()
    }
    #[doc = "The method finds all elements matching the specified selector within the page. If no elements match the selector, the\nreturn value resolves to `[]`.\n\nShortcut for main frame's [`method: Frame.querySelectorAll`]."]
    fn query_selector_all(
        &self,
        #[doc = "A selector to query for. See [working with selectors](./selectors.md) for more details."]
        selector: String
    ) -> Result<Vec<ElementHandle>, Arc<Error>> {
        todo!()
    }
    #[doc = "Returns the main resource response. In case of multiple redirects, the navigation will resolve with the response of the\nlast redirect."]
    fn reload(
        &self,
        #[doc = "options"]
        #[doc = "Maximum operation time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be\nchanged by using the [`method: BrowserContext.setDefaultNavigationTimeout`],\n[`method: BrowserContext.setDefaultTimeout`], [`method: Page.setDefaultNavigationTimeout`] or\n[`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>,
        #[doc = "When to consider operation succeeded, defaults to `load`. Events can be either:\n- `'domcontentloaded'` - consider operation to be finished when the `DOMContentLoaded` event is fired.\n- `'load'` - consider operation to be finished when the `load` event is fired.\n- `'networkidle'` - consider operation to be finished when there are no network connections for at least `500` ms."]
        wait_until: Option<WaitUntilState>
    ) -> Result<Option<Response>, Arc<Error>> {
        todo!()
    }
    #[doc = "Routing provides the capability to modify network requests that are made by a page.\n\nOnce routing is enabled, every request matching the url pattern will stall unless it's continued, fulfilled or aborted.\n\n> NOTE: The handler will only be called for the first url if the response is a redirect.\n\nAn example of a naive handler that aborts all image requests:\n\n```js\nconst page = await browser.newPage();\nawait page.route('**/*.{png,jpg,jpeg}', route => route.abort());\nawait page.goto('https://example.com');\nawait browser.close();\n```\n\n```java\nPage page = browser.newPage();\npage.route(\"**/*.{png,jpg,jpeg}\", route -> route.abort());\npage.navigate(\"https://example.com\");\nbrowser.close();\n```\n\n```python async\npage = await browser.new_page()\nawait page.route(\"**/*.{png,jpg,jpeg}\", lambda route: route.abort())\nawait page.goto(\"https://example.com\")\nawait browser.close()\n```\n\n```python sync\npage = browser.new_page()\npage.route(\"**/*.{png,jpg,jpeg}\", lambda route: route.abort())\npage.goto(\"https://example.com\")\nbrowser.close()\n```\n\nor the same snippet using a regex pattern instead:\n\n```js\nconst page = await browser.newPage();\nawait page.route(/(\\.png$)|(\\.jpg$)/, route => route.abort());\nawait page.goto('https://example.com');\nawait browser.close();\n```\n\n```java\nPage page = browser.newPage();\npage.route(Pattern.compile(\"(\\\\.png$)|(\\\\.jpg$)\"),route -> route.abort());\npage.navigate(\"https://example.com\");\nbrowser.close();\n```\n\n```python async\npage = await browser.new_page()\nawait page.route(re.compile(r\"(\\.png$)|(\\.jpg$)\"), lambda route: route.abort())\nawait page.goto(\"https://example.com\")\nawait browser.close()\n```\n\n```python sync\npage = browser.new_page()\npage.route(re.compile(r\"(\\.png$)|(\\.jpg$)\"), lambda route: route.abort())\npage.goto(\"https://example.com\")\nbrowser.close()\n```\n\nIt is possible to examine the request to decide the route action. For example, mocking all requests that contain some\npost data, and leaving all other requests as is:\n\n```js\nawait page.route('/api/**', route => {\n  if (route.request().postData().includes('my-string'))\n    route.fulfill({ body: 'mocked-data' });\n  else\n    route.continue();\n});\n```\n\n```java\npage.route(\"/api/**\", route -> {\n  if (route.request().postData().contains(\"my-string\"))\n    route.fulfill(new Route.FulfillOptions().setBody(\"mocked-data\"));\n  else\n    route.resume();\n});\n```\n\n```python async\ndef handle_route(route):\n  if (\"my-string\" in route.request.post_data)\n    route.fulfill(body=\"mocked-data\")\n  else\n    route.continue_()\nawait page.route(\"/api/**\", handle_route)\n```\n\n```python sync\ndef handle_route(route):\n  if (\"my-string\" in route.request.post_data)\n    route.fulfill(body=\"mocked-data\")\n  else\n    route.continue_()\npage.route(\"/api/**\", handle_route)\n```\n\nPage routes take precedence over browser context routes (set up with [`method: BrowserContext.route`]) when request\nmatches both handlers.\n\nTo remove a route with its handler you can use [`method: Page.unroute`].\n\n> NOTE: Enabling routing disables http cache."]
    fn route(
        &self,
        #[doc = "A glob pattern, regex pattern or predicate receiving [URL] to match while routing."]
        url: NotImplementedYet,
        #[doc = "handler function to route the request."] handler: function,
        #[doc = "handler function to route the request."] handler: function
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Returns the buffer with the captured screenshot."]
    fn screenshot(
        &self,
        #[doc = "options"]
        #[doc = "An object which specifies clipping of the resulting image. Should have the following fields:"]
        clip: Option<NotImplementedYet>,
        #[doc = "When true, takes a screenshot of the full scrollable page, instead of the currently visible viewport. Defaults to\n`false`."]
        full_page: Option<bool>,
        #[doc = "Hides default white background and allows capturing screenshots with transparency. Not applicable to `jpeg` images.\nDefaults to `false`."]
        omit_background: Option<bool>,
        #[doc = "The file path to save the image to. The screenshot type will be inferred from file extension. If `path` is a relative\npath, then it is resolved relative to the current working directory. If no path is provided, the image won't be saved to\nthe disk."]
        path: Option<path>,
        quality : Option < i64 >,
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>,
        #[doc = "Specify screenshot type, defaults to `png`."] r#type: Option<ScreenshotType>
    ) -> Result<Buffer, Arc<Error>> {
        todo!()
    }
    #[doc = "Returns the array of option values that have been successfully selected.\n\nTriggers a `change` and `input` event once all the provided options have been selected. If there's no `<select>` element\nmatching `selector`, the method throws an error.\n\nWill wait until all specified options are present in the `<select>` element.\n\n```js\n// single selection matching the value\npage.selectOption('select#colors', 'blue');\n\n// single selection matching the label\npage.selectOption('select#colors', { label: 'Blue' });\n\n// multiple selection\npage.selectOption('select#colors', ['red', 'green', 'blue']);\n\n```\n\n```java\n// single selection matching the value\npage.selectOption(\"select#colors\", \"blue\");\n// single selection matching both the value and the label\npage.selectOption(\"select#colors\", new SelectOption().setLabel(\"Blue\"));\n// multiple selection\npage.selectOption(\"select#colors\", new String[] {\"red\", \"green\", \"blue\"});\n```\n\n```python async\n# single selection matching the value\nawait page.select_option(\"select#colors\", \"blue\")\n# single selection matching the label\nawait page.select_option(\"select#colors\", label=\"blue\")\n# multiple selection\nawait page.select_option(\"select#colors\", value=[\"red\", \"green\", \"blue\"])\n```\n\n```python sync\n# single selection matching the value\npage.select_option(\"select#colors\", \"blue\")\n# single selection matching both the label\npage.select_option(\"select#colors\", label=\"blue\")\n# multiple selection\npage.select_option(\"select#colors\", value=[\"red\", \"green\", \"blue\"])\n```\n\nShortcut for main frame's [`method: Frame.selectOption`]"]
    fn select_option(
        &self,
        #[doc = "A selector to search for element. If there are multiple elements satisfying the selector, the first will be used. See\n[working with selectors](./selectors.md) for more details."]
        selector: String,
        #[doc = "Options to select. If the `<select>` has the `multiple` attribute, all matching options are selected, otherwise only the\nfirst option matching one of the passed options is selected. String values are equivalent to `{value:'string'}`. Option\nis considered matching if all specified properties match."]
        values: Option<NotImplementedYet>,
        #[doc = "Options to select. If the `<select>` has the `multiple` attribute, all matching options are selected, otherwise only the\nfirst option matching one of the passed options is selected. String values are equivalent to `{value:'string'}`. Option\nis considered matching if all specified properties match."]
        values: Vec<NotImplementedYet>,
        #[doc = "Option elements to select. Optional."] element: Option<NotImplementedYet>,
        #[doc = "Options to select by index. Optional."] index: Option<NotImplementedYet>,
        #[doc = "Options to select by value. If the `<select>` has the `multiple` attribute, all given options are selected, otherwise\nonly the first option matching one of the passed options is selected. Optional."]
        value: Option<NotImplementedYet>,
        #[doc = "Options to select by label. If the `<select>` has the `multiple` attribute, all given options are selected, otherwise\nonly the first option matching one of the passed options is selected. Optional."]
        label: Option<NotImplementedYet>,
        #[doc = "options"]
        #[doc = "Actions that initiate navigations are waiting for these navigations to happen and for pages to start loading. You can\nopt out of waiting via setting this flag. You would only need this option in the exceptional cases such as navigating to\ninaccessible pages. Defaults to `false`."]
        no_wait_after: Option<bool>,
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>
    ) -> Result<Vec<String>, Arc<Error>> {
        todo!()
    }
    #[doc = ""]
    fn set_content(
        &self,
        #[doc = "HTML markup to assign to the page."] html: String,
        #[doc = "options"]
        #[doc = "Maximum operation time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be\nchanged by using the [`method: BrowserContext.setDefaultNavigationTimeout`],\n[`method: BrowserContext.setDefaultTimeout`], [`method: Page.setDefaultNavigationTimeout`] or\n[`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>,
        #[doc = "When to consider operation succeeded, defaults to `load`. Events can be either:\n- `'domcontentloaded'` - consider operation to be finished when the `DOMContentLoaded` event is fired.\n- `'load'` - consider operation to be finished when the `load` event is fired.\n- `'networkidle'` - consider operation to be finished when there are no network connections for at least `500` ms."]
        wait_until: Option<WaitUntilState>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "This setting will change the default maximum navigation time for the following methods and related shortcuts:\n- [`method: Page.goBack`]\n- [`method: Page.goForward`]\n- [`method: Page.goto`]\n- [`method: Page.reload`]\n- [`method: Page.setContent`]\n- [`method: Page.waitForNavigation`]\n- [`method: Page.waitForURL`]\n\n> NOTE: [`method: Page.setDefaultNavigationTimeout`] takes priority over [`method: Page.setDefaultTimeout`],\n[`method: BrowserContext.setDefaultTimeout`] and [`method: BrowserContext.setDefaultNavigationTimeout`]."]
    fn set_default_navigation_timeout(
        &self,
        #[doc = "Maximum navigation time in milliseconds"] timeout: f64
    ) -> Result<(), Error> {
        todo!()
    }
    #[doc = "This setting will change the default maximum time for all the methods accepting `timeout` option.\n\n> NOTE: [`method: Page.setDefaultNavigationTimeout`] takes priority over [`method: Page.setDefaultTimeout`]."]
    fn set_default_timeout(
        &self,
        #[doc = "Maximum time in milliseconds"] timeout: f64
    ) -> Result<(), Error> {
        todo!()
    }
    #[doc = "The extra HTTP headers will be sent with every request the page initiates.\n\n> NOTE: [`method: Page.setExtraHTTPHeaders`] does not guarantee the order of headers in the outgoing requests."]
    fn set_extra_h_t_t_p_headers(
        &self,
        #[doc = "An object containing additional HTTP headers to be sent with every request. All header values must be strings."]
        headers: Map<String, String>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "This method expects `selector` to point to an\n[input element](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input).\n\nSets the value of the file input to these file paths or files. If some of the `filePaths` are relative paths, then they\nare resolved relative to the the current working directory. For empty array, clears the selected files."]
    fn set_input_files(
        &self,
        #[doc = "A selector to search for element. If there are multiple elements satisfying the selector, the first will be used. See\n[working with selectors](./selectors.md) for more details."]
        selector: String,
        #[doc = ""] files: NotImplementedYet,
        #[doc = "options"]
        #[doc = "Actions that initiate navigations are waiting for these navigations to happen and for pages to start loading. You can\nopt out of waiting via setting this flag. You would only need this option in the exceptional cases such as navigating to\ninaccessible pages. Defaults to `false`."]
        no_wait_after: Option<bool>,
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "In the case of multiple pages in a single browser, each page can have its own viewport size. However,\n[`method: Browser.newContext`] allows to set viewport size (and more) for all pages in the context at once.\n\n`page.setViewportSize` will resize the page. A lot of websites don't expect phones to change size, so you should set the\nviewport size before navigating to the page.\n\n```js\nconst page = await browser.newPage();\nawait page.setViewportSize({\n  width: 640,\n  height: 480,\n});\nawait page.goto('https://example.com');\n```\n\n```java\nPage page = browser.newPage();\npage.setViewportSize(640, 480);\npage.navigate(\"https://example.com\");\n```\n\n```python async\npage = await browser.new_page()\nawait page.set_viewport_size({\"width\": 640, \"height\": 480})\nawait page.goto(\"https://example.com\")\n```\n\n```python sync\npage = browser.new_page()\npage.set_viewport_size({\"width\": 640, \"height\": 480})\npage.goto(\"https://example.com\")\n```\n"]
    fn set_viewport_size(
        &self,
        #[doc = ""] viewport_size: NotImplementedYet,
        #[doc = ""] width: i64,
        #[doc = ""] height: i64
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "This method taps an element matching `selector` by performing the following steps:\n1. Find an element matching `selector`. If there is none, wait until a matching element is attached to the DOM.\n1. Wait for [actionability](https://playwright.dev/docs/actionability/) checks on the matched element, unless `force` option is set. If the\n   element is detached during the checks, the whole action is retried.\n1. Scroll the element into view if needed.\n1. Use [`property: Page.touchscreen`] to tap the center of the element, or the specified `position`.\n1. Wait for initiated navigations to either succeed or fail, unless `noWaitAfter` option is set.\n\nWhen all steps combined have not finished during the specified `timeout`, this method throws a `TimeoutError`. Passing\nzero timeout disables this.\n\n> NOTE: [`method: Page.tap`] requires that the `hasTouch` option of the browser context be set to true.\n\nShortcut for main frame's [`method: Frame.tap`]."]
    fn tap(
        &self,
        #[doc = "A selector to search for element. If there are multiple elements satisfying the selector, the first will be used. See\n[working with selectors](./selectors.md) for more details."]
        selector: String,
        #[doc = "options"]
        #[doc = "Whether to bypass the [actionability](https://playwright.dev/docs/actionability/) checks. Defaults to `false`."]
        force: Option<bool>,
        #[doc = "Modifier keys to press. Ensures that only these modifiers are pressed during the operation, and then restores current\nmodifiers back. If not specified, currently pressed modifiers are used."]
        modifiers: Option<Vec<KeyboardModifier>>,
        #[doc = "Actions that initiate navigations are waiting for these navigations to happen and for pages to start loading. You can\nopt out of waiting via setting this flag. You would only need this option in the exceptional cases such as navigating to\ninaccessible pages. Defaults to `false`."]
        no_wait_after: Option<bool>,
        #[doc = "A point to use relative to the top-left corner of element padding box. If not specified, uses some visible point of the\nelement."]
        position: Option<NotImplementedYet>,
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>,
        #[doc = "When set, this method only performs the [actionability](https://playwright.dev/docs/actionability/) checks and skips the action. Defaults to\n`false`. Useful to wait until the element is ready for the action without performing it."]
        trial: Option<bool>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Returns `element.textContent`."]
    fn text_content(
        &self,
        #[doc = "A selector to search for element. If there are multiple elements satisfying the selector, the first will be used. See\n[working with selectors](./selectors.md) for more details."]
        selector: String,
        #[doc = "options"]
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>
    ) -> Result<Option<String>, Arc<Error>> {
        todo!()
    }
    #[doc = "Returns the page's title. Shortcut for main frame's [`method: Frame.title`]."]
    fn title(&self) -> Result<String, Arc<Error>> { todo!() }
    #[doc = "Sends a `keydown`, `keypress`/`input`, and `keyup` event for each character in the text. `page.type` can be used to send\nfine-grained keyboard events. To fill values in form fields, use [`method: Page.fill`].\n\nTo press a special key, like `Control` or `ArrowDown`, use [`method: Keyboard.press`].\n\n```js\nawait page.type('#mytextarea', 'Hello'); // Types instantly\nawait page.type('#mytextarea', 'World', {delay: 100}); // Types slower, like a user\n```\n\n```java\n// Types instantly\npage.type(\"#mytextarea\", \"Hello\");\n// Types slower, like a user\npage.type(\"#mytextarea\", \"World\", new Page.TypeOptions().setDelay(100));\n```\n\n```python async\nawait page.type(\"#mytextarea\", \"hello\") # types instantly\nawait page.type(\"#mytextarea\", \"world\", delay=100) # types slower, like a user\n```\n\n```python sync\npage.type(\"#mytextarea\", \"hello\") # types instantly\npage.type(\"#mytextarea\", \"world\", delay=100) # types slower, like a user\n```\n\nShortcut for main frame's [`method: Frame.type`]."]
    fn r#type(
        &self,
        #[doc = "A selector to search for element. If there are multiple elements satisfying the selector, the first will be used. See\n[working with selectors](./selectors.md) for more details."]
        selector: String,
        #[doc = "A text to type into a focused element."] text: String,
        #[doc = "options"]
        #[doc = "Time to wait between key presses in milliseconds. Defaults to 0."]
        delay: Option<f64>,
        #[doc = "Actions that initiate navigations are waiting for these navigations to happen and for pages to start loading. You can\nopt out of waiting via setting this flag. You would only need this option in the exceptional cases such as navigating to\ninaccessible pages. Defaults to `false`."]
        no_wait_after: Option<bool>,
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "This method unchecks an element matching `selector` by performing the following steps:\n1. Find an element matching `selector`. If there is none, wait until a matching element is attached to the DOM.\n1. Ensure that matched element is a checkbox or a radio input. If not, this method throws. If the element is already\n   unchecked, this method returns immediately.\n1. Wait for [actionability](https://playwright.dev/docs/actionability/) checks on the matched element, unless `force` option is set. If the\n   element is detached during the checks, the whole action is retried.\n1. Scroll the element into view if needed.\n1. Use [`property: Page.mouse`] to click in the center of the element.\n1. Wait for initiated navigations to either succeed or fail, unless `noWaitAfter` option is set.\n1. Ensure that the element is now unchecked. If not, this method throws.\n\nWhen all steps combined have not finished during the specified `timeout`, this method throws a `TimeoutError`. Passing\nzero timeout disables this.\n\nShortcut for main frame's [`method: Frame.uncheck`]."]
    fn uncheck(
        &self,
        #[doc = "A selector to search for element. If there are multiple elements satisfying the selector, the first will be used. See\n[working with selectors](./selectors.md) for more details."]
        selector: String,
        #[doc = "options"]
        #[doc = "Whether to bypass the [actionability](https://playwright.dev/docs/actionability/) checks. Defaults to `false`."]
        force: Option<bool>,
        #[doc = "Actions that initiate navigations are waiting for these navigations to happen and for pages to start loading. You can\nopt out of waiting via setting this flag. You would only need this option in the exceptional cases such as navigating to\ninaccessible pages. Defaults to `false`."]
        no_wait_after: Option<bool>,
        #[doc = "A point to use relative to the top-left corner of element padding box. If not specified, uses some visible point of the\nelement."]
        position: Option<NotImplementedYet>,
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>,
        #[doc = "When set, this method only performs the [actionability](https://playwright.dev/docs/actionability/) checks and skips the action. Defaults to\n`false`. Useful to wait until the element is ready for the action without performing it."]
        trial: Option<bool>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Removes a route created with [`method: Page.route`]. When `handler` is not specified, removes all routes for the `url`."]
    fn unroute(
        &self,
        #[doc = "A glob pattern, regex pattern or predicate receiving [URL] to match while routing."]
        url: NotImplementedYet,
        #[doc = "Optional handler function to route the request."] handler: Option<function>,
        #[doc = "Optional handler function to route the request."] handler: Option<function>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Shortcut for main frame's [`method: Frame.url`]."]
    fn url(&self) -> Result<String, Error> { todo!() }
    #[doc = "Video object associated with this page."]
    fn video(&self) -> Result<Option<Video>, Error> { todo!() }
    #[doc = ""]
    fn viewport_size(&self) -> Result<Option<NotImplementedYet>, Error> { todo!() }
    #[doc = "Performs action and waits for the Page to close."]
    fn wait_for_close(
        &self,
        #[doc = "Callback that performs the action triggering the event."] callback: Runnable,
        #[doc = "options"]
        #[doc = "Maximum time to wait for in milliseconds. Defaults to `30000` (30 seconds). Pass `0` to disable timeout. The default\nvalue can be changed by using the [`method: BrowserContext.setDefaultTimeout`]."]
        timeout: Option<f64>
    ) -> Result<Page, Arc<Error>> {
        todo!()
    }
    #[doc = "Performs action and waits for a `ConsoleMessage` to be logged by in the page. If predicate is provided, it passes\n`ConsoleMessage` value into the `predicate` function and waits for `predicate(message)` to return a truthy value. Will\nthrow an error if the page is closed before the console event is fired."]
    fn wait_for_console_message(
        &self,
        #[doc = "Callback that performs the action triggering the event."] callback: Runnable,
        #[doc = "options"]
        #[doc = "Receives the `ConsoleMessage` object and resolves to truthy value when the waiting should resolve."]
        predicate: Option<function>,
        #[doc = "Maximum time to wait for in milliseconds. Defaults to `30000` (30 seconds). Pass `0` to disable timeout. The default\nvalue can be changed by using the [`method: BrowserContext.setDefaultTimeout`]."]
        timeout: Option<f64>
    ) -> Result<ConsoleMessage, Arc<Error>> {
        todo!()
    }
    #[doc = "Performs action and waits for a new `Download`. If predicate is provided, it passes `Download` value into the\n`predicate` function and waits for `predicate(download)` to return a truthy value. Will throw an error if the page is\nclosed before the download event is fired."]
    fn wait_for_download(
        &self,
        #[doc = "Callback that performs the action triggering the event."] callback: Runnable,
        #[doc = "options"]
        #[doc = "Receives the `Download` object and resolves to truthy value when the waiting should resolve."]
        predicate: Option<function>,
        #[doc = "Maximum time to wait for in milliseconds. Defaults to `30000` (30 seconds). Pass `0` to disable timeout. The default\nvalue can be changed by using the [`method: BrowserContext.setDefaultTimeout`]."]
        timeout: Option<f64>
    ) -> Result<Download, Arc<Error>> {
        todo!()
    }
    #[doc = "Waits for event to fire and passes its value into the predicate function. Returns when the predicate returns truthy\nvalue. Will throw an error if the page is closed before the event is fired. Returns the event data value.\n\n```js\nconst [frame, _] = await Promise.all([\n  page.waitForEvent('framenavigated'),\n  page.click('button')\n]);\n```\n\n```python async\nasync with page.expect_event(\"framenavigated\") as event_info:\n    await page.click(\"button\")\nframe = await event_info.value\n```\n\n```python sync\nwith page.expect_event(\"framenavigated\") as event_info:\n    page.click(\"button\")\nframe = event_info.value\n```\n"]
    fn wait_for_event(
        &self,
        #[doc = "Event name, same one typically passed into `*.on(event)`."] event: String,
        #[doc = "Either a predicate that receives an event or an options object. Optional."]
        options_or_predicate: Option<NotImplementedYet>,
        #[doc = "options"]
        #[doc = "Receives the event data and resolves to truthy value when the waiting should resolve."]
        predicate: Option<function>,
        #[doc = "Maximum time to wait for in milliseconds. Defaults to `30000` (30 seconds). Pass `0` to disable timeout. The default\nvalue can be changed by using the [`method: BrowserContext.setDefaultTimeout`]."]
        timeout: Option<f64>
    ) -> Result<any, Arc<Error>> {
        todo!()
    }
    #[doc = "Performs action and waits for a new `FileChooser` to be created. If predicate is provided, it passes `FileChooser` value\ninto the `predicate` function and waits for `predicate(fileChooser)` to return a truthy value. Will throw an error if\nthe page is closed before the file chooser is opened."]
    fn wait_for_file_chooser(
        &self,
        #[doc = "Callback that performs the action triggering the event."] callback: Runnable,
        #[doc = "options"]
        #[doc = "Receives the `FileChooser` object and resolves to truthy value when the waiting should resolve."]
        predicate: Option<function>,
        #[doc = "Maximum time to wait for in milliseconds. Defaults to `30000` (30 seconds). Pass `0` to disable timeout. The default\nvalue can be changed by using the [`method: BrowserContext.setDefaultTimeout`]."]
        timeout: Option<f64>
    ) -> Result<FileChooser, Arc<Error>> {
        todo!()
    }
    #[doc = "Returns when the `expression` returns a truthy value. It resolves to a JSHandle of the truthy value.\n\nThe [`method: Page.waitForFunction`] can be used to observe viewport size change:\n\n```js\nconst { webkit } = require('playwright');  // Or 'chromium' or 'firefox'.\n\n(async () => {\n  const browser = await webkit.launch();\n  const page = await browser.newPage();\n  const watchDog = page.waitForFunction(() => window.innerWidth < 100);\n  await page.setViewportSize({width: 50, height: 50});\n  await watchDog;\n  await browser.close();\n})();\n```\n\n```java\nimport com.microsoft.playwright.*;\n\npublic class Example {\n  public static void main(String[] args) {\n    try (Playwright playwright = Playwright.create()) {\n      BrowserType webkit = playwright.webkit();\n      Browser browser = webkit.launch();\n      Page page = browser.newPage();\n      page.setViewportSize(50,  50);\n      page.waitForFunction(\"() => window.innerWidth < 100\");\n      browser.close();\n    }\n  }\n}\n```\n\n```python async\nimport asyncio\nfrom playwright.async_api import async_playwright\n\nasync def run(playwright):\n    webkit = playwright.webkit\n    browser = await webkit.launch()\n    page = await browser.new_page()\n    await page.evaluate(\"window.x = 0; setTimeout(() => { window.x = 100 }, 1000);\")\n    await page.wait_for_function(\"() => window.x > 0\")\n    await browser.close()\n\nasync def main():\n    async with async_playwright() as playwright:\n        await run(playwright)\nasyncio.run(main())\n```\n\n```python sync\nfrom playwright.sync_api import sync_playwright\n\ndef run(playwright):\n    webkit = playwright.webkit\n    browser = webkit.launch()\n    page = browser.new_page()\n    page.evaluate(\"window.x = 0; setTimeout(() => { window.x = 100 }, 1000);\")\n    page.wait_for_function(\"() => window.x > 0\")\n    browser.close()\n\nwith sync_playwright() as playwright:\n    run(playwright)\n```\n\nTo pass an argument to the predicate of [`method: Page.waitForFunction`] function:\n\n```js\nconst selector = '.foo';\nawait page.waitForFunction(selector => !!document.querySelector(selector), selector);\n```\n\n```java\nString selector = \".foo\";\npage.waitForFunction(\"selector => !!document.querySelector(selector)\", selector);\n```\n\n```python async\nselector = \".foo\"\nawait page.wait_for_function(\"selector => !!document.querySelector(selector)\", selector)\n```\n\n```python sync\nselector = \".foo\"\npage.wait_for_function(\"selector => !!document.querySelector(selector)\", selector)\n```\n\nShortcut for main frame's [`method: Frame.waitForFunction`]."]
    fn wait_for_function(
        &self,
        #[doc = "JavaScript expression to be evaluated in the browser context. If it looks like a function declaration, it is interpreted\nas a function. Otherwise, evaluated as an expression."]
        expression: String,
        #[doc = "Optional argument to pass to `expression`."] arg: Option<EvaluationArgument>,
        #[doc = "options"]
        #[doc = "If `polling` is `'raf'`, then `expression` is constantly executed in `requestAnimationFrame` callback. If `polling` is a\nnumber, then it is treated as an interval in milliseconds at which the function would be executed. Defaults to `raf`."]
        polling: Option<NotImplementedYet>,
        #[doc = "If specified, then it is treated as an interval in milliseconds at which the function would be executed. By default if\nthe option is not specified `expression` is executed in `requestAnimationFrame` callback."]
        polling_interval: Option<f64>,
        #[doc = "maximum time to wait for in milliseconds. Defaults to `30000` (30 seconds). Pass `0` to disable timeout. The default\nvalue can be changed by using the [`method: BrowserContext.setDefaultTimeout`]."]
        timeout: Option<f64>
    ) -> Result<JsHandle, Arc<Error>> {
        todo!()
    }
    #[doc = "Returns when the required load state has been reached.\n\nThis resolves when the page reaches a required load state, `load` by default. The navigation must have been committed\nwhen this method is called. If current document has already reached the required state, resolves immediately.\n\n```js\nawait page.click('button'); // Click triggers navigation.\nawait page.waitForLoadState(); // The promise resolves after 'load' event.\n```\n\n```java\npage.click(\"button\"); // Click triggers navigation.\npage.waitForLoadState(); // The promise resolves after \"load\" event.\n```\n\n```python async\nawait page.click(\"button\") # click triggers navigation.\nawait page.wait_for_load_state() # the promise resolves after \"load\" event.\n```\n\n```python sync\npage.click(\"button\") # click triggers navigation.\npage.wait_for_load_state() # the promise resolves after \"load\" event.\n```\n\n```js\nconst [popup] = await Promise.all([\n  page.waitForEvent('popup'),\n  page.click('button'), // Click triggers a popup.\n])\nawait popup.waitForLoadState('domcontentloaded'); // The promise resolves after 'domcontentloaded' event.\nconsole.log(await popup.title()); // Popup is ready to use.\n```\n\n```java\nPage popup = page.waitForPopup(() -> {\n  page.click(\"button\"); // Click triggers a popup.\n});\npopup.waitForLoadState(LoadState.DOMCONTENTLOADED);\nSystem.out.println(popup.title()); // Popup is ready to use.\n```\n\n```python async\nasync with page.expect_popup() as page_info:\n    await page.click(\"button\") # click triggers a popup.\npopup = await page_info.value\n # Following resolves after \"domcontentloaded\" event.\nawait popup.wait_for_load_state(\"domcontentloaded\")\nprint(await popup.title()) # popup is ready to use.\n```\n\n```python sync\nwith page.expect_popup() as page_info:\n    page.click(\"button\") # click triggers a popup.\npopup = page_info.value\n # Following resolves after \"domcontentloaded\" event.\npopup.wait_for_load_state(\"domcontentloaded\")\nprint(popup.title()) # popup is ready to use.\n```\n\nShortcut for main frame's [`method: Frame.waitForLoadState`]."]
    fn wait_for_load_state(
        &self,
        #[doc = "Optional load state to wait for, defaults to `load`. If the state has been already reached while loading current\ndocument, the method resolves immediately. Can be one of:\n- `'load'` - wait for the `load` event to be fired.\n- `'domcontentloaded'` - wait for the `DOMContentLoaded` event to be fired.\n- `'networkidle'` - wait until there are no network connections for at least `500` ms."]
        state: Option<LoadState>,
        #[doc = "options"]
        #[doc = "Maximum operation time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be\nchanged by using the [`method: BrowserContext.setDefaultNavigationTimeout`],\n[`method: BrowserContext.setDefaultTimeout`], [`method: Page.setDefaultNavigationTimeout`] or\n[`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Waits for the main frame navigation and returns the main resource response. In case of multiple redirects, the\nnavigation will resolve with the response of the last redirect. In case of navigation to a different anchor or\nnavigation due to History API usage, the navigation will resolve with `null`.\n\nThis resolves when the page navigates to a new URL or reloads. It is useful for when you run code which will indirectly\ncause the page to navigate. e.g. The click target has an `onclick` handler that triggers navigation from a `setTimeout`.\nConsider this example:\n\n```js\nconst [response] = await Promise.all([\n  page.waitForNavigation(), // The promise resolves after navigation has finished\n  page.click('a.delayed-navigation'), // Clicking the link will indirectly cause a navigation\n]);\n```\n\n```java\n// The method returns after navigation has finished\nResponse response = page.waitForNavigation(() -> {\n  page.click(\"a.delayed-navigation\"); // Clicking the link will indirectly cause a navigation\n});\n```\n\n```python async\nasync with page.expect_navigation():\n    await page.click(\"a.delayed-navigation\") # clicking the link will indirectly cause a navigation\n# Resolves after navigation has finished\n```\n\n```python sync\nwith page.expect_navigation():\n    page.click(\"a.delayed-navigation\") # clicking the link will indirectly cause a navigation\n# Resolves after navigation has finished\n```\n\n> NOTE: Usage of the [History API](https://developer.mozilla.org/en-US/docs/Web/API/History_API) to change the URL is\nconsidered a navigation.\n\nShortcut for main frame's [`method: Frame.waitForNavigation`]."]
    fn wait_for_navigation(
        &self,
        #[doc = "Callback that performs the action triggering the event."] callback: Runnable,
        #[doc = "options"]
        #[doc = "Maximum operation time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be\nchanged by using the [`method: BrowserContext.setDefaultNavigationTimeout`],\n[`method: BrowserContext.setDefaultTimeout`], [`method: Page.setDefaultNavigationTimeout`] or\n[`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>,
        #[doc = "A glob pattern, regex pattern or predicate receiving [URL] to match while waiting for the navigation."]
        url: Option<NotImplementedYet>,
        #[doc = "When to consider operation succeeded, defaults to `load`. Events can be either:\n- `'domcontentloaded'` - consider operation to be finished when the `DOMContentLoaded` event is fired.\n- `'load'` - consider operation to be finished when the `load` event is fired.\n- `'networkidle'` - consider operation to be finished when there are no network connections for at least `500` ms."]
        wait_until: Option<WaitUntilState>
    ) -> Result<Option<Response>, Arc<Error>> {
        todo!()
    }
    #[doc = "Performs action and waits for a popup `Page`. If predicate is provided, it passes [Popup] value into the `predicate`\nfunction and waits for `predicate(page)` to return a truthy value. Will throw an error if the page is closed before the\npopup event is fired."]
    fn wait_for_popup(
        &self,
        #[doc = "Callback that performs the action triggering the event."] callback: Runnable,
        #[doc = "options"]
        #[doc = "Receives the `Page` object and resolves to truthy value when the waiting should resolve."]
        predicate: Option<function>,
        #[doc = "Maximum time to wait for in milliseconds. Defaults to `30000` (30 seconds). Pass `0` to disable timeout. The default\nvalue can be changed by using the [`method: BrowserContext.setDefaultTimeout`]."]
        timeout: Option<f64>
    ) -> Result<Page, Arc<Error>> {
        todo!()
    }
    #[doc = "Waits for the matching request and returns it.  See [waiting for event](./events.md#waiting-for-event) for more details\nabout events.\n\n```js\n// Note that Promise.all prevents a race condition\n// between clicking and waiting for the request.\nconst [request] = await Promise.all([\n  // Waits for the next request with the specified url\n  page.waitForRequest('https://example.com/resource'),\n  // Triggers the request\n  page.click('button.triggers-request'),\n]);\n\n// Alternative way with a predicate.\nconst [request] = await Promise.all([\n  // Waits for the next request matching some conditions\n  page.waitForRequest(request => request.url() === 'https://example.com' && request.method() === 'GET'),\n  // Triggers the request\n  page.click('button.triggers-request'),\n]);\n```\n\n```java\n// Waits for the next response with the specified url\nRequest request = page.waitForRequest(\"https://example.com/resource\", () -> {\n  // Triggers the request\n  page.click(\"button.triggers-request\");\n});\n\n// Waits for the next request matching some conditions\nRequest request = page.waitForRequest(request -> \"https://example.com\".equals(request.url()) && \"GET\".equals(request.method()), () -> {\n  // Triggers the request\n  page.click(\"button.triggers-request\");\n});\n```\n\n```python async\nasync with page.expect_request(\"http://example.com/resource\") as first:\n    await page.click('button')\nfirst_request = await first.value\n\n# or with a lambda\nasync with page.expect_request(lambda request: request.url == \"http://example.com\" and request.method == \"get\") as second:\n    await page.click('img')\nsecond_request = await second.value\n```\n\n```python sync\nwith page.expect_request(\"http://example.com/resource\") as first:\n    page.click('button')\nfirst_request = first.value\n\n# or with a lambda\nwith page.expect_request(lambda request: request.url == \"http://example.com\" and request.method == \"get\") as second:\n    page.click('img')\nsecond_request = second.value\n```\n\n```js\nawait page.waitForRequest(request => request.url().searchParams.get('foo') === 'bar' && request.url().searchParams.get('foo2') === 'bar2');\n```\n"]
    fn wait_for_request(
        &self,
        #[doc = "Request URL string, regex or predicate receiving `Request` object."]
        url_or_predicate: NotImplementedYet,
        #[doc = "Callback that performs the action triggering the event."] callback: Runnable,
        #[doc = "options"]
        #[doc = "Maximum wait time in milliseconds, defaults to 30 seconds, pass `0` to disable the timeout. The default value can be\nchanged by using the [`method: Page.setDefaultTimeout`] method."]
        timeout: Option<f64>
    ) -> Result<Request, Arc<Error>> {
        todo!()
    }
    #[doc = "Returns the matched response. See [waiting for event](./events.md#waiting-for-event) for more details about events.\n\n```js\n// Note that Promise.all prevents a race condition\n// between clicking and waiting for the response.\nconst [response] = await Promise.all([\n  // Waits for the next response with the specified url\n  page.waitForResponse('https://example.com/resource'),\n  // Triggers the response\n  page.click('button.triggers-response'),\n]);\n\n// Alternative way with a predicate.\nconst [response] = await Promise.all([\n  // Waits for the next response matching some conditions\n  page.waitForResponse(response => response.url() === 'https://example.com' && response.status() === 200),\n  // Triggers the response\n  page.click('button.triggers-response'),\n]);\n```\n\n```java\n// Waits for the next response with the specified url\nResponse response = page.waitForResponse(\"https://example.com/resource\", () -> {\n  // Triggers the response\n  page.click(\"button.triggers-response\");\n});\n\n// Waits for the next response matching some conditions\nResponse response = page.waitForResponse(response -> \"https://example.com\".equals(response.url()) && response.status() == 200, () -> {\n  // Triggers the response\n  page.click(\"button.triggers-response\");\n});\n```\n\n```python async\nasync with page.expect_response(\"https://example.com/resource\") as response_info:\n    await page.click(\"input\")\nresponse = response_info.value\nreturn response.ok\n\n# or with a lambda\nasync with page.expect_response(lambda response: response.url == \"https://example.com\" and response.status === 200) as response_info:\n    await page.click(\"input\")\nresponse = response_info.value\nreturn response.ok\n```\n\n```python sync\nwith page.expect_response(\"https://example.com/resource\") as response_info:\n    page.click(\"input\")\nresponse = response_info.value\nreturn response.ok\n\n# or with a lambda\nwith page.expect_response(lambda response: response.url == \"https://example.com\" and response.status === 200) as response_info:\n    page.click(\"input\")\nresponse = response_info.value\nreturn response.ok\n```\n"]
    fn wait_for_response(
        &self,
        #[doc = "Request URL string, regex or predicate receiving `Response` object."]
        url_or_predicate: NotImplementedYet,
        #[doc = "Callback that performs the action triggering the event."] callback: Runnable,
        #[doc = "options"]
        #[doc = "Maximum wait time in milliseconds, defaults to 30 seconds, pass `0` to disable the timeout. The default value can be\nchanged by using the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>
    ) -> Result<Response, Arc<Error>> {
        todo!()
    }
    #[doc = "Returns when element specified by selector satisfies `state` option. Returns `null` if waiting for `hidden` or\n`detached`.\n\nWait for the `selector` to satisfy `state` option (either appear/disappear from dom, or become visible/hidden). If at\nthe moment of calling the method `selector` already satisfies the condition, the method will return immediately. If the\nselector doesn't satisfy the condition for the `timeout` milliseconds, the function will throw.\n\nThis method works across navigations:\n\n```js\nconst { chromium } = require('playwright');  // Or 'firefox' or 'webkit'.\n\n(async () => {\n  const browser = await chromium.launch();\n  const page = await browser.newPage();\n  for (let currentURL of ['https://google.com', 'https://bbc.com']) {\n    await page.goto(currentURL);\n    const element = await page.waitForSelector('img');\n    console.log('Loaded image: ' + await element.getAttribute('src'));\n  }\n  await browser.close();\n})();\n```\n\n```java\nimport com.microsoft.playwright.*;\n\npublic class Example {\n  public static void main(String[] args) {\n    try (Playwright playwright = Playwright.create()) {\n      BrowserType chromium = playwright.chromium();\n      Browser browser = chromium.launch();\n      Page page = browser.newPage();\n      for (String currentURL : Arrays.asList(\"https://google.com\", \"https://bbc.com\")) {\n        page.navigate(currentURL);\n        ElementHandle element = page.waitForSelector(\"img\");\n        System.out.println(\"Loaded image: \" + element.getAttribute(\"src\"));\n      }\n      browser.close();\n    }\n  }\n}\n```\n\n```python async\nimport asyncio\nfrom playwright.async_api import async_playwright\n\nasync def run(playwright):\n    chromium = playwright.chromium\n    browser = await chromium.launch()\n    page = await browser.new_page()\n    for current_url in [\"https://google.com\", \"https://bbc.com\"]:\n        await page.goto(current_url, wait_until=\"domcontentloaded\")\n        element = await page.wait_for_selector(\"img\")\n        print(\"Loaded image: \" + str(await element.get_attribute(\"src\")))\n    await browser.close()\n\nasync def main():\n    async with async_playwright() as playwright:\n        await run(playwright)\nasyncio.run(main())\n```\n\n```python sync\nfrom playwright.sync_api import sync_playwright\n\ndef run(playwright):\n    chromium = playwright.chromium\n    browser = chromium.launch()\n    page = browser.new_page()\n    for current_url in [\"https://google.com\", \"https://bbc.com\"]:\n        page.goto(current_url, wait_until=\"domcontentloaded\")\n        element = page.wait_for_selector(\"img\")\n        print(\"Loaded image: \" + str(element.get_attribute(\"src\")))\n    browser.close()\n\nwith sync_playwright() as playwright:\n    run(playwright)\n```\n"]
    fn wait_for_selector(
        &self,
        #[doc = "A selector to query for. See [working with selectors](./selectors.md) for more details."]
        selector: String,
        #[doc = "options"]
        #[doc = "Defaults to `'visible'`. Can be either:\n- `'attached'` - wait for element to be present in DOM.\n- `'detached'` - wait for element to not be present in DOM.\n- `'visible'` - wait for element to have non-empty bounding box and no `visibility:hidden`. Note that element without\n  any content or with `display:none` has an empty bounding box and is not considered visible.\n- `'hidden'` - wait for element to be either detached from DOM, or have an empty bounding box or `visibility:hidden`.\n  This is opposite to the `'visible'` option."]
        state: Option<WaitForSelectorState>,
        #[doc = "Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by\nusing the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>
    ) -> Result<Option<ElementHandle>, Arc<Error>> {
        todo!()
    }
    #[doc = "Waits for the given `timeout` in milliseconds.\n\nNote that `page.waitForTimeout()` should only be used for debugging. Tests using the timer in production are going to be\nflaky. Use signals such as network events, selectors becoming visible and others instead.\n\n```js\n// wait for 1 second\nawait page.waitForTimeout(1000);\n```\n\n```java\n// wait for 1 second\npage.waitForTimeout(1000);\n```\n\n```python async\n# wait for 1 second\nawait page.wait_for_timeout(1000)\n```\n\n```python sync\n# wait for 1 second\npage.wait_for_timeout(1000)\n```\n\nShortcut for main frame's [`method: Frame.waitForTimeout`]."]
    fn wait_for_timeout(
        &self,
        #[doc = "A timeout to wait for"] timeout: f64
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Waits for the main frame to navigate to the given URL.\n\n```js\nawait page.click('a.delayed-navigation'); // Clicking the link will indirectly cause a navigation\nawait page.waitForURL('**/target.html');\n```\n\n```java\npage.click(\"a.delayed-navigation\"); // Clicking the link will indirectly cause a navigation\npage.waitForURL(\"**/target.html\");\n```\n\n```python async\nawait page.click(\"a.delayed-navigation\") # clicking the link will indirectly cause a navigation\nawait page.wait_for_url(\"**/target.html\")\n```\n\n```python sync\npage.click(\"a.delayed-navigation\") # clicking the link will indirectly cause a navigation\npage.wait_for_url(\"**/target.html\")\n```\n\nShortcut for main frame's [`method: Frame.waitForURL`]."]
    fn wait_for_u_r_l(
        &self,
        #[doc = "A glob pattern, regex pattern or predicate receiving [URL] to match while waiting for the navigation."]
        url: NotImplementedYet,
        #[doc = "options"]
        #[doc = "Maximum operation time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be\nchanged by using the [`method: BrowserContext.setDefaultNavigationTimeout`],\n[`method: BrowserContext.setDefaultTimeout`], [`method: Page.setDefaultNavigationTimeout`] or\n[`method: Page.setDefaultTimeout`] methods."]
        timeout: Option<f64>,
        #[doc = "When to consider operation succeeded, defaults to `load`. Events can be either:\n- `'domcontentloaded'` - consider operation to be finished when the `DOMContentLoaded` event is fired.\n- `'load'` - consider operation to be finished when the `load` event is fired.\n- `'networkidle'` - consider operation to be finished when there are no network connections for at least `500` ms."]
        wait_until: Option<WaitUntilState>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Performs action and waits for a new `WebSocket`. If predicate is provided, it passes `WebSocket` value into the\n`predicate` function and waits for `predicate(webSocket)` to return a truthy value. Will throw an error if the page is\nclosed before the WebSocket event is fired."]
    fn wait_for_web_socket(
        &self,
        #[doc = "Callback that performs the action triggering the event."] callback: Runnable,
        #[doc = "options"]
        #[doc = "Receives the `WebSocket` object and resolves to truthy value when the waiting should resolve."]
        predicate: Option<function>,
        #[doc = "Maximum time to wait for in milliseconds. Defaults to `30000` (30 seconds). Pass `0` to disable timeout. The default\nvalue can be changed by using the [`method: BrowserContext.setDefaultTimeout`]."]
        timeout: Option<f64>
    ) -> Result<WebSocket, Arc<Error>> {
        todo!()
    }
    #[doc = "Performs action and waits for a new `Worker`. If predicate is provided, it passes `Worker` value into the `predicate`\nfunction and waits for `predicate(worker)` to return a truthy value. Will throw an error if the page is closed before\nthe worker event is fired."]
    fn wait_for_worker(
        &self,
        #[doc = "Callback that performs the action triggering the event."] callback: Runnable,
        #[doc = "options"]
        #[doc = "Receives the `Worker` object and resolves to truthy value when the waiting should resolve."]
        predicate: Option<function>,
        #[doc = "Maximum time to wait for in milliseconds. Defaults to `30000` (30 seconds). Pass `0` to disable timeout. The default\nvalue can be changed by using the [`method: BrowserContext.setDefaultTimeout`]."]
        timeout: Option<f64>
    ) -> Result<Worker, Arc<Error>> {
        todo!()
    }
    #[doc = "This method returns all of the dedicated [WebWorkers](https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API)\nassociated with the page.\n\n> NOTE: This does not contain ServiceWorkers"]
    fn workers(&self) -> Result<Vec<Worker>, Error> { todo!() }
    #[doc = "Adds one-off `Dialog` handler. The handler will be removed immediately after next `Dialog` is created.\n\n```java\npage.onceDialog(dialog -> {\n  dialog.accept(\"foo\");\n});\n\n// prints 'foo'\nSystem.out.println(page.evaluate(\"prompt('Enter string:')\"));\n\n// prints 'null' as the dialog will be auto-dismissed because there are no handlers.\nSystem.out.println(page.evaluate(\"prompt('Enter string:')\"));\n```\n\nThis code above is equivalent to:\n\n```java\nConsumer<Dialog> handler = new Consumer<Dialog>() {\n  @Override\n  public void accept(Dialog dialog) {\n    dialog.accept(\"foo\");\n    page.offDialog(this);\n  }\n};\npage.onDialog(handler);\n\n// prints 'foo'\nSystem.out.println(page.evaluate(\"prompt('Enter string:')\"));\n\n// prints 'null' as the dialog will be auto-dismissed because there are no handlers.\nSystem.out.println(page.evaluate(\"prompt('Enter string:')\"));\n```\n"]
    fn once_dialog(
        &self,
        #[doc = "Receives the `Dialog` object, it **must** either [`method: Dialog.accept`] or [`method: Dialog.dismiss`] the dialog -\notherwise the page will [freeze](https://developer.mozilla.org/en-US/docs/Web/JavaScript/EventLoop#never_blocking)\nwaiting for the dialog, and actions like click will never finish."]
        handler: function
    ) -> Result<(), Error> {
        todo!()
    }
    #[doc = "> NOTE: In most cases, you should use [`method: Page.waitForEvent`].\n\nWaits for given `event` to fire. If predicate is provided, it passes event's value into the `predicate` function and\nwaits for `predicate(event)` to return a truthy value. Will throw an error if the socket is closed before the `event` is\nfired."]
    fn wait_for_event2(
        &self,
        #[doc = "Event name, same one typically passed into `*.on(event)`."] event: String,
        #[doc = "options"]
        #[doc = "Receives the event data and resolves to truthy value when the waiting should resolve."]
        predicate: Option<function>,
        #[doc = "Maximum time to wait for in milliseconds. Defaults to `30000` (30 seconds). Pass `0` to disable timeout. The default\nvalue can be changed by using the [`method: BrowserContext.setDefaultTimeout`]."]
        timeout: Option<f64>
    ) -> Result<Any, Arc<Error>> {
        todo!()
    }
}
enum NotImplementedYetscript {
    NotImplementedYet(function),
    NotImplementedYet(String),
    NotImplementedYet(NotImplementedYet)
}
struct NotImplementedYet {
    #[doc = "Path to the JavaScript file. If `path` is a relative path, then it is resolved relative to the current working\ndirectory. Optional."]
    path: Option<path>,
    #[doc = "Raw script content. Optional."]
    content: Option<String>
}
enum NotImplementedYetscript {
    NotImplementedYet(String),
    NotImplementedYet(path)
}
enum NotImplementedYetframeSelector {
    NotImplementedYet(String),
    NotImplementedYet(NotImplementedYet)
}
struct NotImplementedYet {
    #[doc = "Frame name specified in the `iframe`'s `name` attribute. Optional."]
    name: Option<String>,
    #[doc = "A glob pattern, regex pattern or predicate receiving frame's `url` as a [URL] object. Optional."]
    url: Option<NotImplementedYet>
}
enum NotImplementedYeturl {
    NotImplementedYet(String),
    NotImplementedYet(RegExp),
    NotImplementedYet(function)
}
enum NotImplementedYeturl {
    NotImplementedYet(String),
    NotImplementedYet(RegExp),
    NotImplementedYet(function)
}
enum NotImplementedYeturl {
    NotImplementedYet(String),
    NotImplementedYet(RegExp),
    NotImplementedYet(function)
}
enum NotImplementedYetvalues {
    NotImplementedYet(String),
    NotImplementedYet(ElementHandle),
    NotImplementedYet(Vec<String>),
    NotImplementedYet(NotImplementedYet),
    NotImplementedYet(Vec<ElementHandle>),
    NotImplementedYet(Vec<NotImplementedYet>)
}
struct NotImplementedYet {
    #[doc = "Matches by `option.value`. Optional."]
    value: Option<String>,
    #[doc = "Matches by `option.label`. Optional."]
    label: Option<String>,
    #[doc = "Matches by the index. Optional."]
    index: Option<i64>
}
enum NotImplementedYetelement {
    NotImplementedYet(ElementHandle),
    NotImplementedYet(Vec<ElementHandle>)
}
enum NotImplementedYetindex {
    NotImplementedYet(i64),
    NotImplementedYet(Vec<i64>)
}
enum NotImplementedYetvalue {
    NotImplementedYet(String),
    NotImplementedYet(Vec<String>)
}
enum NotImplementedYetlabel {
    NotImplementedYet(String),
    NotImplementedYet(Vec<String>)
}
enum NotImplementedYetfiles {
    NotImplementedYet(path),
    NotImplementedYet(Vec<path>),
    NotImplementedYet(NotImplementedYet),
    NotImplementedYet(Vec<NotImplementedYet>)
}
struct NotImplementedYet {
    #[doc = "File name"]
    name: String,
    #[doc = "File type"]
    mime_type: String,
    #[doc = "File content"]
    buffer: Buffer
}
struct NotImplementedYetviewportSize {
    #[doc = "page width in pixels."]
    width: i64,
    #[doc = "page height in pixels."]
    height: i64
}
enum NotImplementedYeturl {
    NotImplementedYet(String),
    NotImplementedYet(RegExp),
    NotImplementedYet(function)
}
enum NotImplementedYetoptionsOrPredicate {
    NotImplementedYet(function),
    NotImplementedYet(NotImplementedYet)
}
struct NotImplementedYet {
    #[doc = "receives the event data and resolves to truthy value when the waiting should resolve."]
    predicate: function,
    #[doc = "maximum time to wait for in milliseconds. Defaults to `30000` (30 seconds). Pass `0` to disable timeout. The default\nvalue can be changed by using the [`method: BrowserContext.setDefaultTimeout`]."]
    timeout: Option<f64>
}
enum NotImplementedYetstate {
    NotImplementedYet(load),
    NotImplementedYet(domcontentloaded),
    NotImplementedYet(networkidle)
}
enum NotImplementedYeturlOrPredicate {
    NotImplementedYet(String),
    NotImplementedYet(RegExp),
    NotImplementedYet(function)
}
enum NotImplementedYeturlOrPredicate {
    NotImplementedYet(String),
    NotImplementedYet(RegExp),
    NotImplementedYet(function)
}
enum NotImplementedYeturl {
    NotImplementedYet(String),
    NotImplementedYet(RegExp),
    NotImplementedYet(function)
}
enum PageEventType {
    #[doc = "Emitted when the page closes."]
    Close,
    #[doc = "Emitted when JavaScript within the page calls one of console API methods, e.g. `console.log` or `console.dir`. Also\nemitted if the page throws an error or a warning.\n\nThe arguments passed into `console.log` appear as arguments on the event handler.\n\nAn example of handling `console` event:\n\n```js\npage.on('console', async msg => {\n  for (let i = 0; i < msg.args().length; ++i)\n    console.log(`${i}: ${await msg.args()[i].jsonValue()}`);\n});\nawait page.evaluate(() => console.log('hello', 5, {foo: 'bar'}));\n```\n\n```java\npage.onConsole(msg -> {\n  for (int i = 0; i < msg.args().size(); ++i)\n    System.out.println(i + \": \" + msg.args().get(i).jsonValue());\n});\npage.evaluate(\"() => console.log('hello', 5, {foo: 'bar'})\");\n```\n\n```python async\nasync def print_args(msg):\n    for arg in msg.args:\n        print(await arg.json_value())\n\npage.on(\"console\", print_args)\nawait page.evaluate(\"console.log('hello', 5, {foo: 'bar'})\")\n```\n\n```python sync\ndef print_args(msg):\n    for arg in msg.args:\n        print(arg.json_value())\n\npage.on(\"console\", print_args)\npage.evaluate(\"console.log('hello', 5, {foo: 'bar'})\")\n```\n"]
    Console,
    #[doc = "Emitted when the page crashes. Browser pages might crash if they try to allocate too much memory. When the page crashes,\nongoing and subsequent operations will throw.\n\nThe most common way to deal with crashes is to catch an exception:\n\n```js\ntry {\n  // Crash might happen during a click.\n  await page.click('button');\n  // Or while waiting for an event.\n  await page.waitForEvent('popup');\n} catch (e) {\n  // When the page crashes, exception message contains 'crash'.\n}\n```\n\n```java\ntry {\n  // Crash might happen during a click.\n  page.click(\"button\");\n  // Or while waiting for an event.\n  page.waitForPopup(() -> {});\n} catch (PlaywrightException e) {\n  // When the page crashes, exception message contains \"crash\".\n}\n```\n\n```python async\ntry:\n    # crash might happen during a click.\n    await page.click(\"button\")\n    # or while waiting for an event.\n    await page.wait_for_event(\"popup\")\nexcept Error as e:\n    # when the page crashes, exception message contains \"crash\".\n```\n\n```python sync\ntry:\n    # crash might happen during a click.\n    page.click(\"button\")\n    # or while waiting for an event.\n    page.wait_for_event(\"popup\")\nexcept Error as e:\n    # when the page crashes, exception message contains \"crash\".\n```\n"]
    Crash,
    #[doc = "Emitted when a JavaScript dialog appears, such as `alert`, `prompt`, `confirm` or `beforeunload`. Listener **must**\neither [`method: Dialog.accept`] or [`method: Dialog.dismiss`] the dialog - otherwise the page will\n[freeze](https://developer.mozilla.org/en-US/docs/Web/JavaScript/EventLoop#never_blocking) waiting for the dialog, and\nactions like click will never finish.\n\n> NOTE: When no [`event: Page.dialog`] listeners are present, all dialogs are automatically dismissed."]
    Dialog,
    #[doc = "Emitted when the JavaScript [`DOMContentLoaded`](https://developer.mozilla.org/en-US/docs/Web/Events/DOMContentLoaded)\nevent is dispatched."]
    DOMContentLoaded,
    #[doc = "Emitted when attachment download started. User can access basic file operations on downloaded content via the passed\n`Download` instance.\n\n> NOTE: Browser context **must** be created with the `acceptDownloads` set to `true` when user needs access to the\ndownloaded content. If `acceptDownloads` is not set, download events are emitted, but the actual download is not\nperformed and user has no access to the downloaded files."]
    Download,
    #[doc = "Emitted when a file chooser is supposed to appear, such as after clicking the  `<input type=file>`. Playwright can\nrespond to it via setting the input files using [`method: FileChooser.setFiles`] that can be uploaded after that.\n\n```js\npage.on('filechooser', async (fileChooser) => {\n  await fileChooser.setFiles('/tmp/myfile.pdf');\n});\n```\n\n```java\npage.onFileChooser(fileChooser -> {\n  fileChooser.setFiles(Paths.get(\"/tmp/myfile.pdf\"));\n});\n```\n\n```py\npage.on(\"filechooser\", lambda file_chooser: file_chooser.set_files(\"/tmp/myfile.pdf\"))\n```\n"]
    FileChooser,
    #[doc = "Emitted when a frame is attached."]
    FrameAttached,
    #[doc = "Emitted when a frame is detached."]
    FrameDetached,
    #[doc = "Emitted when a frame is navigated to a new url."]
    FrameNavigated,
    #[doc = "Emitted when the JavaScript [`load`](https://developer.mozilla.org/en-US/docs/Web/Events/load) event is dispatched."]
    Load,
    #[doc = "Emitted when an uncaught exception happens within the page."]
    PageError,
    #[doc = "Emitted when the page opens a new tab or window. This event is emitted in addition to the\n[`event: BrowserContext.page`], but only for popups relevant to this page.\n\nThe earliest moment that page is available is when it has navigated to the initial url. For example, when opening a\npopup with `window.open('http://example.com')`, this event will fire when the network request to \"http://example.com\" is\ndone and its response has started loading in the popup.\n\n```js\nconst [popup] = await Promise.all([\n  page.waitForEvent('popup'),\n  page.evaluate(() => window.open('https://example.com')),\n]);\nconsole.log(await popup.evaluate('location.href'));\n```\n\n```java\nPage popup = page.waitForPopup(() -> {\n  page.evaluate(\"() => window.open('https://example.com')\");\n});\nSystem.out.println(popup.evaluate(\"location.href\"));\n```\n\n```python async\nasync with page.expect_event(\"popup\") as page_info:\n    page.evaluate(\"window.open('https://example.com')\")\npopup = await page_info.value\nprint(await popup.evaluate(\"location.href\"))\n```\n\n```python sync\nwith page.expect_event(\"popup\") as page_info:\n    page.evaluate(\"window.open('https://example.com')\")\npopup = page_info.value\nprint(popup.evaluate(\"location.href\"))\n```\n\n> NOTE: Use [`method: Page.waitForLoadState`] to wait until the page gets to a particular state (you should not need it\nin most cases)."]
    Popup,
    #[doc = "Emitted when a page issues a request. The [request] object is read-only. In order to intercept and mutate requests, see\n[`method: Page.route`] or [`method: BrowserContext.route`]."]
    Request,
    #[doc = "Emitted when a request fails, for example by timing out.\n\n> NOTE: HTTP Error responses, such as 404 or 503, are still successful responses from HTTP standpoint, so request will\ncomplete with [`event: Page.requestFinished`] event and not with [`event: Page.requestFailed`]."]
    RequestFailed,
    #[doc = "Emitted when a request finishes successfully after downloading the response body. For a successful response, the\nsequence of events is `request`, `response` and `requestfinished`."]
    RequestFinished,
    #[doc = "Emitted when [response] status and headers are received for a request. For a successful response, the sequence of events\nis `request`, `response` and `requestfinished`."]
    Response,
    #[doc = "Emitted when `WebSocket` request is sent."]
    WebSocket,
    #[doc = "Emitted when a dedicated [WebWorker](https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API) is spawned by the\npage."]
    Worker
}
enum PageEvent {
    #[doc = "Emitted when the page closes."]
    Close(Page),
    #[doc = "Emitted when JavaScript within the page calls one of console API methods, e.g. `console.log` or `console.dir`. Also\nemitted if the page throws an error or a warning.\n\nThe arguments passed into `console.log` appear as arguments on the event handler.\n\nAn example of handling `console` event:\n\n```js\npage.on('console', async msg => {\n  for (let i = 0; i < msg.args().length; ++i)\n    console.log(`${i}: ${await msg.args()[i].jsonValue()}`);\n});\nawait page.evaluate(() => console.log('hello', 5, {foo: 'bar'}));\n```\n\n```java\npage.onConsole(msg -> {\n  for (int i = 0; i < msg.args().size(); ++i)\n    System.out.println(i + \": \" + msg.args().get(i).jsonValue());\n});\npage.evaluate(\"() => console.log('hello', 5, {foo: 'bar'})\");\n```\n\n```python async\nasync def print_args(msg):\n    for arg in msg.args:\n        print(await arg.json_value())\n\npage.on(\"console\", print_args)\nawait page.evaluate(\"console.log('hello', 5, {foo: 'bar'})\")\n```\n\n```python sync\ndef print_args(msg):\n    for arg in msg.args:\n        print(arg.json_value())\n\npage.on(\"console\", print_args)\npage.evaluate(\"console.log('hello', 5, {foo: 'bar'})\")\n```\n"]
    Console(ConsoleMessage),
    #[doc = "Emitted when the page crashes. Browser pages might crash if they try to allocate too much memory. When the page crashes,\nongoing and subsequent operations will throw.\n\nThe most common way to deal with crashes is to catch an exception:\n\n```js\ntry {\n  // Crash might happen during a click.\n  await page.click('button');\n  // Or while waiting for an event.\n  await page.waitForEvent('popup');\n} catch (e) {\n  // When the page crashes, exception message contains 'crash'.\n}\n```\n\n```java\ntry {\n  // Crash might happen during a click.\n  page.click(\"button\");\n  // Or while waiting for an event.\n  page.waitForPopup(() -> {});\n} catch (PlaywrightException e) {\n  // When the page crashes, exception message contains \"crash\".\n}\n```\n\n```python async\ntry:\n    # crash might happen during a click.\n    await page.click(\"button\")\n    # or while waiting for an event.\n    await page.wait_for_event(\"popup\")\nexcept Error as e:\n    # when the page crashes, exception message contains \"crash\".\n```\n\n```python sync\ntry:\n    # crash might happen during a click.\n    page.click(\"button\")\n    # or while waiting for an event.\n    page.wait_for_event(\"popup\")\nexcept Error as e:\n    # when the page crashes, exception message contains \"crash\".\n```\n"]
    Crash(Page),
    #[doc = "Emitted when a JavaScript dialog appears, such as `alert`, `prompt`, `confirm` or `beforeunload`. Listener **must**\neither [`method: Dialog.accept`] or [`method: Dialog.dismiss`] the dialog - otherwise the page will\n[freeze](https://developer.mozilla.org/en-US/docs/Web/JavaScript/EventLoop#never_blocking) waiting for the dialog, and\nactions like click will never finish.\n\n> NOTE: When no [`event: Page.dialog`] listeners are present, all dialogs are automatically dismissed."]
    Dialog(Dialog),
    #[doc = "Emitted when the JavaScript [`DOMContentLoaded`](https://developer.mozilla.org/en-US/docs/Web/Events/DOMContentLoaded)\nevent is dispatched."]
    DOMContentLoaded(Page),
    #[doc = "Emitted when attachment download started. User can access basic file operations on downloaded content via the passed\n`Download` instance.\n\n> NOTE: Browser context **must** be created with the `acceptDownloads` set to `true` when user needs access to the\ndownloaded content. If `acceptDownloads` is not set, download events are emitted, but the actual download is not\nperformed and user has no access to the downloaded files."]
    Download(Download),
    #[doc = "Emitted when a file chooser is supposed to appear, such as after clicking the  `<input type=file>`. Playwright can\nrespond to it via setting the input files using [`method: FileChooser.setFiles`] that can be uploaded after that.\n\n```js\npage.on('filechooser', async (fileChooser) => {\n  await fileChooser.setFiles('/tmp/myfile.pdf');\n});\n```\n\n```java\npage.onFileChooser(fileChooser -> {\n  fileChooser.setFiles(Paths.get(\"/tmp/myfile.pdf\"));\n});\n```\n\n```py\npage.on(\"filechooser\", lambda file_chooser: file_chooser.set_files(\"/tmp/myfile.pdf\"))\n```\n"]
    FileChooser(FileChooser),
    #[doc = "Emitted when a frame is attached."]
    FrameAttached(Frame),
    #[doc = "Emitted when a frame is detached."]
    FrameDetached(Frame),
    #[doc = "Emitted when a frame is navigated to a new url."]
    FrameNavigated(Frame),
    #[doc = "Emitted when the JavaScript [`load`](https://developer.mozilla.org/en-US/docs/Web/Events/load) event is dispatched."]
    Load(Page),
    #[doc = "Emitted when an uncaught exception happens within the page."]
    PageError(Error),
    #[doc = "Emitted when the page opens a new tab or window. This event is emitted in addition to the\n[`event: BrowserContext.page`], but only for popups relevant to this page.\n\nThe earliest moment that page is available is when it has navigated to the initial url. For example, when opening a\npopup with `window.open('http://example.com')`, this event will fire when the network request to \"http://example.com\" is\ndone and its response has started loading in the popup.\n\n```js\nconst [popup] = await Promise.all([\n  page.waitForEvent('popup'),\n  page.evaluate(() => window.open('https://example.com')),\n]);\nconsole.log(await popup.evaluate('location.href'));\n```\n\n```java\nPage popup = page.waitForPopup(() -> {\n  page.evaluate(\"() => window.open('https://example.com')\");\n});\nSystem.out.println(popup.evaluate(\"location.href\"));\n```\n\n```python async\nasync with page.expect_event(\"popup\") as page_info:\n    page.evaluate(\"window.open('https://example.com')\")\npopup = await page_info.value\nprint(await popup.evaluate(\"location.href\"))\n```\n\n```python sync\nwith page.expect_event(\"popup\") as page_info:\n    page.evaluate(\"window.open('https://example.com')\")\npopup = page_info.value\nprint(popup.evaluate(\"location.href\"))\n```\n\n> NOTE: Use [`method: Page.waitForLoadState`] to wait until the page gets to a particular state (you should not need it\nin most cases)."]
    Popup(Page),
    #[doc = "Emitted when a page issues a request. The [request] object is read-only. In order to intercept and mutate requests, see\n[`method: Page.route`] or [`method: BrowserContext.route`]."]
    Request(Request),
    #[doc = "Emitted when a request fails, for example by timing out.\n\n> NOTE: HTTP Error responses, such as 404 or 503, are still successful responses from HTTP standpoint, so request will\ncomplete with [`event: Page.requestFinished`] event and not with [`event: Page.requestFailed`]."]
    RequestFailed(Request),
    #[doc = "Emitted when a request finishes successfully after downloading the response body. For a successful response, the\nsequence of events is `request`, `response` and `requestfinished`."]
    RequestFinished(Request),
    #[doc = "Emitted when [response] status and headers are received for a request. For a successful response, the sequence of events\nis `request`, `response` and `requestfinished`."]
    Response(Response),
    #[doc = "Emitted when `WebSocket` request is sent."]
    WebSocket(WebSocket),
    #[doc = "Emitted when a dedicated [WebWorker](https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API) is spawned by the\npage."]
    Worker(Worker)
}
#[doc = "Playwright module provides a method to launch a browser instance. The following is a typical example of using Playwright\nto drive automation:\n\n```js\nconst { chromium, firefox, webkit } = require('playwright');\n\n(async () => {\n  const browser = await chromium.launch();  // Or 'firefox' or 'webkit'.\n  const page = await browser.newPage();\n  await page.goto('http://example.com');\n  // other actions...\n  await browser.close();\n})();\n```\n\n```java\nimport com.microsoft.playwright.*;\n\npublic class Example {\n  public static void main(String[] args) {\n    try (Playwright playwright = Playwright.create()) {\n      BrowserType chromium = playwright.chromium();\n      Browser browser = chromium.launch();\n      Page page = browser.newPage();\n      page.navigate(\"http://example.com\");\n      // other actions...\n      browser.close();\n    }\n  }\n}\n```\n\n```python async\nimport asyncio\nfrom playwright.async_api import async_playwright\n\nasync def run(playwright):\n    chromium = playwright.chromium # or \"firefox\" or \"webkit\".\n    browser = await chromium.launch()\n    page = await browser.new_page()\n    await page.goto(\"http://example.com\")\n    # other actions...\n    await browser.close()\n\nasync def main():\n    async with async_playwright() as playwright:\n        await run(playwright)\nasyncio.run(main())\n```\n\n```python sync\nfrom playwright.sync_api import sync_playwright\n\ndef run(playwright):\n    chromium = playwright.chromium # or \"firefox\" or \"webkit\".\n    browser = chromium.launch()\n    page = browser.new_page()\n    page.goto(\"http://example.com\")\n    # other actions...\n    browser.close()\n\nwith sync_playwright() as playwright:\n    run(playwright)\n```\n"]
impl Playwright {
    #[doc = "This object can be used to launch or connect to Chromium, returning instances of `Browser`."]
    pub fn chromium(&self) -> BrowserType {}
    #[doc = "Returns a dictionary of devices to be used with [`method: Browser.newContext`] or [`method: Browser.newPage`].\n\n```js\nconst { webkit, devices } = require('playwright');\nconst iPhone = devices['iPhone 6'];\n\n(async () => {\n  const browser = await webkit.launch();\n  const context = await browser.newContext({\n    ...iPhone\n  });\n  const page = await context.newPage();\n  await page.goto('http://example.com');\n  // other actions...\n  await browser.close();\n})();\n```\n\n```python async\nimport asyncio\nfrom playwright.async_api import async_playwright\n\nasync def run(playwright):\n    webkit = playwright.webkit\n    iphone = playwright.devices[\"iPhone 6\"]\n    browser = await webkit.launch()\n    context = await browser.new_context(**iphone)\n    page = await context.new_page()\n    await page.goto(\"http://example.com\")\n    # other actions...\n    await browser.close()\n\nasync def main():\n    async with async_playwright() as playwright:\n        await run(playwright)\nasyncio.run(main())\n```\n\n```python sync\nfrom playwright.sync_api import sync_playwright\n\ndef run(playwright):\n    webkit = playwright.webkit\n    iphone = playwright.devices[\"iPhone 6\"]\n    browser = webkit.launch()\n    context = browser.new_context(**iphone)\n    page = context.new_page()\n    page.goto(\"http://example.com\")\n    # other actions...\n    browser.close()\n\nwith sync_playwright() as playwright:\n    run(playwright)\n```\n"]
    pub fn devices(&self) -> Object {}
    #[doc = "Playwright methods might throw errors if they are unable to fulfill a request. For example,\n[`method: Page.waitForSelector`] might fail if the selector doesn't match any nodes during the given timeframe.\n\nFor certain types of errors Playwright uses specific error classes. These classes are available via\n[`playwright.errors`](#playwrighterrors).\n\nAn example of handling a timeout error:\n\n```js\ntry {\n  await page.waitForSelector('.foo');\n} catch (e) {\n  if (e instanceof playwright.errors.TimeoutError) {\n    // Do something if this is a timeout.\n  }\n}\n```\n\n```python async\ntry:\n    await page.wait_for_selector(\".foo\")\nexcept TimeoutError as e:\n    # do something if this is a timeout.\n```\n\n```python sync\ntry:\n    page.wait_for_selector(\".foo\")\nexcept TimeoutError as e:\n    # do something if this is a timeout.\n```\n"]
    pub fn errors(&self) -> NotImplementedYet {}
    #[doc = "This object can be used to launch or connect to Firefox, returning instances of `Browser`."]
    pub fn firefox(&self) -> BrowserType {}
    #[doc = "Selectors can be used to install custom selector engines. See [Working with selectors](./selectors.md) for more\ninformation."]
    pub fn selectors(&self) -> Selectors {}
    #[doc = "This object can be used to launch or connect to WebKit, returning instances of `Browser`."]
    pub fn webkit(&self) -> BrowserType {}
    #[doc = "Terminates this instance of Playwright, will also close all created browsers if they are still running."]
    fn close(&self) -> Result<(), Error> { todo!() }
    #[doc = "Launches new Playwright driver process and connects to it. [`method: Playwright.close`] should be called when the\ninstance is no longer needed.\n\n```java\nPlaywright playwright = Playwright.create()) {\nBrowser browser = playwright.webkit().launch();\nPage page = browser.newPage();\npage.navigate(\"https://www.w3.org/\");\nplaywright.close();\n```\n"]
    fn create(&self) -> Result<Playwright, Error> { todo!() }
    #[doc = "Terminates this instance of Playwright in case it was created bypassing the Python context manager. This is useful in\nREPL applications.\n\n```py\n>>> from playwright.sync_api import sync_playwright\n\n>>> playwright = sync_playwright().start()\n\n>>> browser = playwright.chromium.launch()\n>>> page = browser.new_page()\n>>> page.goto(\"http://whatsmyuseragent.org/\")\n>>> page.screenshot(path=\"example.png\")\n>>> browser.close()\n\n>>> playwright.stop()\n```\n"]
    fn stop(&self) -> Result<(), Arc<Error>> { todo!() }
}
struct NotImplementedYeterrors {
    #[doc = "A class of `TimeoutError`."]
    timeout_error: function
}
#[doc = "- extends: [RuntimeException]\n\nPlaywrightException is thrown whenever certain operations are terminated abnormally, e.g. browser closes while\n[`method: Page.evaluate`] is running. All Playwright exceptions inherit from this class."]
#[doc = "Extends RuntimeException"]
impl PlaywrightException {}
#[doc = "Whenever the page sends a request for a network resource the following sequence of events are emitted by `Page`:\n- [`event: Page.request`] emitted when the request is issued by the page.\n- [`event: Page.response`] emitted when/if the response status and headers are received for the request.\n- [`event: Page.requestFinished`] emitted when the response body is downloaded and the request is complete.\n\nIf request fails at some point, then instead of `'requestfinished'` event (and possibly instead of 'response' event),\nthe  [`event: Page.requestFailed`] event is emitted.\n\n> NOTE: HTTP Error responses, such as 404 or 503, are still successful responses from HTTP standpoint, so request will\ncomplete with `'requestfinished'` event.\n\nIf request gets a 'redirect' response, the request is successfully finished with the 'requestfinished' event, and a new\nrequest is  issued to a redirected url."]
impl Request {
    #[doc = "The method returns `null` unless this request has failed, as reported by `requestfailed` event.\n\nExample of logging of all the failed requests:\n\n```js\npage.on('requestfailed', request => {\n  console.log(request.url() + ' ' + request.failure().errorText);\n});\n```\n\n```java\npage.onRequestFailed(request -> {\n  System.out.println(request.url() + \" \" + request.failure());\n});\n```\n\n```py\npage.on(\"requestfailed\", lambda request: print(request.url + \" \" + request.failure))\n```\n"]
    fn failure(&self) -> Result<Option<String>, Error> { todo!() }
    #[doc = "Returns the `Frame` that initiated this request."]
    fn frame(&self) -> Result<Frame, Error> { todo!() }
    #[doc = "An object with HTTP headers associated with the request. All header names are lower-case."]
    fn headers(&self) -> Result<Map<String, String>, Error> { todo!() }
    #[doc = "Whether this request is driving frame's navigation."]
    fn is_navigation_request(&self) -> Result<bool, Error> { todo!() }
    #[doc = "Request's method (GET, POST, etc.)"]
    fn method(&self) -> Result<String, Error> { todo!() }
    #[doc = "Request's post body, if any."]
    fn post_data(&self) -> Result<Option<String>, Error> { todo!() }
    #[doc = "Request's post body in a binary form, if any."]
    fn post_data_buffer(&self) -> Result<Option<Buffer>, Error> { todo!() }
    #[doc = "Returns parsed request's body for `form-urlencoded` and JSON as a fallback if any.\n\nWhen the response is `application/x-www-form-urlencoded` then a key/value object of the values will be returned.\nOtherwise it will be parsed as JSON."]
    fn post_data_j_s_o_n(&self) -> Result<Option<any>, Error> { todo!() }
    #[doc = "Request that was redirected by the server to this one, if any.\n\nWhen the server responds with a redirect, Playwright creates a new `Request` object. The two requests are connected by\n`redirectedFrom()` and `redirectedTo()` methods. When multiple server redirects has happened, it is possible to\nconstruct the whole redirect chain by repeatedly calling `redirectedFrom()`.\n\nFor example, if the website `http://example.com` redirects to `https://example.com`:\n\n```js\nconst response = await page.goto('http://example.com');\nconsole.log(response.request().redirectedFrom().url()); // 'http://example.com'\n```\n\n```java\nResponse response = page.navigate(\"http://example.com\");\nSystem.out.println(response.request().redirectedFrom().url()); // \"http://example.com\"\n```\n\n```python async\nresponse = await page.goto(\"http://example.com\")\nprint(response.request.redirected_from.url) # \"http://example.com\"\n```\n\n```python sync\nresponse = page.goto(\"http://example.com\")\nprint(response.request.redirected_from.url) # \"http://example.com\"\n```\n\nIf the website `https://google.com` has no redirects:\n\n```js\nconst response = await page.goto('https://google.com');\nconsole.log(response.request().redirectedFrom()); // null\n```\n\n```java\nResponse response = page.navigate(\"https://google.com\");\nSystem.out.println(response.request().redirectedFrom()); // null\n```\n\n```python async\nresponse = await page.goto(\"https://google.com\")\nprint(response.request.redirected_from) # None\n```\n\n```python sync\nresponse = page.goto(\"https://google.com\")\nprint(response.request.redirected_from) # None\n```\n"]
    fn redirected_from(&self) -> Result<Option<Request>, Error> { todo!() }
    #[doc = "New request issued by the browser if the server responded with redirect.\n\nThis method is the opposite of [`method: Request.redirectedFrom`]:\n\n```js\nconsole.log(request.redirectedFrom().redirectedTo() === request); // true\n```\n\n```java\nSystem.out.println(request.redirectedFrom().redirectedTo() == request); // true\n```\n\n```py\nassert request.redirected_from.redirected_to == request\n```\n"]
    fn redirected_to(&self) -> Result<Option<Request>, Error> { todo!() }
    #[doc = "Contains the request's resource type as it was perceived by the rendering engine. ResourceType will be one of the\nfollowing: `document`, `stylesheet`, `image`, `media`, `font`, `script`, `texttrack`, `xhr`, `fetch`, `eventsource`,\n`websocket`, `manifest`, `other`."]
    fn resource_type(&self) -> Result<String, Error> { todo!() }
    #[doc = "Returns the matching `Response` object, or `null` if the response was not received due to error."]
    fn response(&self) -> Result<Option<Response>, Arc<Error>> { todo!() }
    #[doc = "Returns resource timing information for given request. Most of the timing values become available upon the response,\n`responseEnd` becomes available when request finishes. Find more information at\n[Resource Timing API](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming).\n\n```js\nconst [request] = await Promise.all([\n  page.waitForEvent('requestfinished'),\n  page.goto('http://example.com')\n]);\nconsole.log(request.timing());\n```\n\n```java\npage.onRequestFinished(request -> {\n  Timing timing = request.timing();\n  System.out.println(timing.responseEnd - timing.startTime);\n});\npage.navigate(\"http://example.com\");\n```\n\n```python async\nasync with page.expect_event(\"requestfinished\") as request_info:\n    await page.goto(\"http://example.com\")\nrequest = await request_info.value\nprint(request.timing)\n```\n\n```python sync\nwith page.expect_event(\"requestfinished\") as request_info:\n    page.goto(\"http://example.com\")\nrequest = request_info.value\nprint(request.timing)\n```\n"]
    fn timing(&self) -> Result<NotImplementedYet, Error> { todo!() }
    #[doc = "URL of the request."]
    fn url(&self) -> Result<String, Error> { todo!() }
    #[doc = "Returns a [JsonDocument] representation of [`method: Request.postDataBuffer`]."]
    fn get_payload_as_json(
        &self,
        #[doc = "options"]
        #[doc = "The options that control custom behaviour when parsing the JSON."]
        document_options: Option<Option<JsonDocumentOptions>>
    ) -> Result<JsonDocument, Error> {
        todo!()
    }
}
struct NotImplementedYettiming {
    #[doc = "Request start time in milliseconds elapsed since January 1, 1970 00:00:00 UTC"]
    start_time: f64,
    #[doc = "Time immediately before the browser starts the domain name lookup for the resource. The value is given in milliseconds\nrelative to `startTime`, -1 if not available."]
    domain_lookup_start: f64,
    #[doc = "Time immediately after the browser starts the domain name lookup for the resource. The value is given in milliseconds\nrelative to `startTime`, -1 if not available."]
    domain_lookup_end: f64,
    #[doc = "Time immediately before the user agent starts establishing the connection to the server to retrieve the resource. The\nvalue is given in milliseconds relative to `startTime`, -1 if not available."]
    connect_start: f64,
    #[doc = "Time immediately before the browser starts the handshake process to secure the current connection. The value is given in\nmilliseconds relative to `startTime`, -1 if not available."]
    secure_connection_start: f64,
    #[doc = "Time immediately before the user agent starts establishing the connection to the server to retrieve the resource. The\nvalue is given in milliseconds relative to `startTime`, -1 if not available."]
    connect_end: f64,
    #[doc = "Time immediately before the browser starts requesting the resource from the server, cache, or local resource. The value\nis given in milliseconds relative to `startTime`, -1 if not available."]
    request_start: f64,
    #[doc = "Time immediately after the browser starts requesting the resource from the server, cache, or local resource. The value\nis given in milliseconds relative to `startTime`, -1 if not available."]
    response_start: f64,
    #[doc = "Time immediately after the browser receives the last byte of the resource or immediately before the transport connection\nis closed, whichever comes first. The value is given in milliseconds relative to `startTime`, -1 if not available."]
    response_end: f64
}
#[doc = "`Response` class represents responses which are received by page."]
impl Response {
    #[doc = "Returns the buffer with response body."]
    fn body(&self) -> Result<Buffer, Arc<Error>> { todo!() }
    #[doc = "Waits for this response to finish, returns failure error if request failed."]
    fn finished(&self) -> Result<Option<String>, Arc<Error>> { todo!() }
    #[doc = "Returns the `Frame` that initiated this response."]
    fn frame(&self) -> Result<Frame, Error> { todo!() }
    #[doc = "Returns the object with HTTP headers associated with the response. All header names are lower-case."]
    fn headers(&self) -> Result<Map<String, String>, Error> { todo!() }
    #[doc = "Returns the JSON representation of response body.\n\nThis method will throw if the response body is not parsable via `JSON.parse`."]
    fn json(&self) -> Result<Serializable, Arc<Error>> { todo!() }
    #[doc = "Contains a boolean stating whether the response was successful (status in the range 200-299) or not."]
    fn ok(&self) -> Result<bool, Error> { todo!() }
    #[doc = "Returns the matching `Request` object."]
    fn request(&self) -> Result<Request, Error> { todo!() }
    #[doc = "Contains the status code of the response (e.g., 200 for a success)."]
    fn status(&self) -> Result<i64, Error> { todo!() }
    #[doc = "Contains the status text of the response (e.g. usually an \"OK\" for a success)."]
    fn status_text(&self) -> Result<String, Error> { todo!() }
    #[doc = "Returns the text representation of response body."]
    fn text(&self) -> Result<String, Arc<Error>> { todo!() }
    #[doc = "Contains the URL of the response."]
    fn url(&self) -> Result<String, Error> { todo!() }
    #[doc = "Gets the [System.Net.HttpStatusCode] code of the response."]
    fn status_code(&self) -> Result<u16, Error> { todo!() }
}
#[doc = "Whenever a network route is set up with [`method: Page.route`] or [`method: BrowserContext.route`], the `Route` object\nallows to handle the route."]
impl Route {
    #[doc = "Aborts the route's request."]
    fn abort(
        &self,
        #[doc = "Optional error code. Defaults to `failed`, could be one of the following:\n- `'aborted'` - An operation was aborted (due to user action)\n- `'accessdenied'` - Permission to access a resource, other than the network, was denied\n- `'addressunreachable'` - The IP address is unreachable. This usually means that there is no route to the specified\n  host or network.\n- `'blockedbyclient'` - The client chose to block the request.\n- `'blockedbyresponse'` - The request failed because the response was delivered along with requirements which are not\n  met ('X-Frame-Options' and 'Content-Security-Policy' ancestor checks, for instance).\n- `'connectionaborted'` - A connection timed out as a result of not receiving an ACK for data sent.\n- `'connectionclosed'` - A connection was closed (corresponding to a TCP FIN).\n- `'connectionfailed'` - A connection attempt failed.\n- `'connectionrefused'` - A connection attempt was refused.\n- `'connectionreset'` - A connection was reset (corresponding to a TCP RST).\n- `'internetdisconnected'` - The Internet connection has been lost.\n- `'namenotresolved'` - The host name could not be resolved.\n- `'timedout'` - An operation timed out.\n- `'failed'` - A generic failure occurred."]
        error_code: Option<String>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Continues route's request with optional overrides.\n\n```js\nawait page.route('**/*', (route, request) => {\n  // Override headers\n  const headers = {\n    ...request.headers(),\n    foo: 'bar', // set \"foo\" header\n    origin: undefined, // remove \"origin\" header\n  };\n  route.continue({headers});\n});\n```\n\n```java\npage.route(\"**/*\", route -> {\n  // Override headers\n  Map<String, String> headers = new HashMap<>(route.request().headers());\n  headers.put(\"foo\", \"bar\"); // set \"foo\" header\n  headers.remove(\"origin\"); // remove \"origin\" header\n  route.resume(new Route.ResumeOptions().setHeaders(headers));\n});\n```\n\n```python async\nasync def handle(route, request):\n    # override headers\n    headers = {\n        **request.headers,\n        \"foo\": \"bar\" # set \"foo\" header\n        \"origin\": None # remove \"origin\" header\n    }\n    await route.continue_(headers=headers)\n}\nawait page.route(\"**/*\", handle)\n```\n\n```python sync\ndef handle(route, request):\n    # override headers\n    headers = {\n        **request.headers,\n        \"foo\": \"bar\" # set \"foo\" header\n        \"origin\": None # remove \"origin\" header\n    }\n    route.continue_(headers=headers)\n}\npage.route(\"**/*\", handle)\n```\n"]
    fn r#continue(
        &self,
        #[doc = "options"]
        #[doc = "If set changes the request HTTP headers. Header values will be converted to a string."]
        headers: Option<Map<String, String>>,
        #[doc = "If set changes the request method (e.g. GET or POST)"] method: Option<String>,
        #[doc = "If set changes the post data of request"] post_data: Option<NotImplementedYet>,
        #[doc = "If set changes the request URL. New URL must have same protocol as original one."]
        url: Option<String>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "Fulfills route's request with given response.\n\nAn example of fulfilling all requests with 404 responses:\n\n```js\nawait page.route('**/*', route => {\n  route.fulfill({\n    status: 404,\n    contentType: 'text/plain',\n    body: 'Not Found!'\n  });\n});\n```\n\n```java\npage.route(\"**/*\", route -> {\n  route.fulfill(new Route.FulfillOptions()\n    .setStatus(404)\n    .setContentType(\"text/plain\")\n    .setBody(\"Not Found!\"));\n});\n```\n\n```python async\nawait page.route(\"**/*\", lambda route: route.fulfill(\n    status=404,\n    content_type=\"text/plain\",\n    body=\"not found!\"))\n```\n\n```python sync\npage.route(\"**/*\", lambda route: route.fulfill(\n    status=404,\n    content_type=\"text/plain\",\n    body=\"not found!\"))\n```\n\nAn example of serving static file:\n\n```js\nawait page.route('**/xhr_endpoint', route => route.fulfill({ path: 'mock_data.json' }));\n```\n\n```java\npage.route(\"**/xhr_endpoint\", route -> route.fulfill(\n  new Route.FulfillOptions().setPath(Paths.get(\"mock_data.json\")));\n```\n\n```python async\nawait page.route(\"**/xhr_endpoint\", lambda route: route.fulfill(path=\"mock_data.json\"))\n```\n\n```python sync\npage.route(\"**/xhr_endpoint\", lambda route: route.fulfill(path=\"mock_data.json\"))\n```\n"]
    fn fulfill(
        &self,
        #[doc = "options"]
        #[doc = "Response body."]
        body: Option<NotImplementedYet>,
        #[doc = "Optional response body as text."] body: Option<String>,
        #[doc = "Optional response body as raw bytes."] body_bytes: Option<Buffer>,
        #[doc = "If set, equals to setting `Content-Type` response header."] content_type: Option<
            String
        >,
        #[doc = "Response headers. Header values will be converted to a string."] headers: Option<
            Map<String, String>
        >,
        #[doc = "File path to respond with. The content type will be inferred from file extension. If `path` is a relative path, then it\nis resolved relative to the current working directory."]
        path: Option<path>,
        #[doc = "Response status code, defaults to `200`."] status: Option<i64>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
    #[doc = "A request to be routed."]
    fn request(&self) -> Result<Request, Error> { todo!() }
}
#[doc = "Selectors can be used to install custom selector engines. See [Working with selectors](./selectors.md) for more\ninformation."]
impl Selectors {
    #[doc = "An example of registering selector engine that queries elements based on a tag name:\n\n```js\nconst { selectors, firefox } = require('playwright');  // Or 'chromium' or 'webkit'.\n\n(async () => {\n  // Must be a function that evaluates to a selector engine instance.\n  const createTagNameEngine = () => ({\n    // Returns the first element matching given selector in the root's subtree.\n    query(root, selector) {\n      return root.querySelector(selector);\n    },\n\n    // Returns all elements matching given selector in the root's subtree.\n    queryAll(root, selector) {\n      return Array.from(root.querySelectorAll(selector));\n    }\n  });\n\n  // Register the engine. Selectors will be prefixed with \"tag=\".\n  await selectors.register('tag', createTagNameEngine);\n\n  const browser = await firefox.launch();\n  const page = await browser.newPage();\n  await page.setContent(`<div><button>Click me</button></div>`);\n\n  // Use the selector prefixed with its name.\n  const button = await page.$('tag=button');\n  // Combine it with other selector engines.\n  await page.click('tag=div >> text=\"Click me\"');\n  // Can use it in any methods supporting selectors.\n  const buttonCount = await page.$$eval('tag=button', buttons => buttons.length);\n\n  await browser.close();\n})();\n```\n\n```java\n// Script that evaluates to a selector engine instance.\nString createTagNameEngine = \"{\\n\" +\n  \"  // Returns the first element matching given selector in the root's subtree.\\n\" +\n  \"  query(root, selector) {\\n\" +\n  \"    return root.querySelector(selector);\\n\" +\n  \"  },\\n\" +\n  \"  // Returns all elements matching given selector in the root's subtree.\\n\" +\n  \"  queryAll(root, selector) {\\n\" +\n  \"    return Array.from(root.querySelectorAll(selector));\\n\" +\n  \"  }\\n\" +\n  \"}\";\n// Register the engine. Selectors will be prefixed with \"tag=\".\nplaywright.selectors().register(\"tag\", createTagNameEngine);\nBrowser browser = playwright.firefox().launch();\nPage page = browser.newPage();\npage.setContent(\"<div><button>Click me</button></div>\");\n// Use the selector prefixed with its name.\nElementHandle button = page.querySelector(\"tag=button\");\n// Combine it with other selector engines.\npage.click(\"tag=div >> text=\\\"Click me\\\"\");\n// Can use it in any methods supporting selectors.\nint buttonCount = (int) page.evalOnSelectorAll(\"tag=button\", \"buttons => buttons.length\");\nbrowser.close();\n```\n\n```python async\n# FIXME: add snippet\n```\n\n```python sync\n# FIXME: add snippet\n```\n"]
    fn register(
        &self,
        #[doc = "Name that is used in selectors as a prefix, e.g. `{name: 'foo'}` enables `foo=myselectorbody` selectors. May only\ncontain `[a-zA-Z0-9_]` characters."]
        name: String,
        #[doc = "Script that evaluates to a selector engine instance."] script: NotImplementedYet,
        #[doc = "Script that evaluates to a selector engine instance."] script: NotImplementedYet,
        #[doc = "options"]
        #[doc = "Whether to run this selector engine in isolated JavaScript environment. This environment has access to the same DOM, but\nnot any JavaScript objects from the frame's scripts. Defaults to `false`. Note that running as a content script is not\nguaranteed when this engine is used together with other registered engines."]
        content_script: Option<bool>,
        #[doc = "Path to the JavaScript file. If `path` is a relative path, then it is resolved relative to the current working\ndirectory."]
        path: Option<path>,
        #[doc = "Raw script content."] script: Option<String>
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
}
enum NotImplementedYetscript {
    NotImplementedYet(function),
    NotImplementedYet(String),
    NotImplementedYet(NotImplementedYet)
}
struct NotImplementedYet {
    #[doc = "Path to the JavaScript file. If `path` is a relative path, then it is resolved relative to the current working\ndirectory. Optional."]
    path: Option<path>,
    #[doc = "Raw script content. Optional."]
    content: Option<String>
}
enum NotImplementedYetscript {
    NotImplementedYet(String),
    NotImplementedYet(path)
}
#[doc = "- extends: `Error`\n\nTimeoutError is emitted whenever certain operations are terminated due to timeout, e.g. [`method: Page.waitForSelector`]\nor [`method: BrowserType.launch`]."]
#[doc = "Extends Error"]
impl TimeoutError {}
#[doc = "The Touchscreen class operates in main-frame CSS pixels relative to the top-left corner of the viewport. Methods on the\ntouchscreen can only be used in browser contexts that have been initialized with `hasTouch` set to true."]
impl Touchscreen {
    #[doc = "Dispatches a `touchstart` and `touchend` event with a single touch at the position (`x`,`y`)."]
    fn tap(&self, #[doc = ""] x: f64, #[doc = ""] y: f64) -> Result<(), Arc<Error>> { todo!() }
}
#[doc = "When browser context is created with the `recordVideo` option, each page has a video object associated with it.\n\n```js\nconsole.log(await page.video().path());\n```\n\n```java\nSystem.out.println(page.video().path());\n```\n\n```python async\nprint(await page.video.path())\n```\n\n```python sync\nprint(page.video.path())\n```\n"]
impl Video {
    #[doc = "Deletes the video file. Will wait for the video to finish if necessary."]
    fn delete(&self) -> Result<(), Arc<Error>> { todo!() }
    #[doc = "Returns the file system path this video will be recorded to. The video is guaranteed to be written to the filesystem\nupon closing the browser context. This method throws when connected remotely."]
    fn path(&self) -> Result<path, Arc<Error>> { todo!() }
    #[doc = "Saves the video to a user-specified path. It is safe to call this method while the video is still in progress, or after\nthe page has closed. This method waits until the page is closed and the video is fully saved."]
    fn save_as(
        &self,
        #[doc = "Path where the video should be saved."] path: path
    ) -> Result<(), Arc<Error>> {
        todo!()
    }
}
#[doc = "The `WebSocket` class represents websocket connections in the page."]
impl WebSocket {
    #[doc = "Indicates that the web socket has been closed."]
    fn is_closed(&self) -> Result<bool, Error> { todo!() }
    #[doc = "Contains the URL of the WebSocket."]
    fn url(&self) -> Result<String, Error> { todo!() }
    #[doc = "Waits for event to fire and passes its value into the predicate function. Returns when the predicate returns truthy\nvalue. Will throw an error if the webSocket is closed before the event is fired. Returns the event data value."]
    fn wait_for_event(
        &self,
        #[doc = "Event name, same one would pass into `webSocket.on(event)`."] event: String,
        #[doc = "Either a predicate that receives an event or an options object. Optional."]
        options_or_predicate: Option<NotImplementedYet>,
        #[doc = "options"]
        #[doc = "Receives the event data and resolves to truthy value when the waiting should resolve."]
        predicate: Option<function>,
        #[doc = "Maximum time to wait for in milliseconds. Defaults to `30000` (30 seconds). Pass `0` to disable timeout. The default\nvalue can be changed by using the [`method: BrowserContext.setDefaultTimeout`]."]
        timeout: Option<f64>
    ) -> Result<any, Arc<Error>> {
        todo!()
    }
    #[doc = "Performs action and waits for a frame to be sent. If predicate is provided, it passes `WebSocketFrame` value into the\n`predicate` function and waits for `predicate(webSocketFrame)` to return a truthy value. Will throw an error if the\nWebSocket or Page is closed before the frame is received."]
    fn wait_for_frame_received(
        &self,
        #[doc = "Callback that performs the action triggering the event."] callback: Runnable,
        #[doc = "options"]
        #[doc = "Receives the `WebSocketFrame` object and resolves to truthy value when the waiting should resolve."]
        predicate: Option<function>,
        #[doc = "Maximum time to wait for in milliseconds. Defaults to `30000` (30 seconds). Pass `0` to disable timeout. The default\nvalue can be changed by using the [`method: BrowserContext.setDefaultTimeout`]."]
        timeout: Option<f64>
    ) -> Result<WebSocketFrame, Arc<Error>> {
        todo!()
    }
    #[doc = "Performs action and waits for a frame to be sent. If predicate is provided, it passes `WebSocketFrame` value into the\n`predicate` function and waits for `predicate(webSocketFrame)` to return a truthy value. Will throw an error if the\nWebSocket or Page is closed before the frame is sent."]
    fn wait_for_frame_sent(
        &self,
        #[doc = "Callback that performs the action triggering the event."] callback: Runnable,
        #[doc = "options"]
        #[doc = "Receives the `WebSocketFrame` object and resolves to truthy value when the waiting should resolve."]
        predicate: Option<function>,
        #[doc = "Maximum time to wait for in milliseconds. Defaults to `30000` (30 seconds). Pass `0` to disable timeout. The default\nvalue can be changed by using the [`method: BrowserContext.setDefaultTimeout`]."]
        timeout: Option<f64>
    ) -> Result<WebSocketFrame, Arc<Error>> {
        todo!()
    }
    #[doc = "> NOTE: In most cases, you should use [`method: WebSocket.waitForEvent`].\n\nWaits for given `event` to fire. If predicate is provided, it passes event's value into the `predicate` function and\nwaits for `predicate(event)` to return a truthy value. Will throw an error if the socket is closed before the `event` is\nfired."]
    fn wait_for_event2(
        &self,
        #[doc = "Event name, same one typically passed into `*.on(event)`."] event: String,
        #[doc = "options"]
        #[doc = "Receives the event data and resolves to truthy value when the waiting should resolve."]
        predicate: Option<function>,
        #[doc = "Maximum time to wait for in milliseconds. Defaults to `30000` (30 seconds). Pass `0` to disable timeout. The default\nvalue can be changed by using the [`method: BrowserContext.setDefaultTimeout`]."]
        timeout: Option<f64>
    ) -> Result<Any, Arc<Error>> {
        todo!()
    }
}
struct NotImplementedYetframeReceived {
    #[doc = "frame payload"]
    payload: NotImplementedYet
}
enum NotImplementedYetpayload {
    NotImplementedYet(String),
    NotImplementedYet(Buffer)
}
struct NotImplementedYetframeSent {
    #[doc = "frame payload"]
    payload: NotImplementedYet
}
enum NotImplementedYetpayload {
    NotImplementedYet(String),
    NotImplementedYet(Buffer)
}
enum NotImplementedYetoptionsOrPredicate {
    NotImplementedYet(function),
    NotImplementedYet(NotImplementedYet)
}
struct NotImplementedYet {
    #[doc = "receives the event data and resolves to truthy value when the waiting should resolve."]
    predicate: function,
    #[doc = "maximum time to wait for in milliseconds. Defaults to `30000` (30 seconds). Pass `0` to disable timeout. The default\nvalue can be changed by using the [`method: BrowserContext.setDefaultTimeout`]."]
    timeout: Option<f64>
}
enum WebSocketEventType {
    #[doc = "Fired when the websocket closes."]
    Close,
    #[doc = "Fired when the websocket receives a frame."]
    FrameReceived,
    #[doc = "Fired when the websocket sends a frame."]
    FrameSent,
    #[doc = "Fired when the websocket has an error."]
    SocketError
}
enum WebSocketEvent {
    #[doc = "Fired when the websocket closes."]
    Close(WebSocket),
    #[doc = "Fired when the websocket receives a frame."]
    FrameReceived(NotImplementedYet),
    #[doc = "Fired when the websocket sends a frame."]
    FrameSent(NotImplementedYet),
    #[doc = "Fired when the websocket has an error."]
    SocketError(String)
}
#[doc = "The `WebSocketFrame` class represents frames sent over `WebSocket` connections in the page. Frame payload is returned by\neither [`method: WebSocketFrame.text`] or [`method: WebSocketFrame.binary`] method depending on the its type."]
impl WebSocketFrame {
    #[doc = "Returns binary payload."]
    fn binary(&self) -> Result<Option<Buffer>, Error> { todo!() }
    #[doc = "Returns text payload."]
    fn text(&self) -> Result<Option<String>, Error> { todo!() }
}
#[doc = "The Worker class represents a [WebWorker](https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API). `worker`\nevent is emitted on the page object to signal a worker creation. `close` event is emitted on the worker object when the\nworker is gone.\n\n```js\npage.on('worker', worker => {\n  console.log('Worker created: ' + worker.url());\n  worker.on('close', worker => console.log('Worker destroyed: ' + worker.url()));\n});\n\nconsole.log('Current workers:');\nfor (const worker of page.workers())\n  console.log('  ' + worker.url());\n```\n\n```java\npage.onWorker(worker -> {\n  System.out.println(\"Worker created: \" + worker.url());\n  worker.onClose(worker1 -> System.out.println(\"Worker destroyed: \" + worker1.url()));\n});\nSystem.out.println(\"Current workers:\");\nfor (Worker worker : page.workers())\n  System.out.println(\"  \" + worker.url());\n```\n\n```py\ndef handle_worker(worker):\n    print(\"worker created: \" + worker.url)\n    worker.on(\"close\", lambda: print(\"worker destroyed: \" + worker.url))\n\npage.on('worker', handle_worker)\n\nprint(\"current workers:\")\nfor worker in page.workers:\n    print(\"    \" + worker.url)\n```\n\n```csharp\nPage.Worker += (_, worker) =>\n{\n    Console.WriteLine($\"Worker created: {worker.Url}\");\n    worker.Close += (_, _) => Console.WriteLine($\"Worker closed {worker.Url}\");\n};\n\nConsole.WriteLine(\"Current Workers:\");\nforeach(var pageWorker in Page.Workers)\n{\n    Console.WriteLine($\"\\tWorker: {pageWorker.Url}\");\n}\n```\n"]
impl Worker {
    #[doc = "Returns the return value of `expression`.\n\nIf the function passed to the [`method: Worker.evaluate`] returns a [Promise], then [`method: Worker.evaluate`] would\nwait for the promise to resolve and return its value.\n\nIf the function passed to the [`method: Worker.evaluate`] returns a non-[Serializable] value, then\n[`method: Worker.evaluate`] returns `undefined`. Playwright also supports transferring some additional values that are\nnot serializable by `JSON`: `-0`, `NaN`, `Infinity`, `-Infinity`."]
    fn evaluate(
        &self,
        #[doc = "JavaScript expression to be evaluated in the browser context. If it looks like a function declaration, it is interpreted\nas a function. Otherwise, evaluated as an expression."]
        expression: String,
        #[doc = "Optional argument to pass to `expression`."] arg: Option<EvaluationArgument>
    ) -> Result<Serializable, Arc<Error>> {
        todo!()
    }
    #[doc = "Returns the return value of `expression` as a `JSHandle`.\n\nThe only difference between [`method: Worker.evaluate`] and [`method: Worker.evaluateHandle`] is that\n[`method: Worker.evaluateHandle`] returns `JSHandle`.\n\nIf the function passed to the [`method: Worker.evaluateHandle`] returns a [Promise], then\n[`method: Worker.evaluateHandle`] would wait for the promise to resolve and return its value."]
    fn evaluate_handle(
        &self,
        #[doc = "JavaScript expression to be evaluated in the browser context. If it looks like a function declaration, it is interpreted\nas a function. Otherwise, evaluated as an expression."]
        expression: String,
        #[doc = "Optional argument to pass to `expression`."] arg: Option<EvaluationArgument>
    ) -> Result<JsHandle, Arc<Error>> {
        todo!()
    }
    #[doc = ""]
    fn url(&self) -> Result<String, Error> { todo!() }
    #[doc = "Performs action and waits for the Worker to close."]
    fn wait_for_close(
        &self,
        #[doc = "Callback that performs the action triggering the event."] callback: Runnable,
        #[doc = "options"]
        #[doc = "Maximum time to wait for in milliseconds. Defaults to `30000` (30 seconds). Pass `0` to disable timeout. The default\nvalue can be changed by using the [`method: BrowserContext.setDefaultTimeout`]."]
        timeout: Option<f64>
    ) -> Result<Worker, Arc<Error>> {
        todo!()
    }
}
enum WorkerEventType {
    #[doc = "Emitted when this dedicated [WebWorker](https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API) is terminated."]
    Close
}
enum WorkerEvent {
    #[doc = "Emitted when this dedicated [WebWorker](https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API) is terminated."]
    Close(Worker)
}
// vim: foldnestmax=0 ft=rust
