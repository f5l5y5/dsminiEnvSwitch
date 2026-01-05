import { defineStore } from 'pinia';
import { invoke } from '@tauri-apps/api/core';
import type { AppConfig, Environment, AppSettings } from '../types/config';

export const useConfigStore = defineStore('config', {
  state: () => ({
    config: null as AppConfig | null,
    currentEnvironment: null as Environment | null,
    loading: false,
    error: null as string | null,
  }),

  getters: {
    environments: (state): Environment[] => state.config?.environments || [],
    settings: (state): AppSettings | undefined => state.config?.settings,
    currentEnvironmentId: (state): string | undefined => state.config?.settings.currentEnvironmentId,
    currentAppliedEnvironmentId: (state): string | undefined => state.config?.settings.currentAppliedEnvironmentId,
    currentAppliedEnvironment: (state): Environment | undefined => {
      const appliedId = state.config?.settings.currentAppliedEnvironmentId;
      return state.config?.environments.find((env) => env.id === appliedId);
    },
  },

  actions: {
    // 加载配置
    async loadConfig() {
      this.loading = true;
      this.error = null;
      try {
        const config = await invoke<AppConfig>('load_config');
        this.config = config;

        // 如果有当前环境ID,加载当前环境
        if (config.settings.currentEnvironmentId) {
          this.currentEnvironment = config.environments.find(
            (env) => env.id === config.settings.currentEnvironmentId
          ) || null;
        }
      } catch (err) {
        this.error = err as string;
        console.error('Failed to load config:', err);
      } finally {
        this.loading = false;
      }
    },

    // 保存配置
    async saveConfig() {
      if (!this.config) return;

      try {
        await invoke('save_config', { config: this.config });
      } catch (err) {
        console.error('Failed to save config:', err);
        throw err;
      }
    },

    // 添加环境
    async addEnvironment(env: Omit<Environment, 'id' | 'created_at' | 'updated_at'>) {
      try {
        const newEnv = await invoke<Environment>('add_environment', { environment: env });
        if (this.config) {
          this.config.environments.push(newEnv);
        }
        return newEnv;
      } catch (err) {
        console.error('Failed to add environment:', err);
        throw err;
      }
    },

    // 更新环境
    async updateEnvironment(id: string, updates: Partial<Environment>) {
      try {
        await invoke('update_environment', { id, environment: updates });
        if (this.config) {
          const index = this.config.environments.findIndex((env) => env.id === id);
          if (index !== -1) {
            this.config.environments[index] = {
              ...this.config.environments[index],
              ...updates,
            };
          }
        }
        await this.loadConfig();
      } catch (err) {
        console.error('Failed to update environment:', err);
        throw err;
      }
    },

    // 删除环境
    async deleteEnvironment(id: string) {
      try {
        await invoke('delete_environment', { id });
        if (this.config) {
          this.config.environments = this.config.environments.filter((env) => env.id !== id);
          if (this.currentEnvironment?.id === id) {
            this.currentEnvironment = null;
          }
        }
      } catch (err) {
        console.error('Failed to delete environment:', err);
        throw err;
      }
    },

    // 设置当前环境
    async setCurrentEnvironment(id: string) {
      if (!this.config) return;

      const env = this.config.environments.find((e) => e.id === id);
      if (env) {
        this.currentEnvironment = env;
        this.config.settings.currentEnvironmentId = id;
        await this.saveConfig();
      }
    },

    // 应用环境配置
    async applyEnvironment(envId: string) {
      try {
        const result = await invoke<string>('apply_environment', { envId });
        // 重新加载配置以获取更新后的 currentAppliedEnvironmentId
        await this.loadConfig();
        return result;
      } catch (err) {
        console.error('Failed to apply environment:', err);
        throw err;
      }
    },

    // 更新设置
    async updateSettings(settings: Partial<AppSettings>) {
      if (!this.config) return;

      this.config.settings = {
        ...this.config.settings,
        ...settings,
      };
      await this.saveConfig();
    },
  },
});
