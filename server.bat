@echo off
setlocal enabledelayedexpansion

if "%~1"=="start" (
    start /B cmd /C "cargo watch -x run && npx tailwindcss -i .\input.css -o .\out\output.css && cargo run"
    start /B python -m http.server --directory out
)

if "%~1"=="stop" (
    set "prevServer="
    for /f "tokens=5" %%a in ('netstat -ano ^| findstr :8000 ^| find "LISTENING"') do (
        set "server=%%~a"
        if not "!server!"=="!prevServer!" (
            @REM echo !server!
            taskkill /PID !server! /F
        )
        set "prevServer=!server!"
    )

    :: Find and kill the process named cargo-watch
    :: image name == executable file name
    for /f "tokens=2 delims=," %%b in ('tasklist /nh /fo csv /fi "imagename eq cargo-watch.exe"') do (
        set "watch=%%~b"
        @REM echo !watch!
        taskkill /PID !watch! /F
    )

    echo Cargo-watch processes have been terminated.
    pause
)