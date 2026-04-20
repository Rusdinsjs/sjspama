@echo off
setlocal
TITLE SJS-PAMA Launcher

echo ======================================================
echo           SJS-PAMA SYSTEM - ALL-IN-ONE STARTER
echo ======================================================
echo.

:: 1. Check Podman
echo [1/3] Memeriksa Infrastruktur (Podman)...
:: Pastikan podman machine sudah jalan (khusus Windows)
podman machine start >nul 2>&1
if %ERRORLEVEL% EQU 0 (
    echo [INFO] Podman machine baru saja diaktifkan.
)

podman compose ps >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo [ERROR] Podman tidak terdeteksi atau belum siap.
    echo Pastikan Podman sudah terinstall dan podman machine sudah running.
    pause
    exit /b
)
podman compose up -d
echo [OK] Database ^& Redis sudah jalan.
echo.

:: 2. Check Backend
if not exist "backend" (
    echo [ERROR] Folder 'backend' tidak ditemukan!
    pause
    exit /b
)
echo [2/3] Menyiapkan Backend (Rust)...
:: Gunakan start cmd /k agar jendela tetap terbuka untuk melihat log
start "SJS-PAMA BACKEND (Port 8081)" cmd /k "cd backend && echo Menjalankan Rust Backend... && cargo run"
echo [OK] Backend dijalankan di jendela baru.
echo.

:: 3. Check Frontend
if not exist "frontend" (
    echo [ERROR] Folder 'frontend' tidak ditemukan!
    pause
    exit /b
)
echo [3/3] Menyiapkan Frontend (SvelteKit)...
start "SJS-PAMA FRONTEND (Port 5173)" cmd /k "cd frontend && echo Menjalankan Svelte Frontend... && npm run dev"
echo [OK] Frontend dijalankan di jendela baru.
echo.

echo ======================================================
echo SEMUA MODUL SEDANG DIJALANKAN!
echo.
echo URL Akses:
echo - Frontend : http://localhost:5173
echo - Backend  : http://localhost:8081/api/health
echo - Database : localhost:5432 (Postgres)
echo.
echo Anda dapat menutup jendela ini. 
echo Backend dan Frontend akan tetap berjalan di jendela masing-masing.
echo ======================================================
pause
