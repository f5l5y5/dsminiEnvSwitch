@echo off
REM 代码同步脚本 - 同时推送到 Gitee 和 GitHub

echo ========================================
echo 同步代码到 Gitee 和 GitHub
echo ========================================
echo.

echo [1/5] 检查当前分支...
for /f "tokens=2" %%i in ('git branch --show-current') do set CURRENT_BRANCH=%%i
if not defined CURRENT_BRANCH (
    set CURRENT_BRANCH=main
)
echo 当前分支: %CURRENT_BRANCH%
echo.

echo [2/5] 拉取最新代码...
git pull gitee %CURRENT_BRANCH%
echo.

echo [3/5] 推送到 Gitee...
git push gitee %CURRENT_BRANCH%
if %ERRORLEVEL% NEQ 0 (
    echo ⚠️  推送到 Gitee 失败，但继续推送到 GitHub...
)
echo.

echo [4/5] 推送到 GitHub...
git push github %CURRENT_BRANCH%
if %ERRORLEVEL% NEQ 0 (
    echo ⚠️  推送到 GitHub 失败
    echo 请检查:
    echo   1. GitHub 仓库是否已创建
    echo   2. 网络连接是否正常
    echo   3. GitHub 凭证是否正确
    goto :error
)
echo.

echo [5/5] 同步标签...
git push gitee --tags
git push github --tags
echo.

echo ========================================
echo ✅ 同步完成！
echo ========================================
echo.
echo 📦 构建触发提示:
echo   - 如需触发 GitHub Actions 构建，请推送标签
echo   - 命令: git tag v1.0.0 && sync.bat
echo   - 或访问 GitHub Actions 页面手动触发
echo.
goto :end

:error
echo.
echo ========================================
echo ❌ 同步失败
echo ========================================
echo.
echo 请检查错误信息并重试
echo.

:end
pause
