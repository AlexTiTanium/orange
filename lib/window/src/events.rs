use math::Vec2;

///
/// An input event generated for window
///
#[derive(Clone, Debug, PartialEq)]
pub enum WindowInputEvent {
  /// The integration detected a "copy" event (e.g. Cmd+C).
  Copy,

  /// The integration detected a "cut" event (e.g. Cmd+X).
  Cut,

  /// The integration detected a "paste" event (e.g. Cmd+V).
  Paste(String),

  /// Text input, e.g. via keyboard.
  ///
  /// When the user presses enter/return, do not send a [`Text`](Event::Text) (just [`Key::Enter`]).
  Text(String),

  /// A key was pressed or released.
  // Key {
  //   key: Key,

  //   /// Was it pressed or released?
  //   pressed: bool,

  //   /// The state of the modifier keys at the time of the event.
  //   modifiers: Modifiers,
  // },

  /// Resize event
  Resized(u32, u32),

  /// The mouse or touch moved to a new place.
  PointerMoved(f64, f64),

  /// A mouse button was pressed or released (or a touch started or stopped).
  PointerButton {
    /// What mouse button? For touches, use [`PointerButton::Primary`].
    button: PointerButton,

    /// Was it the button/touch pressed this frame, or released?
    pressed: bool,
  },

  /// The mouse left the screen, or the last/primary touch input disappeared.
  ///
  /// This means there is no longer a cursor on the screen for hovering etc.
  ///
  /// On touch-up first send `PointerButton{pressed: false, …}` followed by `PointerLeft`.
  PointerGone,

  /// How many points (logical pixels) the user scrolled.
  ///
  /// The direction of the vector indicates how to move the _content_ that is being viewed.
  /// So if you get positive values, the content being viewed should move to the right and down,
  /// revealing new things to the left and up.
  ///
  /// A positive X-value indicates the content is being moved right,
  /// as when swiping right on a touch-screen or track-pad with natural scrolling.
  ///
  /// A positive Y-value indicates the content is being moved down,
  /// as when swiping down on a touch-screen or track-pad with natural scrolling.
  ///
  /// Shift-scroll should result in horizontal scrolling (it is up to the integrations to do this).
  Scroll(Vec2),

  /// Zoom scale factor this frame (e.g. from ctrl-scroll or pinch gesture).
  /// * `zoom = 1`: no change.
  /// * `zoom < 1`: pinch together
  /// * `zoom > 1`: pinch spread
  Zoom(f32),

  // No result
  None,
}

///
/// Position
#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct InputPosition(pub f64, pub f64);

// Keyboard keys.
//
// Includes all keys egui is interested in (such as `Home` and `End`)
// plus a few that are useful for detecting keyboard shortcuts.
//
// Many keys are omitted because they are not always physical keys (depending on keyboard language), e.g. `;` and `§`,
// and are therefore unsuitable as keyboard shortcuts if you want your app to be portable.
// #[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum Key {
  ArrowDown,
  ArrowLeft,
  ArrowRight,
  ArrowUp,

  Escape,
  Tab,
  Backspace,
  Enter,
  Space,

  Insert,
  Delete,
  Home,
  End,
  PageUp,
  PageDown,

  /// Either from the main row or from the numpad.
  Num0,
  /// Either from the main row or from the numpad.
  Num1,
  /// Either from the main row or from the numpad.
  Num2,
  /// Either from the main row or from the numpad.
  Num3,
  /// Either from the main row or from the numpad.
  Num4,
  /// Either from the main row or from the numpad.
  Num5,
  /// Either from the main row or from the numpad.
  Num6,
  /// Either from the main row or from the numpad.
  Num7,
  /// Either from the main row or from the numpad.
  Num8,
  /// Either from the main row or from the numpad.
  Num9,

  A, // Used for cmd+A (select All)
  B,
  C, // |CMD COPY|
  D, // |CMD BOOKMARK|
  E, // |CMD SEARCH|
  F, // |CMD FIND firefox & chrome|
  G, // |CMD FIND chrome|
  H, // |CMD History|
  I, // italics
  J, // |CMD SEARCH firefox/DOWNLOAD chrome|
  K, // Used for ctrl+K (delete text after cursor)
  L,
  M,
  N,
  O, // |CMD OPEN|
  P, // |CMD PRINT|
  Q,
  R, // |CMD REFRESH|
  S, // |CMD SAVE|
  T, // |CMD TAB|
  U, // Used for ctrl+U (delete text before cursor)
  V, // |CMD PASTE|
  W, // Used for ctrl+W (delete previous word)
  X, // |CMD CUT|
  Y,
  Z, // |CMD UNDO|

  // The function keys:
  F1,
  F2,
  F3,
  F4,
  F5, // |CMD REFRESH|
  F6,
  F7,
  F8,
  F9,
  F10,
  F11,
  F12,
  F13,
  F14,
  F15,
  F16,
  F17,
  F18,
  F19,
  F20,
}

/// State of the modifier keys. These must be fed to egui.
///
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Modifiers {
  /// Either of the alt keys are down (option ⌥ on Mac).
  pub alt: bool,

  /// Either of the control keys are down.
  /// When checking for keyboard shortcuts, consider using [`Self::command`] instead.
  pub ctrl: bool,

  /// Either of the shift keys are down.
  pub shift: bool,

  /// The Mac ⌘ Command key. Should always be set to `false` on other platforms.
  pub mac_cmd: bool,

  /// On Windows and Linux, set this to the same value as `ctrl`.
  /// On Mac, this should be set whenever one of the ⌘ Command keys are down (same as `mac_cmd`).
  /// This is so that egui can, for instance, select all text by checking for `command + A`
  /// and it will work on both Mac and Windows.
  pub command: bool,
}

/// Mouse button (or similar for touch input)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PointerButton {
  /// The primary mouse button is usually the left one.
  Primary = 0,

  /// The secondary mouse button is usually the right one,
  /// and most often used for context menus or other optional things.
  Secondary = 1,

  /// The tertiary mouse button is usually the middle mouse button (e.g. clicking the scroll wheel).
  Middle = 2,

  /// The first extra mouse button on some mice. In web typically corresponds to the Browser back button.
  Extra1 = 3,

  /// The second extra mouse button on some mice. In web typically corresponds to the Browser forward button.
  Extra2 = 4,
}
