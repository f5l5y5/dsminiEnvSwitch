// 环境配置
export interface Environment {
  id: string;                    // 环境唯一标识
  name: string;                  // 环境名称(如 "开发环境", "测试环境")
  desc?: string;                 // 环境描述
  ext: any;                      // 环境配置的 JSON 对象
  targetFilePath?: string;       // 替换文件内容的路径(可选，单独设置时优先使用此值)
  createdAt: string;             // 创建时间
  updatedAt: string;             // 更新时间
}

// 应用配置(完整的持久化数据)
export interface AppSettings {
  closeToTray: boolean;                  // 关闭时是否最小化到托盘
  currentEnvironmentId?: string;         // 当前选中的环境ID
  currentAppliedEnvironmentId?: string;  // 当前应用的环境ID（点击"应用"按钮后记录）
  autoBackup: boolean;                   // 应用配置前是否自动备份
  showNotifications: boolean;            // 是否显示通知
  externalConfigPath?: string;           // 外部配置文件路径
  targetFilePath?: string;               // 全局默认的替换文件内容路径(环境未单独设置时使用此值)
}

// 应用配置(完整的持久化数据)
export interface AppConfig {
  environments: Environment[];   // 所有环境配置
  settings: AppSettings;         // 应用设置
}
