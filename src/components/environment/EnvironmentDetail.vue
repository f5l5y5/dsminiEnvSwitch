<template>
  <div class="environment-detail">
    <div class="detail-header">
      <div class="env-info">
        <h2>{{ environment.name }}</h2>
        <p v-if="environment.desc" class="description">
          {{ environment.desc }}
        </p>
      </div>
    </div>

    <div class="config-section">
      <div class="section-header">
        <h3>环境配置</h3>
      </div>

      <div class="info-item">
        <span class="info-label">目标文件路径</span>
        <span class="info-value" :class="{ 'info-value-global': isUsingGlobalPath }">
          {{ displayTargetFilePath }}
          <svg v-if="isUsingGlobalPath" width="14" height="14" viewBox="0 0 14 14" fill="none" xmlns="http://www.w3.org/2000/svg" class="global-icon">
            <path d="M7 2L12 7M7 2L2 7M7 2V12" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        </span>
      </div>

      <div class="config-display">
        <pre class="json-code">{{ formattedConfig }}</pre>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import type { Environment } from '../../types/config';
import { useConfigStore } from '../../stores/config';

const configStore = useConfigStore();

const props = defineProps<{
  environment: Environment;
}>();

// 格式化显示 JSON
const formattedConfig = computed(() => {
  return JSON.stringify(props.environment.ext, null, 2);
});

// 显示的目标文件路径
const displayTargetFilePath = computed(() => {
  if (props.environment.targetFilePath) {
    return props.environment.targetFilePath;
  }
  if (configStore.settings?.targetFilePath) {
    return `${configStore.settings.targetFilePath} (全局默认)`;
  }
  return '未设置';
});

// 是否使用全局路径
const isUsingGlobalPath = computed(() => {
  return !props.environment.targetFilePath && !!configStore.settings?.targetFilePath;
});
</script>

<style scoped>
.environment-detail {
  height: 100%;
  display: flex;
  flex-direction: column;
  background: var(--bg-secondary);
  margin: 24px;
  border-radius: var(--radius-xl);
  overflow: hidden;
  box-shadow: var(--shadow-xl);
}

.detail-header {
  padding: 36px;
  background: linear-gradient(135deg, var(--primary-color) 0%, var(--secondary-color) 100%);
  color: white;
  display: flex;
  justify-content: space-between;
  align-items: start;
  position: relative;
  overflow: hidden;
}

.detail-header::before {
  content: '';
  position: absolute;
  top: -50%;
  right: -10%;
  width: 400px;
  height: 400px;
  background: radial-gradient(circle, rgba(255, 255, 255, 0.1) 0%, transparent 70%);
  pointer-events: none;
}

.env-info {
  position: relative;
  z-index: 1;
}

.env-info h2 {
  margin: 0 0 10px 0;
  font-size: 28px;
  font-weight: 700;
  letter-spacing: -0.5px;
}

.description {
  margin: 0;
  opacity: 0.95;
  font-size: 15px;
  line-height: 1.5;
}

.header-actions {
  position: relative;
  z-index: 1;
  display: flex;
  gap: 12px;
}

.btn-edit {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 13px 26px;
  background-color: rgba(255, 255, 255, 0.2);
  color: white;
  border: 1px solid rgba(255, 255, 255, 0.3);
  border-radius: var(--radius-md);
  cursor: pointer;
  font-size: 15px;
  font-weight: 600;
  transition: all var(--transition-base);
}

.btn-edit:hover {
  background-color: rgba(255, 255, 255, 0.3);
}

.btn-apply {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 13px 26px;
  background-color: white;
  color: var(--primary-color);
  border: none;
  border-radius: var(--radius-md);
  cursor: pointer;
  font-size: 15px;
  font-weight: 700;
  transition: all var(--transition-base);
  box-shadow: 0 4px 14px rgba(0, 0, 0, 0.2);
}

.btn-apply:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 20px rgba(0, 0, 0, 0.25);
}

.btn-apply:active {
  transform: translateY(0);
}

.config-section {
  flex: 1;
  overflow-y: auto;
  padding: 24px 36px;
}

.section-header {
  margin-bottom: 20px;
}

.section-header h3 {
  margin: 0;
  font-size: 18px;
  color: var(--text-primary);
  font-weight: 700;
  display: flex;
  align-items: center;
  gap: 8px;
}

.section-header h3::before {
  content: '';
  display: inline-block;
  width: 4px;
  height: 18px;
  background: linear-gradient(180deg, var(--primary-color) 0%, var(--secondary-color) 100%);
  border-radius: 2px;
}

.info-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 14px 18px;
  background: var(--bg-tertiary);
  border-radius: var(--radius-md);
  margin-bottom: 20px;
  border: 1px solid var(--border-color);
}

.info-label {
  font-size: 13px;
  font-weight: 600;
  color: var(--text-secondary);
}

.info-value {
  font-size: 13px;
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
  color: var(--text-primary);
  display: flex;
  align-items: center;
  gap: 6px;
}

.info-value-global {
  color: var(--primary-color);
}

.global-icon {
  flex-shrink: 0;
}

.form-hint {
  display: block;
  margin-top: 6px;
  font-size: 12px;
  color: var(--text-tertiary);
  font-style: italic;
}

.config-display {
  background: var(--bg-tertiary);
  border-radius: var(--radius-md);
  padding: 20px;
  overflow-x: auto;
}

.json-code {
  margin: 0;
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
  font-size: 13px;
  line-height: 1.6;
  color: var(--text-primary);
}

/* Dialog styles */
.dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.6);
  backdrop-filter: blur(4px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.dialog {
  background: var(--bg-secondary);
  border-radius: var(--radius-xl);
  width: 90%;
  max-width: 700px;
  max-height: 90vh;
  display: flex;
  flex-direction: column;
  box-shadow: var(--shadow-xl);
  overflow: hidden;
}

.dialog-header {
  padding: 24px;
  border-bottom: 1px solid var(--border-color);
  display: flex;
  justify-content: space-between;
  align-items: center;
  background: linear-gradient(135deg, var(--primary-light) 0%, rgba(139, 92, 246, 0.05) 100%);
}

.dialog-header h2 {
  margin: 0;
  font-size: 20px;
  font-weight: 700;
  color: var(--text-primary);
}

.btn-close {
  display: flex;
  align-items: center;
  justify-content: center;
  background: transparent;
  border: none;
  color: var(--text-tertiary);
  cursor: pointer;
  padding: 8px;
  width: 40px;
  height: 40px;
  border-radius: var(--radius-md);
  transition: all var(--transition-fast);
  font-size: 24px;
  font-weight: 300;
  line-height: 1;
}

.btn-close:hover {
  color: var(--danger-color);
  background: rgba(239, 68, 68, 0.1);
}

.dialog-body {
  padding: 24px;
  overflow-y: auto;
  flex: 1;
}

.form-group {
  margin-bottom: 20px;
}

.form-group:last-child {
  margin-bottom: 0;
}

.form-group label {
  display: block;
  margin-bottom: 8px;
  font-weight: 600;
  color: var(--text-primary);
  font-size: 14px;
}

.form-input,
.form-textarea {
  width: 100%;
  padding: 11px 14px;
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  font-size: 14px;
  font-family: inherit;
  transition: all var(--transition-base);
  box-sizing: border-box;
  background: var(--bg-secondary);
  color: var(--text-primary);
}

.form-input:hover,
.form-textarea:hover {
  border-color: var(--border-hover);
}

.form-input:focus,
.form-textarea:focus {
  outline: none;
  border-color: var(--primary-color);
  box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.1);
}

.form-textarea {
  resize: vertical;
  min-height: 80px;
}

.code-editor {
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
  font-size: 13px;
  line-height: 1.6;
  background: var(--bg-tertiary);
}

.error-message {
  margin-top: 6px;
  font-size: 12px;
  color: #dc2626;
  display: flex;
  align-items: center;
  gap: 4px;
}

.error-message::before {
  content: '⚠';
}

.dialog-footer {
  padding: 20px 24px;
  border-top: 1px solid var(--border-color);
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  background: var(--bg-tertiary);
}

.btn-primary,
.btn-secondary {
  padding: 11px 22px;
  border: none;
  border-radius: var(--radius-md);
  cursor: pointer;
  font-size: 14px;
  font-weight: 600;
  transition: all var(--transition-base);
}

.btn-primary {
  background: linear-gradient(135deg, var(--primary-color) 0%, var(--secondary-color) 100%);
  color: white;
  box-shadow: 0 2px 8px rgba(99, 102, 241, 0.3);
}

.btn-primary:hover:not(:disabled) {
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(99, 102, 241, 0.4);
}

.btn-primary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
  box-shadow: none;
}

.btn-secondary {
  background: var(--bg-secondary);
  color: var(--text-primary);
  border: 1px solid var(--border-color);
}

.btn-secondary:hover {
  background: var(--bg-tertiary);
  border-color: var(--border-hover);
}
</style>
