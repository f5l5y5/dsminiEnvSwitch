<template>
  <div class="help-view">
    <div class="help-header">
      <h1>
        <svg width="32" height="32" viewBox="0 0 32 32" fill="none" xmlns="http://www.w3.org/2000/svg">
          <circle cx="16" cy="16" r="13" stroke="currentColor" stroke-width="2.5"/>
          <path d="M12 12C12.3 11.1 12.9 10.4 13.7 9.9C17.3 9.9 20 12.6 20 16C20 19.4 17.3 22.1 13.7 22.1C13.2 22.1 12.7 22 12.2 21.9M12 12C11.7 12.8 11.5 13.6 11.5 14.6" stroke="currentColor" stroke-width="2.5" stroke-linecap="round"/>
        </svg>
        操作说明
      </h1>
      <p class="subtitle">了解如何使用环境配置管理工具</p>
    </div>

    <div class="help-content">
      <div class="help-category">
        <div class="category-icon">📦</div>
        <h3>环境管理</h3>
        <ul>
          <li><strong>添加环境</strong>：点击右上角"添加环境"按钮，填写环境名称、描述和配置JSON</li>
          <li><strong>编辑环境</strong>：点击环境卡片上的"编辑"按钮修改配置</li>
          <li><strong>复制环境</strong>：点击"复制"按钮快速创建环境的副本</li>
          <li><strong>删除环境</strong>：点击环境卡片右上角的删除按钮</li>
          <li><strong>查看详情</strong>：点击环境卡片查看完整配置信息</li>
        </ul>
      </div>

      <div class="help-category">
        <div class="category-icon">⚡</div>
        <h3>应用环境</h3>
        <ul>
          <li><strong>应用配置</strong>：点击"应用"按钮将环境的JSON配置写入目标文件</li>
          <li><strong>目标文件路径</strong>：
            <ul>
              <li>在设置中配置全局默认路径</li>
              <li>在环境配置中单独设置路径（优先级更高）</li>
            </ul>
          </li>
          <li><strong>自动备份</strong>：启用后，应用配置前会自动备份原文件（文件名后加时间戳）</li>
          <li><strong>智能检测</strong>：如果文件内容已是最新，会提示无需更新</li>
        </ul>
      </div>

      <div class="help-category">
        <div class="category-icon">🔄</div>
        <h3>环境切换</h3>
        <ul>
          <li><strong>当前环境</strong>：在列表中点击环境卡片，紫色边框表示当前选中</li>
          <li><strong>已应用环境</strong>：绿色边框和"已应用"徽章表示此环境当前正在使用</li>
          <li><strong>托盘切换</strong>：右键系统托盘图标 → 切换环境 → 选择环境（勾选的为当前已应用环境）</li>
        </ul>
      </div>

      <div class="help-category">
        <div class="category-icon">💾</div>
        <h3>配置同步</h3>
        <ul>
          <li><strong>本地同步</strong>：导入配置文件后，所有变更会自动同步到该文件</li>
          <li><strong>远程同步</strong>：
            <ul>
              <li>在设置页面"远程配置同步"中输入远程配置 URL</li>
              <li>点击"同步"按钮从远程获取环境配置</li>
              <li>已存在的环境会更新，新环境会自动添加</li>
              <li>URL 会自动保存，下次可快速再次同步</li>
              <li>仅同步 environments 配置，不影响其他设置</li>
            </ul>
          </li>
          <li><strong>手动备份</strong>：随时可以导出配置文件进行备份</li>
          <li><strong>配置格式</strong>：JSON格式，可用文本编辑器直接编辑</li>
        </ul>
      </div>

      <div class="help-category">
        <div class="category-icon">🎯</div>
        <h3>快捷操作</h3>
        <ul>
          <li><strong>单击托盘图标</strong>：显示主窗口并聚焦</li>
          <li><strong>右键托盘图标</strong>：显示托盘菜单，可快速切换环境、查看当前环境、显示窗口或退出应用</li>
          <li><strong>关闭窗口</strong>：启用"最小化到托盘"后，关闭窗口会隐藏到系统托盘而非退出</li>
        </ul>
      </div>

      <div class="help-tips">
        <div class="tips-header">
          <svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M12 2L15.09 8.26L22 9.27L17 14.14L18.18 21.02L12 17.77L5.82 21.02L7 14.14L2 9.27L8.91 8.26L12 2Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
          <h4>实用提示</h4>
        </div>
        <div class="tips-content">
          <p>💡 <strong>配置灵活性</strong>：配置文件的 ext 字段包含完整的 JSON 配置，支持任意嵌套结构</p>
          <p>💡 <strong>多项目管理</strong>：为常用环境设置不同的 targetFilePath，可同时管理多个项目的配置</p>
          <p>💡 <strong>快速切换</strong>：使用系统托盘右键菜单可以快速切换环境，勾选项表示当前已应用的环境，无需打开主窗口</p>
          <p>💡 <strong>安全备份</strong>：建议启用自动备份功能，防止配置错误导致的问题</p>
          <p>💡 <strong>远程同步</strong>：使用远程同步功能可以快速获取团队共享的环境配置，保持团队环境一致性</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
</script>

<style scoped>
.help-view {
  padding: 32px;
  max-width: 1200px;
  margin: 0 auto;
}

.help-header {
  text-align: center;
  margin-bottom: 48px;
}

.help-header h1 {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 16px;
  margin: 0 0 16px 0;
  font-size: 36px;
  font-weight: 700;
  color: var(--text-primary);
  letter-spacing: -1px;
}

.help-header h1 svg {
  color: var(--primary-color);
}

.subtitle {
  margin: 0;
  font-size: 18px;
  color: var(--text-secondary);
  font-weight: 400;
}

.help-content {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
  gap: 28px;
}

.help-category {
  background: var(--bg-secondary);
  border-radius: var(--radius-xl);
  padding: 28px;
  border: 1px solid var(--border-color);
  box-shadow: var(--shadow-md);
  transition: all var(--transition-base);
}

.help-category:hover {
  transform: translateY(-4px);
  box-shadow: var(--shadow-lg);
  border-color: var(--primary-color);
}

.category-icon {
  font-size: 36px;
  margin-bottom: 16px;
}

.help-category h3 {
  margin: 0 0 20px 0;
  font-size: 20px;
  font-weight: 700;
  color: var(--text-primary);
  display: flex;
  align-items: center;
  gap: 10px;
  padding-bottom: 16px;
  border-bottom: 2px solid var(--border-color);
}

.help-category ul {
  margin: 0;
  padding-left: 24px;
  list-style: none;
}

.help-category > ul > li {
  margin-bottom: 14px;
  padding-left: 8px;
  position: relative;
  color: var(--text-secondary);
  font-size: 15px;
  line-height: 1.7;
}

.help-category > ul > li::before {
  content: '•';
  position: absolute;
  left: 0;
  color: var(--primary-color);
  font-weight: bold;
  font-size: 18px;
}

.help-category ul ul {
  margin-top: 10px;
  padding-left: 20px;
}

.help-category ul ul li {
  font-size: 14px;
  color: var(--text-tertiary);
  margin-bottom: 8px;
}

.help-category strong {
  color: var(--text-primary);
  font-weight: 600;
}

.help-tips {
  grid-column: 1 / -1;
  margin-top: 16px;
  padding: 32px;
  background: linear-gradient(135deg, rgba(99, 102, 241, 0.08) 0%, rgba(139, 92, 246, 0.06) 100%);
  border-radius: var(--radius-xl);
  border: 2px solid rgba(99, 102, 241, 0.15);
  box-shadow: var(--shadow-md);
}

.tips-header {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 20px;
}

.tips-header svg {
  color: #fbbf24;
  flex-shrink: 0;
}

.tips-header h4 {
  margin: 0;
  font-size: 20px;
  font-weight: 700;
  color: var(--text-primary);
}

.tips-content p {
  margin: 0;
  padding: 12px 0;
  font-size: 15px;
  line-height: 1.7;
  color: var(--text-secondary);
}

.tips-content p:first-child {
  padding-top: 0;
}

.tips-content p:last-child {
  padding-bottom: 0;
}

/* 响应式布局 */
@media (max-width: 900px) {
  .help-content {
    grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
    gap: 20px;
  }
}

@media (max-width: 768px) {
  .help-view {
    padding: 20px;
  }

  .help-header {
    margin-bottom: 32px;
  }

  .help-header h1 {
    font-size: 28px;
    flex-wrap: wrap;
  }

  .subtitle {
    font-size: 16px;
  }

  .help-content {
    grid-template-columns: 1fr;
    gap: 16px;
  }

  .help-category {
    padding: 20px;
  }

  .category-icon {
    font-size: 28px;
    margin-bottom: 12px;
  }

  .help-category h3 {
    font-size: 18px;
    margin-bottom: 16px;
    padding-bottom: 12px;
  }

  .help-category > ul > li {
    font-size: 14px;
  }

  .help-tips {
    padding: 24px;
  }

  .tips-header h4 {
    font-size: 18px;
  }

  .tips-content p {
    font-size: 14px;
    padding: 10px 0;
  }
}

@media (max-width: 480px) {
  .help-view {
    padding: 16px;
  }

  .help-header h1 {
    font-size: 24px;
    gap: 12px;
  }

  .help-header h1 svg {
    width: 24px;
    height: 24px;
  }

  .subtitle {
    font-size: 14px;
  }

  .help-category {
    padding: 16px;
  }

  .help-category h3 {
    font-size: 16px;
  }

  .help-tips {
    padding: 20px;
  }

  .tips-header h4 {
    font-size: 16px;
  }

  .tips-content p {
    font-size: 13px;
  }
}
</style>
