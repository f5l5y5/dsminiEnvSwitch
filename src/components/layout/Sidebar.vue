<template>
  <div class="sidebar" :class="{ 'is-open': isOpen }">
    <div class="sidebar-header">
      <div class="logo">
        <svg width="32" height="32" viewBox="0 0 32 32" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="32" height="32" rx="8" fill="url(#logo-gradient)"/>
          <path d="M16 8L20 12H18V16H20L16 20L12 16H14V12H12L16 8Z" fill="white"/>
          <defs>
            <linearGradient id="logo-gradient" x1="0" y1="0" x2="32" y2="32" gradientUnits="userSpaceOnUse">
              <stop offset="0%" stop-color="#3b82f6"/>
              <stop offset="100%" stop-color="#06b6d4"/>
            </linearGradient>
          </defs>
        </svg>
        <h2>环境配置管理</h2>
      </div>
      <button class="close-btn" @click="$emit('close')" v-if="isOpen">
        <svg width="20" height="20" viewBox="0 0 20 20" fill="none" xmlns="http://www.w3.org/2000/svg">
          <path d="M15 5L5 15M5 5L15 15" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
      </button>
    </div>
    <nav class="sidebar-nav">
      <router-link to="/environments" class="nav-link" @click="handleNavClick">
        <svg width="20" height="20" viewBox="0 0 20 20" fill="none" xmlns="http://www.w3.org/2000/svg">
          <path d="M4 3H16C16.5523 3 17 3.44772 17 4V16C17 16.5523 16.5523 17 16 17H4C3.44772 17 3 16.5523 3 16V4C3 3.44772 3.44772 3 4 3Z" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
          <path d="M3 7H17" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
          <path d="M7 3V5" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
          <path d="M13 3V5" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
        </svg>
        <span>环境列表</span>
      </router-link>
      <router-link to="/settings" class="nav-link" @click="handleNavClick">
        <svg width="20" height="20" viewBox="0 0 20 20" fill="none" xmlns="http://www.w3.org/2000/svg">
          <path d="M10 2C10.5523 2 11 2.44772 11 3V4.07001C11.7166 4.19986 12.4007 4.43625 13.0367 4.76441L13.7929 4.0082C14.1834 3.61768 14.8166 3.61768 15.2071 4.0082L16.6213 5.42241C17.0118 5.81294 17.0118 6.4461 16.6213 6.83663L15.8651 7.59284C16.1933 8.22884 16.4296 8.91297 16.5595 9.62955H17.6295C18.1818 9.62955 18.6295 10.0773 18.6295 10.6295V12.6295C18.6295 13.1818 18.1818 13.6295 17.6295 13.6295H16.5595C16.4296 14.3461 16.1933 15.0303 15.8651 15.6663L16.6213 16.4225C17.0118 16.813 17.0118 17.4462 16.6213 17.8367L15.2071 19.2509C14.8166 19.6414 14.1834 19.6414 13.7929 19.2509L13.0367 18.4947C12.4007 18.8229 11.7166 19.0592 11 19.1891V20.2591C11 20.8114 10.5523 21.2591 10 21.2591C9.44772 21.2591 9 20.8114 9 20.2591V19.1891C8.28342 19.0592 7.59929 18.8229 6.96329 18.4947L6.20711 19.2509C5.81658 19.6414 5.18342 19.6414 4.79289 19.2509L3.37868 17.8367C2.98816 17.4462 2.98816 16.813 3.37868 16.4225L4.13486 15.6663C3.8067 15.0303 3.57032 14.3461 3.44046 13.6295H2.37047C1.81819 13.6295 1.37047 13.1818 1.37047 12.6295V10.6295C1.37047 10.0773 1.81819 9.62955 2.37047 9.62955H3.44046C3.57032 8.91297 3.8067 8.22884 4.13486 7.59284L3.37868 6.83663C2.98816 6.4461 2.98816 5.81294 3.37868 5.42241L4.79289 4.0082C5.18342 3.61768 5.81658 3.61768 6.20711 4.0082L6.96329 4.76441C7.59929 4.43625 8.28342 4.19986 9 4.07001V3C9 2.44772 9.44772 2 10 2Z" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
          <circle cx="10" cy="11.6295" r="2.5" stroke="currentColor" stroke-width="1.5"/>
        </svg>
        <span>设置</span>
      </router-link>
      <router-link to="/help" class="nav-link" @click="handleNavClick">
        <svg width="20" height="20" viewBox="0 0 20 20" fill="none" xmlns="http://www.w3.org/2000/svg">
          <circle cx="10" cy="10" r="8" stroke="currentColor" stroke-width="1.5"/>
          <path d="M7.5 7.5C7.7 6.8 8.2 6.2 8.8 5.9C11.3 5.9 13.3 8 13.3 10.5C13.3 13 11.3 15.1 8.8 15.1C8.4 15.1 8.1 15 7.7 14.9M7.5 7.5C7.3 8.1 7.2 8.8 7.2 9.6" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
        </svg>
        <span>操作说明</span>
      </router-link>
    </nav>
    <div class="sidebar-footer">
      <div class="version">v1.0.0</div>
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  isOpen?: boolean;
}>();

const emit = defineEmits<{
  close: [];
}>();

const handleNavClick = () => {
  // 仅在移动端关闭侧边栏
  if (window.innerWidth <= 768) {
    emit('close');
  }
};
</script>

<style scoped>
.sidebar {
  width: 260px;
  min-width: 200px;
  background: linear-gradient(180deg, #ffffff 0%, #f8fafc 100%);
  color: #475569;
  display: flex;
  flex-direction: column;
  border-right: 1px solid var(--border-color);
  box-shadow: var(--shadow-md);
  transition: transform var(--transition-base);
}

.sidebar-header {
  padding: 24px 20px;
  background: transparent;
  border-bottom: 1px solid var(--border-color);
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.logo {
  display: flex;
  align-items: center;
  gap: 12px;
}

.logo h2 {
  margin: 0;
  font-size: 18px;
  font-weight: 700;
  background: linear-gradient(135deg, var(--primary-color) 0%, var(--secondary-color) 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.close-btn {
  display: none;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border: none;
  background: var(--bg-tertiary);
  border-radius: var(--radius-sm);
  cursor: pointer;
  color: var(--text-secondary);
  transition: all var(--transition-base);
  flex-shrink: 0;
}

.close-btn:hover {
  background: var(--border-color);
  color: var(--text-primary);
}

.sidebar-nav {
  flex: 1;
  padding: 20px 12px;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.nav-link {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 14px 16px;
  color: var(--text-secondary);
  text-decoration: none;
  border-radius: var(--radius-md);
  transition: all var(--transition-base);
  font-weight: 500;
  position: relative;
  overflow: hidden;
  white-space: nowrap;
}

.nav-link::before {
  content: '';
  position: absolute;
  inset: 0;
  background: linear-gradient(135deg, rgba(59, 130, 246, 0.1) 0%, rgba(6, 182, 212, 0.1) 100%);
  opacity: 0;
  transition: opacity var(--transition-base);
}

.nav-link:hover {
  color: var(--primary-color);
  background: var(--bg-tertiary);
  transform: translateX(4px);
}

.nav-link.router-link-active {
  color: #ffffff;
  background: linear-gradient(135deg, var(--primary-color) 0%, var(--secondary-color) 100%);
  box-shadow: 0 4px 12px rgba(59, 130, 246, 0.3);
}

.nav-link.router-link-active::before {
  opacity: 1;
}

.nav-link svg {
  flex-shrink: 0;
}

.sidebar-footer {
  padding: 16px;
  border-top: 1px solid var(--border-color);
}

.version {
  text-align: center;
  font-size: 12px;
  color: var(--text-tertiary);
  font-weight: 500;
}

/* 响应式布局 */
@media (max-width: 768px) {
  .sidebar {
    position: fixed;
    top: 0;
    left: 0;
    bottom: 0;
    width: 280px;
    max-width: 80vw;
    z-index: 101;
    transform: translateX(-100%);
  }

  .sidebar.is-open {
    transform: translateX(0);
  }

  .sidebar-header {
    padding: 16px;
  }

  .logo h2 {
    font-size: 16px;
  }

  .close-btn {
    display: flex;
  }

  .nav-link {
    padding: 12px 16px;
    font-size: 14px;
  }
}

@media (min-width: 769px) {
  .sidebar {
    position: relative;
    transform: none !important;
  }

  .close-btn {
    display: none !important;
  }
}
</style>
