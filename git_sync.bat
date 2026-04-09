@echo off
echo ============================================
echo      SYNC SJS-PAMA TO GITHUB (MAIN)
echo ============================================
echo.

echo 1. Menarik seluruh perubahan file kodemu...
git add .
echo [OK]

echo.
echo 2. Menyimpan catatan perubahan (Commit)...
git commit -m "Update SJS-PAMA System: Employee Master, Excel Import, and Logic Fixes - %date% %time%"
echo [OK]

echo.
echo 3. Mengirimkan ke GitHub (Push)...
git push origin main
echo [OK]

echo.
echo ============================================
echo     Selesai! Kodemu sudah aman di GitHub.
echo ============================================
pause
