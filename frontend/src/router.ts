import { createRouter, createWebHistory } from 'vue-router'
import HomeView from './views/HomeView.vue'
import AuthenticatorView from './views/authenticator/AuthenticatorView.vue';
import LoginView from './views/auth/LoginView.vue';
// password views
import PasswordView from './views/passwords/PasswordView.vue';
import CredentialView from './views/passwords/CredentialView.vue';
import NewView from './views/passwords/NewView.vue';

import IdentitiesView from './views/IdentitiesView.vue';
import CreditCardView from './views/CreditCardView.vue';
import SettingsView from './views/SettingsView.vue';

const routes = [
  {
    path: '/',
    component: HomeView,
  },
  {
    path: '/passwords',
    children: [
      { path: '', component: PasswordView },
      { path: 'view', component: CredentialView },
      { path: 'new', component: NewView },
    ]
  },
  {
    path: '/authenticator',
    component: AuthenticatorView,
  },
  {
    path: '/login',
    component: LoginView,
  },
  {
    path: '/identities',
    component: IdentitiesView,
  },
  {
    path: '/credit-cards',
    component: CreditCardView,
  },
  {
    path: '/profile/settings',
    component: SettingsView,
  },
];

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
})

export default router
