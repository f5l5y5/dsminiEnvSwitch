<template>
  <div class="environments-view">
    <div class="left-panel">
      <div class="header">
        <div class="header-content">
          <h1>环境列表</h1>
          <p class="subtitle">管理您的配置环境</p>
        </div>
        <button @click="showAddDialog = true" class="btn-primary">
          <svg width="18" height="18" viewBox="0 0 18 18" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M9 3.75V14.25M3.75 9H14.25" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
          </svg>
          添加环境
        </button>
      </div>

      <div class="content">
        <EnvironmentList
          :environments="environments"
          :current-id="currentEnvironmentId"
          :applied-id="currentAppliedEnvironmentId"
          @select="handleSelect"
          @delete="handleDelete"
          @copy="handleCopy"
          @edit="handleEdit"
          @apply="handleApply"
        />
      </div>
    </div>

    <div class="right-panel">
      <EnvironmentDetail
        v-if="currentEnvironment"
        :environment="currentEnvironment"
      />
      <div v-else class="empty-detail">
        <svg width="120" height="120" viewBox="0 0 120 120" fill="none" xmlns="http://www.w3.org/2000/svg">
          <circle cx="60" cy="60" r="50" fill="url(#empty-gradient)" opacity="0.1"/>
          <path d="M60 35V85M35 60H85" stroke="url(#empty-gradient)" stroke-width="4" stroke-linecap="round"/>
          <defs>
            <linearGradient id="empty-gradient" x1="35" y1="35" x2="85" y2="85" gradientUnits="userSpaceOnUse">
              <stop offset="0%" stop-color="#667eea"/>
              <stop offset="100%" stop-color="#764ba2"/>
            </linearGradient>
          </defs>
        </svg>
        <h3>选择一个环境</h3>
        <p>从左侧列表中选择一个环境查看详情</p>
      </div>
    </div>

    <EnvironmentDialog
      v-if="showAddDialog"
      @close="showAddDialog = false"
      @save="handleSave"
    />

    <EnvironmentDialog
      v-if="showEditDialog && editingEnvironment"
      :environment="editingEnvironment"
      @close="showEditDialog = false"
      @save="handleEditSave"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';
import { useConfigStore } from '../stores/config';
import { showSuccessNotification, showErrorNotification } from '../utils/notification';
import EnvironmentList from '../components/environment/EnvironmentList.vue';
import EnvironmentDetail from '../components/environment/EnvironmentDetail.vue';
import EnvironmentDialog from '../components/environment/EnvironmentDialog.vue';
import type { Environment } from '../types/config';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
import { confirm } from '@tauri-apps/plugin-dialog';

const configStore = useConfigStore();
const showAddDialog = ref(false);
const showEditDialog = ref(false);
const editingEnvironment = ref<Environment | null>(null);

const environments = computed(() => configStore.environments);
const currentEnvironmentId = computed(() => configStore.currentEnvironmentId);
const currentAppliedEnvironmentId = computed(() => configStore.currentAppliedEnvironmentId);
const currentEnvironment = computed(() => configStore.currentEnvironment);

let unlistenTraySwitch: (() => void) | null = null;

onMounted(async () => {
  await configStore.loadConfig();

  // 监听托盘切换环境事件
  unlistenTraySwitch = await listen<string>('tray-switch-environment', async (event) => {
    try {
      await invoke('apply_environment', { envId: event.payload });
      await configStore.loadConfig();
    } catch (error) {
      await showErrorNotification(`应用失败: ${error}`);
    }
  });
});

onUnmounted(() => {
  if (unlistenTraySwitch) {
    unlistenTraySwitch();
  }
});

const handleSelect = (id: string) => {
  configStore.setCurrentEnvironment(id);
};

const handleDelete = async (id: string) => {
  // 先确认，再删除
  let confirmed = false;

  try {
    // 使用 Tauri 的原生对话框 API
    confirmed = await confirm('确定要删除这个环境吗？此操作不可恢复。', {
      title: '确认删除',
      kind: 'warning'
    });
  } catch {
    // 如果 Tauri 对话框不可用，回退到浏览器 confirm
    confirmed = window.confirm('确定要删除这个环境吗？此操作不可恢复。');
  }

  // 只有确认后才删除
  if (!confirmed) {
    return;
  }

  try {
    await configStore.deleteEnvironment(id);
    await showSuccessNotification('环境已删除');
  } catch (error) {
    await showErrorNotification(`删除失败: ${error}`);
  }
};

const handleCopy = async (id: string) => {
  const env = environments.value.find(e => e.id === id);
  if (!env) return;

  try {
    // 创建副本，名称加上 " - 副本"
    const copyEnv: any = {
      name: `${env.name} - 副本`,
      desc: env.desc,
      ext: env.ext,
      targetFilePath: env.targetFilePath,
    };
    await configStore.addEnvironment(copyEnv);
    await showSuccessNotification('环境已复制');
  } catch (error) {
    await showErrorNotification(`复制失败: ${error}`);
  }
};

const handleSave = async (env: any) => {
  try {
    await configStore.addEnvironment(env);
    showAddDialog.value = false;
    await showSuccessNotification('环境已添加');
  } catch (error) {
    await showErrorNotification(`添加失败: ${error}`);
  }
};

const handleEdit = (id: string) => {
  const env = environments.value.find(e => e.id === id);
  if (env) {
    editingEnvironment.value = env;
    showEditDialog.value = true;
  }
};

const handleEditSave = async (env: any) => {
  if (!editingEnvironment.value) return;

  try {
    await configStore.updateEnvironment(editingEnvironment.value.id, env);
    showEditDialog.value = false;
    editingEnvironment.value = null;
    await showSuccessNotification('环境已更新');
  } catch (error) {
    await showErrorNotification(`更新失败: ${error}`);
  }
};

const handleApply = async (envId: string) => {
  try {
    const result = await configStore.applyEnvironment(envId);
    await showSuccessNotification(result);
  } catch (error) {
    await showErrorNotification(`应用失败: ${error}`);
  }
};
</script>

<style scoped>
.environments-view {
  height: 100%;
  display: flex;
  gap: 0;
}

.left-panel {
  width: 420px;
  min-width: 300px;
  max-width: 50vw;
  border-right: 1px solid var(--border-color);
  display: flex;
  flex-direction: column;
  background: var(--bg-secondary);
  position: relative;
  z-index: 1;
}

.right-panel {
  flex: 1;
  min-width: 0;
  background: transparent;
  overflow-y: auto;
  position: relative;
}

.header {
  padding: 28px 24px;
  border-bottom: 1px solid var(--border-color);
  display: flex;
  justify-content: space-between;
  align-items: center;
  background: linear-gradient(135deg, rgba(99, 102, 241, 0.03) 0%, rgba(139, 92, 246, 0.03) 100%);
  gap: 16px;
}

.header-content h1 {
  margin: 0;
  font-size: 22px;
  font-weight: 700;
  color: var(--text-primary);
  letter-spacing: -0.5px;
}

.subtitle {
  margin: 4px 0 0 0;
  font-size: 13px;
  color: var(--text-secondary);
}

.btn-primary {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 11px 18px;
  background: linear-gradient(135deg, var(--primary-color) 0%, var(--secondary-color) 100%);
  color: white;
  border: none;
  border-radius: var(--radius-md);
  cursor: pointer;
  font-size: 14px;
  font-weight: 600;
  transition: all var(--transition-base);
  box-shadow: 0 2px 8px rgba(99, 102, 241, 0.3);
  white-space: nowrap;
  flex-shrink: 0;
}

.btn-primary:hover {
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(99, 102, 241, 0.4);
}

.btn-primary:active {
  transform: translateY(0);
}

.content {
  flex: 1;
  overflow-y: auto;
  padding: 20px;
}

.empty-detail {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  padding: 40px;
  text-align: center;
}

.empty-detail h3 {
  margin: 24px 0 8px 0;
  font-size: 20px;
  font-weight: 600;
  color: var(--text-primary);
}

.empty-detail p {
  margin: 0;
  font-size: 14px;
  color: var(--text-secondary);
}

/* 响应式布局 */
@media (max-width: 900px) {
  .left-panel {
    width: 350px;
    min-width: 280px;
  }

  .header {
    flex-direction: column;
    align-items: flex-start;
    padding: 20px;
  }

  .btn-primary {
    width: 100%;
    justify-content: center;
  }
}

@media (max-width: 768px) {
  .environments-view {
    flex-direction: column;
  }

  .left-panel {
    width: 100%;
    max-width: 100%;
    border-right: none;
    border-bottom: 1px solid var(--border-color);
    max-height: 50vh;
  }

  .right-panel {
    height: 50vh;
  }
}

@media (max-width: 480px) {
  .header {
    padding: 16px;
  }

  .header-content h1 {
    font-size: 18px;
  }

  .subtitle {
    font-size: 12px;
  }

  .content {
    padding: 16px;
  }

  .empty-detail {
    padding: 20px;
  }

  .empty-detail svg {
    width: 80px;
    height: 80px;
  }

  .empty-detail h3 {
    font-size: 16px;
  }
}
</style>
