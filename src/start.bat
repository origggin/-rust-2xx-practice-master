@echo off
setlocal enabledelayedexpansion

for /l %%i in (12, 1, 31) do (
    set "filename=chapter%%i.rs"
    echo // Content for chapter %%i > "!filename!"
)

echo Files created from chapter12.rs to chapter31.rs.
