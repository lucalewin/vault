<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
import DashboardLayout from './layouts/DashboardLayout.vue';
import SidebarItem from '@/components/SidebarItem.vue';
</script>

<template>
  <DashboardLayout>
    <template #sidebar_items>
      <SidebarItem title="Home" to="/" />
      <SidebarItem title="Passwords" to="/passwords" />
      <SidebarItem title="Authenticator" to="/authenticator" />
      <SidebarItem title="Login" to="/login" />
    </template>

    <RouterView />
  </DashboardLayout>
</template>

<style scoped>

</style>
