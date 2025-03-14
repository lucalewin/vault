<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
import DashboardLayout from './layouts/DashboardLayout.vue';
import SidebarItem from '@/components/SidebarItem.vue';
import { ref } from 'vue';

const isTokenExpired = (token: string): boolean => Date.now() >= (JSON.parse(atob(token.split('.')[1]))).exp * 1000;
const api_token = localStorage.getItem('api_token');
const isLoggedIn = ref(api_token && !isTokenExpired(api_token));

</script>

<template>
  <div class="bg-neutral-800 text-white">
    <DashboardLayout>
      <template #sidebar_items>
        <SidebarItem title="Overview" icon="home" to="/" />
        <SidebarItem title="Passwords" icon="lock" to="/passwords" />
        <SidebarItem title="Authenticator" icon="key" to="/authenticator" />
        <SidebarItem title="Credit Cards" icon="credit-card" to="/credit-cards" />
        <SidebarItem title="Identities" icon="id-card" to="/identities" />
      </template>
      <template #sidebar_items_bottom>
        <SidebarItem title="Login" icon="sign-in" to="/login" v-if="!isLoggedIn" />
        <SidebarItem title="Log out" icon="sign-out" to="/logout" v-else />
        <SidebarItem title="Settings" icon="cog" to="/profile/settings" />
      </template>
  
      <RouterView />
    </DashboardLayout>
  </div>
</template>

<style scoped>

</style>
