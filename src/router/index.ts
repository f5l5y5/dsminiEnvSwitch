import { createRouter, createWebHistory } from 'vue-router';

const routes = [
  {
    path: '/',
    redirect: '/environments',
  },
  {
    path: '/environments',
    name: 'Environments',
    component: () => import('../views/EnvironmentsView.vue'),
  },
  {
    path: '/settings',
    name: 'Settings',
    component: () => import('../views/SettingsView.vue'),
  },
  {
    path: '/help',
    name: 'Help',
    component: () => import('../views/HelpView.vue'),
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
