import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/plugin-notification';
import { useConfigStore } from '../stores/config';

/**
 * 显示系统通知
 * @param title 通知标题
 * @param body 通知内容
 */
export async function showNotification(title: string, body: string) {
  const configStore = useConfigStore();

  // 检查用户是否启用了通知
  if (!configStore.settings?.showNotifications) {
    return;
  }

  try {
    // 检查通知权限
    let permission = await isPermissionGranted();

    // 如果没有权限，请求权限
    if (!permission) {
      const result = await requestPermission();
      permission = result === 'granted';
    }

    // 发送通知
    if (permission) {
      await sendNotification({
        title,
        body,
      });
    }
  } catch (error) {
    console.error('Failed to send notification:', error);
  }
}

/**
 * 显示成功通知
 */
export async function showSuccessNotification(message: string) {
  await showNotification('操作成功', message);
}

/**
 * 显示错误通知
 */
export async function showErrorNotification(message: string) {
  await showNotification('操作失败', message);
}
