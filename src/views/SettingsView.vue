<template>
  <div class="settings-view">
    <h1>应用设置</h1>
    <div class="settings-content">
      <div class="setting-item">
        <div class="setting-label">
          <h3>关闭行为</h3>
          <p>选择点击关闭按钮时的行为</p>
        </div>
        <div class="setting-control">
          <label class="checkbox-label">
            <input type="checkbox" v-model="settings.closeToTray" @change="saveSettings" />
            最小化到系统托盘
          </label>
        </div>
      </div>

      <div class="setting-item">
        <div class="setting-label">
          <h3>自动备份</h3>
          <p>应用配置前自动备份原文件</p>
        </div>
        <div class="setting-control">
          <label class="checkbox-label">
            <input type="checkbox" v-model="settings.autoBackup" @change="saveSettings" />
            启用自动备份
          </label>
        </div>
      </div>

      <div class="setting-item">
        <div class="setting-label">
          <h3>通知</h3>
          <p>显示操作成功或失败的系统通知</p>
        </div>
        <div class="setting-control">
          <label class="checkbox-label">
            <input type="checkbox" v-model="settings.showNotifications" @change="saveSettings" />
            显示通知
          </label>
        </div>
      </div>

      <div class="setting-item">
        <div class="setting-label">
          <h3>目标文件路径</h3>
          <p>设置全局默认的替换文件内容路径（环境未单独设置时使用此值）</p>
        </div>
        <div class="setting-control full-width">
          <div class="path-input-group">
            <input
              v-model="targetFilePathInput"
              type="text"
              placeholder="选择或输入目标文件路径..."
              class="path-input"
              @blur="updateTargetFilePath"
            />
            <button class="btn-browse" @click="selectTargetFilePath">
              <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M13.5 8H10V4.5M10 8V3.5C10 3.22386 9.77614 3 9.5 3H4.5C4.22386 3 4 3.22386 4 3.5V12.5C4 12.7761 4.22386 13 4.5 13H9.5C9.77614 13 10 12.7761 10 12.5V8H13.5C13.7761 8 14 7.77614 14 7.5V4.5C14 3.67157 13.3284 3 12.5 3H10.5C10.2239 3 10 3.22386 10 3.5V8ZM13 7V4.5C13 4.22386 12.7761 4 12.5 4H11V7H13Z" fill="currentColor"/>
              </svg>
              浏览
            </button>
            <button v-if="settings.targetFilePath" class="btn-clear-small" @click="clearTargetFilePath">
              <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M12.5 3.5L3.5 12.5M3.5 3.5L12.5 12.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
            </button>
          </div>
          <div v-if="settings.targetFilePath" class="config-file-path">
            <svg width="14" height="14" viewBox="0 0 14 14" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M7 2L12 7M7 2L2 7M7 2V12" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
            {{ settings.targetFilePath }}
          </div>
        </div>
      </div>

      <div class="setting-item">
        <div class="setting-label">
          <h3>远程配置同步</h3>
          <p>从远程URL同步environments配置，不影响其他设置</p>
        </div>
        <div class="setting-control full-width">
          <div class="path-input-group">
            <input
              v-model="remoteConfigUrlInput"
              type="text"
              placeholder="https://example.com/config.json"
              class="path-input"
            />
            <button class="btn-browse" @click="syncRemoteConfig">
              <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M8 1.5V11.5M8 11.5L11 8.5M8 11.5L5 8.5M13.5 11.5V13C13.5 13.8284 12.8284 14.5 12 14.5H4C3.17157 14.5 2.5 13.8284 2.5 13V11.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
              同步
            </button>
            <button v-if="settings.remoteConfigUrl" class="btn-clear-small" @click="clearRemoteConfigUrl">
              <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M12.5 3.5L3.5 12.5M3.5 3.5L12.5 12.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
            </button>
          </div>
          <div v-if="settings.remoteConfigUrl" class="config-file-path">
            <svg width="14" height="14" viewBox="0 0 14 14" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M7 2L12 7M7 2L2 7M7 2V12" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
            {{ settings.remoteConfigUrl }}
          </div>
        </div>
      </div>

      <div class="setting-item">
        <div class="setting-label">
          <h3>导出/导入配置</h3>
          <p>手动备份或恢复配置文件，导入后将自动同步</p>
        </div>
        <div class="setting-control full-width">
          <div class="config-file-actions">
            <button class="btn-secondary" @click="exportConfig">
              <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M13.5 8H10V4.5M10 8V3.5C10 3.22386 9.77614 3 9.5 3H4.5C4.22386 3 4 3.22386 4 3.5V12.5C4 12.7761 4.22386 13 4.5 13H9.5C9.77614 13 10 12.7761 10 12.5V8H13.5C13.7761 8 14 7.77614 14 7.5V4.5C14 3.67157 13.3284 3 12.5 3H10.5C10.2239 3 10 3.22386 10 3.5V8ZM13 7V4.5C13 4.22386 12.7761 4 12.5 4H11V7H13Z" fill="currentColor"/>
              </svg>
              导出配置
            </button>
            <button class="btn-primary" @click="importConfig">
              <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M9.5 3H4.5C4.22386 3 4 3.22386 4 3.5V12.5C4 12.7761 4.22386 13 4.5 13H11.5C11.7761 13 12 12.7761 12 12.5V9M9.5 3V7.5M9.5 3H12.5L14 4.5M7.5 8.5L6 10L7.5 11.5M6 10H10" stroke="currentColor" stroke-width="1" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
              导入配置
            </button>
            <button v-if="settings.externalConfigPath" class="btn-danger" @click="clearConfigPath">
              <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M5 3H11M12 5V13C12 13.5523 11.5523 14 11 14H5C4.44772 14 4 13.5523 4 13V5M6 5V4M10 5V4" stroke="currentColor" stroke-width="1" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
              清除路径
            </button>
          </div>
          <div v-if="settings.externalConfigPath" class="config-file-path">
            <svg width="14" height="14" viewBox="0 0 14 14" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M7 2L12 7M7 2L2 7M7 2V12" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
            {{ settings.externalConfigPath }}
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useConfigStore } from '../stores/config';
import { invoke } from '@tauri-apps/api/core';
import { open, save } from '@tauri-apps/plugin-dialog';
import type { AppSettings, Environment } from '../types/config';

const configStore = useConfigStore();

const settings = ref<AppSettings>({
  closeToTray: true,
  autoBackup: true,
  showNotifications: true,
  externalConfigPath: undefined,
  targetFilePath: undefined,
  remoteConfigUrl: undefined,
});

const targetFilePathInput = ref('');
const remoteConfigUrlInput = ref('');

onMounted(async () => {
  await configStore.loadConfig();
  if (configStore.settings) {
    settings.value = { ...configStore.settings };
    targetFilePathInput.value = settings.value.targetFilePath || '';
    remoteConfigUrlInput.value = settings.value.remoteConfigUrl || '';
  }
});

const saveSettings = async () => {
  await configStore.updateSettings(settings.value);
};

const exportConfig = async () => {
  try {
    const filePath = await save({
      defaultPath: 'dsmini-env-config.json',
      filters: [{
        name: 'JSON',
        extensions: ['json']
      }]
    });

    if (filePath) {
      await invoke('export_config_to_json', { path: filePath });
      alert('配置导出成功！\n路径：' + filePath);
    }
  } catch (error) {
    alert('导出失败：' + error);
  }
};

const importConfig = async () => {
  try {
    const filePath = await open({
      multiple: false,
      filters: [{
        name: 'JSON',
        extensions: ['json']
      }]
    });

    if (filePath) {
      await invoke('import_config_from_json', { path: filePath as string });
      await configStore.loadConfig();
      if (configStore.settings) {
        settings.value = { ...configStore.settings };
      }
      // 导入后自动设置同步路径
      settings.value.externalConfigPath = filePath as string;
      await saveSettings();
      alert('配置导入成功！\n之后所有配置变更将自动同步到此文件');
    }
  } catch (error) {
    alert('导入失败：' + error);
  }
};

const clearConfigPath = async () => {
  if (confirm('确定要清除同步路径吗？\n清除后将不再自动同步配置。')) {
    settings.value.externalConfigPath = undefined;
    await saveSettings();
  }
};

const selectTargetFilePath = async () => {
  try {
    const filePath = await open({
      multiple: false,
    });

    if (filePath) {
      settings.value.targetFilePath = filePath as string;
      targetFilePathInput.value = filePath as string;
      await saveSettings();
    }
  } catch (error) {
    alert('选择文件失败：' + error);
  }
};

const updateTargetFilePath = async () => {
  const input = targetFilePathInput.value.trim();
  if (input && input !== settings.value.targetFilePath) {
    settings.value.targetFilePath = input;
    await saveSettings();
  } else if (!input) {
    targetFilePathInput.value = settings.value.targetFilePath || '';
  }
};

const clearTargetFilePath = async () => {
  settings.value.targetFilePath = undefined;
  targetFilePathInput.value = '';
  await saveSettings();
};

const syncRemoteConfig = async () => {
  const url = remoteConfigUrlInput.value.trim();
  if (!url) {
    alert('请输入远程配置URL');
    return;
  }

  // 验证URL格式
  if (!url.startsWith('http://') && !url.startsWith('https://')) {
    alert('URL必须以http://或https://开头');
    return;
  }

  try {
    const environments = await invoke<Environment[]>('sync_remote_environments', { url });
    // 先重新加载配置以获取最新的 environments
    await configStore.loadConfig();
    // 更新前端 settings 状态（不保存，因为后端已经保存了）
    settings.value.remoteConfigUrl = url;
    // 更新前端显示的 settings
    if (configStore.config) {
      settings.value = { ...configStore.config.settings };
      remoteConfigUrlInput.value = settings.value.remoteConfigUrl || '';
    }
    alert(`同步成功！\n已获取 ${environments.length} 个环境配置`);
  } catch (error) {
    alert('同步失败：' + error);
  }
};

const clearRemoteConfigUrl = async () => {
  settings.value.remoteConfigUrl = undefined;
  remoteConfigUrlInput.value = '';
  await saveSettings();
};
</script>

<style scoped>
.settings-view {
  padding: 24px;
  max-width: 900px;
  margin: 0 auto;
}

.settings-view h1 {
  margin: 0 0 32px 0;
  font-size: 28px;
  font-weight: 700;
  color: var(--text-primary);
  letter-spacing: -0.5px;
}

.settings-content {
  background: var(--bg-secondary);
  border-radius: var(--radius-xl);
  padding: 0;
  overflow: hidden;
  box-shadow: var(--shadow-lg);
}

.setting-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 24px;
  border-bottom: 1px solid var(--border-color);
  transition: background-color var(--transition-base);
  gap: 24px;
}

.setting-item:last-child {
  border-bottom: none;
}

.setting-item:hover {
  background: var(--bg-tertiary);
}

.setting-label {
  flex: 1;
  min-width: 0;
}

.setting-label h3 {
  margin: 0 0 6px 0;
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
  display: flex;
  align-items: center;
  gap: 8px;
}

.setting-label h3::before {
  content: '';
  display: inline-block;
  width: 4px;
  height: 16px;
  background: linear-gradient(180deg, var(--primary-color) 0%, var(--secondary-color) 100%);
  border-radius: 2px;
  flex-shrink: 0;
}

.setting-label p {
  margin: 0;
  font-size: 14px;
  color: var(--text-secondary);
  line-height: 1.5;
}

.setting-control {
  flex-shrink: 0;
}

.setting-control.full-width {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  gap: 12px;
}

.checkbox-label {
  display: flex;
  align-items: center;
  gap: 10px;
  cursor: pointer;
  font-size: 14px;
  color: var(--text-primary);
  font-weight: 500;
  padding: 8px 16px;
  border-radius: var(--radius-md);
  transition: all var(--transition-base);
  user-select: none;
  white-space: nowrap;
}

.checkbox-label:hover {
  background: var(--bg-primary);
}

.checkbox-label input[type="checkbox"] {
  width: 20px;
  height: 20px;
  cursor: pointer;
  accent-color: var(--primary-color);
  transition: transform var(--transition-fast);
  flex-shrink: 0;
}

.checkbox-label input[type="checkbox"]:hover {
  transform: scale(1.05);
}

.checkbox-label input[type="checkbox"]:checked {
  transform: scale(1);
}

.config-file-actions {
  display: flex;
  align-items: center;
  gap: 12px;
  flex-wrap: wrap;
}

.config-file-actions button {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 18px;
  border-radius: var(--radius-md);
  cursor: pointer;
  font-size: 14px;
  font-weight: 600;
  transition: all var(--transition-base);
}

.config-file-actions button.btn-secondary {
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  color: var(--text-primary);
}

.config-file-actions button.btn-secondary:hover {
  background: var(--bg-tertiary);
  border-color: var(--primary-color);
}

.config-file-actions button.btn-primary {
  background: var(--primary-color);
  border: 1px solid var(--primary-color);
  color: white;
}

.config-file-actions button.btn-primary:hover {
  background: var(--secondary-color);
  border-color: var(--secondary-color);
}

.config-file-actions button.btn-danger {
  background: #fee2e2;
  border: 1px solid #fecaca;
  color: #dc2626;
}

.config-file-actions button.btn-danger:hover {
  background: #fecaca;
  border-color: #f87171;
}

.config-file-path {
  margin-top: 12px;
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 14px;
  background: var(--bg-tertiary);
  border-radius: var(--radius-md);
  font-size: 13px;
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
  color: var(--text-primary);
  word-break: break-all;
}

.config-file-path svg {
  flex-shrink: 0;
  color: var(--primary-color);
}

.path-input-group {
  display: flex;
  align-items: center;
  gap: 8px;
  width: 100%;
  flex-wrap: wrap;
}

.path-input {
  flex: 1;
  min-width: 200px;
  padding: 10px 14px;
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  font-size: 14px;
  font-family: inherit;
  background: var(--bg-secondary);
  color: var(--text-primary);
  transition: all var(--transition-base);
}

.path-input:hover {
  border-color: var(--border-hover);
}

.path-input:focus {
  outline: none;
  border-color: var(--primary-color);
  box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.1);
}

.btn-browse {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 18px;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  cursor: pointer;
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
  transition: all var(--transition-base);
  white-space: nowrap;
}

.btn-browse:hover {
  background: var(--bg-tertiary);
  border-color: var(--primary-color);
}

.btn-clear-small {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 10px;
  background: #fee2e2;
  border: 1px solid #fecaca;
  border-radius: var(--radius-md);
  cursor: pointer;
  color: #dc2626;
  transition: all var(--transition-base);
  width: 42px;
  flex-shrink: 0;
}

.btn-clear-small:hover {
  background: #fecaca;
  border-color: #f87171;
}

/* 响应式布局 */
@media (max-width: 768px) {
  .settings-view {
    padding: 16px;
  }

  .settings-view h1 {
    font-size: 24px;
    margin-bottom: 24px;
  }

  .setting-item {
    flex-direction: column;
    align-items: flex-start;
    padding: 20px;
    gap: 16px;
  }

  .setting-control {
    width: 100%;
  }

  .setting-control.full-width {
    align-items: flex-start;
  }

  .checkbox-label {
    width: 100%;
    padding: 10px 14px;
  }

  .config-file-actions {
    flex-direction: column;
    width: 100%;
  }

  .config-file-actions button {
    width: 100%;
    justify-content: center;
  }

  .path-input-group {
    flex-direction: column;
    align-items: stretch;
  }

  .path-input {
    min-width: 0;
    width: 100%;
  }

  .btn-browse {
    width: 100%;
    justify-content: center;
  }

  .btn-clear-small {
    width: 100%;
  }
}

@media (max-width: 480px) {
  .settings-view {
    padding: 12px;
  }

  .settings-view h1 {
    font-size: 20px;
  }

  .setting-item {
    padding: 16px;
  }

  .setting-label h3 {
    font-size: 14px;
  }

  .setting-label p {
    font-size: 13px;
  }
}
</style>
