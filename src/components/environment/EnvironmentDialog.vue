<template>
  <div class="dialog-overlay">
    <div class="dialog">
      <div class="dialog-header">
        <h2>{{ isEditMode ? '编辑环境' : '添加环境' }}</h2>
        <button @click="$emit('close')" class="btn-close">×</button>
      </div>
      <div class="dialog-body">
        <div class="form-group">
          <label>环境名称*</label>
          <input
            v-model="form.name"
            type="text"
            placeholder="例如: 开发环境"
            class="form-input"
            required
          />
        </div>
        <div class="form-group">
          <label>描述</label>
          <textarea
            v-model="form.desc"
            placeholder="环境描述..."
            class="form-textarea"
            rows="2"
          ></textarea>
        </div>
        <div class="form-group">
          <label>目标文件路径（可选）</label>
          <div class="path-input-group">
            <input
              v-model="form.targetFilePath"
              type="text"
              placeholder="选择或输入要替换的文件路径..."
              class="form-input"
            />
            <button type="button" class="btn-browse-small" @click="selectTargetFilePath">
              <svg width="14" height="14" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M13.5 8H10V4.5M10 8V3.5C10 3.22386 9.77614 3 9.5 3H4.5C4.22386 3 4 3.22386 4 3.5V12.5C4 12.7761 4.22386 13 4.5 13H9.5C9.77614 13 10 12.7761 10 12.5V8H13.5C13.7761 8 14 7.77614 14 7.5V4.5C14 3.67157 13.3284 3 12.5 3H10.5C10.2239 3 10 3.22386 10 3.5V8ZM13 7V4.5C13 4.22386 12.7761 4 12.5 4H11V7H13Z" fill="currentColor"/>
              </svg>
              浏览
            </button>
          </div>
          <small class="form-hint">留空时使用全局设置的默认路径</small>
        </div>
        <div class="form-group">
          <label>环境配置 (JSON)*</label>
          <textarea
            v-model="form.ext"
            placeholder='{ "key": "value" }'
            class="form-textarea code-editor"
            rows="10"
          ></textarea>
          <div v-if="jsonError" class="error-message">{{ jsonError }}</div>
        </div>
      </div>
      <div class="dialog-footer">
        <button @click="$emit('close')" class="btn-secondary">取消</button>
        <button @click="handleSave" class="btn-primary" :disabled="!form.name || !!jsonError">
          保存
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted, computed } from 'vue';
import { open } from '@tauri-apps/plugin-dialog';
import { useConfigStore } from '../../stores/config';
import type { Environment } from '../../types/config';

const configStore = useConfigStore();

const props = defineProps<{
  environment?: Environment;
}>();

const emit = defineEmits<{
  close: [];
  save: [data: { name: string; desc?: string; ext: any; targetFilePath?: string }];
}>();

const isEditMode = computed(() => !!props.environment);

const form = ref({
  name: '',
  desc: '',
  targetFilePath: '',
  ext: '',
});

// 初始化表单
onMounted(() => {
  if (props.environment) {
    // 编辑模式
    form.value = {
      name: props.environment.name,
      desc: props.environment.desc || '',
      targetFilePath: props.environment.targetFilePath || '',
      ext: JSON.stringify(props.environment.ext, null, 2),
    };
  } else {
    // 添加模式 - 使用全局的 targetFilePath
    if (configStore.settings?.targetFilePath) {
      form.value.targetFilePath = configStore.settings.targetFilePath;
    }
  }
});

const jsonError = ref('');

// 验证 JSON 格式
watch(() => form.value.ext, (newVal) => {
  if (!newVal.trim()) {
    jsonError.value = '';
    return;
  }
  try {
    JSON.parse(newVal);
    jsonError.value = '';
  } catch (e) {
    jsonError.value = 'JSON 格式错误: ' + (e as Error).message;
  }
});

const handleSave = () => {
  if (!form.value.name) return;

  try {
    // 如果为空，保存为空对象，否则解析 JSON
    const extData = form.value.ext.trim() ? JSON.parse(form.value.ext) : {};
    emit('save', {
      name: form.value.name,
      desc: form.value.desc || undefined,
      ext: extData,
      targetFilePath: form.value.targetFilePath || undefined,
    });
  } catch (e) {
    jsonError.value = 'JSON 格式错误: ' + (e as Error).message;
  }
};

const selectTargetFilePath = async () => {
  try {
    const filePath = await open({
      multiple: false,
    });

    if (filePath) {
      form.value.targetFilePath = filePath as string;
    }
  } catch (error) {
    console.error('选择文件失败：', error);
  }
};
</script>

<style scoped>
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
  animation: fadeIn var(--transition-base);
}

@keyframes fadeIn {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}

.dialog {
  background: var(--bg-secondary);
  border-radius: var(--radius-xl);
  width: 90%;
  max-width: 500px;
  box-shadow: var(--shadow-xl);
  animation: slideUp var(--transition-base);
  overflow: hidden;
}

@keyframes slideUp {
  from {
    opacity: 0;
    transform: translateY(20px) scale(0.95);
  }
  to {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
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
  letter-spacing: -0.3px;
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

.btn-primary:active:not(:disabled) {
  transform: translateY(0);
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

.path-input-group {
  display: flex;
  align-items: center;
  gap: 8px;
}

.path-input-group .form-input {
  flex: 1;
}

.btn-browse-small {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 10px 14px;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  cursor: pointer;
  font-size: 13px;
  font-weight: 600;
  color: var(--text-primary);
  transition: all var(--transition-base);
  white-space: nowrap;
}

.btn-browse-small:hover {
  background: var(--bg-primary);
  border-color: var(--primary-color);
}

.form-hint {
  display: block;
  margin-top: 6px;
  font-size: 12px;
  color: var(--text-tertiary);
  font-style: italic;
}
</style>
