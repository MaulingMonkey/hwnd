Common errors returned by windows / this crate and their conditions.

| winresult                         | condition |
| --------------------------------- | --------- |
| ERROR::ACCESS_DENIED              | HWND belongs to another process.  (Blocked by UIPI (User Interface Process Isolation)?)
| ERROR::CLASS_ALREADY_EXISTS       | Duplicate window class registration
| ERROR::DATATYPE_MISMATCH          | Internal bugs
| ERROR::INVALID_INDEX              | Invalid GWLP_\* for GetWindowLongPtr
| ERROR::INVALID_PARAMETER          | Too many to count!
| ERROR::INVALID_THREAD_ID          | Invalid thread ID for e.g. PostThreadMessage
| ERROR::INVALID_WINDOW_HANDLE      | Invalid hwnd, or HWND belongs to another process/thread
| ERROR::MESSAGE_SYNC_ONLY          | System message contains pointers, yet tried to send to another thread or message queue
| ERROR::NOT_ENOUGH_MEMORY          | Ran out of 16-bit atoms for class or message names.
| ERROR::NOT_ENOUGH_QUOTA           | Message queue full
| ERROR::RESOURCE_DATA_NOT_FOUND    | Invalid resource atom/# (icons etc.)
| ERROR::RESOURCE_NAME_NOT_FOUND    | Invalid resource name (icons etc.)
| ERROR::WINDOW_OF_OTHER_THREAD     | Thread-local hwnd data accessed from the wrong thread
