# Raw Notes

Each WM_* event bellow corresponds to:
1.  Calling and then returning from `WH_CALLWNDPROC`
2.  Calling and then returning from the `WNDPROC`.  Child nodes indicate what the wndproc called before returning.
3.  Calling and then returning from `WH_CALLWNDPROCRET`
*   See SetWindowsHookExW for WH_\*
*   Windows 10.0.19043.1526

### Typical call/event tree of calling create_window_ex_w

```text
create_window_ex_w
    WH_CBT: HCBT_CREATEWND
    WM_GETMINMAXINFO
    WM_NCCREATE
    WM_NCCALCSIZE
    WM_CREATE
```

### Typical call/event tree of clicking `X`

```text
WM_NCLBUTTONDOWN
    WM_CAPTURECHANGED
    WM_SYSCOMMAND
        WM_CLOSE
            WH_CBT: HCBT_DESTROYWND
            WM_UAHDESTROYWINDOW
            WM_WINDOWPOSCHANGING
            WM_WINDOWPOSCHANGED
            WM_NCACTIVATE
            WM_ACTIVATE
            WM_ACTIVATEAPP
            WM_KILLFOCUS
            WM_IME_SETCONTEXT
                WM_IME_NOTIFY
            WM_DESTROY
            WM_NCDESTROY
            IsWindow finally starts to return FALSE
```

### `Alt`+`F4` instead of clicking, with a single recursive destroy inside WM_DESTROY

```diff
-WM_NCLBUTTONDOWN
-   WM_CAPTURECHANGED
    WM_SYSCOMMAND
        WM_CLOSE
            WH_CBT: HCBT_DESTROYWND
            WM_UAHDESTROYWINDOW
            WM_WINDOWPOSCHANGING
            WM_WINDOWPOSCHANGED
            WM_NCACTIVATE
            WM_ACTIVATE
            WM_ACTIVATEAPP
            WM_KILLFOCUS
            WM_IME_SETCONTEXT
                WM_IME_NOTIFY
            WM_DESTROY
+               WH_CBT: HCBT_DESTROYWND
+               WM_UAHDESTROYWINDOW
+               WM_DESTROY
+               WM_NCDESTROY
+               IsWindow finally starts to return FALSE
            WM_NCDESTROY
-           IsWindow finally starts to return FALSE
```
