<template>
  <div class="layout">
    <Sidebar :is-open="isSidebarOpen" @close="isSidebarOpen = false" />
    <MainContent :is-sidebar-open="isSidebarOpen" @toggle-sidebar="isSidebarOpen = !isSidebarOpen" />

    <!-- 移动端遮罩层 -->
    <div
      v-if="isSidebarOpen"
      class="sidebar-overlay"
      @click="isSidebarOpen = false"
    ></div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import Sidebar from './Sidebar.vue';
import MainContent from './MainContent.vue';

const isSidebarOpen = ref(false);
</script>

<style scoped>
.layout {
  display: flex;
  height: 100vh;
  overflow: hidden;
  position: relative;
}

.sidebar-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  z-index: 100;
  animation: fadeIn 0.2s ease;
}

@keyframes fadeIn {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}

@media (min-width: 769px) {
  .sidebar-overlay {
    display: none;
  }
}
</style>
