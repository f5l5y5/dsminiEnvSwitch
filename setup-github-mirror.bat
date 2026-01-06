@echo off
REM GitHub 镜像仓库配置脚本
REM 用于在现有的 Gitee 仓库基础上添加 GitHub 镜像

echo ========================================
echo GitHub 镜像仓库配置助手
echo ========================================
echo.
echo 当前远程仓库:
git remote -v
echo.

echo ========================================
echo 设置步骤：
echo ========================================
echo.
echo 1️⃣  在 GitHub 上创建新仓库
echo    - 访问: https://github.com/new
echo    - 仓库名建议: dsmini-env-switch
echo    - 可以设置为私有
echo    - 创建后复制仓库 URL
echo.
pause

echo.
set /p GITHUB_URL="请输入 GitHub 仓库 URL (例如: https://github.com/username/dsmini-env-switch.git): "

echo.
echo 2️⃣  添加 GitHub 为远程仓库...
git remote add github %GITHUB_URL%

if %ERRORLEVEL% NEQ 0 (
    echo ❌ 添加远程仓库失败
    echo 可能远程仓库 'github' 已存在，尝试更新...
    git remote set-url github %GITHUB_URL%
)

echo ✅ GitHub 远程仓库配置完成
echo.
echo ========================================
echo 当前远程仓库配置:
echo ========================================
git remote -v
echo.

echo ========================================
echo 📝 后续使用说明:
echo ========================================
echo.
echo 推送到 Gitee:
echo   git push gitee main
echo.
echo 推送到 GitHub:
echo   git push github main
echo.
echo 同时推送到两个平台:
echo   git push gitee github main
echo.
echo 或使用同步脚本:
echo   sync.bat
echo.

pause
