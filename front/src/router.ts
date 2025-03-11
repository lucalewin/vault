import { createRouter, createWebHistory } from 'vue-router'
import HomeView from './views/HomeView.vue'
import PasswordView from './views/PasswordView.vue';
import AuthenticatorView from './views/AuthenticatorView.vue';
import LoginView from './views/LoginView.vue';
import CredentialView from './views/CredentialView.vue';
import NewView from './views/NewView.vue';

const routes = [
  {
    path: '/',
    name: 'home',
    component: HomeView,
  },
  {
    path: '/passwords',
    name: 'passwords',
    component: PasswordView,
  },
  {
    path: '/authenticator',
    name: 'authenticator',
    component: AuthenticatorView,
  },
  {
    path: '/login',
    name: 'login',
    component: LoginView,
  },
  {
    path: '/view',
    name: 'view',
    component: CredentialView,
  },
  {
    path: '/new',
    name: 'new',
    component: NewView,
  },
];

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
})

export default router
