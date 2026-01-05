<template>
  <div class="environment-list">
    <div v-if="environments.length === 0" class="empty-state">
      <p>还没有环境配置,点击右上角添加按钮创建第一个环境吧!</p>
    </div>
    <div v-else class="list">
      <EnvironmentItem
        v-for="env in sortedEnvironments"
        :key="env.id"
        :environment="env"
        :is-current="env.id === currentId"
        :is-applied="env.id === appliedId"
        @select="$emit('select', env.id)"
        @delete="$emit('delete', $event)"
        @copy="$emit('copy', env.id)"
        @edit="$emit('edit', env.id)"
        @apply="$emit('apply', env.id)"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import type { Environment } from '../../types/config';
import EnvironmentItem from './EnvironmentItem.vue';

const props = defineProps<{
  environments: Environment[];
  currentId?: string;
  appliedId?: string;
}>();

defineEmits<{
  select: [id: string];
  delete: [id: string];
  copy: [id: string];
  edit: [id: string];
  apply: [id: string];
}>();

// 已应用的环境排在最前面
const sortedEnvironments = computed(() => {
  if (!props.appliedId) {
    return props.environments;
  }
  const applied = props.environments.filter(e => e.id === props.appliedId);
  const others = props.environments.filter(e => e.id !== props.appliedId);
  return [...applied, ...others];
});
</script>

<style scoped>
.environment-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.empty-state {
  text-align: center;
  padding: 60px 20px;
  color: var(--text-secondary);
  background: var(--bg-tertiary);
  border-radius: var(--radius-lg);
  border: 2px dashed var(--border-color);
}

.empty-state p {
  margin: 0;
  font-size: 15px;
  line-height: 1.6;
}

.list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}
</style>
