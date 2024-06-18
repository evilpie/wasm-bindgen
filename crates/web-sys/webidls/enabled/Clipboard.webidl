/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/*
 * Clipboard API and events
 * Editor’s Draft, 4 June 2024
 *
 * The origin of this IDL file is:
 * https://w3c.github.io/clipboard-apis/#idl-index
 */

partial interface Navigator {
  [SecureContext, SameObject] readonly attribute Clipboard clipboard;
};

typedef Promise<(DOMString or Blob)> ClipboardItemData;

[SecureContext, Exposed=Window]
interface ClipboardItem {
  constructor(record<DOMString, ClipboardItemData> items,
              optional ClipboardItemOptions options = {});

  readonly attribute PresentationStyle presentationStyle;
  readonly attribute FrozenArray<DOMString> types;

  Promise<Blob> getType(DOMString type);

  // Not supported by WebKit
  // static boolean supports(DOMString type);
};

enum PresentationStyle { "unspecified", "inline", "attachment" };

dictionary ClipboardItemOptions {
  PresentationStyle presentationStyle = "unspecified";
};

typedef sequence<ClipboardItem> ClipboardItems;

[SecureContext, Exposed=Window]
interface Clipboard : EventTarget {
  Promise<ClipboardItems> read(/* optional ClipboardUnsanitizedFormats formats = {} */);
  Promise<DOMString> readText();
  Promise<undefined> write(ClipboardItems data);
  Promise<undefined> writeText(DOMString data);
};

// There is no consensus by browsers to implement this API.
// See https://github.com/w3c/clipboard-apis/pull/197.
// dictionary ClipboardUnsanitizedFormats {
//   sequence<DOMString> unsanitized;
// };

// Also Chrome-only
// dictionary ClipboardPermissionDescriptor : PermissionDescriptor {
//   boolean allowWithoutGesture = false;
// };