import { createRouter, createWebHistory } from 'vue-router';
import Home from '../views/Home.vue';
import Login from '../views/Login.vue';
import Register from '../views/Register.vue';
import New from '../views/New.vue';
import View from '../views/View.vue';

const routes = [
  { path: '/', component: Home },
  { path: '/login', component: Login },
  { path: '/register', component: Register },
  { path: '/new', component: New },
  { path: '/view', component: View },
];

const router = createRouter({
  history: createWebHistory(),
  routes
});

export default router;
