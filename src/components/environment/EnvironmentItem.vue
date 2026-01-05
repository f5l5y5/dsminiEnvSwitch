<template>
  <div class="environment-item" :class="{ 'is-current': isCurrent, 'is-applied': isApplied }" @click="$emit('select')">
    <div class="item-header">
      <div class="name-wrapper">
        <h3>{{ environment.name }}</h3>
        <span v-if="isApplied" class="applied-badge-small">已应用</span>
      </div>
      <button @click.stop="handleDeleteClick" :disabled="isDeleting" class="btn-delete" title="删除">
        <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
          <path d="M12 4L4 12M4 4L12 12" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
        </svg>
      </button>
    </div>
    <p v-if="environment.desc" class="description">
      {{ environment.desc }}
    </p>
    <div class="item-footer">
      <div class="footer-left">
        <span v-if="isCurrent" class="current-badge" :class="{ 'single': !isApplied }">
          <svg width="10" height="10" viewBox="0 0 10 10" fill="none" xmlns="http://www.w3.org/2000/svg">
            <circle cx="5" cy="5" r="4" stroke="currentColor" stroke-width="1.5"/>
            <path d="M3.5 5L4.5 6L6.5 4" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
          当前环境
        </span>
      </div>
      <div class="footer-actions">
        <button @click.stop="$emit('copy')" class="btn-action btn-copy" title="复制环境">
          <svg width="14" height="14" viewBox="0 0 14 14" fill="none" xmlns="http://www.w3.org/2000/svg">
            <rect x="3" y="3" width="7" height="7" rx="1" stroke="currentColor" stroke-width="1.5"/>
            <path d="M6 6H11C11.5523 6 12 6.44772 12 7V11C12 11.5523 11.5523 12 11 12H7C6.44772 12 6 11.5523 6 11V6Z" fill="currentColor" opacity="0.3"/>
            <path d="M9 5V4C9 3.44772 8.55228 3 8 3H4C3.44772 3 3 3.44772 3 4V8C3 8.55228 3.44772 9 4 9H5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
          </svg>
          复制
        </button>
        <button @click.stop="$emit('edit')" class="btn-action btn-edit" title="编辑配置">
          <svg width="14" height="14" viewBox="0 0 14 14" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M1 9V13H5L11.5 6.5L7.5 2.5L1 9Z" stroke="currentColor" stroke-width="1" stroke-linecap="round" stroke-linejoin="round"/>
            <path d="M10.5 1.5L12.5 3.5" stroke="currentColor" stroke-width="1" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
          编辑
        </button>
        <button @click.stop="$emit('apply')" class="btn-action btn-apply" title="应用此环境">
          <svg width="14" height="14" viewBox="0 0 14 14" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M9 5L4 10L1 7" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
          应用
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import type { Environment } from '../../types/config';

const props = defineProps<{
  environment: Environment;
  isCurrent: boolean;
  isApplied: boolean;
}>();

const emit = defineEmits<{
  select: [];
  delete: [id: string];
  copy: [];
  edit: [];
  apply: [];
}>();

const isDeleting = ref(false);

const handleDeleteClick = () => {
  if (isDeleting.value) {
    return;
  }
  isDeleting.value = true;
  emit('delete', props.environment.id);
  // 延迟重置，防止快速重复点击
  setTimeout(() => {
    isDeleting.value = false;
  }, 500);
};
</script>

<style scoped>
.environment-item {
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 12px;
  padding: 16px;
  cursor: pointer;
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  position: relative;
  overflow: hidden;
}

.environment-item::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  width: 3px;
  height: 100%;
  background: linear-gradient(180deg, var(--primary-color) 0%, var(--secondary-color) 100%);
  opacity: 0;
  transition: opacity 0.2s ease;
}

.environment-item::after {
  content: '';
  position: absolute;
  inset: 0;
  background: radial-gradient(circle at top right, rgba(99, 102, 241, 0.03), transparent 60%);
  opacity: 0;
  transition: opacity 0.2s ease;
  pointer-events: none;
}

.environment-item:hover {
  border-color: var(--primary-color);
  box-shadow: 0 4px 12px rgba(99, 102, 241, 0.15), 0 2px 4px rgba(0, 0, 0, 0.05);
  transform: translateY(-1px);
}

.environment-item:hover::before {
  opacity: 1;
}

.environment-item:hover::after {
  opacity: 1;
}

.environment-item.is-current {
  border-color: var(--primary-color);
  background: linear-gradient(135deg, rgba(99, 102, 241, 0.06) 0%, rgba(139, 92, 246, 0.04) 100%);
  box-shadow: 0 4px 16px rgba(99, 102, 241, 0.2), 0 2px 6px rgba(0, 0, 0, 0.08);
}

.environment-item.is-current::before {
  opacity: 1;
  background: linear-gradient(180deg, #6366f1 0%, #8b5cf6 100%);
}

.environment-item.is-applied {
  border-color: #10b981;
  background: linear-gradient(135deg, rgba(16, 185, 129, 0.06) 0%, rgba(5, 150, 105, 0.03) 100%);
  box-shadow: 0 4px 16px rgba(16, 185, 129, 0.15), 0 2px 6px rgba(0, 0, 0, 0.06);
}

.environment-item.is-applied::before {
  opacity: 1;
  background: linear-gradient(180deg, #10b981 0%, #059669 100%);
}

.item-header {
  display: flex;
  align-items: flex-start;
  gap: 10px;
  margin-bottom: 10px;
}

.name-wrapper {
  flex: 1;
  position: relative;
  padding-top: 8px;
  min-height: 36px;
}

.name-wrapper h3 {
  margin: 0;
  font-size: 15px;
  font-weight: 600;
  color: var(--text-primary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  letter-spacing: -0.2px;
}

.applied-badge-small {
  position: absolute;
  top: 0;
  right: 0;
  display: inline-flex;
  align-items: center;
  gap: 3px;
  padding: 2px 8px;
  border-radius: 10px;
  font-size: 10px;
  font-weight: 600;
  background: linear-gradient(135deg, #10b981 0%, #059669 100%);
  color: white;
  box-shadow: 0 2px 6px rgba(16, 185, 129, 0.25);
  letter-spacing: 0.3px;
}

.btn-delete {
  display: flex;
  align-items: center;
  justify-content: center;
  background: transparent;
  border: none;
  color: var(--text-tertiary);
  cursor: pointer;
  padding: 5px;
  border-radius: 8px;
  transition: all 0.15s ease;
  flex-shrink: 0;
  opacity: 0.6;
}

.btn-delete:hover {
  color: #ef4444;
  background: rgba(239, 68, 68, 0.1);
  opacity: 1;
}

.description {
  color: var(--text-secondary);
  margin: 0 0 14px 0;
  font-size: 13px;
  line-height: 1.5;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.item-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding-top: 12px;
  border-top: 1px solid var(--border-color);
  gap: 10px;
}

.footer-left {
  flex: 1;
  display: flex;
  gap: 6px;
  flex-wrap: wrap;
}

.footer-actions {
  display: flex;
  gap: 6px;
  opacity: 0;
  transform: translateX(-5px);
  transition: all 0.2s ease;
}

.environment-item:hover .footer-actions {
  opacity: 1;
  transform: translateX(0);
}

.btn-action {
  display: flex;
  align-items: center;
  gap: 5px;
  padding: 6px 12px;
  border: 1px solid var(--border-color);
  border-radius: 8px;
  cursor: pointer;
  font-size: 12px;
  font-weight: 500;
  transition: all 0.15s ease;
  background: var(--bg-tertiary);
  color: var(--text-primary);
  white-space: nowrap;
}

.btn-action:hover {
  transform: translateY(-1px);
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.1);
}

.btn-action.btn-edit {
  border-color: transparent;
  background: rgba(99, 102, 241, 0.08);
  color: #6366f1;
}

.btn-action.btn-edit:hover {
  background: rgba(99, 102, 241, 0.15);
  border-color: rgba(99, 102, 241, 0.2);
  box-shadow: 0 2px 8px rgba(99, 102, 241, 0.2);
}

.btn-action.btn-copy {
  border-color: transparent;
  background: rgba(16, 185, 129, 0.08);
  color: #10b981;
}

.btn-action.btn-copy:hover {
  background: rgba(16, 185, 129, 0.15);
  border-color: rgba(16, 185, 129, 0.2);
  box-shadow: 0 2px 8px rgba(16, 185, 129, 0.2);
}

.btn-action.btn-apply {
  border-color: transparent;
  background: linear-gradient(135deg, #6366f1 0%, #8b5cf6 100%);
  color: white;
  box-shadow: 0 2px 6px rgba(99, 102, 241, 0.25);
}

.btn-action.btn-apply:hover {
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(99, 102, 241, 0.35);
}

.btn-action:active {
  transform: translateY(0);
}

.btn-action svg {
  width: 13px;
  height: 13px;
}

.current-badge {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  padding: 4px 10px;
  border-radius: 6px;
  font-size: 11px;
  font-weight: 600;
  letter-spacing: 0.2px;
  white-space: nowrap;
  background: linear-gradient(135deg, #6366f1 0%, #8b5cf6 100%);
  color: white;
  box-shadow: 0 2px 8px rgba(99, 102, 241, 0.3);
}

.current-badge svg {
  width: 10px;
  height: 10px;
}
</style>
