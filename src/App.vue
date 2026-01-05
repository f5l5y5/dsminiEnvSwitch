<script setup lang="ts">
import Layout from './components/layout/Layout.vue';
</script>

<template>
  <Layout />
</template>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

:root {
  /* 现代化配色方案 - 清新明亮 */
  --primary-color: #3b82f6;
  --primary-hover: #2563eb;
  --primary-light: #eff6ff;
  --secondary-color: #06b6d4;
  --success-color: #10b981;
  --danger-color: #f43f5e;
  --warning-color: #f59e0b;

  /* 背景色 */
  --bg-primary: #fafafa;
  --bg-secondary: #ffffff;
  --bg-tertiary: #f3f4f6;

  /* 文字颜色 */
  --text-primary: #111827;
  --text-secondary: #6b7280;
  --text-tertiary: #9ca3af;

  /* 边框颜色 */
  --border-color: #e5e7eb;
  --border-hover: #d1d5db;

  /* 阴影 */
  --shadow-sm: 0 1px 2px 0 rgb(0 0 0 / 0.05);
  --shadow-md: 0 4px 6px -1px rgb(0 0 0 / 0.07), 0 2px 4px -2px rgb(0 0 0 / 0.06);
  --shadow-lg: 0 10px 15px -3px rgb(0 0 0 / 0.08), 0 4px 6px -4px rgb(0 0 0 / 0.06);
  --shadow-xl: 0 20px 25px -5px rgb(0 0 0 / 0.08), 0 8px 10px -6px rgb(0 0 0 / 0.06);

  /* 圆角 */
  --radius-sm: 6px;
  --radius-md: 10px;
  --radius-lg: 14px;
  --radius-xl: 20px;

  /* 过渡 */
  --transition-fast: 150ms ease;
  --transition-base: 200ms ease;
  --transition-slow: 300ms ease;
}

body {
  font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
  font-size: 14px;
  line-height: 1.6;
  color: var(--text-primary);
  background: linear-gradient(135deg, #e0f2fe 0%, #dbeafe 50%, #ede9fe 100%);
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

#app {
  height: 100vh;
  overflow: hidden;
}

/* 滚动条美化 */
::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

::-webkit-scrollbar-track {
  background: var(--bg-tertiary);
  border-radius: 4px;
}

::-webkit-scrollbar-thumb {
  background: var(--text-tertiary);
  border-radius: 4px;
  transition: background var(--transition-base);
}

::-webkit-scrollbar-thumb:hover {
  background: var(--text-secondary);
}
</style>