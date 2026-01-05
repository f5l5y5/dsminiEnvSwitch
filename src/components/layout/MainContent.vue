<template>
  <div class="main-content">
    <button class="menu-toggle" @click="$emit('toggle-sidebar')" aria-label="Toggle menu">
      <svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path d="M3 12H21M3 6H21M3 18H21" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
      </svg>
    </button>
    <router-view />
  </div>
</template>

<script setup lang="ts">
defineEmits<{
  'toggle-sidebar': [];
}>();
</script>

<style scoped>
.main-content {
  flex: 1;
  overflow-y: auto;
  background: var(--bg-primary);
  position: relative;
}

.main-content::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: radial-gradient(circle at top right, rgba(99, 102, 241, 0.05) 0%, transparent 40%),
              radial-gradient(circle at bottom left, rgba(139, 92, 246, 0.05) 0%, transparent 40%);
  pointer-events: none;
}

.menu-toggle {
  position: fixed;
  top: 16px;
  left: 16px;
  z-index: 50;
  display: none;
  align-items: center;
  justify-content: center;
  width: 44px;
  height: 44px;
  border: none;
  background: var(--bg-secondary);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-lg);
  cursor: pointer;
  color: var(--text-primary);
  transition: all var(--transition-base);
}

.menu-toggle:hover {
  background: var(--bg-tertiary);
  box-shadow: var(--shadow-xl);
  transform: scale(1.05);
}

.menu-toggle:active {
  transform: scale(0.95);
}

@media (max-width: 768px) {
  .menu-toggle {
    display: flex;
  }
}
</style>
