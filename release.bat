@echo off
REM Tauri 应用发布脚本
REM 用于创建版本标签并同步到远程仓库，触发 GitHub Actions 构建

echo ========================================
echo Tauri 应用发布助手
echo ========================================
echo.

echo 当前版本信息:
for /f "tokens=2" %%i in ('findstr "\"name\": \"dsminienvswitch\"" package.json') do set APP_NAME=%%i
for /f "tokens=2" %%i in ('findstr "\"version\"" package.json') do set VERSION=%%i
echo 应用名称: %APP_NAME%
echo 当前版本: %VERSION%
echo.

echo ========================================
echo 选择发布类型:
echo ========================================
echo.
echo 1. 修订版本 (1.0.0 -^> 1.0.1)
echo 2. 次版本 (1.0.0 -^> 1.1.0)
echo 3. 主版本 (1.0.0 -^> 2.0.0)
echo 4. 自定义版本号
echo 5. 取消
echo.

set /p CHOICE="请选择 (1-5): "

if "%CHOICE%"=="1" (
    call npm version patch
    goto :sync
)
if "%CHOICE%"=="2" (
    call npm version minor
    goto :sync
)
if "%CHOICE%"=="3" (
    call npm version major
    goto :sync
)
if "%CHOICE%"=="4" (
    set /p CUSTOM_VERSION="请输入版本号 (例如: v1.2.3): "
    if not "%CUSTOM_VERSION:~0,1%"=="v" (
        echo ❌ 版本号必须以 'v' 开头
        pause
        exit /b 1
    )
    set TAG_NAME=%CUSTOM_VERSION%
    goto :create_tag
)
if "%CHOICE%"=="5" (
    echo 已取消
    pause
    exit /b 0
)

echo ❌ 无效选择
pause
exit /b 1

:sync
echo.
echo ========================================
echo 正在同步...
echo.

call npm version patch >nul 2>&1
if %ERRORLEVEL% EQU 0 (
    echo ✅ 版本已更新
    for /f "tokens=2" %%i in ('findstr "\"version\"" package.json') do set NEW_VERSION=%%i
    set TAG_NAME=v%NEW_VERSION:~1,-2%
) else (
    echo ⚠️  npm version 失败，使用当前版本
    set TAG_NAME=v%VERSION:~1,-2%
)

:create_tag
echo.
echo [1/4] 提交版本更新...
git add package.json
git commit -m "chore: release %TAG_NAME%" --allow-empty

echo.
echo [2/4] 创建标签 %TAG_NAME%...
git tag %TAG_NAME%
if %ERRORLEVEL% NEQ 0 (
    echo ❌ 创建标签失败
    echo 标签可能已存在，使用 --force 参数强制创建
    git tag -d %TAG_NAME% 2>nul
    git tag %TAG_NAME%
)

echo.
echo [3/4] 推送到 Gitee...
git push gitee main
git push gitee --tags
if %ERRORLEVEL% NEQ 0 (
    echo ⚠️  推送到 Gitee 失败
)

echo.
echo [4/4] 推送到 GitHub 并触发构建...
git push github main
git push github --tags
if %ERRORLEVEL% NEQ 0 (
    echo ❌ 推送到 GitHub 失败
    echo 请检查:
    echo   1. GitHub 远程仓库是否已配置
    echo   2. 网络连接是否正常
    echo   3. GitHub 凭证是否正确
    pause
    exit /b 1
)

echo.
echo ========================================
echo ✅ 发布成功！
echo ========================================
echo.
echo 📦 版本: %TAG_NAME%
echo.
echo 🔨 GitHub Actions 构建状态:
echo    https://github.com/你的用户名/dsmini-env-switch/actions
echo.
echo 📥 构建完成后下载安装包:
echo    https://github.com/你的用户名/dsmini-env-switch/actions
echo.
echo 💡 提示:
echo    - 构建通常需要 10-15 分钟
echo    - 构建完成后会在 Artifacts 中生成安装包
echo    - 可以创建 GitHub Release 正式发布版本
echo.

pause
